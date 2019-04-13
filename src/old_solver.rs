use crate::structures::*;

// fn get_first_formula(forms : &CNF){
//     let mut all_together : Formula = forms.0.iter()
//         .flat_map(|array| array.iter())
//         .cloned()
//         .collect();
//     all_together.sort();
//     println!("{:?}", all_together);
// }


fn main() {
    let to_solve : CNF = (vec![vec![-2, -1], vec![2, 1], vec![2, -1], vec![-2, 1]], 2);
    //println!("{:?}", crate::solver::brute_force::solve(to_solve));
}
