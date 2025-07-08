# Hanseatic Trading Game - Design Document

## Executive Summary

This game is a multiplayer trading simulation set in the Hanseatic League era (circa 1400 CE) around the North and Baltic Seas. Players take on the role of merchants building trading empires across approximately 50 cities. The core innovation lies in realistic information asymmetry: players can only be in one location at a time, and all information travels at the speed of ships, creating authentic medieval trading challenges.

The game combines real-time elements with strategic automation, requiring players to program their ships and trading operations while managing incomplete, time-delayed information about distant markets.

## Game Overview

### Setting and Theme
- **Time Period**: Circa 1400 CE
- **Geographic Scope**: North and Baltic Sea regions under Hanseatic League influence
- **Player Role**: Merchant building a trading empire
- **Core Challenge**: Managing trade operations with realistic communication delays and information asymmetry

### Key Design Principles
1. **Information travels at ship speed** - No instant communication exists
2. **Physical presence matters** - Players can only be in one location at a time
3. **Automation necessity** - Complex operations require programmed ship behaviors
4. **Economic realism** - Supply and demand drive prices with realistic market depth
5. **Social interaction** - Multiplayer dynamics with visible consequences for poor planning

## Core Game Mechanics

### Time and Multiplayer System

#### Time Progression
- **Base Rate**: 72 seconds real-time = 1 game day (3 seconds real-time = 1 game hour)
- **Scaling**: Designed for small empires initially, with automation enabling management of larger operations
- **Fast-Forward Mechanism**: 
  - Players can request fast-forward to specific future times
  - Game advances to earliest time agreed upon by ALL players
  - Automatic interruption on events or message arrivals

#### Multiplayer Scope
- **Target**: 1-6 players per game instance
- **Consensus-based timing**: All players must agree for time acceleration
- **Competitive elements**: Visible mistakes (idle ships) create strategic opportunities

### Information Asymmetry System

#### Core Principle
Players only access information available at their current location, with all distant information time-delayed by travel time.

#### Information Types
- **Price Data**: Available for all cities, but delayed by travel time from player's current location
- **Market Conditions**: Supply/demand visible with same time delays
- **Event Notifications**: Players only learn of events when news reaches them
- **Ship Status**: Direct control only when co-located; otherwise requires message exchanges

#### Message System
- **Automatic Messages**: Price and market information transmitted continuously
- **Explicit Messages**: Player-to-ship commands for distant operations
- **Status Updates**: Idle ships send daily status messages
- **Event Interruption**: Message arrivals interrupt fast-forward mode

### Ship Management and Automation

#### Programming System
**Phase 1**: Simple command sequences
- Basic commands: `TRAVEL_TO city`, `SELL_ALL good`, `BUY good quantity`, `BUY good MAX`
- Linear program execution
- Discoverable GUI interface

**Phase 2**: Conditional logic and loops
- Visual programming interface (Scratch-style)
- Conditional statements and loop structures
- More sophisticated automation capabilities

**Phase 3**: Advanced scripting
- Custom script upload and execution
- Sandboxed environment for user code
- Power-user optimization tools

#### Operational Constraints
- **No complexity limits**: Real-time pressure naturally constrains over-engineering
- **Reliability**: Programs execute as written; market misinterpretation is player responsibility
- **Visibility**: Idle ships and poor management visible to competing players

### Economic Simulation

#### Market Structure
- **Goods**: 20-30 trade goods divided into:
  - 10 fundamental goods (timber, fish, grain, iron, salt, wool, etc.)
  - 10-20 manufactured goods produced from raw materials
- **Cities**: Approximately 50 cities with individual characteristics
- **Specialization**: Cities have fixed production specializations but variable sizes

#### Price Dynamics
**Base Formula**:
```
base_price = historical_equilibrium_price_for_good
city_market_depth = f(city_size)
supply_demand_ratio = current_supply / current_demand
price_multiplier = (supply_demand_ratio)^(-elasticity / city_market_depth)
current_price = base_price * price_multiplier
```

**Key Properties**:
- Small cities: High price volatility
- Large cities: Price stability and market depth
- Hourly price recalculation based on current supply/demand

#### Supply and Demand Mechanics
- **Demand Generation**: Continuous hourly increases proportional to city size
- **Supply Sources**: Player sales, city production, AI trading
- **Consumption**: Hourly reduction of both supply and demand
- **City Growth**: Linked to trade satisfaction and demand fulfillment

#### AI Trading System
- **Information Delay**: AI traders use same time-delayed information as players
- **Decision Process**: Hourly analysis of surplus/deficit opportunities
- **Action Delay**: Random delay (days) before AI acts on identified opportunities
- **Market Validation**: Shipments only proceed if conditions remain favorable
- **Transparency**: AI shipments published with appropriate time delays
- **Purpose**: Provide market stability while allowing player arbitrage opportunities

### Player Progression and Investment

#### Starting Position
- Two offices in different cities
- Basic ship fleet
- Initial capital for trading operations

#### Expansion Opportunities
- **Additional Offices**: Purchase office space in new cities for storage and operations
- **Manufacturing Investment**: Buy production capabilities in offices
  - Produces profit from converting raw materials
  - Requires initial capital investment
  - Adds to city's total manufacturing capacity
- **Fleet Expansion**: Acquire additional ships for increased trading capacity
- **Office Upgrades**: Expand storage capacity and manufacturing slots

#### Manufacturing System
- **City Base Production**: Automatic production based on city specialization
- **Player Enhancement**: Office-based manufacturing adds to city output
- **Resource Requirements**: Manufacturing requires appropriate raw material supply
- **Profit Generation**: Player-owned manufacturing generates ongoing revenue
- **Storage Integration**: Manufactured goods stored in player offices

### Transportation Network

#### Route Structure
- **Trade Route Graph**: Approximately 50 cities connected by realistic shipping routes
- **Travel Speed**: 7 knots (nautical miles per hour) base assumption
- **Direct Routing**: Ships travel directly between destinations without intermediate stops
- **Historical Accuracy**: Routes and distances based on real 1400 CE geography

#### Travel Mechanics
- **Predictable Timing**: Travel times are known and constant
- **Information Lag**: All distant information delayed by travel time
- **Command Execution**: Orders only executed upon ship arrival at destination

## Technical Architecture Considerations

### Server-Side Requirements
- **Real-time simulation**: Continuous economic simulation with hourly updates
- **Message routing**: Time-delayed information distribution system
- **AI trading logic**: Automated market stabilization with realistic delays
- **Player synchronization**: Consensus-based time acceleration management

### Client-Side Features
- **Information interface**: Time-stamped data presentation with delay indicators
- **Ship programming**: Visual and scripting interfaces for automation
- **Market analysis**: Tools for tracking price trends and identifying opportunities
- **Communication**: Message system for ship management and player interaction

## Future Extension Points

### Enhanced Economic Simulation
- Seasonal trading patterns and weather effects
- Political events and trade route disruptions
- More sophisticated manufacturing chains
- Banking and credit systems

### Advanced Gameplay Features
- Diplomatic negotiations between players
- Trading company partnerships and joint ventures
- Espionage and information trading
- Random events and historical scenarios

### Technical Enhancements
- Advanced AI trading behaviors
- More sophisticated ship programming languages
- Enhanced visualization and market analysis tools
- Mobile and cross-platform support

## Success Metrics

The game succeeds when players experience the authentic challenge of medieval trading: making strategic decisions with incomplete information, managing complex logistics through automation, and competing in a dynamic economic environment where planning, timing, and adaptation determine success.

Victory conditions and specific balancing parameters will be refined through playtesting, but the core experience should reward players who master the information asymmetry challenge while building efficient, automated trading networks.
