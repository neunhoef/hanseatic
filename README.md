# Hanseatic Trading Game

A multiplayer trading simulation game set in the Hanseatic League era (circa 1400 CE) around the North and Baltic Seas. Players take on the role of merchants building trading empires across approximately 50 historical cities, with authentic information asymmetry and realistic medieval trading challenges.

## Overview

This is a Rust library and binary crate for the "Hanseatic" project - currently in early development with basic library structure and a comprehensive database of Hanseatic League cities with historical data.

### Key Features

- **Realistic Information Asymmetry**: Players can only access information at their current location, with all distant information delayed by travel time
- **Ship Programming**: Automate trading operations through programmed ship behaviors
- **Real-time Economic Simulation**: Dynamic supply and demand with hourly market updates
- **Historical Accuracy**: Based on real 1400 CE geography and trade networks
- **Multiplayer Support**: 1-6 players per game instance with consensus-based time acceleration

### Core Innovation

The game's central innovation is authentic medieval trading challenges: information travels at ship speed, players can only be in one location at a time, and all communication requires physical travel or message systems.

## Installation

Ensure you have Rust installed (1.70+ recommended), then:

```bash
git clone <repository-url>
cd hanseatic
cargo build
```

## Usage

### Running the Demo

```bash
cargo run
```

This runs the main binary demonstrating the library's current functionality, including the city database.

### Development Commands

**Build and Test:**
```bash
cargo build          # Build the project
cargo test           # Run all tests
cargo check          # Quick syntax and type checking
```

**Code Quality:**
```bash
cargo fmt            # Format code
cargo clippy         # Lint for common issues
```

## Architecture

### Current Structure

- `src/lib.rs` - Main library with utility functions and module declarations
- `src/main.rs` - Binary entry point demonstrating library usage
- `src/cities.rs` - City data structures and Hanseatic League city database
- `design/overview.md` - Comprehensive game design document

### Cities Database

The `cities.rs` module contains:
- `City` struct with name, population, and geographic coordinates
- `get_hanseatic_cities()` function returning ~50 historical cities
- Major Baltic Sea ports, North Sea ports, and river-connected trading centers
- Historically accurate population estimates and coordinates

## Game Design

The project aims to create a realistic medieval trading simulation featuring:

- **Information Asymmetry**: Players only know local information
- **Real-time Economic Simulation**: Continuous market updates with automation
- **Ship Programming**: Visual and scripting interfaces for complex operations
- **Historical Accuracy**: Authentic geography and trade networks
- **Multiplayer Dynamics**: Visible consequences for poor planning

### Time System

- **Base Rate**: 72 seconds real-time = 1 game day
- **Fast-Forward**: Players can request time acceleration (requires consensus)
- **Event Interruption**: Messages and events interrupt fast-forward mode

### Economic Model

- **20-30 Trade Goods**: Mix of raw materials and manufactured products
- **Dynamic Pricing**: Supply/demand driven with market depth based on city size
- **AI Trading**: Automated market stabilization with realistic information delays
- **Manufacturing**: Player-owned production facilities in cities

## Testing

Tests are embedded in each module using `#[cfg(test)]`. The test suite includes:
- Unit tests for utility functions  
- City data validation (coordinate bounds checking)
- City count verification

Run tests with:
```bash
cargo test
```

## Development Status

**Current Phase**: Early development with basic library structure

**Completed**:
- ✅ Project structure and build system
- ✅ Comprehensive city database with historical data
- ✅ Basic library functions with full test coverage
- ✅ Design documentation

**Next Steps**:
- Economic simulation engine
- Ship management system
- Basic multiplayer infrastructure
- Simple ship programming interface

## Contributing

This project follows standard Rust development practices:

1. Run `cargo fmt` before committing
2. Ensure `cargo clippy` passes without warnings
3. All tests must pass (`cargo test`)
4. Follow existing code style and patterns

## License

[License information to be added]

## Historical Context

The Hanseatic League was a medieval commercial and defensive confederation of merchant guilds and market towns in Northwestern and Central Europe. At its height (14th-15th centuries), it dominated Baltic maritime trade and had significant influence over North Sea and inland trade routes.

This game recreates the authentic challenges faced by Hanseatic merchants: managing trade operations across vast distances with limited communication, navigating complex supply chains, and competing in markets with incomplete information.