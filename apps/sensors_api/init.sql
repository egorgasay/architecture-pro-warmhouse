SELECT 'CREATE DATABASE smarthome'
WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = 'smarthome')\gexec

\c sensors_api;

-- Create the sensors table
CREATE TABLE IF NOT EXISTS sensors (
   id SERIAL PRIMARY KEY,
   name VARCHAR(100) NOT NULL,
   type VARCHAR(50) NOT NULL,
   location VARCHAR(100) NOT NULL,
   last_updated TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
   created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_sensors_type ON sensors(type);
CREATE INDEX IF NOT EXISTS idx_sensors_location ON sensors(location);
