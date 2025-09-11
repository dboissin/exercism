// Package weather <package_description>.
package weather

// CurrentCondition ...
var CurrentCondition string

// CurrentLocation ...
var CurrentLocation string

// Forecast ...
func Forecast(city, condition string) string {
	CurrentLocation, CurrentCondition = city, condition
	return CurrentLocation + " - current weather condition: " + CurrentCondition
}
