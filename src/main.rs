use std::any::Any;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use clap::{Arg, ArgMatches, Command};
use clap::builder::Str;

use regex::{Match, Regex};

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for (index, line_) in reader.lines().enumerate() {
        let line = line_.unwrap();
        match re.find(&line) {
            None => {}
            Some(_) => { println!("{}", line); }
        }
    }
}


fn main() {
    let commands = Command::new("clap-tool");

    // 用于检索
    let grepCommand = Command::new("grep")
        .version("0.1")
        .about("用于搜索的命令行工具")
        .arg(Arg::new("pattern")
            .help("The pattern to search for")
            // 值必须存在
            .required(true))
        .arg(Arg::new("input")
            .help("File to search")
            .required(true)
        );
    // 用于对比
    let diffCommand = Command::new("diff")
        .version("0.1")
        .about("用于对比的命令行工具");
    let commands = commands
        .subcommand(grepCommand)
        .subcommand(diffCommand);
    let matches = commands.get_matches();

    if let Some(args) = matches.subcommand_matches("grep") {
        let pattern = args.get_one::<String>("pattern").unwrap();
        let re = Regex::new(pattern).unwrap();
        let flag = "-".to_string();

        let input = args.get_one::<String>("input").unwrap_or(&flag);
        if input == "-"{
            let stdin = io::stdin();
            let reader = stdin.lock();
            process_lines(reader,re);
        }else{
            let f = File::open(input).unwrap();
            let reader = BufReader::new(f);
            process_lines(reader,re);
        }

    }
    
    if let Some(_) = matches.subcommand_matches("diff") {
        println!("diff")
    }
}
