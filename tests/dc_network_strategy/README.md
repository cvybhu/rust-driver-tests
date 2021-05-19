# DCAware Network Topology Strategy test
This test ensures that when using a DCAware strategy all queries are sent to the chosen datacenter

* Start the cluster
* Run the test: `./dc_network_strategy.sh`
* Look at queries sent after the line: `STARTING NETWORK_TOPOLOOGY_STRATEGY LOAD BALANCED QUERIES`
* They should be sent mainly to nodes with adresses `.11`, `.12` and `.13`
* Stop the cluster
