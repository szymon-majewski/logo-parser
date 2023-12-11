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
        set.insert("back", CommandType::BACKWARD);
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
        set.insert("window", CommandType::WINDOW);
        set
    };

    static ref COMMANDS_NO_PARAMETER: HashMap<&'static str, CommandType> = 
    {
        let mut set = HashMap::new();
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
        set.insert("stop", CommandType::STOP);
        set.insert("window", CommandType::WINDOW);
        set
    };
}

#[derive(Clone, Copy)]
pub enum CommandType
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
    SET_TURTLE,
    WINDOW
}

pub enum CodeBlockType
{
    PROCEDURE(Procedure),
    LOOP(Loop),
    IF(If)
}

pub enum ParserSymbol 
{
    COMMAND(Command),
    CODE_BLOCK(CodeBlock),
    PROCEDURE_CALL(ProcedureCall)
}

pub struct Command
{
    pub command_type: CommandType,
    pub call_parameter: Expression
}
impl Command
{
    fn new(command_type: CommandType, call_parameter: Expression) -> Self
    {
        Self { command_type, call_parameter }
    }
}

pub struct ProcedureCall
{
    pub procedure_name: String,
    pub parameter_expressions: LinkedList<Expression>
}
impl ProcedureCall
{
    pub fn new(procedure_name: String, parameter_expressions: LinkedList<Expression>) -> Self
    {
        Self { procedure_name, parameter_expressions }
    }
}

pub struct CodeBlock
{
    instructions: LinkedList<ParserSymbol>,
    pub code_block_type: CodeBlockType
}
impl CodeBlock
{
    fn new(code_block_type: CodeBlockType) -> Self
    {
        Self { instructions: LinkedList::new(), code_block_type }
    }
    
    pub fn add_instruction(&mut self, instruction: ParserSymbol) 
    { 
        self.instructions.push_back(instruction); 
    }

    pub fn last_instruction(&mut self) -> &mut ParserSymbol
    {
        self.instructions.back_mut().unwrap()
    }

    pub fn get_instructions(&self) -> &LinkedList<ParserSymbol>
    {
        &self.instructions
    }
}

pub struct Procedure
{
    pub call_parameters: HashMap<String, i32>
}
impl Procedure
{
    fn new() -> Self
    {
        Self { call_parameters: HashMap::new() }
    }
}

pub struct Loop
{
    pub repeats: Expression
}
impl Loop
{
    fn new(repeats: Expression) -> Self
    {
        Self { repeats }
    }
}

pub struct If
{
    pub condition: Expression
}
impl If
{
    fn new(condition: Expression) -> Self
    {
        Self { condition }
    }
}

enum ParserState
{
    READING_SYMBOL,
    READING_PROCEDURE_NAME,
    READING_PROCEDURE_PARAMETERS
}

pub fn parse_logo_code(code: &str) -> HashMap<String, CodeBlock>
{
    let mut code_iterator = code.chars().peekable();
    let mut state = ParserState::READING_SYMBOL;
    let mut current_symbol = "".to_string();

    let MAIN_PROCEDURE_NAME = "_".to_string();
    let mut procedures: HashMap<String, CodeBlock> = HashMap::new();
    let mut procedures_info: HashMap<String, i32> = HashMap::new();
    //procedures.insert(MAIN_PROCEDURE_NAME.clone(), CodeBlock::new(CodeBlockType::PROCEDURE(Procedure::new())));
    let mut current_procedure_name = MAIN_PROCEDURE_NAME.clone();
    let mut current_procedure = CodeBlock::new(CodeBlockType::PROCEDURE(Procedure::new()));
    let mut current_code_block: &mut CodeBlock = &mut current_procedure;

    while let Some(ch) = code_iterator.next()
    {
        match state
        {
            ParserState::READING_SYMBOL => 
            {
                if ch.is_whitespace()
                {
                    if LOGO_SYMBOLS.contains(current_symbol.as_str())
                    {
                        match current_symbol.as_str()
                        {
                            "to" => 
                            { 
                                state = ParserState::READING_PROCEDURE_NAME;
                            }
                            "end" => 
                            { 
                                procedures.insert(current_procedure_name.clone(), current_procedure);
                                state = ParserState::READING_SYMBOL;
                                current_procedure = CodeBlock::new(CodeBlockType::PROCEDURE(Procedure::new()));
                                current_code_block = &mut current_procedure;
                                current_procedure_name = MAIN_PROCEDURE_NAME.clone();
                            }
                            "repeat" =>
                            {
                                let parsed_expression = read_expression(&mut code_iterator);
                                let loop_repeats = parsed_expression.iter().map(|s| s.as_str()).collect();
                                let mut new_loop = CodeBlock::new(CodeBlockType::LOOP(Loop::new(Expression::new(loop_repeats))));
                                current_code_block.add_instruction(ParserSymbol::CODE_BLOCK(new_loop));
                                let last_instruction;
                                {
                                    let last_code_block_ref = current_code_block.last_instruction();
                                    if let ParserSymbol::CODE_BLOCK(last_code_block) = last_code_block_ref 
                                    {
                                        last_instruction = last_code_block;
                                    } else { panic!(); }
                                }
                                current_code_block = last_instruction;
                            }
                            "if" =>
                            {
                                let parsed_expression = read_expression(&mut code_iterator);
                                let if_condition = parsed_expression.iter().map(|s| s.as_str()).collect();
                                let mut new_if = CodeBlock::new(CodeBlockType::IF(If::new(Expression::new(if_condition))));
                                current_code_block.add_instruction(ParserSymbol::CODE_BLOCK(new_if));
                                let last_instruction;
                                {
                                    let last_code_block_ref = current_code_block.last_instruction();
                                    if let ParserSymbol::CODE_BLOCK(last_code_block) = last_code_block_ref 
                                    {
                                        last_instruction = last_code_block;
                                    } else { panic!(); }
                                }
                                current_code_block = last_instruction;
                            }
                            _ => {}
                        }
                    }
                    else if COMMANDS_NO_PARAMETER.contains_key(current_symbol.as_str())
                    {
                        current_code_block.add_instruction(ParserSymbol::COMMAND(Command::new(*COMMANDS_NO_PARAMETER.get(current_symbol.as_str()).unwrap(),
                                                                               Expression::new(vec!()))));
                    }
                    else if COMMANDS.contains_key(current_symbol.as_str())
                    {
                        let parameter = read_expression(&mut code_iterator);
                        let parameter_str = parameter.iter().map(|s| s.as_str()).collect();
                        current_code_block.add_instruction(ParserSymbol::COMMAND(Command::new(*COMMANDS.get(current_symbol.as_str()).unwrap(),
                                                                               Expression::new(parameter_str))));
                    }
                    else if procedures_info.contains_key(current_symbol.as_str())
                    {
                        //TODO: read procedure parameters
                        let call_parameters_count = *procedures_info.get(current_symbol.as_str()).unwrap();
                        println!("{call_parameters_count}");
                        let mut call_parameters = LinkedList::new();
                        for _ in 0..call_parameters_count
                        {
                            let parameter = read_expression(&mut code_iterator);
                            let parameter_str = parameter.iter().map(|s| s.as_str()).collect();
                            call_parameters.push_back(Expression::new(parameter_str));
                        }
                        current_code_block.add_instruction(ParserSymbol::PROCEDURE_CALL(ProcedureCall{ procedure_name: current_symbol.clone(), parameter_expressions: call_parameters}));
                    }
                    current_symbol.clear();
                }
                else if ch == ']'
                {
                    current_code_block = &mut current_procedure;
                }
                else if ch == '[' {}
                else 
                {
                    current_symbol.push(ch);
                }
            }
            ParserState::READING_PROCEDURE_NAME => 
            {
                if ch.is_whitespace()
                {
                    //procedures.insert(current_symbol.clone(), CodeBlock::new(CodeBlockType::PROCEDURE(Procedure::new())));
                    current_procedure_name = current_symbol.clone();
                    //code_block_stack.push(procedures.get_mut(&current_symbol).unwrap());
                    current_symbol.clear();
                    state = ParserState::READING_PROCEDURE_PARAMETERS;
                }
                else 
                {
                    current_symbol.push(ch);
                }
            }
            ParserState::READING_PROCEDURE_PARAMETERS => 
            {
                if ch == ':'
                {
                    current_symbol.push(ch);
                }
                else if ch.is_whitespace()
                {
                    if current_symbol.len() > 0
                    {
                        if let CodeBlockType::PROCEDURE(ref mut procedure) = &mut current_code_block.code_block_type 
                        {
                            procedure.call_parameters.insert(current_symbol[1..].to_string(), 0);
                        }
                        current_symbol.clear();
                    }
                }
                else if current_symbol.len() > 0
                {
                    current_symbol.push(ch);
                }
                else 
                {
                    state = ParserState::READING_SYMBOL;
                    current_symbol.push(ch);
                    if let CodeBlockType::PROCEDURE(ref mut procedure) = &mut current_code_block.code_block_type 
                    {
                        procedures_info.insert(current_procedure_name.clone(), procedure.call_parameters.len() as i32);
                    }
                }
            }
        }
        println!("{}",current_symbol.clone());
    }
    procedures.insert(current_procedure_name.clone(), current_procedure);

    procedures
}

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
        else if next_char == ':' || next_char == '\"'
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