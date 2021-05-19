# Remove node event test
A test which tests that removing a node creates the proper event

* Start the cluster
* Connect the driver to the cluster: `./connect_wait.sh`
* Remove node2 (Requires sudo or docker group): `sudo ./remove_node.sh`
* Now the connected driver should show a log of this event in its logs
* Stop the driver (Ctrl + C)
* Stop the cluster 
