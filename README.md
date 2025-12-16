# Plants vs. Zombies Clone (Bevy)

A "Plants vs. Zombies" clone built using the [Bevy game engine](https://bevyengine.org/) (v0.13) in Rust. This project runs natively on desktop and compiles to WebAssembly (WASM) to run in the browser as a Progressive Web App (PWA).

## Features

-   **Game Loop**: Complete core game loop with state management.
-   **Plants**:
    -   **Sunflower**: Generates sun currency.
    -   **Peashooter**: Shoots straight-flying peas.
    -   **SnowPea**: Shoots peas that slow down zombies (blue tint).
    -   **WallNut**: High health defensive barrier.
    -   **PotatoMine**: Explodes on contact with zombies after arming time.
-   **Zombies**: Basic enemies that spawn and move towards the house.
-   **Economy**: Sun collection system to purchase plants.
-   **Grid System**: 9x5 grid for plant placement.
-   **UI/HUD**: Plant selection, sun counter, and cost indicators.
-   **Cross-Platform**: Runs on Linux/Windows/macOS and Web (WASM).
-   **PWA**: Installable as a Progressive Web App.

## Prerequisites

-   [Rust](https://www.rust-lang.org/tools/install) (latest stable)
-   [Trunk](https://trunkrs.dev/) (for web build): `cargo install --locked trunk`
-   WASM target: `rustup target add wasm32-unknown-unknown`

## How to Run

### Native (Desktop)

```bash
cargo run
```

### Web (WASM)

```bash
trunk serve
```
Open your browser to `http://localhost:8080`.

## Controls

-   **Mouse Click**: Select plants from the top HUD.
-   **Mouse Click (Grid)**: Place the selected plant on the lawn (if you have enough sun).

## Project Structure

```
├── src/
│   ├── main.rs          # Entry point, App configuration
│   ├── components.rs    # ECS Components (Plant, Zombie, etc.)
│   ├── resources.rs     # ECS Resources (GameState, Sun, etc.)
│   ├── constants.rs     # Game constants (Grid size, Z-indices)
│   └── systems/         # Game logic systems
│       ├── setup.rs     # Camera and Board setup
│       ├── ui.rs        # HUD and Interface logic
│       ├── combat.rs    # Shooting, damage, and collision
│       ├── spawning.rs  # Plant and Zombie spawning logic
│       └── ...
├── assets/              # Game assets
├── public/              # Static web assets (manifest.json, icons)
├── index.html           # Web entry point
└── Cargo.toml           # Dependencies
```

## License

This project is for educational purposes.
