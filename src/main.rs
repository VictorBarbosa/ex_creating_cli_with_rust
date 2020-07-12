use clap::{App, Arg};
fn main() {
    cli_create();
}
fn cli_create() {
    let _matches = App::new("myapp")
        .arg(Arg::with_name("name").long("--nam").takes_value(true))
        .arg(Arg::with_name("age").long("--age").takes_value(true))
        .get_matches();

    if let Some(name) = _matches.value_of("name") {
        println!("The name is   : {}", name.to_string())
    }
    if let Some(age) = _matches.value_of("age") {
        println!("The age is   : {}", age.to_string())
    }
}
