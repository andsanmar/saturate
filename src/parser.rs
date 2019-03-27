
use std::io::{self, Read};
//use std::env;

mod lib;

mod parser{
    use crate::lib::structures::Formula;
    
    pub fn get_formulas(input : String) -> (u8, usize, Vec<Formula>) {
        let mut forms = Vec::<String>::new();
        let elems : std::str::SplitWhitespace = input.split_whitespace();
        for elem in elems {
            forms.push(elem.to_string());
        }
        let (defs, all_formulas) = forms.split_at(4);
        let n_vars : u8 = defs.get(2).unwrap().to_string().parse().unwrap();
        let n_formulas : usize = defs.get(3).unwrap().to_string().parse().unwrap();
        let mut formulas = Vec::<Formula>::new();
        for i in all_formulas.split(|s| s == "0"){
            formulas.push(create_formula(i.to_vec()));
        }
        // The last element of the vector must be empty
        assert_eq!(n_formulas + 1, formulas.len(), "Number of formulas and size don't match!");
        (n_vars, n_formulas, formulas)
    }

    fn create_formula(input : Vec<String>) -> Formula {
        Formula(input)
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
    let form = parser::get_formulas(buffer);
    println!("{}", form.0);
    println!("{}", form.1);
    for i in form.2 {
        for j in i.0 {
            print!("{} ", j);
        }
        println!("");
    }
}
