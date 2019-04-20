use crate::structures::*;

type CdclVec = Vec<(Option<bool>)>;
// TODO: Use data structure for fast search

type StepHistory = Vec<usize>;

pub fn solve(forms : &CNF) -> Option<Assignation> {
    let mut ass : CdclVec = (0..forms.1).map(|_| None).collect(); // TODO
    for first_assignment in vec![true, false] {
        match solve_by_cdcl(forms, &mut ass, first_assignment) {
            Some(x) => return Some(x),
            None => ()
        }}
    None
}

// return history of each branch
fn solve_by_cdcl(forms : &CNF, ass : &mut CdclVec, next : bool) -> Option<Assignation> {
    // history of the step
    let mut step : StepHistory = Vec::new();
    match assign_next(ass, next) {
        None => (),
        Some(n) => step.push(n)
    }
    unit_propagation(&forms, ass, &mut step);
    match conflict(&forms, ass) {
        None => (),
        Some(_) => {
            for e in &step { ass[*e] = None }; // rollback this step
            return None }
    };
    match get_result(ass) {
        Some(y) => return Some(y),
        None => {
            for next_assignment in vec![true, false] {
                match solve_by_cdcl(forms, ass, next_assignment) {
                    Some(x) => return Some(x),
                    None => ()
                }
            }
        }
    }
    for e in &step { ass[*e] = None }; // rollback this step
    None
}

fn assign_next(ass : &mut CdclVec, next : bool) -> Option<usize> {
    for (index, x) in ass.iter().enumerate() { match x {
        None => { ass[index] = Some(next);
                  return Some(index) },
        _ => ()
    }}
    None
}

fn conflict (forms : &CNF, ass : &CdclVec) -> Option<Clause> {
    for clause in &forms.0 {
        // true if all the assignations are done and wrong
        if !clause.is_empty() && clause.iter().all(|x| { match ass[x.0] {
            None => false,
            Some(y) => x.1 != y }})
        { return Some(clause.to_vec()) }
    };
    None
}

fn unit_propagation(forms : &CNF, assignment : &mut CdclVec, step : &mut Vec<usize>){
    for clause in &forms.0 {
        let (mut not_assigned, mut assigned) : (Clause, Clause) = (Vec::new(), Vec::new());
        for var in clause {
            match assignment[var.0] {
                None => not_assigned.push(*var),
                Some(z) => {if var.1 == z
                            // Check satisfiability of every element
                            {assigned.push(*var)}}
            }};
        if assigned.is_empty() && not_assigned.len() == 1 {
            // Unit propagation
            let index = not_assigned.first().unwrap();
            assignment[index.0] = Some(index.1);
            step.push(index.0);
            // If adding a new variable, we do again the unit_propagation
            unit_propagation(forms, assignment, step);
        }
    }
}

fn get_result( vec : &CdclVec ) -> Option<Assignation> {
    let mut result = Vec::new();
    for e in vec {
        match e {
            None => return None,
            Some(i) => result.push(*i)
        }}
    Some(result)
}
