use std::fs;
use std::collections::HashMap;

struct Wire {
  active: bool,
  value: u16,
}

#[derive(PartialEq)]
enum GateType {
  PassThrough,
  Not,
  LeftShift,
  RightShift,
  And,
  Or,
}

fn gate_type_to_string(gate_type: GateType) -> String {
  return String::from(match gate_type {
    GateType::PassThrough => "THRU",
    GateType::Not => "NOT",
    GateType::LeftShift => "LSHIFT",
    GateType::RightShift => "RSHIFT",
    GateType::And => "AND",
    GateType::Or => "OR",
  });
}

struct Gate {
  in_wire_1: Option<String>,
  in_wire_2: Option<String>,
  in_value: Option<u16>,
  gate_type: GateType,
  shift_value: Option<u8>,
  out_wire: String,
}

fn add_wire(wires: &mut HashMap<String, Wire>, wire: &str) {
  wires.insert(String::from(wire), Wire { active: false, value: 0 } );
}

fn parse_gate(connection_def: Vec<&str>) -> Option<Gate> {
  let output_wire = String::from(connection_def[connection_def.len() - 1]);
  match connection_def.len() {
    3 => {
      let mut input_value: u16 = 0;
      let mut input_wire_name: Option<String> = None;
      match connection_def[0].parse::<u16>() {
        Ok(n) => input_value = n,
        Err(e) => input_wire_name = Some(String::from(connection_def[0])),
      }

      return Some(Gate {
        in_wire_1: input_wire_name,
        in_wire_2: None,
        in_value: match input_value {
          0 => None,
          _ => Some(input_value),
        },
        gate_type: GateType::PassThrough,
        shift_value: None,
        out_wire: output_wire,
      });
    }
    4 => {
      let mut input_value: u16 = 0;
      let mut input_wire_name: Option<String> = None;
      match connection_def[1].parse::<u16>() {
        Ok(n) => input_value = n,
        Err(e) => (),
      }
      if input_value == 0 {
        input_wire_name = Some(String::from(connection_def[1]));
      }

      return Some(Gate {
        in_wire_1: input_wire_name,
        in_wire_2: None,
        in_value: match input_value {
          0 => None,
          _ => Some(input_value),
        },
        gate_type: GateType::Not,
        shift_value: None,
        out_wire: output_wire,
      });
    }
    5 => {
      match connection_def[1] {
        "AND" => {
          return Some(Gate {
            in_wire_1: Some(String::from(connection_def[0])),
            in_wire_2: Some(String::from(connection_def[2])),
            in_value: None,
            gate_type: GateType::And,
            shift_value: None,
            out_wire: output_wire,
          });
        }
        "OR" => {
          return Some(Gate {
            in_wire_1: Some(String::from(connection_def[0])),
            in_wire_2: Some(String::from(connection_def[2])),
            in_value: None,
            gate_type: GateType::Or,
            shift_value: None,
            out_wire: output_wire,
          });
        }
        "LSHIFT" => {
          match connection_def[2].parse::<u8>() {
            Ok(n) => {
              return Some(Gate {
                in_wire_1: Some(String::from(connection_def[0])),
                in_wire_2: None,
                in_value: None,
                gate_type: GateType::LeftShift,
                shift_value: None,
                out_wire: output_wire,
              });
            }
            Err(e) => (),
          }
        }
        "RSHIFT" => {
          match connection_def[2].parse::<u8>() {
            Ok(n) => {
              return Some(Gate {
                in_wire_1: Some(String::from(connection_def[0])),
                in_wire_2: None,
                in_value: None,
                gate_type: GateType::RightShift,
                shift_value: None,
                out_wire: output_wire,
              });
            }
            Err(e) => (),
          }
        }
        _ => ()
      }
    }
    _ => ()
  }

  return None;
}

fn parse_connection_definitions(wires: &mut HashMap<String, Wire>, gates: &mut Vec<Gate>, strings: Vec<&str>) {
  for string in strings {
    let connection_def: Vec<&str> = string.split(" ").collect();
    add_wire(wires, parse_wire(connection_def.clone()));
    match parse_gate(connection_def) {
      Some(gate) => gates.push(gate),
      _ => ()
    }
  }
}

fn parse_wire(connection_def: Vec<&str>) -> &str {
  return connection_def[connection_def.len() - 1];
}

fn print_gate_info(gate: Gate) {
  println!("IN1: {}, IN2: {}, INV: {}, TYP: {}, SFT: {}, OUT: {}",
    gate.in_wire_1.unwrap_or(String::new()), gate.in_wire_2.unwrap_or(String::new()),
    gate.in_value.unwrap_or(0), gate_type_to_string(gate.gate_type),
    gate.shift_value.unwrap_or(0), gate.out_wire);
}

fn initialize_sim(wires: &mut HashMap<String, Wire>, gates: Vec<Gate>) {
  for gate in gates {
    if gate.gate_type == GateType::PassThrough && gate.in_wire_1 == None {
      let wire = wires.get(&gate.out_wire).unwrap();

    }
  }
}

fn main() {
  let file_path = "inputs/07_input.txt";
  let file_contents = fs::read_to_string(file_path).expect("Should read file");

  let mut wires: HashMap<String, Wire> = HashMap::new();
  let mut gates: Vec<Gate> = Vec::new();

  parse_connection_definitions(&mut wires, &mut gates, file_contents.split("\n").collect());

  initialize_sim(&mut wires, gates);
}