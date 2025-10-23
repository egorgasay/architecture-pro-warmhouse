package main

import (
	"context"
	"log"
	"net/http"
	"os"
	"os/signal"
	"syscall"
	"time"

	"smarthome/db"
	"smarthome/handlers"
	"smarthome/models"
	"smarthome/services"

	"github.com/gin-gonic/gin"
)

func main() {
	// Set up database connection
	config := models.NewConfig()
	logger := log.New(os.Stdout, "", log.LstdFlags)

	database, err := db.New(config.DatabaseURL)
	if err != nil {
		logger.Fatalf("Unable to connect to database: %v\n", err)
	}
	defer database.Close()

	logger.Println("Connected to database successfully")

	// Initialize temperature service
	temperatureAPIURL := config.TemperatureAPIURL
	temperatureService := services.NewTemperatureService(temperatureAPIURL)
	logger.Printf("Temperature service initialized with API URL: %s\n", temperatureAPIURL)

	// Initialize sensors service
	sensorsAPIURL := config.SensorsAPIURL
	sensorsService := services.NewSensorsService(sensorsAPIURL, logger)
	logger.Printf("Sensors service initialized with API URL: %s\n", sensorsAPIURL)

	// Initialize state monitoring service
	stateMonitoringAPIURL := config.StateMonitoringAPIURL
	stateMonitoringService := services.NewStateMonitoringService(stateMonitoringAPIURL, logger)
	logger.Printf("State monitoring service initialized with API URL: %s\n", stateMonitoringAPIURL)

	// Initialize router
	router := gin.Default()

	// Health check endpoint
	router.GET("/health", func(c *gin.Context) {
		c.JSON(http.StatusOK, gin.H{
			"status": "ok",
		})
	})

	// API routes
	apiRoutes := router.Group("/api/v1")

	// Register sensor routes
	sensorHandler := handlers.NewSensorHandler(config, database, temperatureService, sensorsService, stateMonitoringService)
	sensorHandler.RegisterRoutes(apiRoutes)

	// Start server
	srv := &http.Server{
		Addr:    config.ServerAddress,
		Handler: router,
	}

	// Start the server in a goroutine
	go func() {
		logger.Printf("Server starting on %s\n", srv.Addr)
		if err := srv.ListenAndServe(); err != nil && err != http.ErrServerClosed {
			log.Fatalf("Failed to start server: %v\n", err)
		}
	}()

	// Wait for interrupt signal to gracefully shut down the server
	quit := make(chan os.Signal, 1)
	signal.Notify(quit, syscall.SIGINT, syscall.SIGTERM)
	<-quit
	logger.Println("Shutting down server...")

	// Create a deadline for server shutdown
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()
	if err := srv.Shutdown(ctx); err != nil {
		logger.Fatalf("Server forced to shutdown: %v\n", err)
	}

	logger.Println("Server exited properly")
}
