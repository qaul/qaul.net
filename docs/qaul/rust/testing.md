# Test qaul application

## Display Rust Logs

To display rust logs you can set the log level as an environment variable.

There are the following log levels:

* error - log level displayed by default
* warn
* info
* debug
* trace

You can can set the log level for the run of the application directly
on the command line when starting the program:

```sh
# start qaul-cli with log level info
RUST_LOG=info ./qaul-cli
```

For permanently seeing the rust logs of a certain level, you can 
set the log level as an environment variable of your system:

```sh
# set the log level to info
export RUST_LOG=info

# show a backtrace in case of application crash
export RUST_BACKTRACE=1
```

## Monitor Traffic

Monitor IP traffic with the network analysis program wireshark.

Here some handy filter options to see qaul related traffic:

* Filter for remote peers
  * `tcp.port eq 9229 and ip.dst==144.91.74.192`


Analyze open ports on Linux

```sh
# list local ports programs listen to
sudo lsof -i -P -n | grep LISTEN

# list all active firewall rules on linux
sudo /sbin/iptables -S
```
