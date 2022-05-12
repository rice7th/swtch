// ███████╗ ██╗    ██╗ ████████╗  ██████╗ ██╗  ██╗
// ██╔════╝ ██║    ██║ ╚══██╔══╝ ██╔════╝ ██║  ██║
// ███████╗ ██║ █╗ ██║    ██║    ██║      ███████║
// ╚════██║ ██║███╗██║    ██║    ██║      ██╔══██║
// ███████║ ╚███╔███╔╝    ██║    ╚██████╗ ██║  ██║
// ╚══════╝  ╚══╝╚══╝     ╚═╝     ╚═════╝ ╚═╝  ╚═╝
// Made by JhonnyRice

use std::env;

macro_rules! help {
    () => {
        println!("
Swatch: a simple palette tool made by Jhonny Rice.
Licensed under GPLv3.
usage:
    swatch <option>
options are:
    -e | --extended-colors\t\tPrints the palette with extended colors
    -h | --help\t\tPrints this text")
    };
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let mut colors: Vec<&str> = Vec::new();
    let option = &arguments.get(1);
    match option {
        Some(tmp) => match tmp.as_str() {
            "-e" | "--extended-colors" => {
                colors.push("\x1b[41m");
                colors.push("\x1b[43m");
                colors.push("\x1b[42m");
                colors.push("\x1b[46m");
                colors.push("\x1b[44m");
                colors.push("\x1b[45m");
                colors.push("\x1b[47m");
                colors.push("\x1b[40m");
            },

            "-h" | "--help" => help!(),
            _ => {
                println!("\nUnknown command \"{}\"", tmp);
                help!()
            },
        },

        None => { // No option
            colors.push("\x1b[41m");
            colors.push("\x1b[43m");
            colors.push("\x1b[42m");
            colors.push("\x1b[46m");
            colors.push("\x1b[44m");
            colors.push("\x1b[45m");
        },
    }
    
    for _i in 0..3 {
        for j in 0..colors.len() {
            for _k in 0..9 {
                print!("{} ", colors[j])
            }
            print!("\x1b[0m  ")
        }
        println!("")
    }

    for _l in 0..colors.len() {
        for _m in 0..9 {
            print!("\x1b[47m ")
        }
        print!("\x1b[0m  ")
    }
    print!("\x1b[0m\r\n")
}
