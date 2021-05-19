#!/bin/bash

# Choose a random keyspace name
KS_NAME=$(cat /dev/urandom | tr -cd a-z | head -c 16)

echo "Creating keyspace: $KS_NAME..."

# Create a keyspace with this name on node2
cqlsh 172.18.0.12 -e "CREATE KEYSPACE $KS_NAME WITH REPLICATION = {'class' : 'SimpleStrategy', 'replication_factor' : 1}"
