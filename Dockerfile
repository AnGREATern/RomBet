FROM rust:1.86-bookworm

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    libsqlite3-dev \
    curl \
    unzip \
    openjdk-17-jre \
    && rm -rf /var/lib/apt/lists/*

RUN rustup toolchain install nightly --component rust-src

RUN cargo install junitify

RUN curl -Lo allure-2.27.0.tgz https://github.com/allure-framework/allure2/releases/download/2.27.0/allure-2.27.0.tgz \
    && tar -xzf allure-2.27.0.tgz -C /opt/ \
    && ln -s /opt/allure-2.27.0/bin/allure /usr/local/bin/allure \
    && rm allure-2.27.0.tgz

RUN cargo install diesel_cli --no-default-features --features postgres,sqlite

WORKDIR /app

COPY Cargo.toml ./
COPY Cargo.lock ./
COPY src ./src
COPY crates ./crates
COPY make_report.sh ./

RUN chmod +x make_report.sh
