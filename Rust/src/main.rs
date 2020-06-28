use leet_code::reverse_k_group::*;

fn main() {
    let a = vec![1, 2, 3, 4, 5];
    let l = list_from_vec(a);
    println!("The input list is {:?}", l);
    println!(
        "Reversal result - {:?}",
        list_to_vec(&reverse_k_group(l, 4))
    );
}
