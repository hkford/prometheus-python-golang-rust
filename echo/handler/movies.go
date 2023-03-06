package handler

import (
	"math/rand"
	"net/http"
	"strconv"
	"time"

	"github.com/labstack/echo-contrib/prometheus"
	"github.com/labstack/echo/v4"
	prom "github.com/prometheus/client_golang/prometheus"
)

var MoviesLength = &prometheus.Metric{
	Name:        "movies_length",
	Description: "Number of movies in array",
	Type:        "gauge_vec",
}

func incMoviesLength() {
	(*MoviesLength).MetricCollector.(*prom.GaugeVec).With(nil).Add(1)
}

func decMoviesLength() {
	(*MoviesLength).MetricCollector.(*prom.GaugeVec).With(nil).Sub(1)
}

type Movie struct {
	ID int
}

var movies = []Movie{}

func RegisterMovie(c echo.Context) error {
	rand.Seed(time.Now().UnixNano())
	id := rand.Intn(1000)
	movie := Movie{ID: id}
	movies = append(movies, movie)
	incMoviesLength()
	return c.JSON(http.StatusOK, movies)
}

func GetAllMovies(c echo.Context) error {
	return c.JSON(http.StatusOK, movies)
}

func DeleteMovie(c echo.Context) error {
	idFromRequest := c.Param("id")
	id, err := strconv.Atoi(idFromRequest)
	if err != nil {
		return echo.NewHTTPError(http.StatusBadRequest, err.Error())
	}
	if id <= len(movies) {
		movies = append(movies[:id], movies[id+1:]...)
		decMoviesLength()
	}
	return c.JSON(http.StatusOK, movies)
}
