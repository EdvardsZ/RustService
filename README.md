# RustService

## Description

A very simple app that consumes real-time weather data every second from this site:

[open-meteo.com](https://open-meteo.com/en/docs)

By default, it gets weather from Latitude: 52.52, Longitude: 13.41 (Just only this location is used because of the limitation of the API and for it to be simpler).

It receives current weather metrics like temperature, relative_humidity, wind_speed.

The app calculates the "dew point" and pushes the calculation together with the real-time data to a simple Vector.

There is a second task which fires every 10 seconds, which writes the contents of the Vector to a database.

## Run it locally

1. Startup the Postgres database and pgAdmin

```
docker-compose up
```

2. Run the service
```
cargo run
```
## Run it in a docker container

To run the service and Postgres database with pgAdmin
```
docker compose --profile release up
```

## View the calculations table in pgAdmin

1. After starting up the service go to [localhost:8888](http://localhost:8888/login)
2. Sign in: ```admin@admin.com```, pwd```admin```
3. Add server with host name, username, pwd: ```admin```.
4. View 'rustservice' db








