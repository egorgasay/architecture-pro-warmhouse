-- Create the database if it doesn't exist, ignore error if already exists
DO $$
BEGIN
    PERFORM 1 FROM pg_database WHERE datname = 'smarthome';
    IF NOT FOUND THEN
        CREATE DATABASE smarthome;
    END IF;
    PERFORM 1 FROM pg_database WHERE datname = 'statemonitoring';
    IF NOT FOUND THEN
        CREATE DATABASE statemonitoring;
    END IF;
EXCEPTION WHEN others THEN
    -- Ignore error if database exists or other error occurs
    NULL;
END
$$;

-- Connect to the smarthome database
\c smarthome;

-- Create the sensors table
CREATE TABLE IF NOT EXISTS sensors (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    type VARCHAR(50) NOT NULL,
    location VARCHAR(100) NOT NULL,
    value FLOAT DEFAULT 0,
    unit VARCHAR(20),
    status VARCHAR(20) NOT NULL DEFAULT 'inactive',
    last_updated TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create indexes for common queries
CREATE INDEX IF NOT EXISTS idx_sensors_type ON sensors(type);
CREATE INDEX IF NOT EXISTS idx_sensors_location ON sensors(location);
CREATE INDEX IF NOT EXISTS idx_sensors_status ON sensors(status);

-- Connect to the statemonitoring database
\c statemonitoring;

-- Create Diesel helper functions
CREATE OR REPLACE FUNCTION diesel_manage_updated_at(_tbl regclass) RETURNS VOID AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION diesel_set_updated_at() RETURNS trigger AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create service_contexts table
CREATE TABLE IF NOT EXISTS service_contexts (
    id INT PRIMARY KEY,
    maintenance BOOLEAN NOT NULL DEFAULT FALSE
);

-- Create sensor_data table
CREATE TABLE IF NOT EXISTS sensor_data (
    id SERIAL PRIMARY KEY,
    value VARCHAR NOT NULL,
    status VARCHAR NOT NULL,
    ts TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Create todos table (for compatibility with existing migrations)
CREATE TABLE IF NOT EXISTS todos (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    description TEXT NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT FALSE
);
