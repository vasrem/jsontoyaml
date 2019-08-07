# jsontoyaml

Command line tool to convert JSON to YAML and vice versa.

***Attention:*** This is a small project which I did to get familiar with Rust and it is not meant to be used in production.

## Dependencies

* Rust 1.36.0 (most likely it works with previous versions as well but this is what I had at this point)

## Build

First you need to build the project using the following command:

``` bash
cargo build
# or
cargo build --release
```

Then copy the binary which is under `target/{release,debug}/jsontoyaml` to your preferred `$PATH` directory.

## Usage


``` bash
$ jsontoyaml -h
jsontoyaml 0.1.0
Converts JSON to YAML and vice versa using stdin

USAGE:
    jsontoyaml [FLAGS]

FLAGS:
    -h, --help       Prints help information
    -r, --reverse    YAML to JSON
    -V, --version    Prints version information
```

You can use the binary as shown below:

``` bash
# JSON to YAML
cat <<EOF | jsontoyaml
{
  "field1": "val1",
  "arr": [
      "val1", "val2"
  ]
}
EOF

# YAML to JSON
cat <<EOF | jsontoyaml -r
---
field1: val1
arr:
  - val1
  - val2
EOF
```

## Contribution

Feel free to file any Issue/PR regarding bugs or code refactoring. It will be really helpful for me to get more into Rust. :)
