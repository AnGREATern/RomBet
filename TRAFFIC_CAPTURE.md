# Traffic Capture Guide

This guide explains how to simulate E2E actions with curl and capture network traffic for debugging purposes.

## Prerequisites

- `curl` installed (usually comes pre-installed on most systems)
- `tcpdump` or Wireshark installed for traffic capture
- The RomBet application running locally on port 8080

## Starting the Application

First, start the RomBet application:

```bash
cargo run
```

The application should start on `http://localhost:8080`.

## Simulating E2E Actions with curl

### 1. Start Simulation

```bash
curl -X GET http://localhost:8080/api/start
```

Expected response:
```json
{
  "id": "uuid-string",
  "balance": 1000.0
}
```

Save the `id` from the response for later use.

### 2. Create Game Round

```bash
curl -X POST http://localhost:8080/api/create_round
```

Expected response:
```json
{
  "round": 1,
  "games": [
    {
      "id": "game-uuid",
      "home_team": {
        "id": "team-uuid-1",
        "name": "Team Name 1"
      },
      "guest_team": {
        "id": "team-uuid-2",
        "name": "Team Name 2"
      }
    }
    // ... more games
  ]
}
```

Save the first game's `id`, `home_team.id`, and `guest_team.id` for coefficient calculation.

### 3. Calculate Betting Coefficients

Replace the placeholders with actual IDs from the previous steps:

```bash
curl -X POST http://localhost:8080/api/calculate_coefficients \
  -H "Content-Type: application/json" \
  -d '{
    "game_id": "GAME_ID_HERE",
    "home_team_id": "HOME_TEAM_ID_HERE",
    "guest_team_id": "GUEST_TEAM_ID_HERE"
  }'
```

Expected response:
```json
{
  "events": [
    {"WDL": "W1"},
    {"WDL": "X"},
    {"WDL": "W2"}
    // ... more events
  ],
  "coefficients": [
    2.5,
    3.0,
    4.2
    // ... more coefficients
  ]
}
```

### 4. Place Bet

Select an event and coefficient from the previous response:

```bash
curl -X POST http://localhost:8080/api/make_bet \
  -H "Content-Type: application/json" \
  -d '{
    "game": {
      "id": "GAME_ID_HERE",
      "home_team_id": "HOME_TEAM_ID_HERE",
      "guest_team_id": "GUEST_TEAM_ID_HERE",
      "round": 1
    },
    "event": {"WDL": "W1"},
    "coefficient": 2.5,
    "value": 100.0
  }'
```

Expected response:
```
200 OK (empty response body)
```

### 5. Randomize Round Results

```bash
curl -X POST http://localhost:8080/api/randomize_round
```

Expected response:
```json
{
  "round": 1,
  "games_stat": [
    {
      "game_id": "game-uuid",
      "home_team_total": 2,
      "guest_team_total": 1
    }
    // ... more game stats
  ],
  "profit": 150.0
}
```

### 6. Get Balance

```bash
curl -X GET http://localhost:8080/api/balance
```

Expected response:
```json
{
  "balance": 1150.0
}
```

## Capturing Network Traffic

### Using tcpdump

To capture traffic to and from the application:

```bash
sudo tcpdump -i lo0 -w rombet_traffic.pcap host localhost and port 8080
```

This will save the captured traffic to `rombet_traffic.pcap`, which can be opened with Wireshark for analysis.

To capture traffic for a specific duration:

```bash
sudo tcpdump -i lo0 -G 60 -w rombet_traffic.pcap host localhost and port 8080
```

This captures traffic for 60 seconds.

### Using Wireshark

1. Open Wireshark
2. Select the loopback interface (lo0 on macOS, lo on Linux)
3. Set the filter to `host localhost and port 8080`
4. Click "Start"
5. Run the curl commands above
6. Stop the capture in Wireshark
7. Analyze the captured packets

## Analyzing Captured Traffic

When analyzing the captured traffic in Wireshark:

1. Look for HTTP requests and responses
2. Filter by HTTP protocol: `http`
3. Examine request headers, bodies, and response codes
4. Check for any errors or unexpected behavior

Common filters in Wireshark:
- `http.request.method == "GET"` - Show only GET requests
- `http.request.method == "POST"` - Show only POST requests
- `http.response.code == 200` - Show only successful responses
- `http.response.code >= 400` - Show only error responses

## Troubleshooting

If you encounter issues:

1. Ensure the application is running on port 8080
2. Check that all required IDs are correctly substituted in the requests
3. Verify that the Content-Type header is set correctly for POST requests
4. Make sure you're using the correct HTTP methods (GET vs POST)

## Example Complete Flow Script

Here's a bash script that executes the complete flow:

```bash
#!/bin/bash

echo "Starting RomBet E2E test flow..."

# Start simulation
echo "1. Starting simulation..."
response=$(curl -s -X GET http://localhost:8080/api/start)
echo "Response: $response"

# Extract simulation ID (this is a simplified example)
# In practice, you'd want to parse the JSON properly
SIM_ID=$(echo $response | grep -o '"id":"[^"]*"' | head -1 | cut -d'"' -f4)
echo "Simulation ID: $SIM_ID"

# Create round
echo "2. Creating game round..."
curl -s -X POST http://localhost:8080/api/create_round

# Calculate coefficients (you'd need to extract actual game IDs)
echo "3. Calculating coefficients..."
curl -s -X POST http://localhost:8080/api/calculate_coefficients \
  -H "Content-Type: application/json" \
  -d '{
    "game_id": "example-game-id",
    "home_team_id": "example-home-team-id",
    "guest_team_id": "example-guest-team-id"
  }'

# Place bet
echo "4. Placing bet..."
curl -s -X POST http://localhost:8080/api/make_bet \
  -H "Content-Type: application/json" \
  -d '{
    "game": {
      "id": "example-game-id",
      "home_team_id": "example-home-team-id",
      "guest_team_id": "example-guest-team-id",
      "round": 1
    },
    "event": {"WDL": "W1"},
    "coefficient": 2.5,
    "value": 100.0
  }'

# Randomize results
echo "5. Randomizing results..."
curl -s -X POST http://localhost:8080/api/randomize_round

# Get balance
echo "6. Getting balance..."
curl -s -X GET http://localhost:8080/api/balance

echo "E2E test flow completed!"
```

This documentation provides a complete guide for simulating E2E actions and capturing network traffic for the RomBet application.
