# Bevy Pathfinding Demo

<p align="center">
    <img src="./assets/icon.webp" alt="Bevy Pathfinding Demo" width="200" height="200"/>
</p>

Example of how to implement pathfinding in Bevy operating on a 2D tilemap.

## Overview

This small demo shows how to implement A\* pathfinding in Bevy, using a tilemap to represent the landscape of the world.

First, a `Map` is generated, formed of a 2D array of `Tiles`.
A visual representation of the map is then created by using the `Tilemap` plugin.

Starting and finishing points can be set by hovering the cursor over the map, and pressing the `S` and `F` keys respectively.
The path is then calculated using the A\* algorithm, and displayed on the map, when the `Space` key is pressed.

## Running the demo

After cloning the repository, navigate to the top-level directory:

```bash
git clone https://github.com/FreddyWordingham/Pathfinding.git
cd Pathfinding
```

Compile and run the application by execute the following command:

```bash
cargo run --release
```

## Controls

### Camera

- `W` - Move the camera up
- `A` - Move the camera left
- `S` - Move the camera down
- `D` - Move the camera right
- `Q` - Zoom out
- `E` - Zoom in

### Map

- `Left Click` - Set a wall
- `Right Click` - Remove a wall

### Pathfinding

- `S` - Set the starting point
- `F` - Set the finishing point
- `Space` - Calculate the path
