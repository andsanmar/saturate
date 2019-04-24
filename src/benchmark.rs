use std::io::Read;
use std::fs::File;

extern crate saturate;
#[macro_use]
extern crate criterion;

use criterion::Criterion;

use saturate::parser;
use saturate::solver;

fn benchmark(c : &mut Criterion) {
    let mut contents : String = String::new();
    let mut file : File = File::open("file_to_analyze.cnf").expect("Opening file failed");
    file.read_to_string(&mut contents).expect("Reading file failed");

    c.bench_function("solve", move |b| b.iter(|| solver::cdcl::solve(&parser::get_formulas(contents.clone()))));
}


criterion_group!(benches, benchmark);
criterion_main!(benches);
