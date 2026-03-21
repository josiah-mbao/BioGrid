# BioGrid

**BioGrid** is a grid-based ecosystem simulation game built in **Rust** using the **Bevy** game engine.

The goal of the project is to create an **infinite tile-based world** where autonomous organisms interact through simple biological rules such as:

- **Predation**
- **Resource consumption**
- **Energy metabolism**
- **Reproduction and mutation**

Over time, these simple systems combine to produce **emergent ecosystem behavior**.

The project is also an experiment in **clean architecture**, **DevOps practices**, and **data-oriented design** from the very beginning.

---

## Vision

BioGrid aims to simulate a living ecosystem inside an **infinite procedural grid world**. Organisms operate based on simple rules:

- **Predators** hunt prey
- **Prey** search for food
- **Energy** determines survival
- **Successful organisms** reproduce with mutations

Rather than scripting complex behavior, the system focuses on **minimal rules that generate complex outcomes**.

---

## Core Features (Planned)

- Infinite tile-based world
- Grid-snapped movement
- Autonomous organisms
- Predator–prey interactions
- Energy metabolism and death
- Reproduction with mutation
- Procedural chunk loading
- Emergent ecosystem dynamics

---

## Tech Stack

- **Language:** Rust
- **Engine:** Bevy game engine
- **Level Design:** LDtk
- **Architecture:** Entity Component System (**ECS**)

Development also emphasizes modern engineering practices such as:
- **Containerization** with Docker
- **CI pipelines** with GitHub Actions
- **Test-driven development (**TDD**)**

---

## Project Structure

```text biogrid/ ├── assets/              # World templates and sprites ├── src/ │   ├── main.rs          # Application entry point │   ├── components.rs    # **ECS** data structures │   ├── systems/         # Simulation logic │   │   ├── bio.rs │   │   └── world.rs │   └── map_loader.rs    # LDtk world loading ├── tests/               # Simulation logic tests ├── Dockerfile ├── docker-compose.yml └── Cargo.toml ```

---

## Development Principles

This project intentionally prioritizes:
- **Simplicity** over complexity
- **Data-oriented design**
- **Testable** simulation logic
- **Clear separation** between simulation and rendering

Game logic is written so it can be tested independently of the rendering engine.

---

## Running the Project

Clone the repository and run: ```bash cargo run ```

To run tests: ```bash cargo test ```

---

## Long-Term Goals

- Support hundreds or thousands of autonomous agents
- Observe emergent ecological behavior
- Experiment with evolutionary simulation
- Maintain a clean and maintainable Rust codebase

---

## License

This project is currently under development.

---
