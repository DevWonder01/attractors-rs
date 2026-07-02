# Attractor 3D

A real-time, interactive 3D simulation of various chaotic strange attractors, built in Rust using Macroquad and Egui.

## Description

This project visualizes mathematical systems known as strange attractors. It numerically integrates these differential equations over time to produce continuous 3D paths. The simulation includes a built-in graphical user interface that allows you to switch between different attractor models on the fly, tweak their internal parameters, and adjust the camera to explore their unique geometric properties in 3D space.

<img width="1842" height="1158" alt="image" src="https://github.com/user-attachments/assets/c3f5962f-08b9-4a83-b54c-34c85497217d" />


## Features

- Real-time 3D rendering of complex mathematical attractor paths.
- Interactive UI to change simulation parameters instantly.
- Smooth camera controls with an auto-rotation feature.
- Seamless switching between 10 different mathematical models.
- Dynamically rendered path trails to visualize the chaotic flow.

## Supported Attractors

The following chaotic systems are currently implemented:

- Lorenz
- Thomas
- Chen
- Dadras
- Rossler
- Halvorsen
- Rabinovich-Fabrikant
- Three Scroll
- Sprott
- Four Wing
- Langford (Aizawa)

## Installation and Running

Ensure you have Rust and Cargo installed on your system. 

1. Navigate to the project directory.
2. Run the project using cargo:

```bash
cargo run --release
```

Note: Running in release mode is highly recommended as numerical integration and 3D rendering benefit significantly from compiler optimizations.

## Controls

- **Attractor Selection**: Use the on-screen "Controls" window to select the desired attractor type from the dropdown menu.
- **Parameters**: Adjust the mathematical parameter sliders to see the attractor morph in real-time.
- **Visuals**: Toggle "Show Particles" to enable/disable the 500-particle swarm, and adjust "Sim Speed (dt)" to control integration speed.
- **Camera Position**: Toggle "Auto Rotate" to have the camera continuously orbit, or manually adjust the Cam X, Y, and Z sliders.

## Dependencies

- macroquad: For the core simulation loop and 3D rendering capabilities.
- egui-macroquad: For the immediate mode graphical user interface elements.
