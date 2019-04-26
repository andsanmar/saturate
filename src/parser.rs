use crate::structures::*;

// type CnfDef = (u8, usize);

enum ParsedLine {
    F(Clause),
    Empty,
}

static mut N_VARS : usize = 0;
static mut _N_FORMULAS : usize = 0;

pub fn get_formulas(input : String) -> CNF {
    // TODO: Another way of splitting newlines
    let formulas : Vec<Clause> = input.split("\n").map(|x| parse_line(x.to_string())).collect::<Vec<ParsedLine>>().iter().fold(Vec::new(), |mut v, x| match x {
        ParsedLine::F(f) => {v.push(f.to_vec()); v},
        _ => v,
    });
    // The last element of the vector must be empty
    //assert_eq!(N_FORMULAS + 1, formulas.len(), "Number of formulas and size don't match!");
    unsafe{(formulas, N_VARS)}
}

fn parse_line(input_string : String) -> ParsedLine {
    let input : Vec<String> = input_string.split_whitespace().map(|x| x.to_string()).collect();
    if input.first() == Some(&"c".to_string()) || input.first() == Some(&"%".to_string()) { return ParsedLine::Empty }
    if input.first() == Some(&"p".to_string()) {
        unsafe{ //TODO: evict data races
            N_VARS = input.get(2).unwrap().to_string().parse().unwrap();
            _N_FORMULAS = input.get(3).unwrap().to_string().parse().unwrap();}
        return ParsedLine::Empty } // TODO: Parse the cnf def
    let formula = match input.split_last() {
        Some((l, f)) => { assert_eq!(l.parse::<u8>().unwrap(), 0, "Bad definition of line"); f},
        None => &input,
    };
    for i in &input {
        unsafe{assert!(i.parse::<i16>().unwrap().abs() as usize <= N_VARS,"More variables than specified!");}
    }
    ParsedLine::F(formula.iter().map(|x| {
        let y : isize = x.parse().unwrap();
        (y.abs() as usize - 1, y > 0)
    }).collect())
}
