# qaul.net documentation

This section outlines various pieces of the qaul.net project that
aren't directly code related.


## Manuals

Because this manual is part of the main source repository, you can
build it from the same environment as the main code.  If you use [nix]
to scope dependencies, you can simply run `mdbook serve` to build and
serve the built html files.

```console
$ cd docs/contributors
$ mdbook serve
```

If you have access to the qaul.net servers, you can run `./deploy.sh`
at any level to recursively deploy sections to production.


## Websites

qaul.net runs many different web services:

* [qaul.net](https://qaul.net) the qaul.net web site
  * [contributor manual](https://docs.qaul.net/contributors) (this document)
  * [user manual](https://docs.qaul.net/users)
  * [http-api](https://docs.qaul.net/http-api) qaul.net REST API guide
  * [Rust documentaiion](https://docs.qaul.net/api) the qaul.net rust software API documentation (automatically created from the qaul.net code sources)
* [get.qaul.net](https://get.qaul.net) the qaul.net download directory for the qaul.net binaries and big content files (e.g. videos, etc.).

This chapter explains how they are hosted, updated, where to look to
edit and change them and who to contact when the service is not
working or you would like to have access to it.


### qaul.net Web Site

There is an [own chapter] in this guide on the editing of the qaul.net
web site.  Please have a look there on how to edit and translate the
web site.

* Server: https://qaul.net
* Source repository: https://git.open-communication.net/qaul/website
* Updated: by deploy script
* Admin contact: contact@qaul.net

[own chapter]: /website


### docs.qaul.net Documentation

The software documentation & guides of qaul.net.

* Server: https://docs.qaul.net
* Source repository: https://git.open-communication.net/qaul/qaul.net/tree/master/docs/
* Updated: by deploy script
* Admin contact: contact@qaul.net


### get.qaul.net Download Directory

The Download server for the binary builds and big content files such
as videos etc.

* Server: https://get.qaul.net
* Updated:
  * the builds are uploaded by CI
  * content is uploaded manually by the administrators
* Admin contact: contact@qaul.net
