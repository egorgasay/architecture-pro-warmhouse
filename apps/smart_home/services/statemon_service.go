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

// StateMonitoringService handles fetching state monitoring data from external API
type StateMonitoringService struct {
	BaseURL    string
	HTTPClient *http.Client
	logger     *log.Logger
}

// NewStateMonitoringService creates a new state monitoring service
func NewStateMonitoringService(baseURL string, logger *log.Logger) *StateMonitoringService {
	return &StateMonitoringService{
		BaseURL: baseURL,
		HTTPClient: &http.Client{
			Timeout: 10 * time.Second,
		},
		logger: logger,
	}
}

func logErrFromResponse(resp *http.Response, logger *log.Logger) error {
	body, err := io.ReadAll(resp.Body)
	if err != nil {
		logger.Printf("error reading response body: %v", err)
		return models.ErrorResponse{
			Err: fmt.Sprintf("error reading response body: %v", err),
		}
	}
	logger.Printf("unexpected status code: %d, response: %s", resp.StatusCode, string(body))

	var errorResponse models.ErrorResponse
	if err := json.Unmarshal(body, &errorResponse); err != nil {
		err = fmt.Errorf("error decoding error response: %w", err)
		logger.Print(err)
		return err
	}

	err = fmt.Errorf("unexpected status code: %d, response: %w", resp.StatusCode, errorResponse)

	logger.Print(err)
	return err
}

// api/v1/sensor/data
func (s *StateMonitoringService) GetSensorData(sensorID int) (*models.SensorData, error) {
	url := fmt.Sprintf("%s/api/v1/sensor/data?sensor_id=%d", s.BaseURL, sensorID)

	resp, err := s.HTTPClient.Get(url)
	if err != nil {
		return nil, fmt.Errorf("error fetching sensor data: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return nil, logErrFromResponse(resp, s.logger)
	}

	var sensorData models.SensorData
	if err := json.NewDecoder(resp.Body).Decode(&sensorData); err != nil {
		return nil, fmt.Errorf("error decoding sensor data response: %w", err)
	}

	return &sensorData, nil
}

// api/v1/sensor/update
func (s *StateMonitoringService) UpdateSensorData(sensorID int, sensorData models.SensorData) error {
	url := fmt.Sprintf("%s/api/v1/sensor/data?sensor_id=%d", s.BaseURL, sensorID)

	jsonData, err := json.Marshal(sensorData)
	if err != nil {
		return fmt.Errorf("error marshalling sensor data: %w", err)
	}

	resp, err := s.HTTPClient.Post(url, "application/json", bytes.NewBuffer(jsonData))
	if err != nil {
		return fmt.Errorf("error updating sensor data: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusNoContent {
		return logErrFromResponse(resp, s.logger)
	}

	return nil
}
