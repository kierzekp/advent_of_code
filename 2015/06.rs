use std::fs;

/* this section is relevant for both parts of the task */

fn main() {
  let file_path = "inputs/06_input.txt";
  let file_contents = fs::read_to_string(file_path).expect("Should read file");

  part_one(&file_contents);
  part_two(&file_contents);
}

enum ControlType {
  On,
  Off,
  Toggle,
}

struct Control {
  control_type: ControlType,
  x_start: usize,
  x_end: usize,
  y_start: usize,
  y_end: usize,
}

fn parse_control_string(control_string_parts: &[&str]) -> Option<Control> {
  match control_string_parts.len() {
    4 => {
      if control_string_parts[0] == "toggle" {
        let first_pair: Vec<&str> = control_string_parts[1].split(",").collect();
        let second_pair: Vec<&str> = control_string_parts[3].split(",").collect();

        return Some(Control {
          control_type: ControlType::Toggle,
          x_start: first_pair[0].parse().unwrap(),
          x_end: second_pair[0].parse().unwrap(),
          y_start: first_pair[1].parse().unwrap(),
          y_end: second_pair[1].parse().unwrap(),
        });
      }
    }
    5 => {
      let first_pair: Vec<&str> = control_string_parts[2].split(",").collect();
      let second_pair: Vec<&str> = control_string_parts[4].split(",").collect();

      if control_string_parts[1] == "on" {
        return Some(Control {
          control_type: ControlType::On,
          x_start: first_pair[0].parse().unwrap(),
          x_end: second_pair[0].parse().unwrap(),
          y_start: first_pair[1].parse().unwrap(),
          y_end: second_pair[1].parse().unwrap(),
        });
      } else if control_string_parts[1] == "off" {
        return Some(Control {
          control_type: ControlType::Off,
          x_start: first_pair[0].parse().unwrap(),
          x_end: second_pair[0].parse().unwrap(),
          y_start: first_pair[1].parse().unwrap(),
          y_end: second_pair[1].parse().unwrap(),
        });
      }
    }
    _ => ()
  }

  return None;
}

/* this section is relevant for part one */

// in this part we treat each pixel in the lights array as a boolean
fn part_one(data: &String) {
  // init lights array
  let mut lights_array: Vec<Vec<bool>> = Vec::with_capacity(1000);
  for n in 0..1000 {
    let mut horizontal_line : Vec<bool> = Vec::with_capacity(1000);
    for i in 0..1000 {
      horizontal_line.push(false);
    }
    lights_array.push(horizontal_line);
  }
  
  for control_string in data.split("\n") {
    let mut parts: Vec<&str> = control_string.split(" ").collect();
    let control = parse_control_string(&parts);

    match control {
      Some(c) => {
        activate_lights_part_one(&mut lights_array, &c);
      }
      None => (),
    }
  }
  println!("Number of lights turned on in part one: {}", count_lights_turned_on(&lights_array));
}

fn activate_lights_part_one(lights_array: &mut Vec<Vec<bool>>, control: &Control) {
  match control.control_type {
    ControlType::On => {
      for y in control.y_start..control.y_end+1 {
        for x in control.x_start..control.x_end+1 {
          lights_array[y][x] = true;
        }
      }
    }
    ControlType::Off => {
      for y in control.y_start..control.y_end+1 {
        for x in control.x_start..control.x_end+1 {
          lights_array[y][x] = false;
        }
      }
    }
    ControlType::Toggle => {
      for y in control.y_start..control.y_end+1 {
        for x in control.x_start..control.x_end+1 {
          if lights_array[y][x] {
            lights_array[y][x] = false;
          } else {
            lights_array[y][x] = true;
          }
        }
      }
    }
  }
}

fn count_lights_turned_on(lights_array: &Vec<Vec<bool>>) -> u32 {
  let mut count: u32 = 0;

  for y in 0..lights_array.len() {
    for x in 0..lights_array[y].len() {
      if lights_array[y][x] {
        count += 1;
      }
    }
  }

  return count;
}

/* this section is relevant for part two */

// in this part the lights array includes pixels of variable unsigned brightness
fn part_two(data: &String) {
  // init lights array
  let mut lights_array: Vec<Vec<u16>> = Vec::with_capacity(1000);
  for n in 0..1000 {
    let mut horizontal_line : Vec<u16> = Vec::with_capacity(1000);
    for i in 0..1000 {
      horizontal_line.push(0);
    }
    lights_array.push(horizontal_line);
  }

  for control_string in data.split("\n") {
    let mut parts: Vec<&str> = control_string.split(" ").collect();
    let control = parse_control_string(&parts);

    match control {
      Some(c) => {
        activate_lights_part_two(&mut lights_array, &c);
      }
      None => (),
    }
  }
  println!("Total brightness in part two: {}", count_total_brightness(&lights_array));
}

fn activate_lights_part_two(lights_array: &mut Vec<Vec<u16>>, control: &Control) {
  match control.control_type {
    ControlType::On => {
      for y in control.y_start..control.y_end+1 {
        for x in control.x_start..control.x_end+1 {
          lights_array[y][x] += 1;
        }
      }
    }
    ControlType::Off => {
      for y in control.y_start..control.y_end+1 {
        for x in control.x_start..control.x_end+1 {
          if lights_array[y][x] > 0 {
            lights_array[y][x] -= 1;
          }
        }
      }
    }
    ControlType::Toggle => {
      for y in control.y_start..control.y_end+1 {
        for x in control.x_start..control.x_end+1 {
          lights_array[y][x] += 2;
        }
      }
    }
  }
}

fn count_total_brightness(lights_array: &Vec<Vec<u16>>) -> u64 {
  let mut count: u64 = 0;

  for y in 0..lights_array.len() {
    for x in 0..lights_array[y].len() {
      count += u64::from(lights_array[y][x]);
    }
  }

  return count;
}