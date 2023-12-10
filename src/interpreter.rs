use std::collections::{ HashMap, LinkedList };
use crate::parser::{ self, CodeBlock, ParserSymbol, ProcedureCall, CodeBlockType };
use crate::Expression;

struct Turtle
{
    x: i32,
    y: i32,
    dir: u16, // 0-359, 0 is north, 90 is east...
    lifted: bool
    // Unnecessary?
    // hidden: false
}

pub fn execute_logo_program(procedures: HashMap<String, CodeBlock>) -> String
{
    let canvas_width = 1000;
    let canvas_height = 500;
    //let canvas_offset = 50;
    let turtle = Turtle{ x: canvas_width / 2, y: canvas_height / 2, dir: 0, lifted: false };
    let mut svg = format!("<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\">\n\t<rect width=\"100%\" height=\"100%\" style=\"fill:rgb(255,255,255);stroke-width:3;stroke:rgb(0,0,0)\" />", canvas_width, canvas_height);

    let MAIN_PROCEDURE_NAME = "_".to_string(); // this should be taken from parser
    let main_procedure_call = ProcedureCall::new(MAIN_PROCEDURE_NAME, LinkedList::new());
    call_procedure(&main_procedure_call, &mut svg, &procedures);
    
    svg = format!("{svg}\n\t<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" style=\"stroke:rgb(0,0,0);stroke-width:1\" />", turtle.x, turtle.y, turtle.x + 100, turtle.y + 200);

    format!("{svg}\n</svg>")
}

fn call_procedure(procedure_call: &ProcedureCall, svg: &mut String, procedures: &HashMap<String, CodeBlock>)
{
    // INSTEAD OF CALL_PROCEDURE DO SOMETHING LIKE EXECUTE INSTRUCTION BECAUSE THIS IS BAD
    let procedure = procedures.get(&procedure_call.procedure_name).unwrap();
    for instruction in procedure.get_instructions()
    {
        match instruction
        {
            ParserSymbol::PROCEDURE_CALL(procedure_call) =>
            {
                call_procedure(procedure_call, svg, procedures)
            }
            ParserSymbol::CODE_BLOCK(code_block) =>
            {
                match &code_block.code_block_type
                {
                    CodeBlockType::LOOP(my_loop) =>
                    {
                        let repeats = my_loop.repeats.evaluate(HashMap::new());
                        for i in 0..(repeats)
                        {
                            for loop_instruction in code_block.get_instructions().iter()
                            {

                            }
                        }
                    }
                    CodeBlockType::IF(repeat) =>
                    {

                    }
                    CodeBlockType::PROCEDURE(my_if) => {}
                }
            }
            ParserSymbol::COMMAND(command) =>
            {
                
            }
        }
    }
}   