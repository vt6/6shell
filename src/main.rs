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

#[macro_use]
extern crate log;
extern crate conch_parser;
extern crate conch_runtime;
extern crate tokio_core;

use conch_parser::lexer::Lexer;
use conch_parser::parse::DefaultParser;
use conch_runtime::spawn::sequence;
use std::env;
use std::fs::File;
use std::io;
use tokio_core::reactor::Core;
use conch_runtime::env::DefaultEnv;
use conch_runtime::future::EnvFuture;

fn repl<T: io::BufRead>(script: &mut T) -> io::Result<()> {
    loop {
        // read from file or stdin
        let mut line = String::new();
        script.read_line(& mut line)?;

        // Stop loop when no more lines in file
        // FIXME: Checking on length is clumsy
        if line.len() == 0 {
            return Ok(());
        }

        // lex, parse
        let lex = Lexer::new(line.chars());
        let parser = DefaultParser::new(lex);

        // make event loop
        let mut lp = Core::new().expect("failed to create event loop");
        let env = DefaultEnv::new(lp.remote(), None).expect("failed to create default environment");

        let input_sequence = sequence(
            parser.into_iter().map(|parsed_line| {
            match parsed_line {
                Ok(cmd) => cmd,
                Err(cmd) => panic!("Parser error: {}", cmd),
            }
        }));

        // run
        let _result = lp.run(input_sequence.pin_env(env));

    }
}

fn main() {
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

    // fail if evaluation failed
    if let Err(err) = eval_result {
        error!("{}", err);
    }
}
