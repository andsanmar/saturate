use crate::structures::*;

type CdclVec = Vec<(Vec<usize>, Option<bool>)>;
type CdclCNF<'a> = (Vec<(&'a Clause, bool)>, usize);
// TODO: Use array with solved clauses (additional to CNF)

type StepHistory = Vec<usize>;

pub fn solve(forms : &CNF) -> Option<Assignation> {
    let mut ass : CdclVec = (0..forms.1).map(|n| {
        let mut v = Vec::new();
        for (index, clause) in forms.0.iter().enumerate() {
            if clause.contains(&(n,true)) || clause.contains(&(n,false)) {v.push(index)}
        }
        (v, None)}).collect();
    let mut forms : CdclCNF = (forms.0.iter().map(|x| (x,false)).collect(), forms.1);
    for first_assignment in vec![true, false] {
        match solve_by_cdcl(&mut forms, &mut ass, first_assignment) {
            Some(x) => return Some(x),
            None => ()
        }}
    None
}

fn solve_by_cdcl(forms : &mut CdclCNF, ass : &mut CdclVec, next : bool) -> Option<Assignation> {
    // history of the step, contains the index of the variables that've been modified
    let mut step : StepHistory = Vec::new();
    match assign_next(ass, next) {
        None => (),
        Some(n) => {
            // Update all clauses related with this one
            for x in &ass[n].0 { forms.0[*x].1 = solved(&forms.0[*x].0, ass); }
            step.push(n)
        }
    }
    unit_propagation(forms, ass, &mut step);
    match conflict(&forms, ass) {
        None => (),
        Some(_) => {
            rollback(&step, ass, forms); // rollback this step
            return None }
    };
    match get_result(ass) {
        Some(y) => return Some(y),
        None => {
            for next_assignment in vec![true, false] {
                match solve_by_cdcl(forms, ass, next_assignment) {
                    Some(x) => return Some(x),
                    None => () //update_cnf(forms, ass) // ()
                }
            }
        }
    }
    rollback(&step, ass, forms); // executed if none of the assignation is possible
    None
}

fn rollback(step : &StepHistory, ass : &mut CdclVec, forms : &mut CdclCNF) {
    for e in step {
        ass[*e].1 = None;
        for x in &ass[*e].0 { forms.0[*x].1 = false; }
    }; // executed if none of the assignation is possible
}

fn assign_next(ass : &mut CdclVec, next : bool) -> Option<usize> {
    for (index, x) in ass.iter().enumerate() { match x.1 {
        None => { ass[index].1 = Some(next);
                  return Some(index) },
        _ => ()
    }}
    None
}

fn conflict (forms : &CdclCNF, ass : &CdclVec) -> Option<usize> { // Returns clause index
    for (index, (clause, _)) in forms.0.iter().enumerate() {
        // true if all the assignations are done and wrong
        if !clause.is_empty() && clause.iter().all(|x| { match ass[x.0].1 {
            None => false,
            Some(y) => x.1 != y }})
        { return Some(index) }
    };
    None
}

fn unit_propagation(forms : &mut CdclCNF, assignment : &mut CdclVec, step : &mut Vec<usize>){
    let mut propagated = false;
    for (clause, solved_clause) in forms.0.iter_mut().filter(|(_, solved_clause)| !solved_clause) {
        let (mut not_assigned, mut assigned) : (Clause, Clause) = (Vec::new(), Vec::new());
        for var in *clause {
            match assignment[var.0].1 {
                None => not_assigned.push(*var),
                Some(z) => {if var.1 == z
                            // Check satisfiability of every element
                            {assigned.push(*var)}}
            }};
        if assigned.is_empty() && not_assigned.len() == 1 {
            // Unit propagation
            let (i, value) = not_assigned.first().unwrap();
            assignment[*i].1 = Some(*value);
            step.push(*i);
            *solved_clause = true;
            propagated = true;
            break; // When propagating, we terminate
        }
    }
    if propagated {unit_propagation(forms, assignment, step);} // If adding a new variable, we do again the unit_propagation
}

fn solved(clause : &Clause, assignment : &CdclVec) -> bool {
    clause.iter().all(|(n, v)| match assignment[*n].1 {
        None => false,
        Some(y) => y == *v})
}

fn get_result( vec : &CdclVec ) -> Option<Assignation> {
    let mut result = Vec::new();
    for e in vec {
        match e.1 {
            None => return None,
            Some(i) => result.push(i)
        }}
    Some(result)
}
