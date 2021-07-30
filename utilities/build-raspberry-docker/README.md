# Build qaul.net for Raspberry Pi using Docker

You can build the binaries for Rasperry Pi on 
your computer using a presetup docker container 
to build it easily.


## Prerequisites

* install docker
* install docker-compose


## Build Docker Container

To be able to use the docker container to build
qaul.net you first have to build the docker container.

```sh
# build docker container
docker-compose build
```

## Build Binary for Raspberry Pi

Now you can start the docker container to build qaul.net for 
raspberry pi.

```sh
# Build qaul.net for raspberry pi
docker-compose up
```

When you start the container, it automatically 
builds this repository for Raspberry Pi and closes the container after the build.
You can see all the build messages in your terminal.

The binaries can be found in the folder `target/armv7-unknown-linux-gnueabihf/debug`, which is located in the root folder of this repository.


## Build for Release Target

If you want to build the binaries for release, uncomment the following line 
in docker-compose.yml file:

```
command: "cargo build --release --target=armv7-unknown-linux-gnueabihf"
```


## Build User

These builds are made for the default linux users with the user_id:group_id 1000:1000.
If you have a different user, you can change the user id and group id in the docker-compose.yml file. Uncomment the following line in the docker-compose.yml file and change it to the required user_id:group_id.

```
user: 1000:1000
```