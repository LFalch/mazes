extern crate image;
extern crate mazes;

use std::{env, fmt};
use std::path::Path;
use std::time::{Instant, Duration};

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

struct DisplayDuration(pub Duration);

impl fmt::Display for DisplayDuration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mins = self.0.as_secs() / 60;
        let secs = self.0.as_secs() % 60;
        let nanos = self.0.subsec_nanos();

        if mins > 0 {
            write!(f, "{} m {}.{:09}s", mins, secs, nanos)
        } else {
            write!(f, "{}.{:09}s", secs, nanos)
        }
    }
}

fn run<P: AsRef<Path>>(path: P, out: P) -> Result<(), String> {
    let mut img = image::open(path).map_err(|e| format!("{}", e))?.to_luma();
    let res;
    {
        let maze = Maze::new(&img).map_err(|e| format!("{:?}", e))?;

        let (w, h) = maze.dimensions();
        println!("Maze is {}x{}", w, h);

        let time = Instant::now();
        res = mazes::solve(&maze);

        let time = Instant::now()-time;
        println!("Time taken: {}", DisplayDuration(time));
    }
    let (length, path) = res.ok_or_else(|| "No solution".to_owned())?;

    println!("Solution length: {}", length);

    for Point(x, y) in path.into_iter() {
        img.put_pixel(x, y, Luma{data: [128]});
    }

    img.save(out).map_err(|e| format!("{}", e))?;

    Ok(())
}
