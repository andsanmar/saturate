use crate::structures::*;
use std::sync::{Arc, RwLock};
use std::sync::mpsc;

// TODO global data structure with all the info of the execution!

// Vector of asignations
// 1st component: index the clauses it forms part of (1 if true, 2 false)
// 2nd component: value assigned
type CdclVec = Vec<((Vec<usize>, Vec<usize>), Option<bool>)>;

// Vector of pairs: clause and status (solved)
type CdclCNF<'a> = Vec<(&'a Clause, bool)>;

// The index of the variables of the assignments changed and same with clauses solved
type StepHistory = (Vec<usize>, Vec<usize>);

enum AssignationResult {
    Conflict(usize), // Index of clause
    Ok }

enum CdclResult {
    Conflict(usize), // Index of clause
    Solved(Assignation) }

// Create the data structures and call the algorithm to solve
pub fn solve(forms : &CNF) -> Option<Assignation> {
    let mut ass : CdclVec = (0..forms.1).map(|n| {
        let mut v = (Vec::new(), Vec::new());
        for (index, clause) in forms.0.iter().enumerate() {
            if clause.contains(&(n,true)) {v.0.push(index)}
            if clause.contains(&(n,false)) {v.1.push(index)}
        }
        (v, None)}).collect();

    let forms : Arc<RwLock<CdclCNF>> = Arc::new(RwLock::new(forms.0.iter().map(|x| (x,false)).collect()));
    //let ass_ref : Arc<RwLock<CdclVec>> = Arc::new(RwLock::new(ass));
    for first_assignment in vec![true, false] {
        match solve_by_cdcl(&forms, &mut ass, first_assignment) {
            CdclResult::Solved(x) => return Some(x),
            CdclResult::Conflict(_) => ()
        }}
    None
}

fn solve_by_cdcl (forms : &Arc<RwLock<CdclCNF>>, ass : &mut CdclVec, next : bool) -> CdclResult {
    let mut step : StepHistory = (Vec::new(), Vec::new()); // contains the index of the variables that've been modified
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
            }}
    }
    rollback(&step, ass, forms);
    CdclResult::Conflict(0)
}

fn rollback((step_a, step_c) : &StepHistory, ass : &mut CdclVec, forms : &Arc<RwLock<CdclCNF>>) {
    for assignment in step_a {
        ass[*assignment].1 = None;}
    for clause in step_c {
        forms.write().unwrap()[*clause].1 = false }
}

// Returns true if there's a conflict, updates the clause status if not
fn conflict_on_clause (forms : &Arc<RwLock<CdclCNF>>, clause_index : &usize, ass : &CdclVec, step : &mut StepHistory) -> bool {
    let (clause, solved) = forms.write().unwrap()[*clause_index];
    if solved { return false }
    if !clause.is_empty() {
        // Find if there's some clause not assigned yet or one assignation correct
        match clause.iter().find(|(var,value)| { match ass[*var].1 { None => true,
                                                                     Some (expected_value) => expected_value == *value }}) {
            Some(_) => return false,
            None => ()
        }
    }
    forms.write().unwrap()[*clause_index].1 = true;
    step.1.push(*clause_index);
    true
}

// Returns if there's a conflict when assigning
fn assign_next_and_propagate (forms : &Arc<RwLock<CdclCNF>>, ass : &mut CdclVec, step : &mut StepHistory, next : bool) -> AssignationResult {
    match ass.iter().enumerate().find(|(_, x)| x.1 == None) {
        Some((index, _)) => { ass[index].1 = Some(next);
                              step.0.push(index);
                              // Inspect the contrary (if we make it true inspect the ones where the assignment should be false)
                              match (if next {&(ass[index].0).1} else {&(ass[index].0).0}).iter().find(|clause_index| conflict_on_clause(forms, clause_index, ass, step)) {
                                  // Check if makes some clause false, if so, return clause index
                                  Some(clause_index) => return AssignationResult::Conflict(*clause_index),
                                  None => () }
                              return unit_propagation(forms, ass, (index, next), step) },
        _ => ()
    }
    AssignationResult::Ok
}

// Returns if there's a conflict when propagating
fn unit_propagation (forms : &Arc<RwLock<CdclCNF>>, ass : &mut CdclVec, (last_index, last_assignment) : (usize, bool), step : &mut StepHistory) -> AssignationResult {
    // The process to_propagate tells by this channel the new vars to change
    let (sender_vars, receiver_vars) = mpsc::channel();
    // The process unit_propagation tells by this channel the new clauses to inspect
    let (sender_clauses, receiver_clauses) : (mpsc::Sender<(&Clause, usize, &CdclVec)>, mpsc::Receiver<(&Clause, usize, &CdclVec)>) = mpsc::channel();

    {
        let t = &ass[last_index].0;
        for clause_index in if last_assignment {&t.1} else {&t.0} {
            let (clause, valid) = forms.read().unwrap()[*clause_index];
            if !valid {
                sender_clauses.send((clause, *clause_index, &ass)).unwrap();
            }
        }
        drop(sender_clauses);
    }

    to_propagate(sender_vars, receiver_clauses); // TODO other thread with to_propagate only

    let mut c = None;
    for (i, value, clause_index) in receiver_vars {
        c = Some((i,value));
        // match ass[i].1 {
        //     None => (),
        //     Some(v) => if v != value {return AssignationResult::Conflict(clause_index)}
        // }
        ass[i].1 = Some(value); // TODO: detect conflict at this step (by looking if it's currently assigned with the contrary value) instead of calling "conflict_con_clause"
        step.0.push(i);
        forms.write().unwrap()[clause_index].1 = true;
        step.1.push(clause_index);
        // Check if makes some clause false, if so, return false
        match (if value {&(ass[i].0).1} else {&(ass[i].0).0}).iter().find(|clause_index| { conflict_on_clause(forms, clause_index, ass, step)}) {
            Some(clause_index) => {return AssignationResult::Conflict(*clause_index)}
            None => ()
        }
    } // If adding a new variable, we do again the unit_propagation
    match c {
        Some((i,value)) => unit_propagation(forms, ass, (i, value), step),
        None => AssignationResult::Ok
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

// If only last one element to be assigned and the rest aren't satisfied, returns it
fn get_propagation (clause : &Clause, ass : &CdclVec) -> Option<(usize, bool)> {
    let (mut not_assigned, mut assigned) : (Clause, Clause) = (Vec::new(), Vec::new());
    for var in clause {
        match ass[var.0].1 {
            None => not_assigned.push(*var),
            Some(z) => { if var.1 == z
                         // Check satisfiability of every element
                         { assigned.push(*var) }}
        }};
    if assigned.is_empty() && not_assigned.len() == 1 {
        Some(*not_assigned.first().unwrap())
    } else { None }
}

// Returns the variable assignationof the clause that must be solved
fn to_propagate (send_vars : mpsc::Sender<(usize, bool, usize)>, receive_clauses : mpsc::Receiver<(&Clause, usize, &CdclVec)> ) {
    for (clause, clause_index, ass) in receive_clauses {
        match get_propagation(clause, ass) {
            Some((i, value)) => { send_vars.send((i, value, clause_index)).unwrap() }
            None => () }}
}

