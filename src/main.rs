mod hash;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
    }
    let file_name = std::path::Path::new(&*args[1]).to_str().unwrap();

    let h = hash::calc_sha1(file_name);
    let s = hash::string_from_sha1(h);

    println!("{}", s);
}
