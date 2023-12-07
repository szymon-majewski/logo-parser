use std::fs;

mod expression;
mod parser;
mod interpreter;
use parser::read_expression;
use expression::Expression;

//DEBUG
use std::collections::HashMap;
//DEBUG

fn read_code_from_file(file_path: &str) -> String
{
    fs::read_to_string(file_path)
       .expect(format!("Unable to read the file: {file_path}").as_str())
}

fn main() 
{
    // let logo_file_paths = ["resources/star.logo",
	// 					   "resources/colored_squares.logo",
	// 					   "resources/logo_spiral.logo",
	// 					   "resources/tree.logo",
	// 					   "resources/fern.logo",
	// 					   "resources/turtle_race.logo",];
    
    // for logo_file_path in logo_file_paths
    // {
    //     println!("{}\n", read_code_from_file(logo_file_path));
    // }

    let mut str = "5 +:var /( 12+:a) right 90".chars().peekable();
    let binding = read_expression(&mut str);
    let parsed: Vec<&str> = binding.iter().map(|s| s.as_str()).collect();
    
    for a in parsed.iter()
    {
        println!("{a}");
    }

    let exp = Expression::new(parsed);
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("var".to_string(), 26);
    map.insert("a".to_string(), 1);
    println!("{}", exp.evaluate(map));

    // let exp = vec!["(", "36", "+", ":grum", ")", "*", "2"];
    // let e = parser::Expression::new(exp);
    // let mut map: HashMap<String, i32> = HashMap::new();
    // map.insert(":grum".to_string(), 1);
    // println!("{}", e.evaluate(map));
}