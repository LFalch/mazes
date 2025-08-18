use std::env::args;

use mazes::generate::{backtrack, primms};

fn main() {
    let mut args = args().skip(1);
    let img_path = args.next().unwrap();
    let width = args.next().unwrap().parse().unwrap();
    let height = args.next().unwrap().parse().unwrap();
    let alg = args.next();
    let alg = alg.as_ref().map(|s| &**s).unwrap_or("primms");

    let maze_img = match alg {
        "primms" => primms(width, height),
        "backtrack" => backtrack(width, height),
        _ => unimplemented!(),
    };
    maze_img.save(img_path).unwrap();
}
