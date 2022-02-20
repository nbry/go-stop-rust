### run (fn)
```
fn run() {
	// Start new game
	// Run game loop
}
```

### Game (struct)
```
{
	players: GamePlayers
	round_number: i8
	settings: Settings
}

impl Game {
	new() {}

	initiate_game_loop{
		while "not round limit reached" or "any player has 0 chips" {
			Round::run()
			}
		}
	}
```

### Round (struct)
