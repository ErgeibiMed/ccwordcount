use clap::{Parser, Subcommand};
use std::fs;

#[derive(Debug, Parser)] // requires `derive` feature
#[command(about = "a wordcount linux command tool clone", long_about = None)]
struct WordCount {

    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(global=true,required=false)]
    filename:String,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(name="-c")]
    Count,
    #[command(name="-l")]
    Lines,
    #[command(name="-w")]
    Words,
    #[command(name="-m")]
    Characters,

}
fn main() {
    let args=WordCount::parse();
    match args.command {
        Some(Commands::Count) =>{
            let f= fs::metadata(&args.filename).map_err(|err|
                eprintln!("could not get metadata of {file_name} because of {err}", file_name= args.filename)
            ).unwrap().len();
            println!(" {f} {file_name}", file_name=args.filename);
        },

        Some(Commands::Lines) =>{
            let f= fs::read_to_string(&args.filename).map_err(|err|
                eprintln!("could not read to string the {file_name} because of {err}", file_name= args.filename)
            ).unwrap();
            let lines = f.lines().count();
            println!(" {lines} {file_name}", file_name=args.filename);
        },
        Some(Commands::Words) =>{
            let f= fs::read_to_string(&args.filename).map_err(|err|
                eprintln!("could not read to string the {file_name} because of {err}", file_name= args.filename)
            ).unwrap();
            let words = f.split_whitespace().count();
            println!(" {words} {file_name}", file_name=args.filename);

        },
       Some(Commands::Characters) =>{
            let f= fs::read_to_string(&args.filename).map_err(|err|
                eprintln!("could not read to string the {file_name} because of {err}", file_name= args.filename)
            ).unwrap();
            let characters = f.chars().count();
            println!(" {characters} {file_name}", file_name=args.filename);

        },


        None => {
                 let f=fs::read_to_string(&args.filename).map_err(|err|
                eprintln!("could not read to string the {file_name} because of {err}", file_name= args.filename)
            ).unwrap();
            let (
                lines,
                words,
                bytes
            )=
            (
                f.lines().count(),
                f.split_whitespace().count(),
                f.bytes().count()
            );
            println!("   {lines}  {words}  {bytes}  {file_name}",file_name=args.filename);

        },
    }
}
