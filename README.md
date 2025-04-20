# Maze Generator, Solver, and Exporter

A simple command-line tool written in Rust to generate, solve, and export mazes. It can produce a PNG image of the maze (with solution path overlay) and an optional JSON file describing the maze walls for use in other applications.

## Features

- **Maze Generation**: Uses a depth‐first backtracking algorithm to carve a random perfect maze of customizable size.
- **Maze Solving**: A\* search finds the shortest path from the start (top-left) to the goal (bottom-right), which is then drawn in red.
- **Image Export**: Renders the maze into a PNG image with adjustable cell size and wall thickness.
- **JSON Map Export**: Outputs a `map.json` describing each wall segment (vertical and horizontal) for easy integration into 3D engines or other tools.
- **Flexible CLI**: Configure width, height, cell size, wall thickness, output paths, or skip JSON generation via flags.

## Installation

1. Ensure you have [Rust](https://rust-lang.org) and Cargo installed (Rust 1.60+ recommended).

2. Clone this repository:

   ```sh
   git clone https://github.com/yourusername/krunker-maze-generator.git
   cd krunker-maze-generator
   ```

3. Build the project in release mode:

   ```sh
   cargo build --release
   ```

4. The executable will be in `target/release/krunker-maze-generator`.

## Usage

```sh
USAGE:
    krunker-maze-generator [OPTIONS]

OPTIONS:
    -w, --width <width>             Maze width in cells [default: 100]
    -h, --height <height>           Maze height in cells [default: 100]
    -s, --cell-size <cell_size>     Pixel size of each cell [default: 40]
    -t, --wall-thickness <thick>    Wall thickness in pixels [default: 4]
    -i, --image <image>             Output image file path [default: "maze.png"]
    -m, --map <map>                 Output JSON map file path [default: "map.json"]
        --no-map                    Skip JSON map generation
    -V, --version                   Print version information
    -?, --help                      Print help information
```

### Examples

- Generate a default `100×100` maze and export both PNG and JSON:

  ```sh
  krunker-maze-generator
  ```

- Generate a `50×30` maze, small cells, thick walls:

  ```sh
  krunker-maze-generator -w 50 -h 30 -s 20 -t 8 -i output.png -m walls.json
  ```

- Generate only the PNG (skip JSON):

  ```sh
  krunker-maze-generator --no-map -i maze.png
  ```

## JSON Map Format

The JSON export describes each wall segment in the maze, along with global metadata:

```json
{
  "name": "GeneratedMaze",
  "ambient": "#97a0a8",
  "light": "#f2f8fc",
  "sky": "#dce8ed",
  "fog": "#8d9aa0",
  "fogD": 2000,
  "xyz": [<room_width>, 1, <room_height>, <w1>, 20, <l1>, ...],
  "objects": [
    { "p": [x, y, z], "si": 0 },
    { "p": [x1, 0, z1], "si": 1 },
    ...
  ],
  "spawns": [
    [startX, 0, startZ, 0, 0, 0],
    [endX, 0, endZ, 0, 0, 0]
  ]
}
```

- ``: Width, elevation, depth of the floor, followed by triples of (width, height, thickness) for each wall segment.
- ``: Position `p` and size index `si` for each object (0 is the floor).
- ``: Two spawn points for the start and end of the maze.

Refer to the source code for details on how segments are collected.

## Contributing

1. Fork the repository.
2. Create a feature branch (`git checkout -b feature/YourFeature`).
3. Commit your changes (`git commit -m "Add feature"`).
4. Push to your branch (`git push origin feature/YourFeature`).
5. Open a pull request.

Please ensure your code is formatted with `cargo fmt` and linted with `cargo clippy`.

## License

This project is licensed under the [MIT License](LICENSE).

