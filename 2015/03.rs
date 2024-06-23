use std::fs;
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Position {
  x: i16,
  y: i16,
}

fn move_position(pos: &Position, dir: char) -> Option<Position> {
  match dir {
    '<' => return Some(Position { x: pos.x - 1, y: pos.y }),
    '>' => return Some(Position { x: pos.x + 1, y: pos.y }),
    '^' => return Some(Position { x: pos.x, y: pos.y + 1 }),
    'v' => return Some(Position { x: pos.x, y: pos.y - 1 }),
    _ => return None,
  }
}

fn main() {
  let file_path = "inputs/03_input.txt";

  let file_contents = fs::read_to_string(file_path).expect("Should read file");

  let mut current_position = Position { x: 0, y: 0, };
  let mut visited_positions: HashSet<Position> = HashSet::new();

  visited_positions.insert(current_position);

  for c in file_contents.chars() {
    match move_position(&current_position, c) {
      Some(pos) => {
        visited_positions.insert(pos);
        current_position = pos;
      }
      None => ()
    }
  }

  println!("Visited positions for part one: {}", visited_positions.len());

  // resetting for part two
  visited_positions.drain();
  current_position = Position { x: 0, y: 0, };
  let mut robosanta_current_position = Position { x: 0, y: 0, };

  visited_positions.insert(current_position);

  for (i, c) in file_contents.chars().enumerate() {
    // santa moves on even, robosanta moves on odd
    match i % 2 {
      0 => {
        match move_position(&current_position, c) {
          Some(pos) => {
            visited_positions.insert(pos);
            current_position = pos;
          }
          None => ()
        }
      }
      1 => {
        match move_position(&robosanta_current_position, c) {
          Some(pos) => {
            visited_positions.insert(pos);
            robosanta_current_position = pos;
          }
          None => ()
        }
      }
      _ => ()
    }
  }

  println!("Visited positions for part two: {}", visited_positions.len());
}