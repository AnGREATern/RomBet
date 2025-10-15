#!/bin/bash

RUST_ONLY=false

while [[ $# -gt 0 ]]; do
    case $1 in
        -r|--rust-only)
            RUST_ONLY=true
            shift
            ;;
        *)
            echo "Неизвестный аргумент: $1"
            exit 1
            ;;
    esac
done

cleanup() {
    echo "Stop processes..."
    if [ -n "$RUST_PID" ]; then
        kill $RUST_PID 2>/dev/null
    fi
    if [ -n "$REACT_PID" ]; then
        kill $REACT_PID 2>/dev/null
    fi
    
    cp /test.log /start-"$(date +%Y%m%d-%H%M%S)".log 2>/dev/null
    rm /test.log 2>/dev/null
}
trap cleanup SIGINT SIGTERM

echo "Start Rust server..."
cargo build
cargo run &
RUST_PID=$!

if [ "$RUST_ONLY" = false ]; then
    echo "Start React frontend..."
    cd frontend && npm run dev &
    REACT_PID=$!
    cd ..
fi

if [ "$RUST_ONLY" = false ]; then
    wait "$REACT_PID"
else
    wait $RUST_PID
fi
