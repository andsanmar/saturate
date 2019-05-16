use crate::structures::*;

// Vector of asignations
// 1st component: index the clauses it forms part of (1 if true, 2 false)
// 2nd component: value assigned and which clause corresponds to (in the case of unit-propagtion)
type CdclVec = Vec<((Vec<usize>, Vec<usize>), Option<(bool, Option<usize>)>)>;

// Vector of pairs: clause and status (solved)
type CdclCNF<'a> = Vec<(&'a Clause, bool)>;

// The index of the variables of the assignments changed and same with clauses solved
type StepHistory = (Vec<usize>, Vec<usize>);

enum AssignationResult {
    Conflict(usize, usize), // Index of clause and index of var
    Ok }

enum CdclResult {
    UNSAT, // Index of clause
    SAT(Assignation) }

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
    match solve_by_cdcl(&mut forms, &mut ass, &mut Vec::new(), true) { // TODO not only with true (based on some decission?)
            CdclResult::SAT(x) => Some(x),
            CdclResult::UNSAT => None
    }
}

fn solve_by_cdcl (forms : &mut CdclCNF, ass : &mut CdclVec, all_steps : &mut Vec<StepHistory>, next : bool) -> CdclResult {
    let mut step : StepHistory = (Vec::new(), Vec::new()); // contains the index of the variables that've been modified
    match assign_next_and_propagate(forms, ass, &mut step, next) {  // rollback the step if there's a conflict
        AssignationResult::Conflict(clause_index, var_index) => {
            all_steps.push(step);
            if process_conflict(forms, ass, all_steps, clause_index, var_index) {
                return CdclResult::UNSAT }}
        AssignationResult::Ok => all_steps.push(step)
    }
    match get_result(ass) {
        Some(y) => CdclResult::SAT(y),
        None => solve_by_cdcl(forms, ass, all_steps, true) // TODO not only with true (based on some decission?)
    }
}

fn rollback((step_a, step_c) : &StepHistory, ass : &mut CdclVec, forms : &mut CdclCNF) {
    for assignment in step_a {
        ass[*assignment].1 = None }
    for clause in step_c {
        forms[*clause].1 = false }
}

// TODO it analyzes the conflict and rollbacks all the necessary, also inserting the new clause derived
fn process_conflict(forms : &mut CdclCNF, ass : &mut CdclVec, all_steps : &mut Vec<StepHistory>, clause_index : usize, var_index : usize) -> bool {
    let to_add : Clause = merge_clauses(get_clauses_conflict(forms, ass, all_steps, clause_index, var_index));
    // TODO process conflict
    // http://satsmt2013.ics.aalto.fi/slides/Marques-Silva.pdf
    // What to do? from the contradiction, get the origins of each assignment (as shown in link):
    // OR a specific variable and its origins negated

    // TODO: Add the clause derived to the forms

    // TODO: Rollback all until clause got
    false
}

// returns all clauses (customized) that cause the conflict
fn get_clauses_conflict(forms : &CdclCNF, ass : &CdclVec, all_steps : &Vec<StepHistory>, clause_index : usize, var_index : usize) -> Vec<Clause> {
    let mut all_clauses = Vec::new();
    let highest_step = all_steps.len();
    let mut first_clause : Clause = forms[clause_index].0.clone().iter().map(|(v,b)| (*v,!b)).collect();
    first_clause.retain(|(var,_value)| *var != var_index);
    
    for var in &first_clause {
        match ass[var.0].1.unwrap().1 { // TODO handle the option
            Some(ci) => all_clauses.append( &mut get_clauses_conflict(forms, ass, all_steps, ci, var.0) ),
            None => continue
        }
    }
    all_clauses.push(first_clause);
    //TODO
        
    all_clauses
}

fn merge_clauses(clauses : Vec<Clause>) -> Clause { // TODO correct?
    let mut accumulator = Vec::new();
    let mut repeated = Vec::new();
    for (elem, val) in clauses.iter().flatten() {
        if ! repeated.contains(elem) {
            match accumulator.remove_item(&(*elem,!val)) {
                None => if ! accumulator.contains(&(*elem,*val)) { accumulator.push((*elem,*val)) },
                Some(_) => repeated.push(*elem)
            }
        }
    }
    accumulator
}

// TODO Launch this process in parallel and send the result over a channel
// Returns true if there's a conflict, updates the clause status if not
fn conflict_on_clause (forms : &mut CdclCNF, clause_index : &usize, ass : &CdclVec, step : &mut StepHistory) -> bool {
    let (clause, solved) = forms[*clause_index];
    if solved { return false }
    if !clause.is_empty() {
        // Find if there's some clause not assigned yet or one assignation correct
        match clause.iter().find(|(var,value)| { match ass[*var].1 { None => true,
                                                                     Some (expected_value) => expected_value.0 == *value }}) {
            Some(_) => return false,
            None => ()
        }
    }
    forms[*clause_index].1 = false; // TODO it was wrong!
    step.1.push(*clause_index);
    true
}

// Returns if there's a conflict when assigning
fn assign_next_and_propagate (forms : &mut CdclCNF, ass : &mut CdclVec, step : &mut StepHistory, next : bool) -> AssignationResult {
    match ass.iter().enumerate().find(|(_, x)| x.1 == None) {
        Some((var_index, _)) => { ass[var_index].1 = Some((next, None));
                                  step.0.push(var_index);
                                  // Inspect the contrary (if we make it true inspect the ones where the assignment should be false)
                                  match (if next {&(ass[var_index].0).1} else {&(ass[var_index].0).0}).iter().find(|clause_index| conflict_on_clause(forms, clause_index, ass, step)) {
                                      // Check if makes some clause false, if so, return clause index
                                      Some(clause_index) => return AssignationResult::Conflict(*clause_index, var_index),
                                      None => () }
                                  return unit_propagation(forms, ass, (var_index, next), step) },
        _ => ()
    }
    AssignationResult::Ok
}

// Returns if there's a conflict when propagating
fn unit_propagation (forms : &mut CdclCNF, ass : &mut CdclVec, (last_index, last_assignment) : (usize, bool), step : &mut StepHistory) -> AssignationResult {
    let clauses_to_solve : &Vec<usize> = if last_assignment {&(ass[last_index].0).1} else {&(ass[last_index].0).0};
    match to_propagate(forms, ass, clauses_to_solve) {
        Some((var_index, value, clause_index)) => {
            ass[var_index].1 = Some((value, Some(clause_index)));
            step.0.push(var_index);
            forms[clause_index].1 = true;
            step.1.push(clause_index);
            // Check if makes some clause false, if so, return false
            match (if value {&(ass[var_index].0).1} else {&(ass[var_index].0).0}).iter().find(|clause_index| { conflict_on_clause(forms, clause_index, ass, step)}) {
                Some(clause_index) => {return AssignationResult::Conflict(*clause_index, var_index)}
                None => ()
            }
            unit_propagation(forms, ass, (var_index, value), step) } // If adding a new variable, we do again the unit_propagation
        None => { AssignationResult::Ok }
    }
}

fn get_result (vec : &CdclVec) -> Option<Assignation> {
    let mut result = Vec::new();
    for e in vec {
        match e.1 {
            None => return None,
            Some(i) => result.push(i.0)
        }}
    Some(result)
}

// If only last one element to be assigned and the rest aren't satisfied, returns it
fn get_propagation (clause : &Clause, ass : &CdclVec) -> Option<(usize, bool)> {
    let (mut not_assigned, mut assigned) : (Clause, Clause) = (Vec::new(), Vec::new());
    for var in clause {
        match ass[var.0].1 {
            None => not_assigned.push(*var),
            Some(z) => { if var.1 == z.0
                         // Check satisfiability of every element
                         { assigned.push(*var) }}
        }};
    if assigned.is_empty() && not_assigned.len() == 1 {
        Some(*not_assigned.first().unwrap())
    } else { None }
}

// Returns the variable assignationof the clause that must be solved
fn to_propagate ( forms : &CdclCNF, ass : &CdclVec, clauses_to_solve : &Vec<usize> ) -> Option<(usize, bool, usize)> {
    for clause_index in clauses_to_solve.iter().filter(|index| !forms[**index].1) {
        match get_propagation(forms[*clause_index].0, ass) {
            Some((i, value)) => { return Some((i, value, *clause_index)); }
            None => () }}
    None
}
