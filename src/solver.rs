// use rand::prelude::*;

//use std::io::{self, Read};
// use std::env;
// use std::fs;
mod lib;

pub mod solver {
    use crate::lib::structures::*;
    use crate::lib::bin_func::*;

    pub fn solve(forms : CNF) -> Option<Assignation> {
        let l : usize = forms.first().into_iter().len();
        let mut ass : Option<Assignation> = Some(create(l));
        while ass != None {
            if satisfy(forms.clone(), ass.clone().unwrap()) {
                return ass
            } else {
                ass = grow(ass.unwrap());
            }
        }
        None
    }
    
    fn satisfy(forms : CNF, assignments : Assignation) -> bool {
        forms.iter().all(|x| check(x.to_vec(), assignments.clone()))
        // TODO: Not clone
    }

    fn check(formula : Formula, assignments : Assignation) -> bool {
        for x in formula {
            if x > 0 {
                if !*assignments.get(x as usize).unwrap(){
                    return true;
                }
            } else {
                if *assignments.get(x as usize).unwrap() {
                    return true;
                }
            }
        }
        false
    }
}

fn main() {
    println!("{:?}", solver::solve(vec![vec![0]]));
    
    // let x = crate::lib::bin_func::create(3);
    // let y = crate::lib::bin_func::grow(x.clone()).unwrap();
    // let z = crate::lib::bin_func::grow(y.clone()).unwrap();
    // let a = crate::lib::bin_func::grow(z.clone()).unwrap();
    // let b = crate::lib::bin_func::grow(a.clone()).unwrap();
    // println!("{:?}", x);
    // println!("{:?}", y);
    // println!("{:?}", z);
    // println!("{:?}", a);
    // println!("{:?}", b);
}
