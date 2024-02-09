use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
#[derive(Clone)]
struct FlipFlop{
    name: String,
    device_type: char,
    outputs: Vec<String>,
    memory: bool
}

#[derive(Debug)]
#[derive(Clone)]
struct Conjunction{
    name: String,
    device_type: char,
    outputs: Vec<String>,
    memory: HashMap<String, bool>
}

impl FlipFlop{
    fn new(name: &String, _type: &char, outputs: &Vec<String>) -> Self{
        FlipFlop{
            name: name.clone(),
            device_type: _type.clone(),
            outputs: outputs.clone(),
            memory: false
        }
    }
}

impl Conjunction{
    fn new(name: &String, _type: &char, outputs: &Vec<String>) -> Self{
        Conjunction{
            name: name.clone(),
            device_type: _type.clone(),
            outputs: outputs.clone(),
            memory: HashMap::new()
        }
    }
}

#[derive(Debug)]
#[derive(Clone)]
enum ModuleTypes{
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
}

fn main() {
    let mut modules: HashMap<String, ModuleTypes> = HashMap::new();
    let mut broadcaster_targets: Vec<String> = vec![];
    let file_path = "src/puzzle_data.txt";
    for line in fs::read_to_string(file_path).unwrap().lines(){
        let input = line.split("->").nth(0).unwrap().to_string();
        let out = line.split("->").nth(1)
                                .unwrap()
                                .split(",")
                                .map(|device| device.trim().to_string())
                                .collect::<Vec<String>>();

        if input.trim().to_string() == "broadcaster"{
            broadcaster_targets = out;
        } else {
            if input.chars().nth(0).unwrap() == '%'{
                modules.insert(input.strip_prefix("%").unwrap().trim().to_string(), ModuleTypes::FlipFlop(FlipFlop::new(&(input.strip_prefix("%").unwrap().trim().to_string()), 
                                                            &'%',
                                                            &out)));
            } else if input.chars().nth(0).unwrap() == '&'{
                modules.insert(input.strip_prefix("&").unwrap().trim().to_string(), ModuleTypes::Conjunction(Conjunction::new(&(input.strip_prefix("&").unwrap().trim().to_string()), 
                                                                &'&',
                                                                &out)));
            }
            
        }
    }

    let mut mut_modules = modules.clone();
    for (name, module) in &modules{
        match module{
            ModuleTypes::FlipFlop(fp) => {
                for output in &fp.outputs{
                    if modules.contains_key(output){
                        if let ModuleTypes::Conjunction(cj) = mut_modules.get_mut(output).unwrap(){
                            cj.memory.entry(name.to_string()).or_insert(false);
                        }
                    }
                }
            },
            ModuleTypes::Conjunction(cj_in) => {
                for output in &cj_in.outputs{
                    if modules.contains_key(output){
                        if let ModuleTypes::Conjunction(cj) = mut_modules.get_mut(output).unwrap(){
                            cj.memory.entry(name.to_string()).or_insert(false);
                        }
                    }
                }
            }
        }
    }
    println!("{:?}",mut_modules);
    let mut exec_queue: Vec<(String, String, bool)> = vec![];
    let mut low = 0;
    let mut high = 0;
    for _ in 0..1000{
        low += 1;
        for device in &broadcaster_targets{
            exec_queue.push(("broadcaster".to_string(), device.to_string(), false));
        }
        while exec_queue.len() > 0{
            let (origin, target, pulse);
            (origin, target, pulse) = exec_queue.remove(0);
            
            if pulse == false{
                low += 1;
            } else {
                high += 1;
            }

            if !mut_modules.contains_key(&target){
                continue;
            }

            match mut_modules.get_mut(&target).unwrap() {
                ModuleTypes::FlipFlop(ff) => {
                    if pulse == false{
                        if ff.memory == false {
                            ff.memory = true;
                        } else {
                            ff.memory = false;
                        }

                        let mut outgoing = false;

                        if ff.memory {
                            outgoing = true;
                        } else {
                            outgoing = false;
                        }

                        for device in &ff.outputs{
                            exec_queue.push((ff.name.to_string(), device.to_string(), outgoing))
                        }
 
                    }
                },
                ModuleTypes::Conjunction(cj) => {
                    cj.memory.entry(origin).and_modify(|device_status| *device_status = pulse);
                    let mut outgoing = true;
                    if cj.memory.values().all(|device_status| *device_status){
                        outgoing = false
                    }
                    for device in &cj.outputs{
                        exec_queue.push((cj.name.to_string(), device.to_string(), outgoing))
                    }
                }
            }
        }
    }
    println!("{} {} {}", low, high, low * high)
}