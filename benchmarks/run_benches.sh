#!/bin/bash

set -e

ITERATIONS=100
RESULTS_DIR="benches-results"
K6_IMAGE="grafana/k6:latest"
MONITORING_STACK="benchmarks/docker-compose.monitoring.yml"

mkdir -p $RESULTS_DIR
mkdir -p $RESULTS_DIR/raw
mkdir -p $RESULTS_DIR/summary
mkdir -p $RESULTS_DIR/grafana-dashboards

echo "Starting benchmark with $ITERATIONS iterations..."
echo "Results will be stored in: $RESULTS_DIR"

# Function to start monitoring stack
start_monitoring_stack() {
    echo "Starting monitoring stack (Prometheus + Grafana + Node Exporter)..."
    docker compose -f $MONITORING_STACK up -d
    
    # Wait for services to be ready
    echo "Waiting for monitoring services to start..."
    for i in {1..30}; do
        if curl -s http://localhost:9090/status >/dev/null 2>&1 && \
           curl -s http://localhost:8000/api/health >/dev/null 2>&1; then
            echo "Monitoring stack is ready!"
            return 0
        fi
        echo "Monitoring services not ready yet, waiting... ($i/30)"
        sleep 2
    done
    
    echo "Monitoring stack failed to start in time"
    return 1
}

# Function to stop monitoring stack
stop_monitoring_stack() {
    echo "Stopping monitoring stack..."
    docker compose -f $MONITORING_STACK down
}

# Start monitoring stack
if ! start_monitoring_stack; then
    echo "Failed to start monitoring stack, exiting..."
    exit 1
fi

# Function to start service in isolated container
start_service() {
    local iteration=$1
    echo "Starting service container for iteration $iteration..."
    
    # Use docker run instead of docker compose run for better isolation
    # Use dynamic port mapping to avoid conflicts
    docker run -d \
        --name "benchmark-target-$iteration" \
        --network "app-network" \
        rombet sh -c "cd crates/db && diesel migration run && cd ../.. && ./rombet"
    
    # Wait for service to be ready
    echo "Waiting for service to start..."
    # Use container name for internal communication
    for ssi in {1..30}; do
        if docker exec "benchmark-target-$iteration" curl -s http://localhost:3000/api/v1/teams >/dev/null 2>&1; then
            echo "Service is ready!"
            return 0
        fi
        echo "Service not ready yet, waiting... ($ssi/30)"
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
    
    # Note: We don't remove images as they're reused across iterations
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
        --network "app-network" \
        -v "$(pwd)/benchmarks":/scripts \
        -v "$(pwd)/$RESULTS_DIR":/results \
        -e BASE_URL="http://benchmark-target-$i:3000" \
        $K6_IMAGE run /scripts/benchmark_scenarios.js \
        --out json="/results/raw/k6_results_$i.json" \
        --out experimental-prometheus-rw \
        --tag testid="iteration-$i" \
        --summary-export="/results/raw/k6_summary_$i.json"
    
    # Stop resource monitoring
    stop_resource_monitoring
    
    # Stop service and cleanup
    stop_service $i
    
    echo "Completed iteration $i"
    sleep 3
done

echo "Aggregating results..."
python3 aggregate_results.py --results-dir $RESULTS_DIR

# Keep monitoring stack running throughout all iterations
# Only stop it at the end if specifically requested
if [[ "$KEEP_MONITORING" != "true" ]]; then
    echo "Stopping monitoring stack..."
    stop_monitoring_stack
else
    echo "Monitoring stack will remain running as requested."
fi

echo "Benchmark completed! Results in: $RESULTS_DIR"
