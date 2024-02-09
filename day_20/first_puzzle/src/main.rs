use std::fs;
use std::collections::HashMap;

// true: high; false: low
#[derive(Debug)]
struct FlipFlop{
    name: String,
    state: bool
}

impl FlipFlop{
    fn update_state_by_current(&mut self, current: bool) -> bool{
        if current{
            return false // do not process the flipflop
        } else {
            self.state = !self.state; // set the opposite: if off then send true (high) otherwise send false (low)
            return true // do process the flipflop
        }
    }
}
#[derive(Debug)]
#[derive(Clone)]
struct Conjunction{
    name: String,
    connections: HashMap<String, bool>
}

impl Conjunction{
    fn get_output_current(&self) -> bool{
        if self.connections.values().all(|device_state| *device_state){
            return false // return false (low) if they are all true (high)
        } else {
            return true // return true (high) if they are not all true
        }
    }

    fn update_conn_state(&mut self, flipflop_name: &String, input_current: &bool){
        self.connections.entry(flipflop_name.to_string()).and_modify(|ff| *ff = *input_current); // modify connections state table
    }

}
#[derive(Debug)]
struct Broadcaster{
    name: String, 
    current_to_broadcast: bool
}
#[derive(Debug)]
struct Instruction{
    input: String,
    output: Vec<String>
}

#[derive(Debug)]
enum DeviceTypes{
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    Broadcaster(Broadcaster),
}

fn main() {
    let mut lines: Vec<Instruction> = vec![];
    let mut devices: HashMap<String, DeviceTypes> = HashMap::new();
    let file_path = "src/puzzle_data.txt";
    for line in fs::read_to_string(file_path).unwrap().lines(){
        let input = line.split("->").nth(0).unwrap().to_string();
        let out = line.split("->").nth(1)
                                .unwrap()
                                .split(",")
                                .map(|device| device.trim().to_string())
                                .collect::<Vec<String>>();
        lines.push(Instruction{
            input: input.trim().to_string(),
            output: out.clone()
        });

        

        if input.chars().nth(0).unwrap() == '&'{
            devices.entry(input.trim().to_string()).or_insert(
                DeviceTypes::Conjunction(Conjunction{
                    name: input.trim().to_string(),
                    connections: HashMap::new()
                }));
        } else if input.chars().nth(0).unwrap() == '%'{
            devices.entry(input.trim().to_string()).or_insert(
                DeviceTypes::FlipFlop(FlipFlop{
                    name: input.trim().to_string(),
                    state: false
                })
            );
        } else {
            devices.entry(input.trim().to_string()).or_insert(
                DeviceTypes::Broadcaster(Broadcaster{
                name: input.trim().to_string(),
                current_to_broadcast: false
                })
            );
        }
    }

    for instruction in &lines{
        for device_str in instruction.output.iter(){
            if devices.keys().any(|key| key.contains(device_str)){
                let device_name = devices.keys().filter(|key| key.contains(device_str)).nth(0).unwrap();
                if device_name.chars().nth(0).unwrap() == '&'{
                    
                    let mut conjunction: &mut Conjunction = match devices.get_mut(&device_name.to_string()).unwrap()
                    {
                        DeviceTypes::Conjunction(cj) => cj,
                        _ => unreachable!()
                    };
                    conjunction.connections.entry(instruction.input.to_string()).or_insert(false);
                }
            }
        }
    }

    println!("{:?}",lines);
    println!("{:?}",devices);
    let mut exec_queue: Vec<String> = vec![];
    let mut low = 0;
    let mut high = 0;
    for _ in 0..1000{
        low += 1;
        w
    }

}