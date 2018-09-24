# Controller

[![Build Status](https://travis-ci.org/Bitspleaseee/controller.svg?branch=master)](https://travis-ci.org/Bitspleaseee/controller)

# Docker

## Run controller and database

```bash
$ docker-compose up
```

## Setup database for local development

```bash
$ docker-compose up db
$ # Set the IP of the database URL to the returned IP address
$ docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' <name-of-container>
```
