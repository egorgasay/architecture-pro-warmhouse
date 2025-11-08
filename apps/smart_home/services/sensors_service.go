package services

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
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

	return handleResponse[[]models.Sensor](s, resp, http.StatusOK, err)
}

func handleResponse[V any](s *SensorsService, resp *http.Response, expectedStatusCode int, err error) (V, error) {
	var v V
	if err != nil {
		return v, fmt.Errorf("error fetching sensor data: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != expectedStatusCode {
		return v, logErrFromResponse(resp, s.logger)
	}

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return v, fmt.Errorf("error reading response body: %w", err)
	}

	if err := json.Unmarshal(body, &v); err != nil {
		s.logger.Printf("Error unmarshalling response body: %s", string(body))
		return v, fmt.Errorf("error unmarshalling response body: %w", err)
	}

	return v, nil
}

// GetSensorByID fetches a sensor by its ID from the external API
func (s *SensorsService) GetSensorByID(id int) (models.Sensor, error) {
	url := fmt.Sprintf("%s/api/v1/sensors/%d", s.BaseURL, id)

	resp, err := s.HTTPClient.Get(url)
	return handleResponse[models.Sensor](s, resp, http.StatusOK, err)
}

// CreateSensor creates a new sensor in the external API
func (s *SensorsService) CreateSensor(sensor models.Sensor) (models.Sensor, error) {
	url := fmt.Sprintf("%s/api/v1/sensors", s.BaseURL)

	jsonData, err := json.Marshal(sensor)
	if err != nil {
		return models.Sensor{}, fmt.Errorf("error marshalling sensor: %w", err)
	}

	resp, err := s.HTTPClient.Post(url, "application/json", bytes.NewBuffer(jsonData))
	return handleResponse[models.Sensor](s, resp, http.StatusCreated, err)
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
	return handleResponse[models.Sensor](s, resp, http.StatusOK, err)
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
		return logErrFromResponse(resp, s.logger)
	}

	return nil
}

func (s *SensorsService) GetSensorDataByLocation(location string) (*models.TemperatureResponse, error) {
	url := fmt.Sprintf("%s/api/v1/sensors/location/%s", s.BaseURL, location)

	resp, err := s.HTTPClient.Get(url)
	t, err := handleResponse[models.TemperatureResponse](s, resp, http.StatusOK, err)
	return &t, err
}
