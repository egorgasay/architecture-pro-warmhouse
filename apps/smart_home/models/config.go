package models

import "os"

type Config struct {
	DatabaseURL           string
	TemperatureAPIURL     string
	SensorsAPIURL         string
	StateMonitoringAPIURL string
	ServerAddress         string

	SensorsAPICalls         SensorsAPICalls
	StateMonitoringAPICalls StateMonitoringAPICalls
}

type SensorsAPICalls struct {
	GetSensors        bool
	GetSensorByID     bool
	CreateSensor      bool
	UpdateSensor      bool
	DeleteSensor      bool
	UpdateSensorValue bool
}

type StateMonitoringAPICalls struct {
	GetSensorData bool
	UpdateSensorData bool
}

func NewConfig() *Config {
	return &Config{
		DatabaseURL:           getEnv("DATABASE_URL", "postgres://postgres:qwerty@localhost:6000/smarthome"),
		TemperatureAPIURL:     getEnv("TEMPERATURE_API_URL", "http://temperature_api:5000"),
		SensorsAPIURL:         getEnv("SENSORS_API_URL", "http://sensors_api:8081"),
		StateMonitoringAPIURL: getEnv("STATEMON_API_URL", "http://statemon:7676"),
		ServerAddress:         getEnv("SERVER_ADDRESS", ":8080"),
		SensorsAPICalls: SensorsAPICalls{
			GetSensors:        getEnv("SENSORS_API_GET_SENSORS", "true") == "true",
			GetSensorByID:     getEnv("SENSORS_API_GET_SENSOR_BY_ID", "true") == "true",
			CreateSensor:      getEnv("SENSORS_API_CREATE_SENSOR", "true") == "true",
			UpdateSensor:      getEnv("SENSORS_API_UPDATE_SENSOR", "true") == "true",
			DeleteSensor:      getEnv("SENSORS_API_DELETE_SENSOR", "true") == "true",
			UpdateSensorValue: getEnv("SENSORS_API_UPDATE_SENSOR_VALUE", "true") == "true",
		},
		StateMonitoringAPICalls: StateMonitoringAPICalls{
			GetSensorData: getEnv("STATEMON_API_GET_SENSOR_DATA", "true") == "true",
			UpdateSensorData: getEnv("STATEMON_API_UPDATE_SENSOR_DATA", "true") == "true",
		},
	}
}

// getEnv gets an environment variable or returns a default value
func getEnv(key, defaultValue string) string {
	value := os.Getenv(key)
	if value == "" {
		return defaultValue
	}
	return value
}
