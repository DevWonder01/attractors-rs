# Attractor 3D

A real-time, interactive 3D simulation of various chaotic strange attractors, built in Rust using Macroquad and Egui.

## Description

This project visualizes mathematical systems known as strange attractors. It numerically integrates these differential equations over time to produce continuous 3D paths. The simulation includes a built-in graphical user interface that allows you to switch between different attractor models on the fly, tweak their internal parameters, and adjust the camera to explore their unique geometric properties in 3D space.

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

## Installation and Running

Ensure you have Rust and Cargo installed on your system. 

1. Navigate to the project directory.
2. Run the project using cargo:

```bash
cargo run --release
```

Note: Running in release mode is highly recommended as numerical integration and 3D rendering benefit significantly from compiler optimizations.

## Controls

- Use the on-screen "Controls" window to select the desired attractor type from the dropdown menu.
- Adjust the sliders to change specific mathematical parameters (e.g., Sigma, Rho, Beta) and see the attractor morph in real-time.
- Toggle "Auto Rotate" to have the camera continuously orbit the center of the scene.
- Manually adjust the "Cam X", "Cam Y", and "Cam Z" sliders to change the viewing angle and explore the structures.

## Dependencies

- macroquad: For the core simulation loop and 3D rendering capabilities.
- egui-macroquad: For the immediate mode graphical user interface elements.
