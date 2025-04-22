// src/lib.rs

use image::{Rgb, RgbImage};
use rand::{rng, seq::SliceRandom};
use serde_json::json;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

/// The core maze data (cells & walls) and all operations on it.
pub struct Maze {
    pub width: usize,
    pub height: usize,
    pub vert_walls: Vec<Vec<bool>>,
    pub hor_walls: Vec<Vec<bool>>,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Self {
        let vert_walls = vec![vec![true; width + 1]; height];
        let hor_walls  = vec![vec![true; width]; height + 1];
        Maze { width, height, vert_walls, hor_walls }
    }

    pub fn generate(&mut self) {
        let mut visited = vec![vec![false; self.width]; self.height];
        let mut stack   = Vec::new();
        visited[0][0] = true;
        stack.push((0, 0, 0));

        while let Some((x, y, dir_idx)) = stack.pop() {
            let mut dirs = vec![
                (1isize, 0isize, 'R'),
                (-1, 0, 'L'),
                (0, 1, 'D'),
                (0, -1, 'U'),
            ];
            dirs.shuffle(&mut rng());

            for i in dir_idx..dirs.len() {
                let (dx, dy, dir) = dirs[i];
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0 && nx < self.width as isize && ny >= 0 && ny < self.height as isize {
                    let (nx, ny) = (nx as usize, ny as usize);
                    if !visited[ny][nx] {
                        match dir {
                            'R' => self.vert_walls[y][x + 1] = false,
                            'L' => self.vert_walls[y][x]     = false,
                            'D' => self.hor_walls[y + 1][x]  = false,
                            'U' => self.hor_walls[y][x]      = false,
                            _   => {}
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

    /// Solve via A* from top‑left to bottom‑right
    pub fn solve(&self) -> Vec<(usize, usize)> {
        let total = self.width * self.height;
        let start = 0;
        let goal  = total - 1;

        let mut g_score = vec![usize::MAX; total];
        let mut came_from = HashMap::new();
        let mut open = BinaryHeap::new();

        // Heuristic: Manhattan to goal
        let h = |idx: usize| {
            let x = (idx % self.width)  as isize;
            let y = (idx / self.width)  as isize;
            let gx = (self.width - 1)   as isize;
            let gy = (self.height - 1)  as isize;
            ((gx - x).abs() + (gy - y).abs()) as usize
        };

        g_score[start] = 0;
        open.push((Reverse(h(start)), start));

        while let Some((_, current)) = open.pop() {
            if current == goal { break }
            let cx = current % self.width;
            let cy = current / self.width;

            let neighbors = [
                (cx.wrapping_sub(1), cy,        cx > 0                    && !self.vert_walls[cy][cx]),
                (cx + 1,         cy,        cx + 1 < self.width && !self.vert_walls[cy][cx + 1]),
                (cx,             cy.wrapping_sub(1), cy > 0              && !self.hor_walls[cy][cx]),
                (cx,             cy + 1,        cy + 1 < self.height && !self.hor_walls[cy + 1][cx]),
            ];

            for &(nx, ny, ok) in &neighbors {
                if !ok || nx >= self.width || ny >= self.height { continue }
                let neighbor = ny * self.width + nx;
                let tentative = g_score[current] + 1;
                if tentative < g_score[neighbor] {
                    g_score[neighbor] = tentative;
                    came_from.insert(neighbor, current);
                    open.push((Reverse(tentative + h(neighbor)), neighbor));
                }
            }
        }

        // Reconstruct path
        let mut path = Vec::new();
        let mut cur  = goal;
        while let Some(&p) = came_from.get(&cur) {
            path.push((cur % self.width, cur / self.width));
            cur = p;
        }
        path.push((0, 0));
        path.reverse();
        path
    }

    /// Draw maze + solution into an RGB image
    pub fn draw(&self, cell_size: usize, wall_thick: usize) -> RgbImage {
        let img_w = (self.width * cell_size + wall_thick) as u32;
        let img_h = (self.height * cell_size + wall_thick) as u32;
        let mut img = RgbImage::new(img_w, img_h);

        // Colors
        let white = Rgb([255, 255, 255]);
        let black = Rgb([0,   0,   0  ]);
        let red   = Rgb([255,   0,   0]);

        // Fill background
        for x in 0..img_w {
            for y in 0..img_h {
                img.put_pixel(x, y, white);
            }
        }

        // Walls
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

        // Solution path
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
                let w  = (cx2 as i32 - cx1 as i32).abs() as u32;
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

    /// Build the JSON segments and full map structure
    pub fn to_map_json(&self, cell_size: usize, wall_thick: usize) -> serde_json::Value {
        let mut segments = Vec::new();
        // vertical
        for x in 0..=self.width {
            let mut y = 0;
            while y < self.height {
                if self.vert_walls[y][x] {
                    let y1 = y;
                    let mut y2 = y + 1;
                    while y2 < self.height && self.vert_walls[y2][x] {
                        y2 += 1;
                    }
                    segments.push(json!({"type":"vertical","x":x,"y1":y1,"y2":y2}));
                    y = y2;
                } else {
                    y += 1;
                }
            }
        }
        // horizontal
        for y in 0..=self.height {
            let mut x = 0;
            while x < self.width {
                if self.hor_walls[y][x] {
                    let x1 = x;
                    let mut x2 = x + 1;
                    while x2 < self.width && self.hor_walls[y][x2] {
                        x2 += 1;
                    }
                    segments.push(json!({"type":"horizontal","y":y,"x1":x1,"x2":x2}));
                    x = x2;
                } else {
                    x += 1;
                }
            }
        }
        let fw = (self.width  * cell_size) as i32;
        let fd = (self.height * cell_size) as i32;
        let mut sizes  = vec![fw, 1, fd];
        let mut objects = vec![json!({ "p":[fw/2, -1, fd/2], "si":0 })];

        for (i, seg) in segments.iter().enumerate() {
            let si = i + 1;
            if seg["type"] == "vertical" {
                let x  = seg["x"].as_i64().unwrap() as i32 * cell_size as i32;
                let y1 = seg["y1"].as_i64().unwrap() as i32 * cell_size as i32;
                let y2 = seg["y2"].as_i64().unwrap() as i32 * cell_size as i32;
                let length = y2 - y1;
                sizes.extend([wall_thick as i32, 20, length]);
                objects.push(json!({"p":[x,0,(y1+y2)/2],"si":si}));
            } else {
                let y  = seg["y"].as_i64().unwrap() as i32 * cell_size as i32;
                let x1 = seg["x1"].as_i64().unwrap() as i32 * cell_size as i32;
                let x2 = seg["x2"].as_i64().unwrap() as i32 * cell_size as i32;
                let length = x2 - x1;
                sizes.extend([length,20, wall_thick as i32]);
                objects.push(json!({"p":[(x1+x2)/2,0,y],"si":si}));
            }
        }

        let half = (cell_size as i32) / 2;
        let start_spawn = json!([half,0,half,0,0,0]);
        let end_spawn   = json!([(fw-half),0,(fd-half),0,0,0]);

        json!({
            "name":    "GeneratedMaze",
            "ambient": "#97a0a8",
            "light":   "#f2f8fc",
            "sky":     "#dce8ed",
            "fog":     "#8d9aa0",
            "fogD":    2000,
            "xyz":     sizes,
            "objects": objects,
            "spawns":  [start_spawn, end_spawn],
        })
    }
}
