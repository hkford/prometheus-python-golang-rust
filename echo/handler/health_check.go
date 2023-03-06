package handler

import (
	"math/rand"
	"net/http"
	"time"

	"github.com/labstack/echo-contrib/prometheus"
	"github.com/labstack/echo/v4"
	prom "github.com/prometheus/client_golang/prometheus"
)

type jsonResponse struct {
	Message string
}

var HealthcheckLatencySeconds = &prometheus.Metric{
	Name:        "health_check_latency_seconds",
	Description: "Time for a health check request",
	Type:        "summary_vec",
	Args:        []string{"path"},
}

func incLatency(path string, v float64) {
	labels := prom.Labels{"path": path}
	(*HealthcheckLatencySeconds).MetricCollector.(*prom.SummaryVec).With(labels).Observe(v)
}

func HealthCheck(c echo.Context) error {
	response := &jsonResponse{
		Message: "Healthy",
	}
	rand.Seed(time.Now().UnixNano())
	r := rand.Intn(1000)
	time.Sleep(time.Duration(r) * time.Millisecond)
	pausedSeconds := float64(r) * 0.001
	incLatency(c.Path(), pausedSeconds)
	return c.JSON(http.StatusOK, response)
}
