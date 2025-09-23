#!/bin/bash

cargo +nightly test --all --exclude db -- --format=json -Z unstable-options --report-time | junitify -o test-results/
xunit-viewer -r test-results -o test-report.html
allure generate test-results -o allure-report --clean
allure open allure-report