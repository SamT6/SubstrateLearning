//ref: https://www.tenderisthebyte.com/blog/2019/05/08/parsing-cli-args-with-structopt/

use std::path::PathBuf;
use structopt::StructOpt;
use std::fmt::Display;
use std::fmt;


#[derive(Debug, StructOpt)] 
struct Opts{
    #[structopt(parse(from_os_str))] 
    infile: PathBuf, 

    #[structopt(short, long, parse(from_os_str))]
    outfile: Option<PathBuf>, 
}

impl Display for Opts{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}, {:?})", self.infile, self.outfile)
    }
}

fn main() {
    let opts = Opts::from_args();

    // use this if not implement Display for Opts  
    //println!("{:?}", opts);
    println!("{}", opts);
}
