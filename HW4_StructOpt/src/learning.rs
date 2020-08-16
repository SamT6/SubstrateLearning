//ref: https://www.tenderisthebyte.com/blog/2019/05/08/parsing-cli-args-with-structopt/

use std::path::PathBuf;
use structopt::StructOpt;

//Define a struct called opts 
#[derive(Debug, StructOpt)] //Debug: for default implementation of Display trait, to be able to print this struct
struct Opts{
    #[structopt(parse(from_os_str))] // to be able to parse from command lines
    infile: PathBuf, //a field called infile, with type PathBuf
    #[structopt(parse(from_os_str))] // to be able to parse from command lines
    outfile: PathBuf, //a field called outfile, with type PathBuf 
}


fn main() {
    // Assign an instance of Opts to the opts variable.
    //variables in Rust are immutable by default
    let mut opts = Opts {
        // Data to store in the infile field
        infile: PathBuf::from("/home/sam/infile.txt"),

        // Data to store in the outfile field
        outfile: PathBuf::from("/home/sam/outfile.txt"),
    };

    opts.infile = PathBuf::from("file.txt");

    println!("opts: {:?}", opts);


}
