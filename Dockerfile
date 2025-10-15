FROM rust:1.86-bookworm

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    libsqlite3-dev \
    curl \
    openjdk-17-jre \
    && rm -rf /var/lib/apt/lists/*

RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash - \
    && apt-get install -y nodejs

RUN rustup toolchain install nightly --component rust-src
RUN cargo install diesel_cli --no-default-features --features postgres,sqlite
RUN cargo install junitify

RUN npm install -g allure-commandline newman newman-reporter-allure

WORKDIR /app

COPY Cargo.toml ./
COPY Cargo.lock ./
COPY src ./src
COPY config.toml ./config.toml
COPY crates ./crates
COPY e2e_demo.postman_collection.json ./
COPY make_report.sh ./
COPY start.sh ./

RUN chmod +x make_report.sh
