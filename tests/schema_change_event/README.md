# Schema change event test
A test which tests that changing the schema creates the proper event

* Start the cluster
* Connect the driver to the cluster: `./connect_wait.sh`
* Generate a schema change event: `./change_schema.sh`
* Now the connected driver should show a log of this event in its logs
* Stop the driver (Ctrl + C)
* Stop the cluster
