// This program tests awaiting schema agreement by rapidly creating new tables and selecting from them
// This should break without awaiting schema agreement
// TODO: It seems to work even without schema agreement :/
//       Creating tables takes a long time
use rand::Rng;
use scylla::{Session, SessionBuilder};

#[tokio::main]
async fn main() {
    let session1: Session = SessionBuilder::new()
        .known_node("172.18.0.11:9042")
        .build()
        .await
        .unwrap();
    let session2: Session = SessionBuilder::new()
        .known_node("172.18.0.13:9042")
        .build()
        .await
        .unwrap();

    session1.query("CREATE KEYSPACE IF NOT EXISTS ks WITH REPLICATION = {'class' : 'SimpleStrategy', 'replication_factor' : 1}", &[]).await.unwrap();
    session2.await_schema_agreement().await.unwrap();

    // Create table on session1, then on session2 await agreement and select from this table
    for _ in 0..10_i32 {
        let mut table_name: String = String::new();
        for _ in 0..16_i32 {
            let c: char = rand::thread_rng().gen_range('a'..='z');
            table_name.push(c);
        }

        println!("Creating table ks.{}", table_name);

        session1
            .query(
                format!(
                    "CREATE TABLE IF NOT EXISTS ks.{} (a int primary key)",
                    table_name
                ),
                &[],
            )
            .await
            .unwrap();

        session2.await_schema_agreement().await.unwrap();
        session2
            .query(format!("SELECT * FROM ks.{}", table_name), &[])
            .await
            .unwrap();
    }

    println!("OK");
}
