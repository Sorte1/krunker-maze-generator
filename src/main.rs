use clap::Parser;
use image::{Rgb, RgbImage};
use rand::{seq::SliceRandom, thread_rng};
use serde_json::json;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

/// Simple maze generator, solver, and exporter
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Maze width in cells
    #[arg(short, long, default_value_t = 50)]
    width: usize,
    /// Maze height in cells
    #[arg(short, long, default_value_t = 50)]
    height: usize,
    /// Pixel size of each cell
    #[arg(short = 's', long, default_value_t = 40)]
    cell_size: usize,
    /// Wall thickness in pixels
    #[arg(short = 't', long, default_value_t = 4)]
    wall_thickness: usize,
    /// Output image file path
    #[arg(short, long, default_value = "maze.png")]
    image: PathBuf,
    /// Output map JSON file path
    #[arg(short = 'm', long, default_value = "map.json")]
    map: PathBuf,
    /// Skip JSON map generation
    #[arg(long)]
    no_map: bool,
}

pub struct Maze {
    pub width: usize,
    pub height: usize,
    pub vert_walls: Vec<Vec<bool>>,
    pub hor_walls: Vec<Vec<bool>>,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Self {
        let vert_walls = vec![vec![true; width + 1]; height];
        let hor_walls = vec![vec![true; width]; height + 1];
        Maze {
            width,
            height,
            vert_walls,
            hor_walls,
        }
    }

    pub fn generate(&mut self) {
        let mut visited = vec![vec![false; self.width]; self.height];
        let mut stack = Vec::new();
        visited[0][0] = true;
        stack.push((0, 0, 0));

        while let Some((x, y, dir_idx)) = stack.pop() {
            let mut dirs = vec![
                (1isize, 0isize, 'R'),
                (-1, 0, 'L'),
                (0, 1, 'D'),
                (0, -1, 'U'),
            ];
            dirs.shuffle(&mut thread_rng());
            for i in dir_idx..dirs.len() {
                let (dx, dy, dir) = dirs[i];
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0 && nx < self.width as isize && ny >= 0 && ny < self.height as isize {
                    let (nx, ny) = (nx as usize, ny as usize);
                    if !visited[ny][nx] {
                        match dir {
                            'R' => self.vert_walls[y][x + 1] = false,
                            'L' => self.vert_walls[y][x] = false,
                            'D' => self.hor_walls[y + 1][x] = false,
                            'U' => self.hor_walls[y][x] = false,
                            _ => {}
                        }
                        stack.push((x, y, i + 1));
                        visited[ny][nx] = true;
                        stack.push((nx, ny, 0));
                        break;
                    }
                }
            }
        }
    }

    pub fn solve(&self) -> Vec<(usize, usize)> {
        let total = self.width * self.height;
        let start = 0;
        let goal = total - 1;
        let mut g_score = vec![usize::MAX; total];
        let mut came_from = HashMap::new();
        let mut open = BinaryHeap::new();
        g_score[start] = 0;
        let h = |idx: usize| {
            let x = (idx % self.width) as isize;
            let y = (idx / self.width) as isize;
            let gx = (self.width - 1) as isize;
            let gy = (self.height - 1) as isize;
            ((gx - x).abs() + (gy - y).abs()) as usize
        };
        open.push((Reverse(h(start)), start));

        while let Some((_, current)) = open.pop() {
            if current == goal {
                break;
            }
            let cx = current % self.width;
            let cy = current / self.width;
            let neighbors = [
                (cx.wrapping_sub(1), cy, cx > 0 && !self.vert_walls[cy][cx]),
                (
                    cx + 1,
                    cy,
                    cx + 1 < self.width && !self.vert_walls[cy][cx + 1],
                ),
                (cx, cy.wrapping_sub(1), cy > 0 && !self.hor_walls[cy][cx]),
                (
                    cx,
                    cy + 1,
                    cy + 1 < self.height && !self.hor_walls[cy + 1][cx],
                ),
            ];
            for &(nx, ny, ok) in &neighbors {
                if !ok || nx >= self.width || ny >= self.height {
                    continue;
                }
                let neighbor = ny * self.width + nx;
                let tentative = g_score[current] + 1;
                if tentative < g_score[neighbor] {
                    g_score[neighbor] = tentative;
                    came_from.insert(neighbor, current);
                    open.push((Reverse(tentative + h(neighbor)), neighbor));
                }
            }
        }

        let mut path = Vec::new();
        let mut cur = goal;
        while let Some(&p) = came_from.get(&cur) {
            path.push((cur % self.width, cur / self.width));
            cur = p;
        }
        path.push((0, 0));
        path.reverse();
        path
    }

    pub fn draw(&self, cell_size: usize, wall_thick: usize) -> RgbImage {
        let img_w = (self.width * cell_size + wall_thick) as u32;
        let img_h = (self.height * cell_size + wall_thick) as u32;
        let mut img = RgbImage::new(img_w, img_h);
        let white = Rgb([255, 255, 255]);
        let black = Rgb([0, 0, 0]);
        let red = Rgb([255, 0, 0]);

        for x in 0..img_w {
            for y in 0..img_h {
                img.put_pixel(x, y, white);
            }
        }
        for y in 0..self.height {
            let y0 = (y * cell_size) as u32;
            for x in 0..=self.width {
                if self.vert_walls[y][x] {
                    let x0 = (x * cell_size) as u32;
                    for dx in 0..wall_thick as u32 {
                        for dy in 0..cell_size as u32 {
                            img.put_pixel(x0 + dx, y0 + dy, black);
                        }
                    }
                }
            }
        }
        for y in 0..=self.height {
            let y0 = (y * cell_size) as u32;
            for x in 0..self.width {
                if self.hor_walls[y][x] {
                    let x0 = (x * cell_size) as u32;
                    for dx in 0..cell_size as u32 {
                        for dy in 0..wall_thick as u32 {
                            img.put_pixel(x0 + dx, y0 + dy, black);
                        }
                    }
                }
            }
        }
        let thickness = (cell_size as u32) / 2;
        for window in self.solve().windows(2) {
            let (x1, y1) = window[0];
            let (x2, y2) = window[1];
            let cx1 = x1 as u32 * cell_size as u32 + cell_size as u32 / 2;
            let cy1 = y1 as u32 * cell_size as u32 + cell_size as u32 / 2;
            let cx2 = x2 as u32 * cell_size as u32 + cell_size as u32 / 2;
            let cy2 = y2 as u32 * cell_size as u32 + cell_size as u32 / 2;
            if cx1 == cx2 {
                let x0 = cx1.saturating_sub(thickness / 2);
                let h = (cy2 as i32 - cy1 as i32).abs() as u32;
                let y_min = cy1.min(cy2);
                for dx in 0..thickness {
                    for dy in 0..=h {
                        img.put_pixel(x0 + dx, y_min + dy, red);
                    }
                }
            } else {
                let y0 = cy1.saturating_sub(thickness / 2);
                let w = (cx2 as i32 - cx1 as i32).abs() as u32;
                let x_min = cx1.min(cx2);
                for dy in 0..thickness {
                    for dx in 0..=w {
                        img.put_pixel(x_min + dx, y0 + dy, red);
                    }
                }
            }
        }
        img
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut maze = Maze::new(args.width, args.height);
    println!("Generating maze {}x{}...", args.width, args.height);
    maze.generate();

    println!("Drawing maze to image ({} )...", args.image.display());
    let img = maze.draw(args.cell_size, args.wall_thickness);
    img.save(&args.image)?;
    println!("Image saved to {}", args.image.display());

    if !args.no_map {
        println!("Generating JSON map to {}...", args.map.display());
        let mut segments = Vec::new();
        for x in 0..=args.width {
            let mut y = 0;
            while y < args.height {
                if maze.vert_walls[y][x] {
                    let y1 = y;
                    let mut y2 = y + 1;
                    while y2 < args.height && maze.vert_walls[y2][x] {
                        y2 += 1;
                    }
                    segments.push(json!({"type":"vertical","x":x,"y1":y1,"y2":y2}));
                    y = y2;
                } else {
                    y += 1;
                }
            }
        }
        for y in 0..=args.height {
            let mut x = 0;
            while x < args.width {
                if maze.hor_walls[y][x] {
                    let x1 = x;
                    let mut x2 = x + 1;
                    while x2 < args.width && maze.hor_walls[y][x2] {
                        x2 += 1;
                    }
                    segments.push(json!({"type":"horizontal","y":y,"x1":x1,"x2":x2}));
                    x = x2;
                } else {
                    x += 1;
                }
            }
        }
        let fw = (args.width * args.cell_size) as i32;
        let fd = (args.height * args.cell_size) as i32;
        let mut sizes = vec![fw, 1, fd];
        let mut objects = vec![json!({ "p": [fw/2, -1, fd/2], "si": 0 })];
        for (i, seg) in segments.iter().enumerate() {
            let si = i + 1;
            if seg["type"] == json!("vertical") {
                let x = seg["x"].as_i64().unwrap() as i32 * args.cell_size as i32;
                let y1 = seg["y1"].as_i64().unwrap() as i32 * args.cell_size as i32;
                let y2 = seg["y2"].as_i64().unwrap() as i32 * args.cell_size as i32;
                let length = y2 - y1;
                sizes.extend([args.wall_thickness as i32, 20, length]);
                objects.push(json!({ "p": [x, 0, (y1 + y2) / 2], "si": si }));
            } else {
                let y = seg["y"].as_i64().unwrap() as i32 * args.cell_size as i32;
                let x1 = seg["x1"].as_i64().unwrap() as i32 * args.cell_size as i32;
                let x2 = seg["x2"].as_i64().unwrap() as i32 * args.cell_size as i32;
                let length = x2 - x1;
                sizes.extend([length, 20, args.wall_thickness as i32]);
                objects.push(json!({ "p": [(x1 + x2) / 2, 0, y], "si": si }));
            }
        }
        let half = (args.cell_size as i32) / 2;
        let start_spawn = json!([half, 0, half, 0, 0, 0]);
        let end_spawn = json!([fw - half, 0, fd - half, 0, 0, 0]);
        let map = json!({
            "name": "GeneratedMaze",
            "ambient": "#97a0a8",
            "light": "#f2f8fc",
            "sky": "#dce8ed",
            "fog": "#8d9aa0",
            "fogD": 2000,
            "xyz": sizes,
            "objects": objects,
            "spawns": [ start_spawn, end_spawn ]
        });
        let mut f = File::create(&args.map)?;
        f.write_all(serde_json::to_string_pretty(&map)?.as_bytes())?;
        println!("Map JSON saved to {}", args.map.display());
    }

    Ok(())
}
