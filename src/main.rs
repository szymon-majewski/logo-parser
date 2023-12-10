use std::fs;

mod expression;
mod parser;
mod interpreter;
use parser::read_expression;
use parser::parse_logo_code;
use parser::CodeBlock;
use expression::Expression;
use interpreter::execute_logo_program;

//DEBUG
use std::collections::HashMap;
//DEBUG

fn read_code_from_file(file_path: &str) -> String
{
    fs::read_to_string(file_path)
       .expect(format!("Unable to read the file: {file_path}").as_str())
}

fn save_to_svg_file(file_path: &str, code: String)
{
    fs::write(file_path, code).expect(format!("Unable to write to file: {file_path}", ).as_str())
}

fn main() 
{
    let logo_file_paths = ["resources/star.logo",
						   "resources/colored_squares.logo",
						   "resources/logo_spiral.logo",
						   "resources/tree.logo",
						   "resources/fern.logo",
						   "resources/turtle_race.logo",];
    let mut logo_codes: Vec<String> = vec!();
    for logo_file_path in logo_file_paths
    {
        let mut code = read_code_from_file(logo_file_path);
        code.push(' ');
        logo_codes.push(code);
    }

    let mut logo_parsed_codes: Vec<HashMap<String, CodeBlock>> = vec!();
    for logo_code in logo_codes
    {
        let parsed_code = parse_logo_code(logo_code.as_str());

        for (k, v) in parsed_code.iter()
        {
            println!("{k}");
        }
        println!("");

        logo_parsed_codes.push(parsed_code);
    }

    // star program
    for logo_parsed_code in logo_parsed_codes
    {
        let svg_code = execute_logo_program(logo_parsed_code);
        save_to_svg_file("svg/star.svg", svg_code);
        break;
    }
    
    // let mut str = "5 +:var /( 12+:a) right 90".chars().peekable();
    // let binding = read_expression(&mut str);
    // let parsed: Vec<&str> = binding.iter().map(|s| s.as_str()).collect();
    
    // for a in parsed.iter()
    // {
    //     println!("{a}");
    // }

    // let exp = Expression::new(parsed);
    // let mut map: HashMap<String, i32> = HashMap::new();
    // map.insert("var".to_string(), 26);
    // map.insert("a".to_string(), 1);
    // println!("{}", exp.evaluate(map));

    // let exp = vec!["(", "36", "+", ":grum", ")", "*", "2"];
    // let e = parser::Expression::new(exp);
    // let mut map: HashMap<String, i32> = HashMap::new();
    // map.insert(":grum".to_string(), 1);
    // println!("{}", e.evaluate(map));
}