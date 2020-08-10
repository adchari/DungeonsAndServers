# DungeonsAndServers
A D&D/Adventure-esque multiplayer server

## Initial scope
- There will be a server written, which manages several games, each with a set of players
  - In order to accomplish this, whatever server client is written will need to identify games based on some discerning factors
  - In addition, a combination of a player ID and game ID should allow the server to discern which state machine to update. 
- Therefore, we can better define endpoints for the server, as well as their interaction with internal state. 
- There are several unique objects to generate and store, namely the game world, players, monsters, NPCs, items, and game objects
  - `/worlds/:key` will allow interaction with a specific world. Posting without an ID will give you a new ID, while PUT with a key will update that specific key. GET without a key returns all keys, GET with a key returns the object at that key if it exists, and DELETE requires a key but will remove that world
  - `/players/:key` will allow interaction with a specific player. Posting without an ID will give you a new ID, while PUT with a key will update that specific key. GET without a key returns all keys, GET with a key returns the object at that key if it exists, and DELETE requires a key but will remove that world
  - `/monsters/:key` will allow interaction with a specific monster manual. Posting without an ID will give you a new ID, while PUT with a key will update that specific key. GET without a key returns all keys, GET with a key returns the object at that key if it exists, and DELETE requires a key but will remove that world
  - `/npcs/:key` will allow interaction with a specific NPC set. Posting without an ID will give you a new ID, while PUT with a key will update that specific key. GET without a key returns all keys, GET with a key returns the object at that key if it exists, and DELETE requires a key but will remove that world
  - `/items/:key` will allow interaction with a specific item set. Posting without an ID will give you a new ID, while PUT with a key will update that specific key. GET without a key returns all keys, GET with a key returns the object at that key if it exists, and DELETE requires a key but will remove that world
  - `/game/create/:world_key/:monsters_key/:npc_key/:items_key` should be a simple POST request which looks up each of the specified objects, ensures that they are functional together (i.e. any references in the world to monsters and NPCs and items which are nonexistent in the other three should cause an error), and returns a game key. This should COPY the resources, so that the game object is independent of the resources. 
  - `/game/moves/:game_key/:player_key` is another GET endpoint which requests available moves for a player. On the first request from a player ID, this drops them in the starting room and gives them the options. This should return a unique token as well, which will ensure that a player cannot queue up moves in advance. 
  - `/game/play/:game_key/:player_key/:token` is a POST endpoint which updates the game world as requested. A "move" from the available moves should be selected and returned to this endpoint, and the token ensures that this is the only move that the user has sent with this. Reusing a token causes the second move to be dropped. 
	- From a server perspective, this is super basic. For each game key we store a queue of incoming requests, as well as the players who have requested available moves and their relevant tokens. As requests get dequeued, we make sure the incoming request matches the token, if not we drop it. If the request includes a move that is no longer available, they are asked to try again, which can be done by hitting the moves endpoint again, generating the moves, and picking again. A CLI client could make this process easier, but this is the best way to ensure that players can interact with the game world without turns, but also mutate state. A system which requires every player to respond is asking for in-game failures, since any player using internet stops the gameplay entirely. Any CLI client should wait for the response from this endpoint before allowing a subsequent move.
   - `/game/end/:game_key/:player_key` is a DELETE request. If the player key is specified, it simply drops the player from that game, if it isn't, the game is killed. This may cause errors, but hopefully if users are collaborating, it should be fine. This also may allow game state to be saved for a period of time after all players leave, allowing some to rejoin in the future. 
- There should be a client package written, most likely a CLI application capable of wrapping these requests in convenient ways, so that users can just interact with that.

## Schemas
- I should define a schema for players, NPCs, monsters, items, locations. 