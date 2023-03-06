from flask import Flask, request
import random
import time
from prometheus_flask_exporter import PrometheusMetrics
from prometheus_client import Counter, Gauge, Summary, Histogram

app = Flask(__name__)
metrics = PrometheusMetrics(app)

by_path_counter = metrics.counter(
    "by_path_counter", "Request count by request paths", labels={"path": lambda: request.path}
)

SALES = Counter("sales_total", "Example of increment by arbitrary non-negative value")

MOVIES = Gauge("movies_in_queue", "Current messages in queue")

LATENCY = Summary("health_check_latency_seconds", "Time for a health check request")

LATENCY_HISTOGRAM = Histogram(
    "hello_world_latency_seconds", "Time for a hello world request", buckets=[0.0001, 0.0002, 0.0005, 0.001, 0.01, 0.1]
)

movies: list[str] = []


@app.route("/health")
def health() -> dict[str, str]:
    start = time.time()
    time.sleep(random.random())
    LATENCY.observe(time.time() - start)
    return {"message": "ok"}


@app.route("/")
@LATENCY_HISTOGRAM.time()
def hello_world() -> dict[str, str]:
    time.sleep(random.random())
    return {"message": "Hello World"}


@app.route("/books")
@by_path_counter
def books() -> dict[str, list[str]]:
    sale = random.random()
    SALES.inc(sale)
    print(f"{sale} incremented")
    return {"books": ["book1", "book2"]}


@app.route("/movies", methods=["POST"])
@by_path_counter
def create_movie() -> dict[str, list[str]]:
    id = random.randint(0, 100)
    movies.append(f"movie{id}")
    MOVIES.inc()
    return {"movies": movies}


@app.route("/movies", methods=["GET"])
@by_path_counter
def return_movies() -> dict[str, list[str]]:
    return {"movies": movies}


@app.route("/movies/<id>", methods=["DELETE"])
@by_path_counter
def delete_movie(id: int) -> dict[str, list[str]]:
    id = int(id)
    if id <= len(movies) - 1:
        del movies[id]
        MOVIES.dec()
    return {"movies": movies}


if __name__ == "__main__":
    app.run(debug=False, host="0.0.0.0", port=5000)
