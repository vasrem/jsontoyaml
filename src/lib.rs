pub mod convert {
    use serde_json::{self, map::Map, Value as ValueJSON};
    use serde_yaml::{self, Value as ValueYAML};
    use std::io::{self, Read};
    use std::panic;

    pub mod json_to_yaml {
        use super::*;
        // converts a json parsed from stdin into a yaml and prints it
        pub fn run() {
            let buffer = parse_stdin().expect("failed to parse string from stdin");
            let _json = parse_json(&buffer).expect("failed to parse json");
            match to_yaml(&buffer) {
                Ok(s) => println!("{}", s),
                Err(e) => println!("{}", e),
            }
        }

        // parses string from stdin
        fn parse_stdin() -> io::Result<String> {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            Ok(buffer)
        }

        // parses json from string
        fn parse_json(s: &str) -> Result<(), serde_json::Error> {
            // try to parse json
            let _json: ValueJSON = serde_json::from_str(s)?;
            Ok(())
        }

        // converts json string to yaml string
        fn to_yaml(s: &str) -> Result<String, serde_yaml::Error> {
            let yaml: ValueYAML = serde_yaml::from_str(&s)?;
            let output = serde_yaml::to_string(&yaml)?;
            Ok(output)
        }

    }

    pub mod yaml_to_json {
        use super::*;
        // converts a yaml parsed from stdin into a json and prints it
        pub fn run() {
            let buffer = parse_stdin().expect("failed to parse string from stdin");
            let yaml = parse_yaml(&buffer).expect("failed to parse yaml");
            match to_json(&yaml) {
                Ok(s) => println!("{}", s),
                Err(e) => println!("{}", e),
            }
        }

        // parses string from stdin
        fn parse_stdin() -> io::Result<String> {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            Ok(buffer)
        }

        // parses yaml from string
        fn parse_yaml(s: &str) -> Result<serde_yaml::Mapping, serde_yaml::Error> {
            let yaml: serde_yaml::Mapping = serde_yaml::from_str(&s)?;
            Ok(yaml)
        }

        // converts yaml struct to json string
        fn to_json(s: &serde_yaml::Mapping) -> io::Result<String> {
            let json = to_json_object(s);
            let output = serde_json::to_string(&json)?;

            Ok(output)
        }

        // converts yaml struct to json value
        fn to_json_object(s: &serde_yaml::Mapping) -> ValueJSON {
            let mut m = Map::new();
            for v in s.iter() {
                let key = match v.0.as_str() {
                    Some(s) => s.to_string(),
                    None => panic!("could not parse yaml key"),
                };

                if v.1.is_mapping() {
                    match v.1.as_mapping() {
                        Some(b) => m.insert(key, to_json_object(b)),
                        None => panic!("could not parse mapping"),
                    };
                } else if v.1.is_sequence() {
                    match v.1.as_sequence() {
                        Some(s) => {
                            let mut vec = Vec::new();
                            for o in s {
                                if o.is_mapping() {
                                    let obj = match o.as_mapping() {
                                        Some(b) => b,
                                        None => panic!("could not parse mapping in sequence"),
                                    };
                                    vec.push(to_json_object(obj));
                                    m.insert(key.clone(), to_json_object(obj));
                                } else {
                                    vec.push(create_json_value(o));
                                }
                            }
                            m.insert(key, ValueJSON::Array(vec));
                        }
                        None => panic!("could not parse sequence"),
                    };
                } else {
                    m.insert(key, create_json_value(v.1));
                }
            }

            ValueJSON::Object(m)
        }

        // converts yaml value to json value
        fn create_json_value(v: &ValueYAML) -> ValueJSON {
            if v.is_string() {
                match v.as_str() {
                    Some(s) => ValueJSON::String(s.to_string()),
                    None => panic!("could not parse string"),
                }
            } else if v.is_bool() {
                match v.as_bool() {
                    Some(b) => ValueJSON::String(b.to_string()),
                    None => panic!("could not parse bool"),
                }
            } else if v.is_null() {
                match v.as_null() {
                    Some(_n) => ValueJSON::String("null".to_string()),
                    None => panic!("could not parse null"),
                }
            } else if v.is_i64() {
                match v.as_i64() {
                    Some(n) => ValueJSON::Number(serde_json::Number::from(n)),
                    None => panic!("could not parse i64"),
                }
            } else if v.is_u64() {
                match v.as_u64() {
                    Some(n) => ValueJSON::Number(serde_json::Number::from(n)),
                    None => panic!("could not parse u64"),
                }
            } else if v.is_f64() {
                match v.as_f64() {
                    Some(f) => match serde_json::Number::from_f64(f as f64) {
                        Some(val) => ValueJSON::Number(val),
                        None => panic!("could not parse serde_json::Number"),
                    },
                    None => panic!("could not parse f64"),
                }
            } else {
                panic!();
            }
        }
    }
}
