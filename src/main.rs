extern crate image;
extern crate mazes;

use std::env;
use std::path::Path;

use image::Luma;
use mazes::*;

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();

    let img = &args[0];
    let out = args.get(1).cloned().unwrap_or_else(|| {
        let index = img.rfind('.').unwrap();
        img[..index].to_owned() + "_solution.png"
    });

    match run(img, &out) {
        Ok(()) => (),
        Err(err) => eprintln!("Error: {}", err)
    }
}

fn run<P: AsRef<Path>>(path: P, out: P) -> Result<(), String> {
    let mut img = image::open(path).map_err(|e| format!("{}", e))?.to_luma();
    let maze = img.pixels()
        .map(|p| p.data[0] < 0x7f)
        .collect::<MazeBuilder>()
        .finish(img.width() as usize).map_err(|e| format!("{:?}", e))?;

    let (w, h) = maze.dimensions();
    let start = Point(maze.get_entrance(), 0);
    let end = Point(maze.get_exit(), h-1);
    println!("Maze is {}x{}", w, h);

    let path = mazes::a_star(start, end, maze).ok_or_else(|| "No solution".to_owned())?;

    for Point(x, y) in path.into_iter() {
        img.put_pixel(x as u32, y as u32, Luma{data: [127]});
    }

    img.save(out).map_err(|e| format!("{}", e))?;

    Ok(())
}
