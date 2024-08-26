use clap::{Parser, Subcommand};
use std::fs;

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "ccwordcount")]
#[command(bin_name = "ccwc")]
#[command(about = "a wordcount linux command tool clone", long_about = None)]
struct WordrCount {

    // #[arg(required=true)]
    // filename:String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(name="-c")]
    Count{
        filename:String,
    },
    #[command(name="-l")]
    Lines{filename:String,},
    #[command(name="-w")]
    Words{filename:String,},
    #[command(name="-m")]
    Characters{filename:String,},

}
fn main() {
    let args=WordrCount::parse();
    match args.command {
        Commands::Count{filename} =>{
            let f= fs::metadata(&filename).map_err(|err|
                eprintln!("could not get metadata of {file_name} because of {err}", file_name= filename)
            ).unwrap().len();
            println!(" {f} {file_name}", file_name=filename);
        },

        Commands::Lines{filename} =>{
            let f= fs::read_to_string(&filename).map_err(|err|
                eprintln!("could not read to string the {file_name} because of {err}", file_name= filename)
            ).unwrap();
            let lines = f.lines().count();
            println!(" {lines} {file_name}", file_name=filename);
        },
        Commands::Words{filename} =>{
            let f= fs::read_to_string(&filename).map_err(|err|
                eprintln!("could not read to string the {file_name} because of {err}", file_name= filename)
            ).unwrap();
            let words = f.split_whitespace().count();
            println!(" {words} {file_name}", file_name=filename);

        },
        Commands::Characters{filename} =>{
            let f= fs::read_to_string(&filename).map_err(|err|
                eprintln!("could not read to string the {file_name} because of {err}", file_name= filename)
            ).unwrap();
            let characters = f.chars().count();
            println!(" {characters} {file_name}", file_name=filename);

        },


    }
}
