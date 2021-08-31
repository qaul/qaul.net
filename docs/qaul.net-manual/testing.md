# Test qaul application

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
