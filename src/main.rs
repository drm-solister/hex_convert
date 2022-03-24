#![windows_subsystem = "windows"]
use pancurses::{initscr, endwin};

use std::i64;

fn main() {
    // make window and find out how to use getch without adding the character to the screen
    let window = init_window();

    let mut input_base: u32 = 10;
    let mut input: String = String::from("");
    let output_bits = 8; // by default will output to 16 bits


    
    loop{
        let c = match window.getch().unwrap() {
            pancurses::Input::Character(char) => char,
            _ => '\0',
        };
        // only allow o or x in input if it comes after a 0
        if c.is_digit(input_base){
            println!("input base: {}", input_base);
            input.push(c);
            window.addch(c);
        }else if (c == 'o' || c == 'x' || c == 'b') && input.chars().next() == Some('0') && input.len() == 1{ // make this neater, with matching?
            //println!("{:?}", c);
            window.addch(c);
            input.push(c);
        }else if c == '\u{8}' { // backspace
            input.pop();
            window.addch(c);
            window.delch();
            clear_lines(&window, &[6,9,12,15]);
        }else if c == '\t' { // switch to setting the number of output bits
            //println!("{:?}", c);
        }// if char is escape, end window
        

        input_base = update_base(&input); // try making the reassignment not necessary 

        update_conversions(&window, &input, input_base);
        
    }
}

fn init_window() -> pancurses::Window {
    let window = initscr();
    let cursor_start: (i32, i32) = (0, 1);
    window.printw("Radix conversion");
    window.mv(cursor_start.1, cursor_start.0);
    window.refresh();
    pancurses::noecho();

    window.printw("──────────────────────────────────┬─────────────────────────");
    window.mv(2, 0);
    window.printw(" input:                           ┃ # of output bits:       ");

    window.mv(5, 0);
    window.printw(" hexadecimal:");

    window.mv(8, 0);
    window.printw(" binary:");

    window.mv(11, 0);
    window.printw(" octal:");

    window.mv(14, 0);
    window.printw(" decimal:");

    window.mv(3, 1);

    return window;
}

fn update_base(input: &String) -> u32 {
    // update the base, based on the first 2 characters of the input

    let index = if input.len() < 2 { input.len() } else { 2 };
    
    let input_base = match &input[..index] {
        "0x" => {println!("hex"); 16},
        "0b" => {println!("binary"); 2},
        "0o" => {println!("octal"); 8},
        _ => {println!("decimal"); 10},
    };

    return input_base;
}

fn update_conversions(window: &pancurses::Window, input: &String, input_base: u32) {
    
    if input.len() == 0 {
        // set all the output fields to 0 and return
        return;
    }

    // if input_base isnt 10, cut off the first 2 characters. then convert to an integer
    let input = if input_base != 10 {
        input[2..].to_string() // shouldnt change input outside of this scope
        //println!("input should be trimmed: {}", input);
    } else {
        input.to_string()
    };

    if input.len() == 0 {
        // also here set all the output fields to 0 and return
        return;
    }

    let init_pos: (i32, i32) = window.get_cur_yx();
    let output = convert_to_dec(&input, input_base);
 
    window.mvprintw(6,1, format!("{:#x}", output.parse::<i64>().unwrap())); // input string, input pase, output base
    window.mvprintw(9,1, format!("{:#b}", output.parse::<i64>().unwrap())); 
    window.mvprintw(12,1, format!("{:#o}", output.parse::<i64>().unwrap())); 
    window.mvprintw(15,1, format!("{:?}", output.parse::<i64>().unwrap())); 

    window.mv(init_pos.0, init_pos.1);
}

fn convert_to_dec(input: &String, input_base: u32) -> String {
    if input_base == 10 { // use the prior check to avoid doing this maybe
        //return input.parse::<u128>().unwrap();
    } 

    let output = i64::from_str_radix(input, input_base); // from_str_radix limited to its u32 input
    format!("{:?}", output.unwrap())
    
}

fn clear_lines(window: &pancurses::Window, lines: &[u32]) {

    let current_cursor: (i32, i32) = window.get_cur_yx();

    for line in lines.into_iter() {
        //let line: i32 = *line
        window.mv(i32::try_from(*line).ok().unwrap(), 0);
        window.clrtoeol();
    }

    window.mv(current_cursor.0, current_cursor.1);
}
