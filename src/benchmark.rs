use std::io::Read;
use std::fs::File;
// use std::time::Duration;

extern crate saturate;
// #[macro_use]
extern crate criterion;


use criterion::Criterion;

use criterion::ParameterizedBenchmark;
use criterion::Benchmark;

use saturate::parser;
use saturate::solver;

fn benchmark_multiple(c: &mut Criterion) {
    let parameters : Vec<u8> = (100u8..120u8).collect();

    // One-time setup goes here
    c.bench(
        "solve",
        ParameterizedBenchmark::new(
            "multiple_bench",
            |b, param| b.iter(|| {
                let mut contents : String = String::new();
                let filename : String = "tests/uf100-430/uf100-0".to_string() + &param.to_string() + ".cnf";
                let mut file : File = File::open(filename).expect("Opening file failed");
                file.read_to_string(&mut contents).expect("Reading file failed");
                let parsed_contents = parser::get_formulas(contents);
                solver::cdcl::solve(&parsed_contents)
                // Code to benchmark using param goes here
            }),
            parameters
        )
    );
}

fn benchmark_alone(c: &mut Criterion) {
    c.bench(
        "solve",
        Benchmark::new("normal_bench", |b| b.iter(|| {
            let mut contents : String = String::new();
            let mut file : File = File::open("tests/uf100-430/uf100-0101.cnf").expect("Opening file failed");
            file.read_to_string(&mut contents).expect("Reading file failed");
            let parsed_contents = parser::get_formulas(contents);
            solver::cdcl::solve(&parsed_contents)
        }))
            .throughput(criterion::Throughput::Bytes(1))
            .sample_size(2)
    );
}


fn main(){
    benchmark_multiple(&mut Criterion::default().sample_size(10).with_plots());
    benchmark_alone(&mut Criterion::default().sample_size(10).with_plots())
}


// criterion_group!(benches, benchmark);
