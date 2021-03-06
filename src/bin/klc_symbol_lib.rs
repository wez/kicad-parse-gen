// (c) 2017 Productize SPRL <joost@productize.be>

extern crate env_logger;
extern crate kicad_parse_gen as kicad;
#[macro_use]
extern crate log;

use std::path::PathBuf;
use std::env;

use kicad::checkfix::CheckFix;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init().unwrap();

    let mut args = std::env::args();
    args.next();
    let name = args.next().unwrap();
    let name = PathBuf::from(name);
    let symbol_lib = kicad::read_symbol_lib(&name).unwrap();
    let config = kicad::checkfix::Config::klc();
    for symbol in symbol_lib.symbols {
        let checkres = symbol.check(&config);
        if !checkres.is_empty() {
            info!("Symbol {}", symbol.name);
            for v in checkres {
                v.dump_on_logger(1);
            }
        }
    }
}
