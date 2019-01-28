use criterion::*;

mod parser;
mod misc;

criterion_main! {
    self::misc::core,
    self::parser::parser,
}
