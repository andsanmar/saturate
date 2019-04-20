use crate::structures::*;

// TODO: Use data structure for fast search

pub fn solve(forms : &CNF) -> Option<Assignation> {
    let mut ass : Assignation = Vec::new(); // TODO
        loop {
            if can_continue_cnf(&forms, &ass){
                if ass.len() == forms.1 { return Some(ass); }
                else {ass.push(true);}
            } else {
                match revert(&mut ass) {
                    false => return None,
                    true => ()
                }
            }
        }

fn can_continue_cnf(forms : &CNF, assignation : &Assignation) -> bool {
    forms.0.iter().all(|x| {x.is_empty() || can_continue_clause(x, assignation)})
}}
    

fn can_continue_clause(clause : &Clause, assignation : &Assignation) -> bool {
    clause.iter().any(|x| { match assignation.get(x.0) {
        None => true,
        Some(z) => x.1 == *z
    }})
}

fn revert ( to_rev : &mut Assignation) -> bool {
    match to_rev.pop() {
        None => false,
        Some(true) => {to_rev.push(false); true},
        Some(false) => revert(to_rev)
    }
}
