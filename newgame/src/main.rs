use std::env;
use std::fs;
use fs::read_to_string;
use librustc_lexer; 

fn add_game_to_lib(newgame: &String) {
    let librs = fs::read_to_string("./toybox/lib.rs")
        .expect("This utility should be called from the top level of the toybox-rs repository.")
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Provide the name of a new toybox game");
        return
    } 

    let newgame = (&args[2]).replace("tb_", "to");


    // Steps:
    // 1. Modify the code in toybox/src/lib.rs to include the new game.
    add_game_to_lib(&newgame);

}
