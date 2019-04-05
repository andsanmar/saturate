use crate::structures::*;

pub fn create(l : usize) -> Assignation {
    (0..l).map(|_| false).collect()
}

pub fn grow(ass : Assignation) -> Option<Assignation> {
    if ass.iter().all(|x| *x) { return None };
    Some(add_one(ass))
}

fn add_one(ass : Assignation) -> Assignation {
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
