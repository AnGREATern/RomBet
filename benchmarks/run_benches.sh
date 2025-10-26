#!/bin/bash

set -e

ITERATIONS=100
RESULTS_DIR="results/$(date +%Y%m%d_%H%M%S)"
K6_IMAGE="grafana/k6:latest"
SERVICE_PORT=8080

mkdir -p $RESULTS_DIR
mkdir -p $RESULTS_DIR/raw
mkdir -p $RESULTS_DIR/summary

echo "Starting benchmark with $ITERATIONS iterations..."
echo "Results will be stored in: $RESULTS_DIR"

# Function to start service in isolated container
start_service() {
    local iteration=$1
    echo "Starting service container for iteration $iteration..."
    
    # Build fresh image for each iteration to ensure clean state
    docker build -t rombet-benchmark:$iteration .
    
    # Run container with isolated resources
    docker run -d \
        --name "benchmark-target-$iteration" \
        --memory="512m" \
        --cpus="1.0" \
        -p $SERVICE_PORT:$SERVICE_PORT \
        -e ROM_BET_SOCK="0.0.0.0:$SERVICE_PORT" \
        rombet-benchmark:$iteration \
        sh -c "./start.sh -r"
    
    # Wait for service to be ready
    echo "Waiting for service to start..."
    for i in {1..30}; do
        if curl -s http://localhost:$SERVICE_PORT/api/v1/teams >/dev/null 2>&1; then
            echo "Service is ready!"
            return 0
        fi
        echo "Service not ready yet, waiting... ($i/30)"
        sleep 2
    done
    
    echo "Service failed to start in time"
    return 1
}

# Function to stop and cleanup service
stop_service() {
    local iteration=$1
    echo "Stopping service container for iteration $iteration..."
    
    # Collect container stats before stopping
    docker stats "benchmark-target-$iteration" --no-stream > "$RESULTS_DIR/raw/container_stats_$iteration.txt" 2>/dev/null || true
    
    # Stop and remove container
    docker stop "benchmark-target-$iteration" 2>/dev/null || true
    docker rm "benchmark-target-$iteration" 2>/dev/null || true
    
    # Remove image to save space
    docker rmi "rombet-benchmark:$iteration" 2>/dev/null || true
}

# Function to start resource monitoring
start_resource_monitoring() {
    local iteration=$1
    echo "Starting resource monitoring for iteration $iteration..."
    
    # Start sysstat collection
    sar -u -r -b 1 > "$RESULTS_DIR/raw/system_resources_$iteration.txt" 2>&1 &
    RESOURCE_PID=$!
    
    # Start network monitoring
    sar -n DEV 1 > "$RESULTS_DIR/raw/network_usage_$iteration.txt" 2>&1 &
    NETWORK_PID=$!
}

# Function to stop resource monitoring
stop_resource_monitoring() {
    echo "Stopping resource monitoring..."
    kill $RESOURCE_PID $NETWORK_PID 2>/dev/null || true
    wait $RESOURCE_PID $NETWORK_PID 2>/dev/null || true
}

# Main benchmark loop
for i in $(seq 1 $ITERATIONS); do
    echo "=== Iteration $i/$ITERATIONS ==="
    
    # Start service with clean state
    if ! start_service $i; then
        echo "Failed to start service for iteration $i, skipping..."
        continue
    fi
    
    # Start resource monitoring
    start_resource_monitoring $i
    
    # Run load test
    echo "Starting load test for iteration $i..."
    docker run --rm \
        --name "k6-benchmark-$i" \
        -v "$(pwd)":/scripts \
        -e BASE_URL="http://host.docker.internal:$SERVICE_PORT" \
        $K6_IMAGE run /scripts/benchmark_scenarios.js \
        --out json="$RESULTS_DIR/raw/k6_results_$i.json"
    
    # Stop resource monitoring
    stop_resource_monitoring
    
    # Stop service and cleanup
    stop_service $i
    
    echo "Completed iteration $i"
    sleep 3
done

echo "Aggregating results..."
python3 aggregate_results.py --results-dir $RESULTS_DIR

echo "Benchmark completed! Results in: $RESULTS_DIR"
