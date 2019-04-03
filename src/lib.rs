pub mod structures{
    pub type CNF = Vec<Formula>;
    pub type Formula = Vec<i8>;
    pub type Assignation = Vec<bool>;
}

pub mod bin_func{
    pub fn create(l : usize) -> Vec<bool> {
        (0..l).map(|_| false).collect()
    }

    pub fn grow(ass : Vec<bool>) -> Option<Vec<bool>> {
        if ass.iter().all(|x| *x) { return None };
        Some(add_one(ass))
    }

    fn add_one(ass : Vec<bool>) -> Vec<bool> {
        let (f, t) : (&bool, &[bool]) = ass.split_first().unwrap();
        match f {
            false => {
                let mut a : Vec<bool> = vec![true];
                a.append(&mut t.to_vec());
                a
            },
            true => {
                let mut a = vec![false];
                a.append(&mut add_one(t.to_vec()));
                a
            }
        }
    }
}
