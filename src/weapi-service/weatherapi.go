package main

import (
	"encoding/json"
	"fmt"
	"net/http"
	"time"
)

type ObservationValue struct {
	Temperature float64 `json:"t2m"`
	Humidity    float64 `json:"Humidity"`
	// ISO string
	LastUpdated string `json:"localtime"`
}

type ObservationResponse struct {
	Observations []ObservationValue `json:"observations"`
}

// Locations:
// fmisid 101794 = Oulu, Vihreäsaari satama
// fmisid 101799 = Oulu, Oulunsalo Pellonpää
// fmisid 108040 = Oulu, Kaukovainio
func getWeatherObservationsByLocationFromApi(location string) (ObservationValue, error) {
	res, err := http.Get(fmt.Sprintf("https://www.ilmatieteenlaitos.fi/api/weather/observations?fmisid=%s&observations=true", location))

	var response ObservationResponse
	var observation ObservationValue

	if err != nil {
		return observation, err
	}
	defer res.Body.Close()

	if err := json.NewDecoder(res.Body).Decode(&response); err != nil {
		return observation, err
	}

	if len(response.Observations) == 0 {
		return observation, fmt.Errorf("no weather data found for location: %s", location)
	}

	for _, obs := range response.Observations {
		if obs.Temperature != 0 {
			observation = obs
			break
		}
	}

	// LastUpdated: parse date to "yyyyMMdd'T'HHmmss" format reliably
	t, err := time.Parse("20060102T150405", observation.LastUpdated)
	if err != nil {
		return observation, err
	}

	observation.LastUpdated = t.Format("2006-01-02T15:04:05Z07:00")

	return observation, nil
}
