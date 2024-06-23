use std::fs;
use std::collections::HashSet;

fn count_vowels(string: &str) -> u8 {
  let mut vowel_count = 0;
  let vowels: HashSet<char> = HashSet::from(['a', 'e', 'i', 'o', 'u']);

  for character in string.chars() {
    if vowels.contains(&character) {
      vowel_count += 1;
    }
  }

  return vowel_count;
}

fn has_repeating_character(string: &str) -> bool {
  let mut previous_character = char::MAX;
  for character in string.chars() {
    if character == previous_character {
      return true;
    }
    previous_character = character;
  }

  return false;
}

fn includes_forbidden_sequence(string: &str) -> bool {
  let forbidden_sequences = ["ab", "cd", "xy", "pq"];

  for seq in forbidden_sequences {
    if string.contains(seq) {
      return true;
    }
  }

  return false;
}

fn has_repeating_character_with_another_between(string: &str) -> bool {
  let mut previous_characters = Vec::from([char::MAX, char::MAX]);

  for character in string.chars() {
    if character == previous_characters[0] {
      return true;
    }
    previous_characters[0] = previous_characters[1];
    previous_characters[1] = character;
  }

  return false;
}

fn has_repeated_pair_with_no_overlap(string: &str) -> bool {
  #[derive(Copy, Clone)]
  struct PairInstance {
    index: usize,
    first_char: char,
    second_char: char,
  }

  let mut previous_character = char::MAX;
  let mut pairs: Vec<PairInstance> = Vec::new();

  for (index, character) in string.chars().enumerate() {
    if index > 0 {
      let new_pair = PairInstance { index: index, first_char: previous_character, second_char: character };
      
      for pair in &pairs {
        if pair.first_char == new_pair.first_char && pair.second_char == new_pair.second_char && new_pair.index - pair.index > 1 {
          return true;
        }
      }
      
      pairs.push(new_pair);
    }
    previous_character = character;
  }

  return false;
}

fn is_string_nice_old_way(string: &str) -> bool {
  return count_vowels(string) >= 3 && has_repeating_character(string) && !includes_forbidden_sequence(string);
}

fn is_string_nice_new_way(string: &str) -> bool {
  return has_repeating_character_with_another_between(string) && has_repeated_pair_with_no_overlap(string);
}

fn main() {
  let file_path = "inputs/05_input.txt";
  let file_contents = fs::read_to_string(file_path).expect("Should read file");

  let mut nice_string_count_old_way = 0;
  let mut nice_string_count_new_way = 0;

  for string in file_contents.split("\n") {
    match is_string_nice_old_way(string) {
      true => nice_string_count_old_way += 1,
      false => ()
    }

    match is_string_nice_new_way(string) {
      true => nice_string_count_new_way += 1,
      false => ()
    }
  }

  println!("Number of nice strings according to rules from part one: {}", nice_string_count_old_way);
  println!("Number of nice strings according to rules from part two: {}", nice_string_count_new_way);
}