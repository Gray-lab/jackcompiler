use std::{env};

use jackcompiler::Config;
fn main() {
    let args: Vec<String> = env::args().collect();

    dbg!(&args);

    let config = Config::build(&args).expect("hoping the config builds correctly");

    dbg!(config);

    let text = String::from("{} () [] . , ; + - * & | < > = ~ /");
    let mut iterator = text.chars();
    while let Some(c) = iterator.next() {
        println!("{:?}", c);
    }
    
}

