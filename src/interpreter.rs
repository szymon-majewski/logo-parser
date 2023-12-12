use std::collections::{ HashMap, LinkedList };
use crate::parser::{ self, CodeBlock, ParserSymbol, ProcedureCall, CodeBlockType, CommandType, Command };
use crate::Expression;
use std::f32::consts::PI;

struct Turtle
{
    x: f32,
    y: f32,
    dir_x: f32,
    dir_y: f32,
    lifted: bool,
    label_height: i32
    // Unnecessary?
    // hidden: false
}
impl Turtle
{
    pub fn rotate_right(&mut self, turn_degrees: f32)
    {
        let turn_rad = turn_degrees * PI / 180.0;
        let angle = self.dir_y.atan2(self.dir_x);
        let new_angle = angle + turn_rad;
        self.dir_x = new_angle.cos();
        self.dir_y = new_angle.sin();
    }
}

pub fn execute_logo_program(procedures: HashMap<String, CodeBlock>) -> String
{
    let canvas_width = 1100;
    let canvas_height = 600;
    //let canvas_offset = 50;
    let mut turtles: Vec<Turtle> = vec!
    [
        Turtle{ x: canvas_width as f32 / 2.0, y: canvas_height as f32 / 2.0, dir_x: 0.0, dir_y: -1.0, lifted: false, label_height: 100 },
        Turtle{ x: canvas_width as f32 / 2.0, y: canvas_height as f32 / 2.0, dir_x: 0.0, dir_y: -1.0, lifted: false, label_height: 100 }
    ];
    
    let mut svg = format!("<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\">\n\t<rect width=\"100%\" height=\"100%\" style=\"fill:rgb(255,255,255);stroke-width:10;stroke:rgb(0,0,0)\" />", canvas_width, canvas_height);

    let MAIN_PROCEDURE_NAME = "_".to_string(); // this should be taken from parser
    let main_procedure = ParserSymbol::PROCEDURE_CALL(ProcedureCall::new(MAIN_PROCEDURE_NAME, LinkedList::new()));
    execute_instruction(&main_procedure, &mut svg, &procedures,  &mut turtles, &mut 0, &mut HashMap::new(), &mut false);

    format!("{svg}\n</svg>")
}

fn execute_instruction(instruction: &ParserSymbol, svg: &mut String, procedures: &HashMap<String, CodeBlock>, turtles: &mut Vec<Turtle>, current_turtle: &mut usize, variables: &mut HashMap<String, f32>, stop: &mut bool)
{
    match instruction
    {
        ParserSymbol::PROCEDURE_CALL(procedure_call) =>
        {
            println!("PROCEDURE_CALL {}", &procedure_call.procedure_name);
            //println!("{:?}", procedures);
            let procedure = procedures.get(&procedure_call.procedure_name).unwrap();
            let mut procedure_variables: HashMap<String, f32> = HashMap::new();
            if let CodeBlockType::PROCEDURE(procedure) = &procedure.code_block_type 
            {
                for call_parameter in procedure.call_parameters.iter().zip(procedure_call.parameter_expressions.iter())
                {
                    procedure_variables.insert(call_parameter.0.clone(), call_parameter.1.evaluate(variables));
                }
            }

            for procedure_instruction in procedure.get_instructions()
            {
                execute_instruction(procedure_instruction, svg, procedures, turtles, current_turtle,  &mut procedure_variables, stop);
                if *stop == true { *stop = false; return; }
            }
        }
        ParserSymbol::CODE_BLOCK(code_block) =>
        {
            match &code_block.code_block_type
            {
                CodeBlockType::LOOP(my_loop) =>
                {
                    let repeats = my_loop.repeats.evaluate(variables).round() as i32;
                    println!("LOOP {}", repeats);
                    for i in 0..(repeats)
                    {
                        variables.insert("repcount".to_string(), i as f32);
                        for loop_instruction in code_block.get_instructions().iter()
                        {
                            execute_instruction(loop_instruction, svg, procedures, turtles, current_turtle,  variables, stop);
                        }
                    }
                    variables.remove("repcount");
                }
                CodeBlockType::IF(my_if) =>
                {
                    let condition = my_if.condition.evaluate(variables) != 0.0;
                    println!("IF {}", condition);
                    if condition
                    {
                        for if_instruction in code_block.get_instructions().iter()
                        {
                            execute_instruction(if_instruction, svg, procedures, turtles, current_turtle, variables, stop);
                            if *stop == true { return; }
                        }
                    }
                }
                CodeBlockType::PROCEDURE(_) => {}
            }
        }
        ParserSymbol::COMMAND(command) =>
        {
            match command.command_type
            {
                CommandType::FORWARD =>
                {
                    let distance = command.call_parameter.evaluate(variables);
                    println!("FD {}", distance);
                    let new_x = turtles[*current_turtle].x + (turtles[*current_turtle].dir_x * distance);
                    let new_y = turtles[*current_turtle].y + (turtles[*current_turtle].dir_y * distance);
                    if !turtles[*current_turtle].lifted
                    {
                        svg.push_str(&format!("\n\t<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" style=\"stroke:rgb(0,0,0);stroke-width:1\" />", turtles[*current_turtle].x, turtles[*current_turtle].y, new_x, new_y));
                    }
                    turtles[*current_turtle].x = new_x;
                    turtles[*current_turtle].y = new_y;
                }
                CommandType::BACKWARD =>
                {
                    let distance = command.call_parameter.evaluate(variables);
                    println!("BK {}", distance);
                    let new_x = turtles[*current_turtle].x - (turtles[*current_turtle].dir_x * distance);
                    let new_y = turtles[*current_turtle].y - (turtles[*current_turtle].dir_y * distance);
                    if !turtles[*current_turtle].lifted
                    {
                        svg.push_str(&format!("\n\t<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" style=\"stroke:rgb(0,0,0);stroke-width:1\" />", turtles[*current_turtle].x, turtles[*current_turtle].y, new_x, new_y));
                    }
                    turtles[*current_turtle].x = new_x;
                    turtles[*current_turtle].y = new_y;
                }
                CommandType::TURN_RIGHT =>
                {
                    let turn_degrees = command.call_parameter.evaluate(variables) % 360.0;
                    println!("RT {}", turn_degrees);
                    turtles[*current_turtle].rotate_right(turn_degrees);
                }
                CommandType::TURN_LEFT =>
                {
                    let turn_degrees = command.call_parameter.evaluate(variables) % 360.0;
                    println!("LT {}", turn_degrees);
                    turtles[*current_turtle].rotate_right(360.0-turn_degrees);
                }
                CommandType::PEN_UP =>
                {
                    turtles[*current_turtle].lifted = true;
                }
                CommandType::PEN_DOWN =>
                {
                    turtles[*current_turtle].lifted = false;
                }
                CommandType::STOP =>
                {
                    println!("STOP");
                    *stop = true;
                    return;
                }
                CommandType::SET_LABEL_HEIGHT =>
                {
                    println!("SET_LABEL_HEIGHT");
                    turtles[*current_turtle].label_height = command.call_parameter.evaluate(variables).round() as i32;
                }
                CommandType::LABEL =>
                {
                    println!("LABEL");
                    let text = command.call_parameter.text_literal();
                    let rotation_angle = (turtles[*current_turtle].dir_y.atan2(turtles[*current_turtle].dir_x) * 180.0 / PI).round();
                    svg.push_str(&format!("\n\t<text x=\"{}\" y=\"{}\" fill=\"black\" font-size=\"{}\" font-family=\"Arial\" transform=\"rotate({} {},{})\">{}</text>",
                                                 turtles[*current_turtle].x, turtles[*current_turtle].y, turtles[*current_turtle].label_height, rotation_angle, turtles[*current_turtle].x, turtles[*current_turtle].y, text));
                }
                CommandType::SET_TURTLE =>
                {
                    *current_turtle = command.call_parameter.evaluate(variables).round() as usize;
                }
                
                CommandType::CLEAR_SCREEN | CommandType::HIDE_TURTLE | 
                CommandType::SHOW_TURTLE | CommandType::WINDOW | 
                CommandType::WAIT => {}
                _ => {}
            }
        }
    }
}   