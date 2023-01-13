use std::io;
use std::env;

fn main() {
    let mut buf = String::new();
    let args: Vec<String> = env::args().collect();

    // parse some options

    if args.len() > 1 {
        println!("{}", args[1]);

        // if arg is file
        // cat file
    } else {
        io::stdin().read_line(&mut buf);
        println!("{}", buf);
    }
}
