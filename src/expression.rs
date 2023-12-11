use std::collections::{ LinkedList, HashMap };
use lazy_static::lazy_static;

lazy_static! 
{
    pub static ref OPERATORS: HashMap<&'static str, i8> = 
    {
        let mut map = HashMap::new();
        map.insert("+", 1i8);
        map.insert("-", 1i8);
        map.insert("*", 2i8);
        map.insert("/", 2i8);
        map.insert("<", 0i8);
        map
    };
}

#[derive(Clone)]
enum ExpressionSymbol
{
    VARIABLE(String),
    CONSTANT(i32),
    OPERATOR(String),
    BRACKET_OPENING,
    BRACKET_CLOSING
}

pub struct Expression
{
    postifx_symbol_list: LinkedList<ExpressionSymbol>
}

impl Expression
{
    pub fn new(expression_str: Vec<&str>) -> Self
    {
        let infix_symbol_list = Expression::create_list_of_expression_symbols(expression_str);
        let postifx_symbol_list = Expression::infix_to_postfix(infix_symbol_list);
        Self { postifx_symbol_list }
    }

    pub fn evaluate(&self, variables: &HashMap<String, i32>) -> i32
    {
        let mut stack: Vec<i32> = vec!();
        for symbol in self.postifx_symbol_list.iter()
        {
            match symbol
            {
                ExpressionSymbol::CONSTANT(constant) =>
                {
                    stack.push(*constant);
                }
                ExpressionSymbol::VARIABLE(variable) =>
                {
                    println!("{variable}");
                    println!("{:?}", variables);
                    stack.push(*variables.get(variable).unwrap());
                }
                ExpressionSymbol::OPERATOR(operator) =>
                {
                    let y = stack.pop().unwrap();
                    let x = stack.pop().unwrap();

                    match operator.as_str()
                    {
                        "+" => { stack.push(x + y); }
                        "-" => { stack.push(x - y); }
                        "*" => { stack.push(x * y); }
                        "/" => { stack.push(x / y); }
                        "<" => { if x < y { stack.push(1); } else { stack.push(0); } }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        stack.pop().unwrap()
    }

    fn create_list_of_expression_symbols(expression_str: Vec<&str>) -> LinkedList<ExpressionSymbol>
    {
        let mut result: LinkedList<ExpressionSymbol> = LinkedList::new();
        for &expression_symbol in expression_str.iter()
        {
            if OPERATORS.contains_key(expression_symbol)
            { // Operator
                result.push_back(ExpressionSymbol::OPERATOR(expression_symbol.to_string()));
            }
            else if expression_symbol == "("
            {
                result.push_back(ExpressionSymbol::BRACKET_OPENING);
            }
            else if expression_symbol == ")"
            {
                result.push_back(ExpressionSymbol::BRACKET_CLOSING);
            }
            else if expression_symbol.chars().all(char::is_numeric)
            { // Constant
                result.push_back(ExpressionSymbol::CONSTANT(expression_symbol.parse::<i32>().unwrap()));
            }
            else 
            { // Variable
                result.push_back(ExpressionSymbol::VARIABLE(expression_symbol.to_string()));
            }
        }
        result
    }

    fn infix_to_postfix(infix: LinkedList<ExpressionSymbol>) -> LinkedList<ExpressionSymbol>
    {
        let mut postfix: LinkedList<ExpressionSymbol> = LinkedList::new();
        let mut stack: Vec<ExpressionSymbol> = vec!();

        for symbol in infix.iter()
        {
            match symbol
            {
                ExpressionSymbol::CONSTANT(_) =>
                {
                    postfix.push_back(symbol.clone());
                }
                ExpressionSymbol::VARIABLE(_) =>
                {
                    postfix.push_back(symbol.clone());
                }
                ExpressionSymbol::BRACKET_OPENING =>
                {
                    stack.push(symbol.clone());
                }
                ExpressionSymbol::BRACKET_CLOSING =>
                {
                    while let Some(last_symbol) = stack.last()
                    {
                        match last_symbol
                        {
                            ExpressionSymbol::BRACKET_OPENING => { stack.pop(); break; }
                            _ => { postfix.push_back(stack.pop().unwrap()); }
                        }
                    }
                }
                ExpressionSymbol::OPERATOR(operator1) =>
                {
                    while let Some(last_symbol) = stack.last()
                    {
                        match last_symbol
                        {
                            ExpressionSymbol::OPERATOR(operator2) =>
                            { 
                                if OPERATORS.get(operator1.as_str()) <= OPERATORS.get(operator2.as_str())
                                {
                                    postfix.push_back(stack.pop().unwrap());
                                }
                                else { break; }
                            }
                            _ => { break; }
                        }
                    }
                    stack.push(symbol.clone());
                }
            }
        }
        while let Some(remaining_symbol) = stack.pop()
        {
            postfix.push_back(remaining_symbol);
        }

        postfix
    }

    pub fn text_literal(&self) -> String
    {
        if let ExpressionSymbol::VARIABLE(text) = self.postifx_symbol_list.front().unwrap()
        {
            return text.clone();
        }
        "".to_string()
    }
}