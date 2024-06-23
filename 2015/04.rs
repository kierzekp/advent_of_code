// Since the implementation of MD5 hashing algorithm is not my own and is taken directly from Rosetta Stone,
// the function is not included in this repository due to licensing reasons.
// Source: https://rosettacode.org/wiki/MD5/Implementation#Rust
mod a04_rosetta_md5;

use std::fs;

fn get_full_string(key: &str, value: u32) -> String {
  let input_string = key.to_owned() + &value.to_string();
  return input_string;
}

fn main() {
  let file_path = "inputs/04_input.txt";
  let secret_key = fs::read_to_string(file_path).expect("Should read file");
  let mut input = 1;

  let mut five_zeroes_answer = 0;

  let answer = loop {
    let output = get_full_string(&secret_key, input);

    let mut msg = vec![0u8; 0];
    msg.extend(output.as_bytes());
    let (A, B, C, D) = a04_rosetta_md5::md5(msg);
    let md5_hash_string = format!("{:08x}{:08x}{:08x}{:08x}", A, B, C, D);

    if md5_hash_string.starts_with("00000") && five_zeroes_answer == 0 {
      five_zeroes_answer = input;
    }

    if md5_hash_string.starts_with("000000") {
      break input;
    }

    input += 1;
  };

  println!("The answer for leading five zeroes is {}", five_zeroes_answer);
  println!("The answer for leading six zeroes is {}", answer);
}