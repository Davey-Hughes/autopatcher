mod hash;

fn main() {
    let h = hash::calc_sha1_zip("./Super Mario Bros. (World).zip");
    let s = hash::string_from_sha1(h);

    println!("{}", s);
}
