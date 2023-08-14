mod scan;
use std::process;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        process::exit(-1);
    }

    let start_path = std::path::Path::new(&*args[1]);

    scan::scan(start_path.to_path_buf());
}
