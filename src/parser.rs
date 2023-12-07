use std::collections::{ LinkedList, HashMap, HashSet };
use lazy_static::lazy_static;

use crate::expression;
use crate::expression::Expression;

lazy_static! 
{
    static ref LOGO_SYMBOLS: HashSet<&'static str> = 
    {
        let mut set = HashSet::new();
        set.insert("to");
        set.insert("repeat");
        set.insert("if");
        set.insert("end");
        set
    };

    static ref COMMANDS: HashMap<&'static str, CommandType> = 
    {
        let mut set = HashMap::new();
        set.insert("forward", CommandType::FORWARD);
        set.insert("fd", CommandType::FORWARD);
        set.insert("backward", CommandType::BACKWARD);
        set.insert("bk", CommandType::BACKWARD);
        set.insert("right", CommandType::TURN_RIGHT);
        set.insert("rt", CommandType::TURN_RIGHT);
        set.insert("left", CommandType::TURN_LEFT);
        set.insert("lt", CommandType::TURN_LEFT);
        set.insert("clearscreen", CommandType::CLEAR_SCREEN);
        set.insert("cs", CommandType::CLEAR_SCREEN);
        set.insert("penup", CommandType::PEN_UP);
        set.insert("pu", CommandType::PEN_UP);
        set.insert("pendown", CommandType::PEN_DOWN);
        set.insert("pd", CommandType::PEN_DOWN);
        set.insert("hideturtle", CommandType::HIDE_TURTLE);
        set.insert("ht", CommandType::HIDE_TURTLE);
        set.insert("showturtle", CommandType::SHOW_TURTLE);
        set.insert("st", CommandType::SHOW_TURTLE);
        set.insert("setcolor", CommandType::SET_COLOR);
        set.insert("pick", CommandType::PICK);
        set.insert("random", CommandType::RANDOM);
        set.insert("stop", CommandType::STOP);
        set.insert("wait", CommandType::WAIT);
        set.insert("label", CommandType::LABEL);
        set.insert("setlabelheight", CommandType::SET_LABEL_HEIGHT);
        set.insert("setturtle", CommandType::SET_TURTLE);
        set
    };
}

enum CommandType
{
    FORWARD,
    BACKWARD,
    TURN_RIGHT,
    TURN_LEFT,
    CLEAR_SCREEN,
    PEN_UP,
    PEN_DOWN,
    SET_COLOR,
    PICK,
    RANDOM,
    STOP,
    WAIT,
    HIDE_TURTLE,
    SHOW_TURTLE,
    LABEL,
    SET_LABEL_HEIGHT,
    SET_TURTLE
}

pub enum ParserSymbol 
{
    COMMAND(Command),
    CODE_BLOCK(Box<dyn CodeBlock>)
}

struct Command
{
    command_type: CommandType,
    call_parameter: Expression
}

trait CodeBlock
{
    fn add_instruction(&self, instruction: ParserSymbol);
}

pub struct Procedure
{
    instructions: LinkedList<ParserSymbol>,
    call_parameters: HashMap<String, Expression>
}

impl Procedure
{
    fn new() -> Self
    {
        Self { instructions: LinkedList::new(), call_parameters: HashMap::new() }
    }
}

// impl CodeBlock for Procedure
// {
//     fn add_instruction(&self, instruction: ParserSymbol) { self.instructions.push_back(instruction); }
// }

struct Loop
{
    instructions: LinkedList<ParserSymbol>,
    repeats: Expression,
    counter: usize
}

impl Loop
{
    fn new(repeats: Expression) -> Self
    {
        Self { instructions: LinkedList::new(), repeats, counter: 1 }
    }
}

// impl CodeBlock for Loop
// {
//     fn add_instruction(&self, instruction: ParserSymbol) { self.instructions.push_back(instruction); }
// }

struct If
{
    instructions: LinkedList<ParserSymbol>,
    condition: Expression
}

impl If
{
    fn new(condition: Expression) -> Self
    {
        Self { instructions: LinkedList::new(), condition }
    }
}

// impl CodeBlock for If
// {
//     fn add_instruction(&self, instruction: ParserSymbol) { self.instructions.push_back(instruction); }
// }

enum ParserState
{
    READING_SYMBOL,
    READING_PROCEDURE_NAME,
    READING_PROCEDURE_PARAMETERS
}

// pub fn parse_logo_code(code: &str) -> HashMap<String, Procedure>
// {
//     let mut code_iterator = code.chars().peekable();
//     let mut state = ParserState::READING_SYMBOL;
//     let mut current_symbol = "".to_string();

//     let MAIN_PROCEDURE_NAME = "_".to_string();
//     let mut procedures: HashMap<String, Procedure> = HashMap::new();
//     procedures.insert(MAIN_PROCEDURE_NAME.clone(), Procedure::new());
//     let mut current_procedure_name = MAIN_PROCEDURE_NAME.clone();
//     let mut code_block_stack: Vec<&dyn CodeBlock> = vec![procedures.get(&MAIN_PROCEDURE_NAME).unwrap()];

//     while let Some(ch) = code_iterator.next()
//     {
//         match state
//         {
//             ParserState::READING_SYMBOL => 
//             {
//                 if ch.is_whitespace()
//                 {
//                     if LOGO_SYMBOLS.contains(current_symbol.as_str())
//                     {
//                         match current_symbol.as_str()
//                         {
//                             "to" => { state = ParserState::READING_PROCEDURE_NAME; }
//                             "end" => 
//                             { 
//                                 state = ParserState::READING_SYMBOL; 
//                                 current_procedure_name = MAIN_PROCEDURE_NAME.clone();
//                             }
//                             "loop" =>
//                             {
//                                 let loop_repeats = read_expression(&mut code_iterator).iter().map(|s| s.as_str()).collect();
//                                 let mut new_loop = Loop::new(Expression::new(loop_repeats));
//                                 //code_block_stack.last().unwrap().add_instruction(new_loop);
//                                 code_block_stack.push(&new_loop);
//                             }
//                             "if" =>
//                             {
//                                 let if_condition = read_expression(&mut code_iterator).iter().map(|s| s.as_str()).collect();
//                                 let mut new_if = If::new(Expression::new(if_condition));
//                                 //code_block_stack.last().unwrap().add_instruction(new_if);
//                                 code_block_stack.push(&new_if);
//                             }
//                             _ => {}
//                         }
//                     }
//                     else if COMMANDS.contains_key(current_symbol.as_str())
//                     {
//                         let parameter = read_expression(&mut code_iterator);
//                     }

//                     current_symbol.clear();
//                 }
//                 else 
//                 {
//                     current_symbol.push(ch);
//                 }
//             }
//             ParserState::READING_PROCEDURE_NAME => 
//             {
//                 if ch.is_whitespace()
//                 {
//                     let mut new_procedure = Procedure::new();
//                     procedures.insert(current_symbol.clone(), new_procedure);
//                     current_procedure_name = current_symbol.clone();
//                     code_block_stack.push(&new_procedure);
//                     current_symbol.clear();
//                 }
//                 else 
//                 {
//                     current_symbol.push(ch);
//                 }
//             }
//             ParserState::READING_PROCEDURE_PARAMETERS => 
//             {
                
//             }
//         }
//     }

//     procedures
// }

pub fn read_expression(iter: &mut std::iter::Peekable<std::str::Chars<'_>>) -> Vec<String>
{
    let mut reading_variable = false;
    let mut current_symbol = String::new();
    let mut result: Vec<String> = Vec::new();

    while let Some(&next_char) = iter.peek()
    {
        if next_char.is_whitespace() 
        {
            reading_variable = false;
            if current_symbol.len() > 0
            {
                result.push(current_symbol.clone());
                current_symbol.clear();
            }
        }
        else if next_char == ':'
        {
            reading_variable = true;
        }
        else if next_char.is_numeric() || (next_char.is_alphabetic() && reading_variable)
        {
            current_symbol.push(next_char);
        }
        else if expression::OPERATORS.contains_key(next_char.to_string().as_str()) ||
                next_char == '(' || next_char == ')'
        {
            reading_variable = false;
            if current_symbol.len() > 0
            {
                result.push(current_symbol.clone());
                current_symbol.clear();
            }
            result.push(next_char.to_string());
        }
        else { break; }
        iter.next();
    }
    result
}