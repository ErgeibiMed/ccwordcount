use std::fs;

use clap::Parser;


#[derive(Parser,Debug)]
struct Args {

    #[arg(short,long)]
    count: String,

    filename:String,

}

fn main() {
    let args=Args::parse();

    let f= fs::metadata(&args.filename).map_err(|err|
       eprintln!("could get metadata of {file_name} because of {err}", file_name= args.filename)
        ).unwrap().len();
    println!(" {f} {file_name}", file_name=args.filename);
}
