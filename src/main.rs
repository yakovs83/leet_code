use leet_code::zigzag_conversion;

fn main() {
  let res = zigzag_conversion::convert("PAYPALISHIRING".to_owned(), 3);
  println!("The answer is {:?}", res);
}
