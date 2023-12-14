mod expression;
mod parser;
mod interpreter;
mod logo_manager;

fn main() 
{
    let logo_file_paths = ["resources/star.logo",
						   "resources/colored_squares.logo",
						   "resources/logo_spiral.logo",
						   "resources/tree.logo",
						   "resources/fern.logo",
						   "resources/turtle_race.logo",
                           "resources/flower.logo",
                           "resources/rotating_circle.logo",
                           "resources/sun.logo"];
    let svg_file_paths = ["svg/star.svg",
						   "svg/colored_squares.svg",
						   "svg/logo_spiral.svg",
						   "svg/tree.svg",
						   "svg/fern.svg",
						   "svg/turtle_race.svg",
                           "svg/flower.svg",
                           "svg/rotating_circle.svg",
                           "svg/sun.svg"];
    logo_manager::parse_and_execute(&logo_file_paths, &svg_file_paths);
}