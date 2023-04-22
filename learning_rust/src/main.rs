/*
   Using External Crates
*/

use array_tool::vec::*;

fn main() {
    let vec_1 = vec![1, 1, 3, 5, 6, 7];
    let vec_2 = vec![1, 2, 3];

    let intersection = vec_1.intersect(vec_2.clone());
    println!("The intersection is {:?}", intersection);

    let union_set = vec_1.union(vec_2.clone());
    println!("The union is {:?}", union_set);

    println!("Vec 1 three times displayed = {:?}", vec_2.times(3));
}
