// Vector of clauses and number of vars
pub type CNF = (Vec<Clause>, usize);
// Vector of index on final assignment and symbol
pub type Clause = Vec<(usize, bool)>;
// Final assignment (vector of the size of number of vars)
pub type Assignation = Vec<bool>;
