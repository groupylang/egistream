extern crate clap;

use clap::{App, Arg, SubCommand};

fn main() {
    let egistream = App::new("egistream")
        .version("0.1.0")
        .author("sKyrBBit, capra314cabra")
        .about("compiler for streaming programming");
    let matches = egistream.get_matches();
}
