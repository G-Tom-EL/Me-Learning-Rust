use std::fs::File;
use std::io::Read;

fn main() {
    let args = std::env::args().skip(1);

    for filename in args {
        let mut fi = File::open(filename).unwrap(); 
        let mut buffer = String::new();
        fi.read_to_string(&mut buffer).unwrap();
        println!("{}", buffer);
    }
}
