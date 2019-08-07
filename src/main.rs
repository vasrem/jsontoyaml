use jsontoyaml::convert;

extern crate clap;
use clap::{Arg,App};

fn main() {
    let matches = App::new("jsontoyaml")
        .version("0.1.0")
        .about("Converts JSON to YAML and vice versa using stdin")
        .arg(Arg::with_name("reverse")
             .short("r")
             .long("reverse")
             .help("YAML to JSON"))
        .get_matches();

	if matches.is_present("reverse"){
		convert::yaml_to_json::run();
	}else{
		convert::json_to_yaml::run();
	} 

}
