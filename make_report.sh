#!/bin/bash

ALLURE_RESULTS_DIR="test-results"
ALLURE_REPORT_DIR="allure-report"
ALLURE_HISTORY_DIR="allure-history"

OPEN_REPORT=false

while [[ $# -gt 0 ]]; do
    case $1 in
        -r|--run)
            OPEN_REPORT=true
            shift
            ;;
        *)
            echo "Неизвестный аргумент: $1"
            exit 1
            ;;
    esac
done

run_migrations() {
    cd crates/db && diesel migration redo --all && cd ../..
}

mkdir -p "$ALLURE_HISTORY_DIR"
run_migrations

(cargo +nightly test --all -- --format=json -Z unstable-options --report-time | junitify -o "$ALLURE_RESULTS_DIR/") || run_migrations

if [ -d "$ALLURE_HISTORY_DIR" ]; then
    LATEST_RUN=$(find "$ALLURE_HISTORY_DIR" -maxdepth 1 -type d -name "2*" | sort -r | head -n 1)
    if [ -n "$LATEST_RUN" ] && [ -d "$LATEST_RUN/history" ]; then
        mkdir -p "$ALLURE_RESULTS_DIR/history"
        cp -r "$LATEST_RUN/history"/* "$ALLURE_RESULTS_DIR/history/" 2>/dev/null || true
    fi
fi

CURRENT_RUN="$ALLURE_HISTORY_DIR/$(date +%Y%m%d-%H%M%S)"
allure generate "$ALLURE_RESULTS_DIR" -o "$CURRENT_RUN" --clean

allure generate "$ALLURE_RESULTS_DIR" -o "$ALLURE_REPORT_DIR" --clean

if [ "$OPEN_REPORT" = true ]; then
    allure open "$ALLURE_REPORT_DIR"
fi