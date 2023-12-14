use std::fs;

use crate::parser::parse_logo_code;
use crate::parser::CodeBlock;
use crate::interpreter::execute_logo_program;
use std::collections::HashMap;

fn read_code_from_file(file_path: &str) -> String
{
    fs::read_to_string(file_path)
       .expect(format!("Unable to read the file: {file_path}").as_str())
}

fn save_to_svg_file(file_path: &str, code: String)
{
    fs::write(file_path, code).expect(format!("Unable to write to file: {file_path}", ).as_str())
}

pub fn parse_and_execute(logo_file_paths: &[&str], svg_file_paths: &[&str])
{
    let mut logo_codes: Vec<String> = vec!();
    for logo_file_path in logo_file_paths
    {
        let mut code = read_code_from_file(logo_file_path).replace("repcount", ":repcount");
        code.push(' ');
        logo_codes.push(code);
    }

    let mut logo_parsed_codes: Vec<HashMap<String, CodeBlock>> = vec!();
    for logo_code in logo_codes
    {
        let parsed_code = parse_logo_code(logo_code.as_str());
        logo_parsed_codes.push(parsed_code);
    }

    for logo_parsed_code_pair in logo_parsed_codes.into_iter().zip(svg_file_paths.into_iter())
    {
        let svg_code = execute_logo_program(logo_parsed_code_pair.0);
        save_to_svg_file(logo_parsed_code_pair.1, svg_code);
    }
}