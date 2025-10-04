#!/bin/bash

ALLURE_RESULTS_DIR="test-results"
ALLURE_REPORT_DIR="allure-report"
ALLURE_HISTORY_DIR="allure-history"

run_migrations() {
    cd crates/db && diesel migration redo --all && cd ../..
}

run_units() {
    cargo +nightly test --all --no-fail-fast --lib -- --format=json -Z unstable-options --report-time | junitify -o "$ALLURE_RESULTS_DIR/"
}

run_integrations() {
    run_migrations
    (cargo +nightly test --all --no-fail-fast --test '*' -- --format=json -Z unstable-options --report-time | junitify -o "$ALLURE_RESULTS_DIR/") || run_migrations
}

generate_report() {
    mkdir -p "$ALLURE_HISTORY_DIR"
    LATEST_RUN=$(find "$ALLURE_HISTORY_DIR" -maxdepth 1 -type d -name "2*" | sort -r | head -n 1)
    if [ -n "$LATEST_RUN" ] && [ -d "$LATEST_RUN/history" ]; then
        mkdir -p "$ALLURE_RESULTS_DIR/history"
        cp -r "$LATEST_RUN/history"/* "$ALLURE_RESULTS_DIR/history/" 2>/dev/null || true
    fi
    CURRENT_RUN="$ALLURE_HISTORY_DIR/$(date +%Y%m%d-%H%M%S)"
    allure generate "$ALLURE_RESULTS_DIR" -o "$CURRENT_RUN" --clean
    allure generate "$ALLURE_RESULTS_DIR" -o "$ALLURE_REPORT_DIR" --clean
}

clear() {
    rm -rf "$ALLURE_RESULTS_DIR" "$ALLURE_REPORT_DIR" "$ALLURE_HISTORY_DIR"
}

open_report() {
    allure open "$ALLURE_REPORT_DIR"
}

while [[ $# -gt 0 ]]; do
    case $1 in
        -a|--all)
            clear
            run_units
            run_integrations
            generate_report
            open_report
            shift
            ;;
        -t|--tests)
            run_units
            run_integrations
            generate_report
            shift
            ;;
        -u|--units)
            run_units
            generate_report
            shift
            ;;
        -i|--integrations)
            run_integrations
            generate_report
            shift
            ;;
        -c|--clear)
            clear
            shift
            ;;
        -o|--open-report)
            open_report
            shift
            ;;
        *)
            echo "Undefined flag: $1"
            exit 1
            ;;
    esac
done
