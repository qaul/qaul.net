# Configuration

In order for `qaul-hubd` to work properly it will have to run in the
background to handle incoming and outgoing network connections.  It's
recommended to launch it via a user systemd unit.

```systemd
[Unit]
Description=qaul.net hub daemon
After=network.target

[Service]
Type=simple
ExecStart=$HOME/bin/qaul-hubd <your parameters here>
```

Save this file in `~/.local/share/systemd/user/`

Now you can reload the daemon and start the unit.

```console
$ systemctl daemon-reload --user
$ systemctl enable --user qaul-hubd.service
$ systemctl start --user qaul-hubd.service
```


## Available configuration

Following is a list of qaul-hubd configuration values.  Those marked
with a `*` are mandatory.  Commandline arguments take precedence over
environment variables.

| ENV variable | Runtime argument | Description |
|----------------------|---------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `*` HUBD_PEERS=[PATH]    | -P / --peers [PATH] | Specify the path to a peer file, containing a newline-separated list of peers to connect to                                                                    |
| `*` HUBD_PORT=[PORT]     | -p / --port [PORT]  | Specify a tcp port to which qaul-hubd should bind itself to listen for incoming network traffic                                                                |
| HUBD_UDP_DISCOVERY=0 | --no-udp-discover   | Prevent qaul-hubd from registering a multicast address to find other clients on the same network.  Some networks may forbid this, or cause performance issues. |
| HUBD_SETUP_UPNP=0    | --no-upnp           | Disable automatic UPNP port forwarding.  Some networks may forbid this, or cause performance issues.                                                           |
| HUBD_RUN_MODE=[MODE] | -m / --mode [MODE]  | Specify the peering mode of this hub.  Possible values: "static", "dynamic"                                                                                    |
| HUBD_ADDR=[ADDR]     | -a / --addr [ADDR]  | A valid address to bind to.  Must be a valid ip address format.                                                                                                |

