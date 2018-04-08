use std::fs::File;
use std::io::prelude::*;
use std::{thread, time::Duration};

fn main() {
    // Setup CPU and memory
    let mut program_counter = 0x200;
    let mut current_opcode: u16 = 0;
    let mut index_register = 0;
    let mut stack_pointer = 0;

    let mut memory: Vec<u8> = vec![0; 0x1000];

    // Load rom
    let mut rom = File::open("pong.rom").expect("ROM file missing");
    let mut rom_buffer = Vec::new();
    rom.read_to_end(&mut rom_buffer);

    let mut rom_loading_pointer = 0x200;
    for i in 0..rom_buffer.len() {
        memory[rom_loading_pointer] = rom_buffer[i];
        rom_loading_pointer = rom_loading_pointer + 1;
    }

    // Main loop
    loop {
        current_opcode = ((memory[program_counter] as u16) << 8) | (memory[program_counter + 1] as u16);
        println!("Program counter: {:X}", program_counter);
        println!("Index regster: {:X}", index_register);
        println!("Stack pointer: {:X}", stack_pointer);
        println!("Opcode: {:X}", current_opcode);

        match (current_opcode & 0xF000) {
            0xA000 => {
                println!("ANNN: Sets I to the address NNN");
                index_register = current_opcode & 0x0FFF;
                program_counter = program_counter + 2;
            }
            _ => {
                println!("Unknown opcode: {:X}", current_opcode);
                program_counter = program_counter + 2;
            }
        }

        thread::sleep(Duration::from_millis(500));
        print!("{}[2J", 27 as char);
    }
}
