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

use conch_parser::lexer::Lexer;
use conch_parser::parse::DefaultParser;
use conch_parser::ast::TopLevelCommand;

use std::io;

fn main() {
    if let Err(err) = repl() {
        error!("{}", err);
    }
}

fn repl() -> std::io::Result<()> {
    loop {
        // read
        let mut line_buf: String = String::new();
        io::stdin().read_line(&mut line_buf)?;

        // lex, parse
        let lex = Lexer::new(line_buf.chars());
        let parser = DefaultParser::new(lex);

        // run
        for ast in parser {
            ast.map(execute_cmd).expect("Parser error");
        }
    }
}

fn execute_cmd(ast: TopLevelCommand<String>) {
    println!("Executing command {:?}", ast);
}
