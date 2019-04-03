
use std::io::{self, Read};
//use std::env;

mod lib;

pub mod parser{
    use crate::lib::structures::*;
    
    pub fn get_formulas(input : String) -> CNF {
        let forms : Vec<String> = input.split_whitespace().map(|x| x.to_string()).collect();
        let (defs, all_formulas) = forms.split_at(4); // Insert comments to ignore
        let n_vars : u8 = defs.get(2).unwrap().to_string().parse().unwrap();
        let n_formulas : usize = defs.get(3).unwrap().to_string().parse().unwrap();
        let formulas : Vec<Formula> = all_formulas.split(|s| s == "0").
            map(|i| create_formula(n_vars, i.to_vec())).collect(); // TODO all at the same time , not creating a vector
        // The last element of the vector must be empty
        assert_eq!(n_formulas + 1, formulas.len(), "Number of formulas and size don't match!");
        formulas
    }

    fn create_formula(n_vars : u8, input : Vec<String>) -> Formula {
        for i in input.clone() {
            let n : i8 = i.parse().unwrap();
            assert!(n.abs() as u8 <= n_vars,"More variables than specified!");
        }
        Formula(input.iter().map(|x| x.parse().unwrap()).collect())
    }
}


fn main() {
    // let file = if env::args().count() == 2 {
    //     env::args().nth(1).unwrap()
    // } else {
    //     panic!("Please enter a file")
    // };

    // let formula = get_formula(open(&Path::new(&file)).expect("Opening file failed"));
    let mut buffer : String = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    for i in parser::get_formulas(buffer) {
        for j in i.0 {
            print!("{} ", j);
        }
        println!("");
    }
}
