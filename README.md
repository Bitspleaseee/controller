# Controller

[![Build Status](https://travis-ci.org/Bitspleaseee/controller.svg?branch=master)](https://travis-ci.org/Bitspleaseee/controller)

## Docker setup

```bash
$ docker-compose up
$ # Set the IP of the database URL to the returned IP address
$ docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' <name-of-container>
```
