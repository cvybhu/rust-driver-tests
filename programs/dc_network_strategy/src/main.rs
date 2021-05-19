// This program tests load balancing using the DCAwareTokenAware load balancing
// All queries should be sent to nodes in DC1

use scylla::{Session, SessionBuilder};
use scylla::transport::load_balancing::{DcAwareRoundRobinPolicy, TokenAwarePolicy};
use std::sync::Arc;
use scylla::statement::Consistency;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let local_dc: String = "DC1".to_string();
    let dc_robin = Box::new(DcAwareRoundRobinPolicy::new(local_dc));
    let policy = Arc::new(TokenAwarePolicy::new(dc_robin));

    let session: Session = SessionBuilder::new()
        .known_node("172.18.0.11:9042")
        .load_balancing(policy)
        .build()
        .await.unwrap();

    session.query("CREATE KEYSPACE IF NOT EXISTS net_ks WITH REPLICATION = {'class' : 'NetworkTopologyStrategy', 'DC1' : 2, 'DC2': 2}", &[]).await.unwrap();
    session.await_schema_agreement().await.unwrap();

    session.query("CREATE TABLE IF NOT EXISTS net_ks.tab (a int primary key)", &[]).await.unwrap();
    session.await_schema_agreement().await.unwrap();

    let session = Arc::new(session);

    let mut prepared_insert = session.prepare("INSERT INTO net_ks.tab (a) VALUES(?)").await.unwrap();
    prepared_insert.set_consistency(Consistency::LocalQuorum);
    prepared_insert.set_tracing(true);
    let prepared_insert = Arc::new(prepared_insert);

    let mut prepared_select = session.prepare("SELECT a FROM net_ks.tab").await.unwrap();
    prepared_select.set_consistency(Consistency::LocalQuorum);
    prepared_select.set_tracing(true);
    let prepared_select = Arc::new(prepared_select);

    println!("\nSTARTING NETWORK_TOPOLOOGY_STRATEGY LOAD BALANCED QUERIES\n");

    let mut handles = Vec::new();
    for insert_int in 0..100_i32 {
        let session = session.clone();
        let prepared_insert = prepared_insert.clone();
        let prepared_select = prepared_select.clone();

        handles.push(tokio::spawn(async move {
            session.execute(&prepared_insert, (insert_int,)).await.unwrap();
            session.execute(&prepared_select, &[]).await.unwrap();
        }));
    }

    for h in handles {
        h.await.unwrap();
    }
}
