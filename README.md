## Maze Generator, Solver, and Exporter

A simple command-line tool written in Rust to generate, solve, and export mazes. It can produce a PNG image of the maze (with solution path overlay) and an optional JSON file describing the maze walls for use in other applications.

## Features

- **Maze Generation**: Uses a depth‑first backtracking algorithm to carve a random maze of customizable size.
- **Maze Solving**: Generates a solution path in the exported image.
- **Image Export**: Renders the maze into a PNG image with adjustable cell size and wall thickness.
- **JSON Map Export**: Outputs a `map.json` in the format for krunker.io (tested version 7.5.0).
- **Flexible CLI**: Configure width, height, cell size, wall thickness, output paths, or skip JSON generation via flags.

## Installation

1. Ensure you have [Rust](https://rust-lang.org) and Cargo installed.

2. Clone this repository:

   ```sh
   git clone https://github.com/Sorte1/krunker-maze-generator.git
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
    -W, --width <width>             Maze width in cells [default: 100]
    -H, --height <height>           Maze height in cells [default: 100]
    -S, --cell-size <cell_size>     Pixel size of each cell [default: 40]
    -T, --wall-thickness <thick>    Wall thickness in pixels [default: 4]
    -I, --image <image>             Output image file path [default: "maze.png"]
    -M, --map <map>                 Output JSON map file path [default: "map.json"]
        --no-map                    Skip JSON map generation
    -V, --version                   Print version information
    -h, --help                      Print help information
```

### Examples

- Generate a default `100×100` maze and export both PNG and JSON:

  ```sh
  krunker-maze-generator
  ```

- Generate a `50×30` maze, small cells, thick walls:

  ```sh
  krunker-maze-generator -W 50 -H 30 -S 20 -T 8 -I output.png -M walls.json
  ```

- Generate only the PNG (skip JSON):

  ```sh
  krunker-maze-generator --no-map -i maze.png
  ```

## Using as a Library

1. Add the dependency in your `Cargo.toml`:

   ```toml
   [dependencies]
   krunker-maze-generator = { git = "https://github.com/Sorte1/krunker-maze-generator", tag = "v0.2.0" }
   ```

   Or, if you published to crates.io:

   ```toml
   [dependencies]
   krunker-maze-generator = "0.2.0"
   ```

2. In your code, import and use the `Maze` API:

   ```rust
   use krunker_maze_generator::Maze;
   use std::path::Path;

   fn main() {
       // Create and generate a 50×50 maze
       let mut maze = Maze::new(50, 50);
       maze.generate();

       // Draw to an image
       let img = maze.draw(20, 4);
       img.save(Path::new("custom_maze.png")).unwrap();

       // Optionally, generate the JSON map:
       let map_json = maze.to_map_json(20, 4);
       std::fs::write("custom_map.json", serde_json::to_string_pretty(&map_json).unwrap()).unwrap();
   }
   ```

## JSON Map Format


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


## Contributing

1. Fork the repository.
2. Create a feature branch (`git checkout -b feature/YourFeature`).
3. Commit your changes (`git commit -m "Add feature"`).
4. Push to your branch (`git push origin feature/YourFeature`).
5. Open a pull request.

Please ensure your code is formatted with `cargo fmt` and linted with `cargo clippy`.

## License

This project is licensed under the [MIT License](LICENSE).

