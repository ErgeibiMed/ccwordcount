use clap::{Parser, Subcommand};
use std::{fs, io::{self, Read}};

#[derive(Debug, Parser)] // requires `derive` feature
#[command(about = None, long_about = None,disable_help_subcommand=true)]
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
    #[command(name="-h")]
    Help,
    #[command(name="help")]
    Longhelp,


}
fn main() {
    let args=WordCount::parse();
    match args.filename{
        None => {
            let mut data = String::new();
            let mut stdin= io::stdin();
            match args.command {
                Some(Commands::Help)| Some(Commands::Longhelp)=> {
                    println!("");
                    println!("a wordcount(wc) linux command tool clone");
                    println!("");
                    println!("Usage: ccwc <COMMAND> <FILENAME>");
                    println!("");
                    println!("Commands: ");
                    println!("  -c          print the byte counts ");
                    println!("  -l          print the newline counts ");
                    println!("  -w          print the word counts ");
                    println!("  -m          print the character counts ");
                    println!("  -h, help    print this message ");
                    println!("");


                },
                Some(Commands::Count) => {
                    match stdin.read_to_string(&mut data){
                        Ok(_) => {
                            println!("  {count} ", count=data.len());
                        }
                        Err(error) => println!("error: {error}"),
                    }
                },
                Some(Commands::Lines) => {
                    match stdin.read_to_string(&mut data){
                        Ok(_) => {
                            println!("  {lines} ", lines=data.lines().count());
                        }
                        Err(error) => println!("error: {error}"),
                    }
                },
                Some(Commands::Words) => {
                    match stdin.read_to_string(&mut data){
                        Ok(_) => {
                            println!("  {words} ", words=data.split_whitespace().count());
                        }
                        Err(error) => println!("error: {error}"),
                    }
                },
                Some(Commands::Characters) => {
                    match stdin.read_to_string(&mut data){
                        Ok(_) => {
                            println!("  {characters} ", characters=data.chars().count());
                        }
                        Err(error) => println!("error: {error}"),
                    }
                },
                None => {
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
            let f= fs::read_to_string(&filename).map_err(|err|
                eprintln!("could not read to string the {file_name} because of {err}", file_name= filename)
            ).unwrap();
            match args.command {
                Some(Commands::Help)| Some(Commands::Longhelp)=> {
                    println!("");
                    println!("a wordcount(wc) linux command tool clone");
                    println!("");
                    println!("Usage: ccwc <COMMAND> <FILENAME>");
                    println!("");
                    println!("Commands: ");
                    println!("  -c          print the byte counts ");
                    println!("  -l          print the newline counts ");
                    println!("  -w          print the word counts ");
                    println!("  -m          print the character counts ");
                    println!("  -h, help    print this message ");
                    println!("");

                },

                Some(Commands::Count) =>{
                    println!(" {len} {file_name}",len = f.len(), file_name=filename);
                },
                Some(Commands::Lines) =>{
                    println!(" {lines} {file_name}", lines = f.lines().count(),file_name=filename);
                },
                Some(Commands::Words) =>{
                    println!(" {words} {file_name}", words = f.split_whitespace().count(),file_name=filename);
                },
                Some(Commands::Characters) =>{
                    println!(" {characters} {file_name}", characters = f.chars().count(),file_name=filename);
                },
                None => {
                    let (lines,words,bytes)= (f.lines().count(),f.split_whitespace().count(),f.bytes().count());
                    println!("   {lines}  {words}  {bytes}  {file_name}",file_name=filename);
                },
            }
        },
    }
}
