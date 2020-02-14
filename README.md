# Splitterrust-Discord

This repository contains the code for the Discord bot, which uses the
[Splitterrust-Server](https://github.com/splitterrust/server) to
provide roleplaying capabilities.

## Features

- [ ] Roll any dice with either `<count>d<faces>` or `<anzahl>w<augen>`
- [ ] Print information for a spell
- [ ] More comming soon^TM
- [ ] Import Character from
  [Splitterrust-UI](https://github.com/splitterrust/ui)

## Docker

### Building

To build the release version:
```
$ docker buid splitterrust_discord:latest .
```

To run it:
```
$ docker run -p 8088:8088 -e \
    DISCORD_TOKEN="asdf;dfkajdf;lahdf;asdh" -e \
    BACKEND_SERVER="http://127.0.0.1:8088" splitterrust_discord:latest
```

### Environment

#### `DISCORD_TOKEN` (required)

The token of the Discord bot, which will be used to connect.

```
DISCORD_TOKEN="a;dkf;ahf;djkf"
```

#### `BACKEND_SERVER` (optional)

If you're also running `splitterrust_server` you can specify the URL to that
service here. If this is not specified thinks like `get_spell` will raise an
exception, but the dice bot should work without that.

```
BACKEND_SERVER="http://127.0.0.1:8088"
```

#### `RUST_LOG`

Log level for the application.

Set everythig to one level:
```
RUST_LOG="debug"
```

Set just splitterrust_server to a level:

```
splitterrust_discord=debug
```

Set multiple level:
```
splitterrust_discord=debug,tokio_reactor=debug
```

### docker-compose

There is an example `docker-compose.yml` which will build a complete stack of
server + database + discord.

If you're like me and your running a VPN append this to the end of the
`docker-compose.yml`:

```
networks:
  default:
      external:
        name: my-network
```

And run the following with the VPN off:

```
docker network create my-network --subnet 172.24.24.0/24
```

Also specify the network for each service:

```
    networks:
      - default
```
