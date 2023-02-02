# Self-hosting a qauld instance - your own community node

This image is meant as a helper to get you started in hosting your own qaul.net server.
To know more about what is qauld and what this docker image encapsulates,
read [How to run a qaul community node](https://qaul.net/tutorials/community-node).

There are two ways of running it: using **docker** or through **docker-compose**. We recommend the latter.

### Using docker-compose

A [`docker-compose`](https://github.com/qaul/qaul.net/utilities/qauld-docker/docker-compose.yml) file is available as an example.

Before running it, change the place where the internal qauld configuration is bound to on your host:
`/your/path/to/config:/srv/qaul`

`srv/qaul` is where the server configuration - such as the `config.yaml` file, the database and so on - will be stored
inside the docker container. Binding it to your host machine ensures no data will be lost, should you inadvertently delete the container.

You can also change the port on your host that is bound to the internal `qauld` port (`9229` by default).

```yaml
ports:
  # This reads "Port 8778 on HOST points to Port 9229 on CONTAINER"
  - "8778:9229"
```

Once done, launch the container:
```shell
cd /path/to/docker-compose.yml
docker-compose up --detach
```

### Using docker
You can create a new container by running:
```shell
docker run \
  --detach \
  --publish 8778:9229 \
  --volume /your/path/to/config:/srv/qaul \
  --restart=always \
  qaul.net/qauld
```
