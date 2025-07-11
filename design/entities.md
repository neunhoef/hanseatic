# Entities in the Hanseatic game

List of entities:

 - goods
 - cities
 - trade routes
 - players
 - kontors
 - storage space
 - production capabilities
 - ships
 - messages
 - commands/scripts
 - events
 - news


## Relationships of entities

Trade routes connect cities, together the form the city graph.

Goods exist in quantities and must always be located somewhere. They are not
represented by objects, but rather as counts in other entities. For example,
a ship can have a number of units of wool on board. So the wool is represented
as a number with the ship. To this end we have the type of a vector of goods,
which has a quantity for each possible good.

Players own kontors and ships. Storage space and production capabilities
are owned by kontors. Ships are configured with commands.

Messages travel and arrive, at which point they are visible to the players.
News are public messages.


## The simulation

The simulation constantly computes developments in cities, markets,
ships and moves messages to make them available.

To this end it stores the current state in each city and on each ship through
the time. However, old states must be retained for the following reason:
If player P is in some city C1 at time T and looks at the state of city C2, then
he sees the the state of C2 at time T-D, where D is the travel time from C2 to C1.
Since there is a maximal travel time between any two cities, the time span which
needs to be retained is also finite.

Basically, all state is stored in a deque data structure, where new versions
of the state are appended at the end and eventually old states are discarded from
the beginning.


## Client/server architecture

There is a central game server which runs the simulation. Players interact with
it through a client. They essentially control their ships and kontors, give commands
for buying, selling and building, and do all this on the grounds of information
they get presented about the various entities.


## The API of the server

The following API operations are needed:

 - register a new player (only in the beginning, before time starts)
 - start game
 - stop game
 - get stats and metrics

The following API operations are specific for a player and use that player and
its current position to reflect the player's perspective on things.

 - get player overview (position, capital, owned ships, owned kontors, overall stock
 - get overall game state (time, current conditions, news)
 - get player position and travel state
 - order player travels
 - see "current" prices and supply and demand and dynamics (player perspective)
 - get current kontor information (overview)
 - get current kontor information about one kontor
 - buy storage space for one kontor (effective with potential time delay)
 - buy production capabilities for one kontor (effective with potential time delay)
 - get current ship information (overview)
 - get current ship informatin about one ship
 - buy ship (effective with potential time delay)
 - edit command list for a ship
 - get available messages
 - send message to player
 - edit command list for a kontor


## The UI of the client

We have the following views with obvious links:

 - dashboard with overview
 - kontor list
 - kontor view
 - ship list
 - ship view
 - city list
 - city view
 - map view (visualize distances)
 - messages view (switch publications, business messages and private player messages)
 - fast forward view

