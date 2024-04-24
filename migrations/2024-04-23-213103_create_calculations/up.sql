-- Your SQL goes here
CREATE TABLE calculations (
    id SERIAL PRIMARY KEY,
    temperature FLOAT NOT NULL,
    wind_speed FLOAT NOT NULL,
    relative_humidity FLOAT NOT NULL,
    dew_point FLOAT NOT NULL
);
