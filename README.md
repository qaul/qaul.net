![http://qaul.net/](doc/qaul-net.png)

# qaul.net [![pipeline status](https://gitlab.com/qaul/qaul-net/badges/master/pipeline.svg)](https://gitlab.com/qaul/qaul-net/commits/master)

**qaul.net** is an ad-hoc wireless mesh-network suite that harnesses the 
power of everyday devices such as computers and smartphones to create a 
**Democratic**, **Non-Centralized**, **Mesh Network** on which users can 
share files, have voice chats and send each other messages, however the 
power of qaul.net allows an endless feature set as anyone can run any 
LAN-based application over the network. qaul.net removes the dependence 
on the centralized services such as the Internet and cellular networks 
and creates a network that anyone can be part of and share freely with 
no censorship what so ever.

qaul.net has the following features:

* File sharing (both private and public)
* Voice calling
* Text messaging (both private and public)
* Sharing your Internet connection (via a given interface) to the whole mesh
* Custom network settings (to setup your own **personal** mesh)
* Viewing of the whole mesh as a diagram
* Ability to run `olsr` (the mesh routing procotol) on any interface (wifi, ethernet)
* And because this is a mesh-**network** you can run any service you want on your node and it will be available on the network (Web server, Mail server, SSH server, FTP server etc.)

## Development status

The project is currently being re-written for a more modular and portable approach.
If you want to get involved, please [get in touch]()!

For the latest stable release, check the [`release-1.0.0`][release] branch.

[release]: https://github.com/qaul/qaul.net/tree/release-1.0.0

## Build Instructions

The project is being re-written in Rust, thus using [cargo][cargo] as a build system.
If you don't have Rust installed, you can get it [here](https://rustup.sh) or via your OS.

## License

qaul.net is free open source software licensed under the 
[GNU General Public License version 3](Licenses/GPLv3.txt).

To see all external code's licenses used in this project please 
visit the [License directory](Licenses).
