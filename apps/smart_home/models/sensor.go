package models

import "time"

// SensorType represents the type of sensor
type SensorType string

const (
	Temperature SensorType = "temperature"
)

// Sensor represents a smart home sensor
type Sensor struct {
	ID          int        `json:"id"`
	Name        string     `json:"name"`
	Type        SensorType `json:"type"`
	Location    string     `json:"location"`
	Value       *float64   `json:"value,omitempty"`
	Unit        *string    `json:"unit,omitempty"`
	Status      *string    `json:"status,omitempty"`
	LastUpdated string     `json:"last_updated"`
	CreatedAt   string     `json:"created_at"`
}

// SensorCreate represents the data needed to create a new sensor
type SensorCreate struct {
	Name     string     `json:"name" binding:"required"`
	Type     SensorType `json:"type" binding:"required"`
	Location string     `json:"location" binding:"required"`
	Unit     string     `json:"unit"`
}

// SensorUpdate represents the data that can be updated for a sensor
type SensorUpdate struct {
	Name     string     `json:"name"`
	Type     SensorType `json:"type"`
	Location string     `json:"location"`
	Value    *float64   `json:"value"`
	Unit     string     `json:"unit"`
	Status   string     `json:"status"`
}

// SensorData represents the data of a sensor
type SensorData struct {
	ID        int    `json:"id"`
	Location  string `json:"location"`
	Unit      string `json:"unit"`
	Status    string `json:"status"`
	CreatedAt string `json:"created_at"`

	// Для обратной совместимости
	Value any `json:"value"`
}

type ErrorResponse struct {
	Err        string `json:"error"`
	StatusCode int    `json:"status_code"`
}

func (e ErrorResponse) Error() string {
	return e.Err
}

// TemperatureResponse represents the response from the temperature API
type TemperatureResponse struct {
	Value       float64   `json:"value"`
	Unit        string    `json:"unit"`
	Timestamp   time.Time `json:"timestamp"`
	Location    string    `json:"location"`
	Status      string    `json:"status"`
	SensorID    string    `json:"sensor_id"`
	SensorType  string    `json:"sensor_type"`
	Description string    `json:"description"`
}
