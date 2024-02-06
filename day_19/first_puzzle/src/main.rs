use std::fs;

#[derive(Debug)]
#[derive(Clone)]
struct Part{
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

#[derive(Debug)]
struct Condition{
    part: char,
    comparison: char,
    number: usize,
    next_workflow: String
}

#[derive(Debug)]
struct Workflow{
    name: String,
    conditions: Vec<Condition>,
    default: String
}

fn parse_parts(parts: &Vec<String>) -> Vec<Part>{
    let mut parts_processed: Vec<Part> = vec![];
    for part_str in parts{
        let mut stripped_str = part_str.strip_prefix("{").unwrap().strip_suffix("}").unwrap();
        let parsed_parts = stripped_str.split(",")
                            .map(|part_str| part_str.split("=").collect::<Vec<&str>>())
                            .map(|part_str_vec| part_str_vec[1].parse().unwrap())
                            .collect::<Vec<usize>>();
        parts_processed.push(Part{
            x: parsed_parts[0],
            m: parsed_parts[1],
            a: parsed_parts[2],
            s: parsed_parts[3]
        })
    }
    parts_processed
}

fn parse_conditions(workflows: &Vec<String>) -> Vec<Workflow>{
    let mut workflows_processed: Vec<Workflow> = vec![];
    for workflow_str in workflows{
        let workflow_name = workflow_str.split("{").nth(0).unwrap();
        let workflow_conditions = workflow_str.split("{").nth(1).unwrap();
        let default = workflow_conditions.strip_suffix("}").unwrap().split(",").last().unwrap();
        let contraints = workflow_conditions.strip_suffix("}")
                                            .unwrap()
                                            .split(",")
                                            .filter(|condition_str| condition_str.contains(":"))
                                            .map(|condition_str| Condition{
                                                part: condition_str[0..=0].chars().nth(0).unwrap().clone(),
                                                comparison: condition_str[1..=1].chars().nth(0).unwrap().clone(),
                                                number: condition_str[2..].split(":").nth(0).unwrap().to_string().parse().unwrap(),
                                                next_workflow: condition_str[2..].split(":").nth(1).unwrap().to_string()
                                                })
                                            .collect::<Vec<Condition>>();
        workflows_processed.push(Workflow{
            name: workflow_name.to_string(),
            conditions: contraints,
            default: default.to_string()
        });
    }
    workflows_processed
}

fn check_condition(part: &Part, workflow: &Workflow) -> String{
    let mut workflow_validation = false;
    for condition in &workflow.conditions{
        if condition.part == 'x'{
            if condition.comparison == '>'{
                workflow_validation = part.x > condition.number;
            } else if condition.comparison == '<'{
                workflow_validation = part.x < condition.number;
            } else {
                workflow_validation = part.x == condition.number;
            }
        } else if condition.part == 'm'{
            if condition.comparison == '>'{
                workflow_validation = part.m > condition.number;
            } else if condition.comparison == '<'{
                workflow_validation = part.m < condition.number;
            } else {
                workflow_validation = part.m == condition.number;
            }
        } else if condition.part == 'a'{
            if condition.comparison == '>'{
                workflow_validation = part.a > condition.number;
            } else if condition.comparison == '<'{
                workflow_validation = part.a < condition.number;
            } else {
                workflow_validation = part.a == condition.number;
            }
        } else {
            if condition.comparison == '>'{
                workflow_validation = part.s > condition.number;
            } else if condition.comparison == '<'{
                workflow_validation = part.s < condition.number;
            } else {
                workflow_validation = part.s == condition.number;
            }
        }

        if workflow_validation{
            return condition.next_workflow.to_string()
        }
    }

    workflow.default.clone()
}

fn main() {
    let mut workflows: Vec<String> = vec![];
    let mut parts: Vec<String> = vec![];
    let file_path = "src/puzzle_data.txt";
    for line in fs::read_to_string(file_path).unwrap().lines(){
        if line.len() != 0{
            if line.chars().next().unwrap() == '{'{
                parts.push(line.to_string())
            } else {
                workflows.push(line.to_string());
            }
        }
        
        
    }
    let parts_processed = parse_parts(&parts);
    let workflows_processed = parse_conditions(&workflows);
    let mut accepted_parts: Vec<Part> = vec![];
    println!("Parts: {:?}\n", parts_processed);
    println!("Workflows: {:?}\n", workflows_processed);
    for part in &parts_processed{
        println!("Starting with parts: {:?}", part);
        let mut workflow_struct = workflows_processed.iter().filter(|workflow| workflow.name == "in").nth(0).unwrap();
        let mut next_workflow = check_condition(part, &workflow_struct);
        while !["A".to_string(),"R".to_string()].contains(&next_workflow){
            workflow_struct = workflows_processed.iter().filter(|workflow| workflow.name == next_workflow).nth(0).unwrap();
            next_workflow = check_condition(part, &workflow_struct);
            println!("{:?}", next_workflow);
        }
        println!("{:?}", next_workflow);
        if next_workflow == "A".to_string(){
            accepted_parts.push(part.clone());
        }
    }

    println!("Total ratings: {}", accepted_parts.iter().map(|part| part.x + part.m + part.a + part.s).sum::<usize>())
}
