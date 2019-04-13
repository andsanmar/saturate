use crate::structures::*;

pub fn solve(forms : CNF) -> Option<Assignation> {
    let mut ass : Assignation = Vec::new(); // TODO
        loop {
            if can_continue_cnf(&forms, &ass){
                if ass.len() == forms.1 { return Some(ass); }
                else {ass.push(true);}
            } else {
                match revert(ass) {
                    None => return None,
                    Some(x) => ass = x
                }
            }
        }

fn can_continue_cnf(forms : &CNF, assignation : &Assignation) -> bool {
    forms.0.iter().all(|x| {
        match x.as_slice() {
            [] => true,
            _ => can_continue_clause(x.to_vec(), &assignation)
        }
    })
}}
    

fn can_continue_clause(clause : Clause, assignation : &Assignation) -> bool {
    clause.iter().any(|x| {
        if *x > 0 {
            match assignation.get(*x as usize -1) { // TODO neg numbers
                None => true,
                Some(x) => *x,
        }} else {
            match assignation.get((-*x) as usize -1) { // TODO neg numbers
                None => true,
                Some(x) => !x,
            }
        } 
    })
}

fn revert ( mut to_rev : Assignation) -> Option<Assignation> {
    match to_rev.pop() {
        None => None,
        Some(true) => {to_rev.push(false); Some(to_rev)},
        Some(false) => revert(to_rev)
    }
}
