use clap::{Parser, Subcommand};
use std::{fs, io::{self, Read}};

#[derive(Debug, Parser)] // requires `derive` feature
#[command(about = "a wordcount linux command tool clone", long_about = None)]
struct WordCount {

    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(global=true,required=false)]
    filename:Option<String>,
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
    match args.filename{
        None => {
            match args.command {
                Some(Commands::Lines) => {
                    let mut data = String::new();
                    let mut stdin= io::stdin();
                    match stdin.read_to_string(&mut data){
                        Ok(_) => {
                            println!("  {lines} ", lines=data.lines().count());
                        }
                        Err(error) => println!("error: {error}"),
                    }

                },
                Some(Commands::Words) => {
                    let mut data = String::new();
                    let mut stdin= io::stdin();
                    match stdin.read_to_string(&mut data){
                        Ok(_) => {
                            println!("  {words} ", words=data.split_whitespace().count());
                        }
                        Err(error) => println!("error: {error}"),
                    }

                },
                Some(Commands::Characters) => {
                    let mut data = String::new();
                    let mut stdin= io::stdin();
                    match stdin.read_to_string(&mut data){
                        Ok(_) => {
                            println!("  {characters} ", characters=data.chars().count());
                        }
                        Err(error) => println!("error: {error}"),
                    }

                },

                Some(Commands::Count) => {
                    let mut data = Vec::new();
                    let mut stdin= io::stdin();
                    match stdin.read_to_end(&mut data){
                        Ok(n) => {
                            println!("  {n} ");
                        }
                        Err(error) => println!("error: {error}"),
                    }

                },

                None => {
                    let mut data = String::new();
                    let mut stdin= io::stdin();
                    match stdin.read_to_string(&mut data){
                        Ok(_) => {
                            let (lines,words,bytes)= (data.lines().count(),data.split_whitespace().count(),data.bytes().count());
                            println!("   {lines}  {words}  {bytes}  ");
                        }
                        Err(error) => println!("error: {error}"),
                    }

                },
            }
        },
        Some(filename)=>{
            match args.command {
                Some(Commands::Count) =>{
                    let f= fs::metadata(&filename).map_err(|err|
                        eprintln!("could not get metadata of {file_name} because of {err}", file_name= filename)
                    ).unwrap().len();
                    println!(" {f} {file_name}", file_name=filename);
                },
                Some(Commands::Lines) =>{
                    let f= fs::read_to_string(&filename).map_err(|err|
                        eprintln!("could not read to string the {file_name} because of {err}", file_name= filename)
                    ).unwrap();
                    let lines = f.lines().count();
                    println!(" {lines} {file_name}", file_name=filename);
                },
                Some(Commands::Words) =>{
                    let f= fs::read_to_string(&filename).map_err(|err|
                        eprintln!("could not read to string the {file_name} because of {err}", file_name= filename)
                    ).unwrap();
                    let words = f.split_whitespace().count();
                    println!(" {words} {file_name}", file_name=filename);
                },
                Some(Commands::Characters) =>{
                    let f= fs::read_to_string(&filename).map_err(|err|
                        eprintln!("could not read to string the {file_name} because of {err}", file_name= filename)
                    ).unwrap();
                    let characters = f.chars().count();
                    println!(" {characters} {file_name}", file_name=filename);
                },
                None => {
                    let f=fs::read_to_string(&filename).map_err(|err|
                        eprintln!("could not read to string the {file_name} because of {err}", file_name= filename)
                    ).unwrap();
                    let (lines,words,bytes)= (f.lines().count(),f.split_whitespace().count(),f.bytes().count());
                    println!("   {lines}  {words}  {bytes}  {file_name}",file_name=filename);
                },
            }
        },
    }
}
