UNIX ENVIRONMENT:

systemctl start bitcommit
		    |
	/usr/sbin/bitcommit (argv[0])
			|
			|    <-- /etc/bitcommit/(config)
			|
	-----------------------------------------
	|	|	|	|	|	|
webserver   rsync  container



CARGO ENVIRONMENT:
 .../BitCommit/bitcommit/build.rs
			 Cargo.lock
 			 Cargo.toml
			 libs/container/ 	//container library
			      rsync/ 		//rsync implementation
			      webserver/ 	//webserver
			      tests/ 		//tests for libs
			Makefile
			README.md
			src/
			STRUCTURE.md
