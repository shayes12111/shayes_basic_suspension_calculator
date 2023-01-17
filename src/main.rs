
mod vehicle;
//use crate::vehicle::*;

mod suspension_calculations;
use crate::suspension_calculations::*;

mod cli_prompts;
use crate::cli_prompts::*;

use std::io;


fn main() {

    welcome_menu();
    loop {
        let mut input = String::new();

            io::stdin()
            .read_line(&mut input)
            .expect("Failed");

        let input_clone = input.clone().trim().to_string(); //appends return character? trimming fixes
        if input.contains( &"exit".to_string()) {break}
        else {run_program(input_clone)};  
        
        print_instructions();
        }
}





