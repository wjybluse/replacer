extern crate replacer;
extern crate clap;
use clap::{Arg, App};
use replacer::handler::r::{Handler,Replacer};
use replacer::handler::utils::{CacheProvider,Cache};

fn main() {
    let matches = App::new("Simple Replacer for quickly repalce env variable")
                      .version("1.0")
                      .author("Elian Wan")
                      .about("")
                      .arg(Arg::with_name("source")
                                .short("s")
                                .long("source")
                                .value_name("SOURCE")
                                .help("source file ,env file or template file")
                                .takes_value(true))
                      .arg(Arg::with_name("replaced")
                                .short("r")
                                .long("replaced")
                                .value_name("REPLACED")
                                .help("dest file or directory")
                                .takes_value(true))
                      .get_matches();
    let source = matches.value_of("source").unwrap_or("/path/to/source");
    let replaced= matches.value_of("replaced").unwrap_or("/path/to/dest");
    let map = CacheProvider::new(source).cache();
    let handler = Replacer::new(map);
    handler.replace(replaced);
}
