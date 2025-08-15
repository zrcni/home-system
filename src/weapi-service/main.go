package main

import (
	"log"
	"net/http"
	"os"

	"github.com/gin-contrib/cors"
	"github.com/gin-gonic/gin"
)

func main() {
	var weatherService *WeatherService

	// Initialize the database
	weatherService, err := initDatabase()
	if err != nil {
		log.Printf("Failed to initialize database: %s", err)
		os.Exit(1)
	}

	router := gin.Default()
	router.Use(cors.Default())
	router.Use(WeatherMiddleware(weatherService))
	router.GET("/weather/:location", getWeatherObservation)
	router.GET("/weather/:location/fetch", fetchWeatherObservation)
	router.GET("/health", func(c *gin.Context) {
		c.JSON(http.StatusOK, gin.H{
			"status":  "ok",
			"version": os.Getenv("APP_VERSION"),
		})
	})
	router.Run(":3001")
}

// WeatherMiddleware is a middleware function that injects the weather service into the request context
func WeatherMiddleware(weatherService *WeatherService) gin.HandlerFunc {
	return func(c *gin.Context) {
		c.Set("weatherService", weatherService)
		c.Next()
	}
}

// Gets a single weather from database by location
func getWeatherObservation(c *gin.Context) {
	client, ok := c.MustGet("weatherService").(*WeatherService)
	if !ok {
		log.Printf("Failed to get weather service")
		c.AbortWithStatus(http.StatusInternalServerError)
		return
	}

	location := c.Param("location")

	observation, err := client.repo.GetObservation(location)
	if err != nil {
		log.Printf("Failed to get weather from database: %s", err)
		c.AbortWithStatus(http.StatusInternalServerError)
		return
	}

	c.IndentedJSON(http.StatusOK, observation)
}

func fetchWeatherObservation(c *gin.Context) {
	client, ok := c.MustGet("weatherService").(*WeatherService)
	if !ok {
		log.Printf("Failed to get weather service")
		c.AbortWithStatus(http.StatusInternalServerError)
		return
	}

	location := c.Param("location")

	observationValue, err := getWeatherObservationsByLocationFromApi(location)
	if err != nil {
		log.Printf("Failed to fetch weather from API: %s", err)
		c.AbortWithStatus(http.StatusInternalServerError)
		return
	}

	var weatherObservation WeatherObservation

	weatherObservation.Location = location
	weatherObservation.Temperature = observationValue.Temperature
	weatherObservation.Humidity = observationValue.Humidity
	weatherObservation.LastUpdated = observationValue.LastUpdated

	client.repo.InsertObservation(weatherObservation)

	c.IndentedJSON(http.StatusOK, weatherObservation)
}

// Gets an environment variable or exits if it is not set
func getEnvVar(varName string, fallbackVarNames ...string) string {
	value := os.Getenv(varName)
	if value == "" {
		for _, fallbackVarName := range fallbackVarNames {
			value = os.Getenv(fallbackVarName)
			if value == "" {
				break
			}
		}
		if value == "" {
			log.Printf("%s is not set", varName)
			if len(fallbackVarNames) > 0 {
				log.Printf("Tried fallback variables: %v", fallbackVarNames)
			}
			os.Exit(1)
		}
	}
	return value
}

// Initializes the database based on the API type
func initDatabase() (*WeatherService, error) {
	dbName := getEnvVar("WEATHER_DB_NAME")
	collectionName := getEnvVar("WEATHER_DB_COLLECTION_NAME")
	dbURI := getEnvVar("WEATHER_DB_URI")
	dbUsername := os.Getenv("WEATHER_DB_USERNAME")
	dbPassword := os.Getenv("WEATHER_DB_PASSWORD")

	mongoRepo, err := NewMongoDBWeatherRepo(dbURI, dbName, collectionName, dbUsername, dbPassword)
	if err != nil {
		return nil, err
	}
	return NewWeatherService(mongoRepo), nil

}
