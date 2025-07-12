# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust library and binary crate for the "Hanseatic" project - a multiplayer trading simulation game set in the Hanseatic League era (circa 1400 CE). The project is in early development, currently containing basic library structure and a comprehensive list of Hanseatic League cities with historical data.

## Development Commands

### Build and Test
- `cargo build` - Build the project
- `cargo test` - Run all tests (includes unit tests in lib.rs and cities.rs)
- `cargo run` - Run the main binary application
- `cargo check` - Quick syntax and type checking

### Code Quality
- `cargo fmt` - Format code (standard Rust formatting)
- `cargo clippy` - Lint code for common issues

## Architecture

### Core Structure
- `src/lib.rs` - Main library with utility functions and module declarations
- `src/main.rs` - Binary entry point demonstrating library usage
- `src/cities.rs` - City data structures and Hanseatic League city database
- `design/overview.md` - Comprehensive game design document

### Key Components

#### Cities Module (`src/cities.rs`)
- `City` struct with name, population, and geographic coordinates
- `get_hanseatic_cities()` function returning Vec<City> with ~50 historical cities
- Cities include major Baltic Sea ports, North Sea ports, and river-connected trading centers
- All cities have historically accurate population estimates and coordinates

#### Library Functions (`src/lib.rs`)
- Basic utility functions for demonstration purposes
- Comprehensive test coverage for all functions
- Module structure designed for expansion

## Game Design Context

The project aims to create a realistic medieval trading simulation with:
- Information asymmetry (players only know local information)
- Real-time economic simulation with automation
- Ship programming and trade route management
- Historical accuracy in geography and trade networks

Refer to `design/overview.md` for complete game design specifications including economic mechanics, multiplayer systems, and technical architecture requirements.

## Testing

Tests are embedded in each module using `#[cfg(test)]`. All tests should pass before committing changes. The test suite includes:
- Unit tests for utility functions
- City data validation (coordinate bounds checking)
- City count verification