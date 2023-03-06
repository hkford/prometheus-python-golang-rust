package observability

import (
	"main/handler"

	"github.com/labstack/echo-contrib/prometheus"
	"github.com/labstack/echo/v4"
)

type Metrics struct {
	healthcheckLatency *prometheus.Metric
	moviesLength       *prometheus.Metric
}

func (m *Metrics) MetricList() []*prometheus.Metric {
	return []*prometheus.Metric{
		m.healthcheckLatency,
		m.moviesLength,
	}
}

func NewMetrics() *Metrics {
	return &Metrics{
		healthcheckLatency: handler.HealthcheckLatencySeconds,
		moviesLength:       handler.MoviesLength,
	}
}

const ContextKeyMetrics = "custom_metrics"

func (m *Metrics) AddCustomMetricsMiddleware(next echo.HandlerFunc) echo.HandlerFunc {
	return func(c echo.Context) error {
		c.Set(ContextKeyMetrics, m)
		return next(c)
	}
}
