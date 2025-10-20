-- Drop the existing sensor_data table and recreate it with the correct schema
DROP TABLE sensor_data;

CREATE TABLE sensor_data (
    id SERIAL PRIMARY KEY,
    value VARCHAR NOT NULL,
    status VARCHAR NOT NULL,
    ts TIMESTAMP NOT NULL DEFAULT NOW()
);