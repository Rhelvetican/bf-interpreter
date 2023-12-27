use bf_interpreter::*;
use std::{env, fs::{self, File}, io::Read};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    
    if args.len() != 2 {
        println!("Usage: bf <file.bf>");
        std::process::exit(1);
    }

    let filename = &args[1];
    let mut file = File::open(filename).expect("File not found");
    let mut src = String::new();
    file.read_to_string(&mut src).expect("Could not read file");

    let tokens = lexer(src);

    let ins = parser(tokens);

    //Setup the environment
    let mut tape: Vec<i32> = vec![0; 32767];
    let mut ptr: usize = 0;

    executor(&ins, &mut tape, &mut ptr);
}
