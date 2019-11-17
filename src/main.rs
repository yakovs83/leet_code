use leet_code::palindrome_number::is_palindrome;
fn main() {
  let input: Vec<i32> = vec![1, 0, -1, 22, 121, 9999, 523, -161];
  let output: Vec<bool> = input.iter().map(|x| is_palindrome(*x)).collect();
  input.iter().zip(output.iter()).for_each(|(i, o)| {
    println!(
      "{} is {}",
      i,
      if *o {
        "a palindrome"
      } else {
        "not a palindrome"
      }
    );
  });
}
