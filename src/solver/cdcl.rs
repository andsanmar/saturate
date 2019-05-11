use crate::structures::*;

// Vector of asignations
// 1st component: index the clauses it forms part of (1 if true, 2 false)
// 2nd component: value assigned
type CdclVec = Vec<((Vec<usize>, Vec<usize>), Option<bool>)>;

// Vector of pairs: clause and status (solved)
type CdclCNF<'a> = Vec<(&'a Clause, bool)>;

// The index of the variables of the assignments changed
type StepHistory = Vec<usize>;

enum AssignationResult {
    Conflict(usize), // Index of clause
    Ok
}

enum CdclResult {
    Conflict(usize), // Index of clause
    Solved(Assignation)
}

// Create the data structures and call the algorithm to solve
pub fn solve(forms : &CNF) -> Option<Assignation> {
    let mut ass : CdclVec = (0..forms.1).map(|n| {
        let mut v = (Vec::new(), Vec::new());
        for (index, clause) in forms.0.iter().enumerate() {
            if clause.contains(&(n,true)) {v.0.push(index)}
            if clause.contains(&(n,false)) {v.1.push(index)}
        }
        (v, None)}).collect();

    let mut forms : CdclCNF = forms.0.iter().map(|x| (x,false)).collect();
    for first_assignment in vec![true, false] {
        match solve_by_cdcl(&mut forms, &mut ass, first_assignment) {
            CdclResult::Solved(x) => return Some(x),
            CdclResult::Conflict(_) => ()
        }}
    None
}

// TODO: acumulate steps?
fn solve_by_cdcl(forms : &mut CdclCNF, ass : &mut CdclVec, next : bool) -> CdclResult {
    let mut step : StepHistory = Vec::new(); // contains the index of the variables that've been modified
    match assign_next_and_propagate(forms, ass, &mut step, next) {  // rollback the step if there's a conflict
        AssignationResult::Conflict(index) => {
            rollback(&step, ass, forms);
            return CdclResult::Conflict(index) }
        AssignationResult::Ok => ()
    }
    match get_result(ass) {
        Some(y) => return CdclResult::Solved(y),
        None => {
            for next in vec![true, false] {
                match solve_by_cdcl(forms, ass, next) {
                    CdclResult::Solved(x) => return CdclResult::Solved(x),
                    CdclResult::Conflict(_index) => () // TODO process conflict
                }
            }
        }
    }
    // We should never get here!
    // loop {}
    rollback(&step, ass, forms);
    CdclResult::Conflict(0)
}

fn rollback(step : &StepHistory, ass : &mut CdclVec, forms : &mut CdclCNF) {
    for e in step {
        let prev_value = ass[*e].1.unwrap();
        ass[*e].1 = None;
        for x in if prev_value {&(ass[*e].0).1} else {&(ass[*e].0).0} { forms[*x].1 = false; }
    };
}

// Returns true if there's a conflict, updates the clause status if not
fn conflict_on_clause (forms : &mut CdclCNF, clause_index : usize, ass : &CdclVec) -> bool {
    let (clause, solved) = forms[clause_index];
    if solved { return false }
    if !clause.is_empty() {
        for (var, value) in clause {
            match ass[*var].1 {
                None => return false,
                Some(expected_value) => if *value == expected_value { return false }
            }}}
    forms[clause_index].1 = true;
    true
}

// Returns if there's a conflict when assigning
fn assign_next_and_propagate (forms : &mut CdclCNF, ass : &mut CdclVec, step : &mut StepHistory, next : bool) -> AssignationResult {
    for (index, x) in ass.iter().enumerate() { match x.1 {
        None => { ass[index].1 = Some(next);
                  step.push(index);
                  // Inspect the contrary (if we make it true inspect the ones where the assignment should be false)
                  for clause_index in if next {&(ass[index].0).1} else {&(ass[index].0).0} {
                      // Check if makes some clause false, if so, return clause index
                      if conflict_on_clause(forms, *clause_index, ass) {
                          return AssignationResult::Conflict(*clause_index)
                      }
                  }
                  return unit_propagation(forms, ass, step) },
        _ => ()
    }}
    AssignationResult::Ok
}

// Returns if there's a conflict when propagating
fn unit_propagation (forms : &mut CdclCNF, ass : &mut CdclVec, step : &mut Vec<usize>) -> AssignationResult {
    let mut propagated : Option<(usize, bool)> = None;
    for (clause, solved_clause) in forms.iter_mut().filter(|(_, solved_clause)| !solved_clause) {
        //let clause = forms[*clause_index].0;
        let (mut not_assigned, mut assigned) : (Clause, Clause) = (Vec::new(), Vec::new());
        for var in *clause {
            match ass[var.0].1 {
                None => not_assigned.push(*var),
                Some(z) => { if var.1 == z
                            // Check satisfiability of every element
                             { assigned.push(*var) }}
            }};
        if assigned.is_empty() && not_assigned.len() == 1 {
            // Unit propagation
            let (i, value) = not_assigned.first().unwrap();
            ass[*i].1 = Some(*value);
            step.push(*i);
            *solved_clause = true;
            for clause_index in if *value {&(ass[*i].0).1} else {&(ass[*i].0).0} { // Check if makes some clause false, if so, return false
                if conflict_on_clause(forms, *clause_index, ass) { return AssignationResult::Conflict(*clause_index) }
            }
            propagated = Some((*i, *value));
            break; // When propagating, we terminate
        }
    }
    match propagated {
        Some(_last_assignment) => { unit_propagation(forms, ass, step) } // If adding a new variable, we do again the unit_propagation
        None => { AssignationResult::Ok }
    }
}

fn get_result (vec : &CdclVec) -> Option<Assignation> {
    let mut result = Vec::new();
    for e in vec {
        match e.1 {
            None => return None,
            Some(i) => result.push(i)
        }}
    Some(result)
}
