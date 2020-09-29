# qaul-hubd

This is a multi purpose router and database daemon for qaul.net.  It
handles network driver and user state as a detached process, and
allows both http and unix ipc clients to connect to it.

It doesn't come with it's own user interface, which means you will
have to build one separately and configure it to connect to your
`qaul-hubd` instance.

Because all networking is done in userspace, no root access is
required.
