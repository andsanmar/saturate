use crate::structures::*;
use crate::bins::*;

pub fn solve(forms : CNF) -> Option<Assignation> {
    let mut ass : Option<Assignation> = Some(create(forms.1 as usize));
    loop {
        match ass {
            None => return None,
            Some(a) => {
                if satisfy(&forms, &a){
                    return Some(a)}
                ass = grow(a);
            }}}}

fn satisfy(forms : &CNF, assignments : &Assignation) -> bool {
    forms.0.iter().all(|x| {
        if x.is_empty() { true }
        else {check(x.to_vec(), &assignments)}
    })
}

fn check(formula : Clause, assignments : &Assignation) -> bool {
    formula.iter().any(|x| {
        let y = assignments[x.0];
        x.1 == y
    })
}
