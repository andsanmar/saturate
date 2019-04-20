use std::io::{self, Read};
use std::env;
use std::fs::File;

extern crate sat_solver;

use sat_solver::parser;
use sat_solver::solver;
use sat_solver::structures;

fn main() {
    let mut contents : String = String::new();
    if env::args().count() == 2 {
        let filename : String = env::args().nth(1).unwrap();
        let mut file : File = File::open(&filename).expect("Opening file failed");
        file.read_to_string(&mut contents).expect("Reading file failed");
    } else {
        io::stdin().read_to_string(&mut contents).unwrap();
    };

    //println!("{}", contents);
    
    let to_solve : structures::CNF = parser::get_formulas(contents);
    //println!("{:?}", solver::brute_force::solve(to_solve));
    match solver::cdcl::solve(&to_solve) {
        None => println!("UNSAT"),
        Some(x) => {
            if check_solution(&to_solve, &x){
                print!("s ");
                for (i, elem) in x.iter().enumerate(){
                    print!("{}{} ", if *elem {""} else {"-"}, i+1);
                }
                println!();
            } else {
                println!("Solution provided is not good!");
            }
        }
    }
}

fn check_solution(forms : &structures::CNF, sol : &structures::Assignation) -> bool{
    forms.0.iter().all(|clause| {
        if clause.is_empty() {true}
        else {clause.iter().any(|index| index.1 == sol[index.0])}
    })
}
