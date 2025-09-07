#!/bin/bash

cleanup() {
    echo "Останавливаем процессы..."
    kill $RUST_PID 2>/dev/null
    cp ../test.log ../start-"$(date +%s)".log
    rm ../test.log
    exit 0
}
trap cleanup SIGINT SIGTERM

echo "Запускаем Rust сервер..."
cargo build
cargo run &
RUST_PID=$!

echo "Запускаем React фронтенд..."
cd frontend && npm run dev

cleanup