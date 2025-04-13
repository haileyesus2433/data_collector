# System Resource Monitor

A distributed system monitoring solution that collects and visualizes CPU and RAM usage across multiple machines in real-time.

## Components

The project consists of three main components:

- **Collector**: A lightweight agent that runs on target machines to collect system metrics
- **Server**: A central server that receives, stores, and serves monitoring data
- **Web Interface**: A browser-based dashboard for visualizing system metrics

## Features

- Real-time CPU and RAM usage monitoring
- Multi-collector support with unique IDs
- Interactive time-series graphs
- Raw data table view
- SQLite database for metrics storage
- RESTful API endpoints

## Prerequisites

- Rust (latest stable version)
- SQLite
- Web browser with JavaScript enabled

## Installation

1. Clone the repository:

```bash
git clone <repository-url>
cd system-resource-monitor
```

2. Set up the environment:

```bash
# Create a .env file in the server directory
echo "DATABASE_URL=sqlite:data.db" > server/.env
```

3. Build the components:

```bash
# Build the collector
cd collector
cargo build --release

# Build the server
cd ../server
cargo build --release
```

## Usage

1. Start the server:

```bash
cd server
cargo run --release
```

The server will start on `http://localhost:3000`

2. Run the collector on target machines:

```bash
cd collector
cargo run --release
```

The collector will automatically connect to the server and start sending metrics.

3. Access the web interface by opening `http://localhost:3000` in your browser.

## API Endpoints

- `GET /` - Web interface home page
- `GET /collector` - Collector details page
- `GET /api/all` - Get all collected data
- `GET /api/collectors` - List all active collectors
- `GET /api/collector/{uuid}` - Get data for a specific collector

## Project Structure

```
.
├── collector/          # System metrics collection agent
├── server/            # Central server and web interface
└── shared/            # Shared code and data structures
```

## Technical Details

- **Data Collection**: Uses the `sysinfo` crate for system metrics
- **Communication**: TCP-based protocol with custom binary format
- **Web Framework**: Axum for the REST API
- **Frontend**: Bootstrap and ECharts for visualization
- **Database**: SQLite with SQLx for async operations

## Development

To run the project in development mode:

1. Start the server:

```bash
cd server
cargo run
```

2. Run the collector:

```bash
cd collector
cargo run
```
