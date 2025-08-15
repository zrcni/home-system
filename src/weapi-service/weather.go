package main

type WeatherObservation struct {
	Location    string  `json:"location"`
	Temperature float64 `json:"temperature"`
	Humidity    float64 `json:"humidity"`
	// ISO string
	LastUpdated string `json:"last_updated"`
}

type WeatherRepo interface {
	GetObservation(location string) (WeatherObservation, error)
	InsertObservation(weather WeatherObservation) error
}

type WeatherService struct {
	repo WeatherRepo
}

func NewWeatherService(repo WeatherRepo) *WeatherService {
	return &WeatherService{repo}
}
