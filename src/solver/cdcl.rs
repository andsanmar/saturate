use crate::structures::*;

type CdclVec = Vec<(Option<bool>)>;
type CdclCNF = (Vec<(Clause, bool)>, usize);
// TODO: Use array with solved clauses (additional to CNF)

type StepHistory = Vec<usize>;

pub fn solve(forms : &CNF) -> Option<Assignation> {
    let mut ass : CdclVec = (0..forms.1).map(|_| None).collect();
    let mut forms : CdclCNF = (forms.0.iter().map(|x| (x.clone(),false)).collect(), forms.1);
    for first_assignment in vec![true, false] {
        match solve_by_cdcl(&mut forms, &mut ass, first_assignment) {
            Some(x) => return Some(x),
            None => ()
        }}
    None
}

fn solve_by_cdcl(forms : &mut CdclCNF, ass : &mut CdclVec, next : bool) -> Option<Assignation> {
    // history of the step
    let mut step : StepHistory = Vec::new();
    match assign_next(ass, next) {
        None => (),
        Some(n) => step.push(n)
    }
    update_cnf(forms, ass);
    unit_propagation(forms, ass, &mut step);
    match conflict(&forms, ass) {
        None => (),
        Some(_) => {
            for e in &step { ass[*e] = None; }; // rollback this step
            return None }
    };
    match get_result(ass) {
        Some(y) => return Some(y),
        None => {
            for next_assignment in vec![true, false] {
                match solve_by_cdcl(forms, ass, next_assignment) {
                    Some(x) => return Some(x),
                    None => update_cnf(forms, ass)
                }
            }
        }
    }
    for e in &step { ass[*e] = None; }; // rollback this step
    None
}

fn assign_next(ass : &mut CdclVec, next : bool) -> Option<usize> {
    // TODO: modify relation between assigned variables and clauses solved
    for (index, x) in ass.iter().enumerate() { match x {
        None => { ass[index] = Some(next);
                  return Some(index) },
        _ => ()
    }}
    None
}

fn conflict (forms : &CdclCNF, ass : &CdclVec) -> Option<usize> { // Returns clause index
    for (index, (clause, _)) in forms.0.iter().enumerate() {
        // true if all the assignations are done and wrong
        if !clause.is_empty() && clause.iter().all(|x| { match ass[x.0] {
            None => false,
            Some(y) => x.1 != y }})
        { return Some(index) }
    };
    None
}

fn unit_propagation(forms : &mut CdclCNF, assignment : &mut CdclVec, step : &mut Vec<usize>){
    let mut propagated = false;
    for (clause, solved_clause) in forms.0.iter_mut() {
        if !*solved_clause {
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
                let (i, value) = not_assigned.first().unwrap();
                assignment[*i] = Some(*value);
                step.push(*i);
                *solved_clause = true;
                propagated = true;
                break; // WHen propagating, we terminate
            }}
    }
    if propagated {unit_propagation(forms, assignment, step);} // If adding a new variable, we do again the unit_propagation
}

fn update_cnf(forms : &mut CdclCNF, assignment : &CdclVec) {
    for (clause, solved_clause) in forms.0.iter_mut() {
        *solved_clause = solved(clause, assignment);
    }
}

fn solved(clause : &Clause, assignment : &CdclVec) -> bool {
    clause.iter().all(|(n, v)| match assignment[*n] {
        None => false,
        Some(y) => y == *v})
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
