use std::fs;
use std::cmp;

struct Box {
  width: u16,
  length: u16,
  height: u16,
}

fn smallest_side_area(b: &Box) -> u16 {
  return cmp::min(b.width * b.length, cmp::min(b.length * b.height, b.height * b.width));
}

fn smallest_side_perimeter(b: &Box) -> u16 {
  return cmp::min(2 * b.width + 2 * b.length, cmp::min(2 * b.length + 2 * b.height, 2 * b.height + 2 * b.width));
}

fn box_area(b: &Box) -> u16 {
  return 2 * b.width * b.length + 2 * b.length * b.height + 2 * b.height * b.width;
}

fn box_volume(b: &Box) -> u16 {
  return b.width * b.length * b.height;
}

fn main() {
  let file_path = "inputs/02_input.txt";
  let file_contents = fs::read_to_string(file_path).expect("Should read file");

  let mut total_size: u32 = 0;
  let mut total_ribbon: u32 = 0;

  for box_string in file_contents.split("\n") {
    let dimensions = box_string.split("x").collect::<Vec<&str>>();
    if dimensions.len() == 3 {
      let box_obj = Box {
        width: dimensions[0].parse::<u16>().unwrap(),
        length: dimensions[1].parse::<u16>().unwrap(),
        height: dimensions[2].parse::<u16>().unwrap(),
      };

      let paper_for_wrapping = box_area(&box_obj) + smallest_side_area(&box_obj);
      total_size += u32::from(paper_for_wrapping);

      let ribbon_for_wrapping = box_volume(&box_obj) + smallest_side_perimeter(&box_obj);
      total_ribbon += u32::from(ribbon_for_wrapping);
    }
  }

  println!("Total size of paper for wrapping: {} sq ft", total_size);
  println!("Total length of ribbon: {} ft", total_ribbon);
}