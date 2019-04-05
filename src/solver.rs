use crate::structures::*;
use crate::bins::*;

pub fn solve(forms : CNF) -> Option<Assignation> {
    let mut ass : Option<Assignation> = Some(create(forms.1 as usize));
    loop {
        match ass {
            None => return None,
            Some(_) => {
                // TODO: Not clone
                if satisfy(&forms, ass.clone().unwrap()){
                    return ass}
                ass = grow(ass.unwrap());
            }}}}

fn satisfy(forms : &CNF, assignments : Assignation) -> bool {
    forms.0.iter().all(|x| {
        match x.as_slice() {
            [] => true,
            _ => check(x.to_vec(), &assignments)
        }
    })
}

pub fn check(formula : Formula, assignments : &Assignation) -> bool {
    for x in formula {
        if x > 0 {
            if !*assignments.get(x as usize - 1).unwrap(){ // -1: Due to vec size
                return true;
            }
        } else {
            if *assignments.get((-x as usize) - 1).unwrap() { // -1: Due to vec size
                return true;
            }
        }
    }
    false
}

fn main() {
    println!("{:?}", crate::solver::solve((vec![vec![-2, -1], vec![2, 1], vec![2, -1], vec![-2, 1]], 2)))
}
