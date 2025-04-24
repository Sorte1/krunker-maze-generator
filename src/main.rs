use clap::Parser;
use krunker_maze_generator::Maze;
use std::{error::Error, fs::File, io::Write, path::PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Maze width in cells
    #[arg(short = 'W', long, default_value_t = 50)]
    width: usize,
    /// Maze height in cells
    #[arg(short = 'H', long, default_value_t = 50)]
    height: usize,
    /// Pixel size of each cell
    #[arg(short = 'S', long, default_value_t = 40)]
    cell_size: usize,
    /// Wall thickness in pixels
    #[arg(short = 'T', long, default_value_t = 4)]
    wall_thickness: usize,
    /// Output image file path
    #[arg(short, long, default_value = "maze.png")]
    image: PathBuf,
    /// Output map JSON file path
    #[arg(short = 'M', long, default_value = "map.json")]
    map: PathBuf,
    /// Skip JSON map generation
    #[arg(long)]
    no_map: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    println!("Generating maze {}x{}…", args.width, args.height);
    let mut maze = Maze::new(args.width, args.height);
    maze.generate();

    println!("Drawing maze to image ({})…", args.image.display());
    let img = maze.draw(args.cell_size, args.wall_thickness);
    img.save(&args.image)?;
    println!("Image saved to {}", args.image.display());

    if !args.no_map {
        println!("Generating JSON map to {}…", args.map.display());
        let map_json = maze.to_map_json(args.cell_size, args.wall_thickness);
        let mut f = File::create(&args.map)?;
        write!(f, "{}", serde_json::to_string_pretty(&map_json)?)?;
        println!("Map JSON saved to {}", args.map.display());
    }

    Ok(())
}
