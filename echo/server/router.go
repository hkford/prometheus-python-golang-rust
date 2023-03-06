package server

import (
	"main/handler"
	"main/observability"

	"github.com/labstack/echo-contrib/prometheus"
	"github.com/labstack/echo/v4"
)

func NewRouter() *echo.Echo {
	e := echo.New()
	m := observability.NewMetrics()

	p := prometheus.NewPrometheus("echo", nil, m.MetricList())
	p.Use(e)

	e.Use(m.AddCustomMetricsMiddleware)

	e.GET("/health", handler.HealthCheck)

	e.GET("/movies", handler.GetAllMovies)
	e.POST("/movies", handler.RegisterMovie)
	e.DELETE("/movies/:id", handler.DeleteMovie)
	return e
}
