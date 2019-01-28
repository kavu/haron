use criterion::*;

use haron_parser::parser::parse_string;

fn bench_parser_parse_string(c: &mut Criterion) {
    let input = "\r\n\r\nDAT 1, 1  \n\nMOV 1\n\n\nSPL ,1\n".into();

    c.bench_function("parser::parse_string", move |b| b.iter(|| {
        parse_string(input)
    }));
}

criterion_group!(parser, bench_parser_parse_string);
