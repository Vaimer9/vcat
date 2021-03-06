use structopt::StructOpt;
use ansi_term::Color;
use std::fs;


/// This is the Struct for taking in all the input from the command Line
#[derive(StructOpt, Debug)]
#[structopt(name = "vcat")]
struct Args {
    #[structopt(name = "FILENAME")]
    file: String,
}

fn main() {
    let arg = Args::from_args();
    open_file(arg.file);
}

fn open_file(file_name: String) {

    let data = fs::read_to_string(&file_name);

    match data {
        Ok(e) => print_file_content(file_name, e),
        Err(_) => die()
    }

}


fn print_file_content(file_name: String, s: String) {
    println!("{}:      {}\n", Color::Red.bold().paint("File Name"), file_name);
    println!("{}", s);
}

fn die() {
    eprintln!("No such file or directory");
}

