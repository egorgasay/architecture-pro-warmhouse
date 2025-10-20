CREATE TABLE sensor_data (
    id SERIAL PRIMARY KEY,
    sensor_id VARCHAR NOT NULL,
    value VARCHAR NOT NULL,
    ts VARCHAR NOT NULL
);