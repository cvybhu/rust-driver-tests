#!/bin/bash
docker run --rm -it \
    -v $(pwd)/../../cluster/config/dc1.properties:/etc/scylla/cassandra-rackdc.properties:ro \
    -v $(pwd)/../../cluster/config/scylla.yaml:/etc/scylla/scylla.yaml:ro \
    --network cluster_cluster-net \
    --ip 172.18.0.17 \
    --name added_node \
    scylladb/scylla --seeds=node1,node3 --smp 4 --memory 750M --api-address 0.0.0.0 \
