# PixelPal

A desktop AI pet companion built with [Bevy](https://bevy.org/) game engine.

## Features

### Desktop Companion
- Transparent, always-on-top window
- Draggable to any position on screen
- Animated robot pet with procedural sprite generation

### Pet Stats System
Three core stats that decay over time:
- **Hunger** 🍎 - Satisfied by feeding
- **Happiness** 💕 - Increased by petting
- **Energy** ⚡ - Restored by sleeping

The pet's mood changes based on stat levels:
- **Happy** - All stats are healthy
- **Hungry** - Hunger is low
- **Sleepy** - Energy is low
- **Sad** - Happiness is low

### Interactive Actions
Click on the pet or press `Space` to toggle the action menu:
- **Feed** - Restore hunger
- **Pet** - Increase happiness
- **Dance** - Play a fun animation
- **Sleep** - Restore energy
- **Talk** - Interact with the pet

### Mini-Games

#### Reaction Game
Press `R` to start a reaction test:
1. Wait for the "GO!" prompt
2. Press `Space` as fast as possible
3. Your reaction time is recorded

#### Combo System
Click on the pet 3 times within 0.65 seconds to trigger a hidden dance combo!

### Achievement System
Unlock achievements through gameplay:
- **First Feed** - Feed the pet for the first time
- **Pet Lover** - Pet the pet 10 times
- **Combo Starter** - Trigger a combo
- **Reflex Ace** - Achieve reaction time ≤ 350ms

## Building

```bash
cargo build --release
```

## Running

```bash
cargo run --release
```

## Controls

| Key | Action |
|-----|--------|
| `Left Click` (on pet) | Toggle menu / Combo trigger |
| `Left Click` (outside) | Hide menu |
| `Space` | Toggle action menu |
| `R` | Start reaction game |

## Project Structure

```
src/
├── main.rs          # App entry point and system scheduling
├── pet.rs           # Pet state and stat decay logic
├── window.rs        # Window configuration
├── animation/       # Sprite animation system
├── fun/             # Mini-games and achievements
└── ui/              # User interface components
```

## License

Licensed under the Apache License 2.0. See [LICENSE](LICENSE) for details.
