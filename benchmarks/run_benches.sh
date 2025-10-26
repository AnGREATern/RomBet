#!/bin/bash

set -e

ITERATIONS=100
RESULTS_DIR="results/$(date +%Y%m%d_%H%M%S)"
K6_IMAGE="grafana/k6:latest"

mkdir -p $RESULTS_DIR
mkdir -p $RESULTS_DIR/raw
mkdir -p $RESULTS_DIR/summary

echo "Starting benchmark with $ITERATIONS iterations..."

start_resource_monitoring() {
    local iteration=$1
    echo "Starting resource monitoring for iteration $iteration..."
    
    pidstat -rud -h 1 > "$RESULTS_DIR/raw/resources_iteration_$iteration.csv" &
    RESOURCE_PID=$!
    
    sar -n DEV 1 > "$RESULTS_DIR/raw/network_iteration_$iteration.csv" &
    NETWORK_PID=$!
}

stop_resource_monitoring() {
    kill $RESOURCE_PID $NETWORK_PID 2>/dev/null || true
}

for i in $(seq 1 $ITERATIONS); do
    echo "=== Iteration $i/$ITERATIONS ==="
    
    echo "Starting service container..."
    docker compose run rombet sh -c "./start.sh -i" \
        -p 8080:8080 \
        --memory="512m" \
        --cpus="1.0" \
        --name benchmark-target
    sleep 10
    
    start_resource_monitoring $i
    
    echo "Starting load test..."
    docker run --rm -i \
        -v "$(pwd)":/scripts \
        -e BASE_URL="http://host.docker.internal:8080" \
        $K6_IMAGE run /scripts/benchmark_scenarios.js \
        --out json="$RESULTS_DIR/raw/k6_results_$i.json" \
        --out prometheus=remote-write-url=http://prometheus:9090/api/v1/write
    
    stop_resource_monitoring
    
    echo "Stopping service..."
    docker stop benchmark-target
    docker rm benchmark-target
    
    sleep 5
done

echo "Aggregating results..."
python3 aggregate_results.py --results-dir $RESULTS_DIR

echo "Benchmark completed! Results in: $RESULTS_DIR"
