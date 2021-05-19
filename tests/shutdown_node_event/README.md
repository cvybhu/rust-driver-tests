# Shutdown node test
A test which tests that shutting down a node creates the proper event

* Start the cluster
* Connect the driver to the cluster: `./connect_wait.sh`
* Shutdown node2 (Requires sudo or docker group): `sudo ./shutdown_node.sh`
* Now the connected driver should show a log of this event in its logs
* Stop the driver (Ctrl + C)
* Stop the cluster
