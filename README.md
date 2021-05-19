# Scylla Rust Driver manual tests
This repository contains manual tests for [scylla-rust-driver](https://github.com/scylladb/scylla-rust-driver)

## Contents:
* `tests` - contains individual tests along with instructions how to preform them
* `programs` - contains Rust programs which use the driver to perform tests
* `cluster` - contains `docker-compose` files which allow to easily start a test scylla cluster
* `tls_cluster` - contains `docker-compose` files which allow to easily start a test scylla cluster with TLS enabled
* `scylla-monitoring` - contains configured [`scylla-monitoring`](https://github.com/scylladb/scylla-monitoring) service which allows to view graphs of events happening in the `cluster`
* `scylla-rust-driver` - contains modified driver which has more logs than the normal one

## Cluster
Starting:
* Enter the `cluster` directory
* Run `sudo docker-compose up -d`
* Wait a moment - once the cluster is ready running `sudo ./status.sh` should show a list of 6 nodes

Stopping:
* Enter the `cluster` directory
* Run `sudo docker-compose down`

## Scylla-Monitoring
Starting:
* Enter the `scylla-monitoring` directory
* Run `sudo ./start-all.sh -l`
* Open [http://localhost:3000/d/cql-4-3/scylla-cql?orgId=1&refresh=5s](http://localhost:3000/d/cql-4-3/scylla-cql?orgId=1&refresh=5s) in your browser

Stopping:
* Enter the `scylla-monitoring` directory
* Run `sudo ./kill-all.sh`
