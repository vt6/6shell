/*******************************************************************************
 *
 * Copyright 2018 hkgit03 <22918836+hkgit03@users.noreply.github.com>
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

fn main() {
    simple_logger::init().unwrap();

    let result = match env::args().nth(1) {
        None => {
            let stdin = io::stdin();
            let mut input = stdin.lock();
            repl(&mut input)
        },
        Some(filename) => match File::open(&filename) {
            Ok(file) => repl(&mut io::BufReader::new(file)),
            Err(_) => { error!("Cannot read file {}", filename); return; },
        }
    };

    if let Err(err) = result {
        error!("{}", err);
    }
}

fn repl<T: io::BufRead>(script: &mut T) -> io::Result<()> {
    // TODO: Stop loop when no more lines
    loop {
        // read from file or stdin
        let mut line = String::new();
        script.read_line(& mut line)?;

        // lex, parse
        let lex = Lexer::new(line.chars());
        let parser = DefaultParser::new(lex);

        // run
        for ast in parser {
            execute_cmd(ast.expect("Parser error"));
        }
    }
}

fn execute_cmd(ast: TopLevelCommand<String>) {
    println!("Executing command {:?}", ast);
}
