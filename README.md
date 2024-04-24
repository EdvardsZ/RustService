# RustService

## Description

A very simple app that consumes real-time weather data every second from this site:

[open-meteo.com](https://open-meteo.com/en/docs)

By default, it retrieves weather information from the location with Latitude: 52.52 and Longitude: 13.41. This specific location is chosen due to API limitations and for simplicity.


The app collects current weather metrics such as temperature, relative humidity, and wind speed. Additionally, it calculates the "dew point" and includes this calculation along with the real-time data in a simple Vector.

There is a second task which fires every 10 seconds, writing the contents of the Vector to a database.


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








