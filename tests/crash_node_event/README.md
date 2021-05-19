# Crash node event test
A test which tests that crashing a node creates the proper event

* Start the cluster
* Connect the driver to the cluster: `./connect_wait.sh`
* Shutdown node2 (Requires sudo or docker group): `sudo ./crash_node.sh`
* Now the connected driver should show a log of this event in its logs
* Stop the driver (Ctrl + C)
* Stop the cluster
