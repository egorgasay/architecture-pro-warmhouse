package handlers

import (
	"context"
	"fmt"
	"log"
	"net/http"
	"strconv"
	"time"

	"smarthome/db"
	"smarthome/models"
	"smarthome/services"

	"github.com/gin-gonic/gin"
)

// SensorHandler handles sensor-related requests
type SensorHandler struct {
	DB                     *db.DB
	TemperatureService     *services.TemperatureService
	SensorsService         *services.SensorsService
	StateMonitoringService *services.StateMonitoringService
	Config                 *models.Config
}

// NewSensorHandler creates a new SensorHandler
func NewSensorHandler(
	config *models.Config,
	db *db.DB,
	temperatureService *services.TemperatureService,
	sensorsService *services.SensorsService,
	stateMonitoringService *services.StateMonitoringService,
) *SensorHandler {
	return &SensorHandler{
		Config:                 config,
		DB:                     db,
		TemperatureService:     temperatureService,
		SensorsService:         sensorsService,
		StateMonitoringService: stateMonitoringService,
	}
}

// RegisterRoutes registers the sensor routes
func (h *SensorHandler) RegisterRoutes(router *gin.RouterGroup) {
	sensors := router.Group("/sensors")
	{
		sensors.GET("", h.GetSensors)
		sensors.GET("/:id", h.GetSensorByID)
		sensors.POST("", h.CreateSensor)
		sensors.PUT("/:id", h.UpdateSensor)
		sensors.DELETE("/:id", h.DeleteSensor)
		sensors.PATCH("/:id/value", h.UpdateSensorValue)
		sensors.GET("/temperature/:location", h.GetTemperatureByLocation)
	}
}

// GetSensors handles GET /api/v1/sensors
func (h *SensorHandler) GetSensors(c *gin.Context) {
	var sensors []models.Sensor
	var err error

	if !h.Config.SensorsAPICalls.GetSensors {
		sensors, err = h.DB.GetSensors(context.Background())
		if err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
			return
		}

		// Update temperature sensors with real-time data from the external API
		for i, sensor := range sensors {
			if sensor.Type == models.Temperature {
				tempData, err := h.TemperatureService.GetTemperatureByID(fmt.Sprintf("%d", sensor.ID))
				if err == nil {
					// Update sensor with real-time data
					sensors[i].Value = tempData.Value
					sensors[i].Status = tempData.Status
					sensors[i].LastUpdated = tempData.Timestamp.Format(time.RFC3339)
					log.Printf("Updated temperature data for sensor %d from external API", sensor.ID)
				} else {
					log.Printf("Failed to fetch temperature data for sensor %d: %v", sensor.ID, err)
				}
			}
		}
	} else {
		sensors, err = h.SensorsService.GetSensors()
		if err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
			return
		}
	}

	c.JSON(http.StatusOK, sensors)
}

// GetSensorByID handles GET /api/v1/sensors/:id
func (h *SensorHandler) GetSensorByID(c *gin.Context) {
	id, err := strconv.Atoi(c.Param("id"))
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "Invalid sensor ID"})
		return
	}

	var sensor models.Sensor
	if !h.Config.SensorsAPICalls.GetSensorByID {
		sensor, err = h.DB.GetSensorByID(context.Background(), id)
	} else {
		sensor, err = h.SensorsService.GetSensorByID(id)
	}

	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, sensor)
}

// GetTemperatureByLocation handles GET /api/v1/sensors/temperature/:location
func (h *SensorHandler) GetTemperatureByLocation(c *gin.Context) {
	location := c.Param("location")
	if location == "" {
		c.JSON(http.StatusBadRequest, gin.H{"error": "Location is required"})
		return
	}

	// Fetch temperature data from the external API
	tempData, err := h.TemperatureService.GetTemperature(location)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{
			"error": fmt.Sprintf("Failed to fetch temperature data: %v", err),
		})
		return
	}

	// Return the temperature data
	c.JSON(http.StatusOK, gin.H{
		"location":    tempData.Location,
		"value":       tempData.Value,
		"unit":        tempData.Unit,
		"status":      tempData.Status,
		"timestamp":   tempData.Timestamp,
		"description": tempData.Description,
	})
}

// CreateSensor handles POST /api/v1/sensors
func (h *SensorHandler) CreateSensor(c *gin.Context) {
	var sensorCreate models.SensorCreate
	if err := c.ShouldBindJSON(&sensorCreate); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	var sensor models.Sensor
	var err error

	if !h.Config.SensorsAPICalls.CreateSensor {
		sensor, err = h.DB.CreateSensor(context.Background(), sensorCreate)
	} else {
		sensor, err = h.SensorsService.CreateSensor(models.Sensor{
			Name:     sensorCreate.Name,
			Type:     sensorCreate.Type,
			Location: sensorCreate.Location,
			Unit:     sensorCreate.Unit,
		})
	}

	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusCreated, sensor)
}

// UpdateSensor handles PUT /api/v1/sensors/:id
func (h *SensorHandler) UpdateSensor(c *gin.Context) {
	id, err := strconv.Atoi(c.Param("id"))
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "Invalid sensor ID"})
		return
	}

	var sensorUpdate models.SensorUpdate
	if err := c.ShouldBindJSON(&sensorUpdate); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	var sensor models.Sensor

	if !h.Config.SensorsAPICalls.UpdateSensor {
		sensor, err = h.DB.UpdateSensor(context.Background(), id, sensorUpdate)
	} else {
		sensor, err = h.SensorsService.UpdateSensor(id, models.Sensor{
			Name:     sensorUpdate.Name,
			Type:     sensorUpdate.Type,
			Location: sensorUpdate.Location,
			Unit:     sensorUpdate.Unit,
			Status:   sensorUpdate.Status,
		})
	}

	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, sensor)
}

// DeleteSensor handles DELETE /api/v1/sensors/:id
func (h *SensorHandler) DeleteSensor(c *gin.Context) {
	id, err := strconv.Atoi(c.Param("id"))
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "Invalid sensor ID"})
		return
	}

	if !h.Config.SensorsAPICalls.DeleteSensor {
		err = h.DB.DeleteSensor(context.Background(), id)
	} else {
		err = h.SensorsService.DeleteSensor(id)
	}

	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, gin.H{"message": "Sensor deleted successfully"})
}

// UpdateSensorValue handles PATCH /api/v1/sensors/:id/value
func (h *SensorHandler) UpdateSensorValue(c *gin.Context) {
	id, err := strconv.Atoi(c.Param("id"))
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "Invalid sensor ID"})
		return
	}

	var request models.SensorData

	if err := c.ShouldBindJSON(&request); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	if !h.Config.StateMonitoringAPICalls.UpdateSensorData {
		err = h.DB.UpdateSensorValue(context.Background(), id, request.Value, request.Status)
	} else {
		if request.CreatedAt == "" {
			request.CreatedAt = time.Now().Format(time.RFC3339)
		} 

		if request.Unit == "" {
			request.Unit = "unknown"
		}

		if request.Status == "" {
			request.Status = "unknown"
		}

		err = h.StateMonitoringService.UpdateSensorData(id, request)
	}
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, gin.H{"message": "Sensor value updated successfully"})
}
