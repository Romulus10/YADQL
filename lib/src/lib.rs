#[macro_use]
extern crate serde_derive;
extern crate colored;
extern crate gpgme;

mod core;
mod parser;
mod blockchain;
mod crypt;

use std::process;
use std::error::Error;
use colored::Colorize;

use parser::parser::Parser;
use core::keywords::YADQL;
use blockchain::blockchain::Blockchain;
use crypt::crypt::*;

/*
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
*/

mod yadql {
    use super::Parser;
    use super::Error;
    use super::Colorize;
    use super::process;
    use super::YADQL;
    use super::Blockchain;

    fn parse(query: &str) -> Parser {
        match Parser::new(query) {
            Ok(x) => x,
            Err(e) => {
                println!("Error!: {}, {}", 
                        e.description().red().bold(), e.to_string()
                );
                process::exit(1);
            }
        }
    }
    
    // TODO Accept multiple queries at a time
    fn execute(statement: &str) -> String {
        let blockchain = Blockchain::new();
        let parser: Parser = parse(statement);
        let ret = match *parser.keywords.get(0).unwrap() {
            YADQL::Insert(ref k, ref v) => blockchain.insert(k, v),
            YADQL::Delete(ref k) => blockchain.delete(k),
            YADQL::Update(ref k, ref v) => blockchain.update(k, v),
            YADQL::Read(ref k) => blockchain.read(k),
            _ => panic!("Nothing read in query!")
        };
        return ret.unwrap().payload;
    }

    fn open() {
        unimplemented!();
    }
}
