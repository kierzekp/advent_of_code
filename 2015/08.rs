use std::fs;

fn get_character_length(string: &str) -> usize {
  return string.len();
}

fn get_unescaped_length(string: &str) -> usize {
  return unescape_string(string).len();
}

fn unescape_string(string: &str) -> String {
  let mut output_string = String::from(string);
  output_string = String::from(&output_string[1..output_string.len()-1]);
  return find_and_replace_escaped_characters(&output_string);
}

fn reencode_string(string: &str) -> String {
  let mut reencoded_string = String::from(string);
  reencoded_string = reencoded_string.replace("\\", "\\\\");
  reencoded_string = reencoded_string.replace("\"", "\\\"");
  return String::from("\"") + &reencoded_string + &String::from("\"");
}

fn get_reencoded_length(string: &str) -> usize {
  return reencode_string(string).len();
}

// I'm not writing an unescaping library; I do not care about the factual representation of what the escaped characters stand for
fn find_and_replace_escaped_characters(string: &str) -> String {
  let escaped_string = String::from(string);
  let escaped_string_bytes = escaped_string.as_bytes();
  let dummy_character = "h";
  match escaped_string.find("\\") {
    Some(index) => {
      let next_character = escaped_string_bytes[index+1] as char;
      if next_character != 'x' {
        let pattern: String = vec!['\\', next_character].into_iter().collect();
        return find_and_replace_escaped_characters(&escaped_string.replace(&pattern, dummy_character));
      } else {
        if escaped_string.len() - index <= 3 {
          return escaped_string;
        }
        let hexadecimal_code = i16::from_str_radix(&escaped_string[index+2..index+4], 16);
        match hexadecimal_code {
          Ok(_) => {
            let pattern: String = vec!['\\', 'x', escaped_string_bytes[index+2] as char, escaped_string_bytes[index+3] as char].into_iter().collect();
            return find_and_replace_escaped_characters(&escaped_string.replace(&pattern, dummy_character));
          }
          _ => panic!("Impossible hex pattern"),
        }
      }
    }
    None => return escaped_string,
  }
  
}

fn main() {
  let file_path = "inputs/08_input.txt";
  let file_contents = fs::read_to_string(file_path).expect("Should read file");

  let mut char_count = 0;
  let mut encoding_count = 0;
  let mut reencoded_count = 0;
  
  for string in file_contents.split("\n") {
    char_count += get_character_length(string);
    encoding_count += get_unescaped_length(string);
    reencoded_count += get_reencoded_length(string);
  }

  println!("The difference between the number of characters of code and number of characters in encoding is {} - {} = {}", char_count, encoding_count, char_count - encoding_count);
  println!("The difference between the number of characters of code in reencoded strings and number of characters in original strings is {} - {} = {}", reencoded_count, char_count, reencoded_count - char_count);
}