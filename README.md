# Controller

[![Build Status](https://travis-ci.org/Bitspleaseee/controller.svg?branch=master)](https://travis-ci.org/Bitspleaseee/controller)

## Docker setup

```bash
$ docker-compose up
$ IP=`docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' controller_db_1`
$ source .env
$ # The 'DATABASE_URL' should be changed to be:
$ echo $DATABASE_URL_TMPL
```
