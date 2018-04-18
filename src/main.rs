/*******************************************************************************
 *
 * Copyright 2018 lærling <laerling@posteo.de>
 *
 * This program is free software: you can redistribute it and/or modify it under
 * the terms of the GNU General Public License as published by the Free Software
 * Foundation, either version 3 of the License, or (at your option) any later
 * version.
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT ANY
 * WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR
 * A PARTICULAR PURPOSE. See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along with
 * this program. If not, see <http://www.gnu.org/licenses/>.
 *
 *******************************************************************************/

extern crate conch_parser;
#[macro_use]
extern crate log;
extern crate simple_logger;

use conch_parser::ast::TopLevelCommand;
use conch_parser::lexer::Lexer;
use conch_parser::parse::DefaultParser;
use std::io;
use std::fs::File;
use std::env;

fn execute_cmd(cmd: TopLevelCommand<String>) {
    println!("Executing command {:?}", cmd);
}

fn repl<T: io::BufRead>(script: &mut T) -> io::Result<()> {
    loop {
        // read from file or stdin
        let mut line = String::new();
        script.read_line(& mut line)?;

        // Stop loop when no more lines in file
        if line.len() == 0 {
            return Ok(());
        }

        // lex, parse
        let lex = Lexer::new(line.chars());
        let parser = DefaultParser::new(lex);

        // run
        for parsed_line in parser {
            match parsed_line {
                Ok(cmd) => execute_cmd(cmd),
                Err(cmd) => panic!("Parser error: {}", cmd),
            };
        }
    }
}

fn main() {
    simple_logger::init().unwrap();

    // evaluate command line argument
    let eval_result = match env::args().nth(1) {
        // no argument given, run repl
        None => {
            let stdin = io::stdin();
            let mut input = stdin.lock();
            repl(&mut input)
        },
        // argument given, open file and read commands
        Some(filename) => match File::open(&filename) {
            Ok(file) => repl(&mut io::BufReader::new(file)),
            Err(_) => { error!("Cannot read file {}", filename); return; },
        }
    };

    if let Err(err) = eval_result {
        error!("{}", err);
    }
}
