package services

import (
	"bytes"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"smarthome/models"
	"time"
)

// SensorsService handles fetching sensors data from external API
type SensorsService struct {
	BaseURL    string
	HTTPClient *http.Client
	logger     *log.Logger
}

// NewSensorsService creates a new sensors service
func NewSensorsService(baseURL string, logger *log.Logger) *SensorsService {
	return &SensorsService{
		BaseURL: baseURL,
		HTTPClient: &http.Client{
			Timeout: 10 * time.Second,
		},
		logger: logger,
	}
}

// GetSensors fetches sensors data from the external API
func (s *SensorsService) GetSensors() ([]models.Sensor, error) {
	url := fmt.Sprintf("%s/api/v1/sensors", s.BaseURL)

	resp, err := s.HTTPClient.Get(url)
	if err != nil {
		return nil, fmt.Errorf("error fetching sensors data: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("unexpected status code: %d", resp.StatusCode)
	}

	var sensors []models.Sensor
	if err := json.NewDecoder(resp.Body).Decode(&sensors); err != nil {
		return nil, fmt.Errorf("error decoding sensors response: %w", err)
	}

	return sensors, nil
}

func (s *SensorsService) handleResponse(resp *http.Response, expectedStatusCode int, err error) (models.Sensor, error) {
	if err != nil {
		return models.Sensor{}, fmt.Errorf("error fetching sensor data: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != expectedStatusCode {
		logErrFromResponse(resp, s.logger)
		return models.Sensor{}, fmt.Errorf("unexpected status code: %d", resp.StatusCode)
	}

	var sensor models.Sensor
	if err := json.NewDecoder(resp.Body).Decode(&sensor); err != nil {
		return models.Sensor{}, fmt.Errorf("error decoding sensor response: %w", err)
	}

	return sensor, nil
}

// GetSensorByID fetches a sensor by its ID from the external API
func (s *SensorsService) GetSensorByID(id int) (models.Sensor, error) {
	url := fmt.Sprintf("%s/api/v1/sensors/%d", s.BaseURL, id)

	resp, err := s.HTTPClient.Get(url)
	return s.handleResponse(resp, http.StatusOK, err)
}

// CreateSensor creates a new sensor in the external API
func (s *SensorsService) CreateSensor(sensor models.Sensor) (models.Sensor, error) {
	url := fmt.Sprintf("%s/api/v1/sensors", s.BaseURL)

	jsonData, err := json.Marshal(sensor)
	if err != nil {
		return models.Sensor{}, fmt.Errorf("error marshalling sensor: %w", err)
	}

	resp, err := s.HTTPClient.Post(url, "application/json", bytes.NewBuffer(jsonData))
	return s.handleResponse(resp, http.StatusCreated, err)
}

// UpdateSensor updates an existing sensor in the external API
func (s *SensorsService) UpdateSensor(id int, sensor models.Sensor) (models.Sensor, error) {
	url := fmt.Sprintf("%s/api/v1/sensors/%d", s.BaseURL, id)

	jsonData, err := json.Marshal(sensor)
	if err != nil {
		return models.Sensor{}, fmt.Errorf("error marshalling sensor: %w", err)
	}

	req, err := http.NewRequest(http.MethodPut, url, bytes.NewBuffer(jsonData))
	if err != nil {
		return models.Sensor{}, fmt.Errorf("error creating request: %w", err)
	}

	req.Header.Set("Content-Type", "application/json")

	resp, err := s.HTTPClient.Do(req)
	return s.handleResponse(resp, http.StatusOK, err)
}

// DeleteSensor deletes an existing sensor in the external API
func (s *SensorsService) DeleteSensor(id int) error {
	url := fmt.Sprintf("%s/api/v1/sensors/%d", s.BaseURL, id)

	req, err := http.NewRequest(http.MethodDelete, url, nil)
	if err != nil {
		return fmt.Errorf("error creating request: %w", err)
	}

	req.Header.Set("Content-Type", "application/json")

	resp, err := s.HTTPClient.Do(req)
	if err != nil {
		return fmt.Errorf("error deleting sensor: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusNoContent {
		logErrFromResponse(resp, s.logger)
		return fmt.Errorf("unexpected status code: %d", resp.StatusCode)
	}

	return nil
}
