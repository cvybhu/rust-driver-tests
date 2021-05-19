# Add Node event test
A test which tests that adding a new node creates the proper event

* Start the cluster
* Connect the driver to the cluster: `./connect_wait.sh`
* Add new node (Requires sudo or docker group): `sudo ./add_node.sh`
* Now the connected driver should show a log of this event in its logs
* Stop the driver (Ctrl + C)
* Stop the cluster
* Stop the added node (Ctrl + C)
