use std::fs;

fn main() {
  let file_path = "inputs/01_input.txt";

  let file_contents = fs::read_to_string(file_path).expect("Should read file");

  let mut effective_floor = 0;
  let mut already_went_underground = false;

  for (i, c) in file_contents.chars().enumerate() {
    match c {
      '(' => effective_floor += 1,
      ')' => effective_floor -= 1,
      _ => (),
    }

    if effective_floor == -1 && !already_went_underground {
      println!("Santa went underground with the char at position: {}", i + 1); // i + 1 because we need position starting from 1
      already_went_underground = true;
    }
  }

  println!("Final floor: {}", effective_floor);

}
