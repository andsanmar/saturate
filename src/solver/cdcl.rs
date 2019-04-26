use crate::structures::*;

type CdclVec = Vec<(Vec<usize>, Option<bool>)>;
type CdclCNF<'a> = (Vec<(&'a Clause, bool)>, usize);

type StepHistory = Vec<usize>;

// Create the data structures and call the algorithm to solve
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
    if assign_next_and_propagate(forms, ass, &mut step, next) {
        rollback(&step, ass, forms); // rollback this step
        return None
    }
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
    rollback(&step, ass, forms); // executed if none of the assignation is possible
    None
}

fn rollback(step : &StepHistory, ass : &mut CdclVec, forms : &mut CdclCNF) {
    for e in step {
        ass[*e].1 = None;
        for x in &ass[*e].0 { forms.0[*x].1 = false; }
    }; // executed if none of the assignation is possible
}

// Returns true if there's a conflict, updates the clause status if not
fn conflict_on_clause (forms : &mut CdclCNF, clause_index : usize, ass : &CdclVec) -> bool {
    let (clause, solved) = forms.0[clause_index];
    if solved { return false }
    if !clause.is_empty() {
        for (var, value) in clause {
            match ass[*var].1 {
                None => return false,
                Some(expected_value) => if *value == expected_value { return false }
            }}}
    forms.0[clause_index].1 = true;
    true
}

// Returns if there's a conflict when assigning
fn assign_next_and_propagate (forms : &mut CdclCNF, ass : &mut CdclVec, step : &mut StepHistory, next : bool) -> bool {
    for (index, x) in ass.iter().enumerate() { match x.1 {
        None => { ass[index].1 = Some(next);
                  step.push(index);
                  for clause_index in &ass[index].0 { // Check if makes some clause false, if so, return false
                      if conflict_on_clause(forms, *clause_index, ass) {
                          return true
                      }
                  }
                  return unit_propagation(forms, ass, step) },
        _ => ()
    }}
    false
}

// Returns if there's a conflict when propagating
fn unit_propagation (forms : &mut CdclCNF, ass : &mut CdclVec, step : &mut Vec<usize>) -> bool {
    let mut propagated = false;
    for (clause, solved_clause) in forms.0.iter_mut().filter(|(_, solved_clause)| !solved_clause) {
        let (mut not_assigned, mut assigned) : (Clause, Clause) = (Vec::new(), Vec::new());
        for var in *clause {
            match ass[var.0].1 {
                None => not_assigned.push(*var),
                Some(z) => {if var.1 == z
                            // Check satisfiability of every element
                            {assigned.push(*var)}}
            }};
        if assigned.is_empty() && not_assigned.len() == 1 {
            // Unit propagation
            let (i, value) = not_assigned.first().unwrap();
            ass[*i].1 = Some(*value);
            step.push(*i);
            *solved_clause = true;
            for clause_index in &ass[*i].0 { // Check if makes some clause false, if so, return false
                if conflict_on_clause(forms, *clause_index, ass) {
                          return true
                }
            }
            propagated = true;
            break; // When propagating, we terminate
        }
    }
    if propagated { unit_propagation(forms, ass, step) } // If adding a new variable, we do again the unit_propagation
    else { false }
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
