extern crate rand;

use std::fs::File;
use std::io::prelude::*;
use std::{thread, time::Duration};
use rand::{Rng, thread_rng};

fn main() {
    // Setup CPU and memory
    let mut program_counter: u16 = 0x200;
    let mut current_opcode: u16;
    let mut index_register = 0;
    let mut stack_pointer = 0;

    let mut registers: Vec<u8> = vec![0; 16];
    let mut memory: Vec<u8> = vec![0; 0x1000];
    let mut stack: Vec<u16> = vec![0; 16];

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
        current_opcode = ((memory[program_counter as usize] as u16) << 8) | (memory[(program_counter + 1) as usize] as u16);
        println!("Program counter: {:X}", program_counter);
        println!("Index regster: {:X}", index_register);
        println!("Stack pointer: {:X}", stack_pointer);
        println!("Stack : {:?}", stack);
        println!("Registers: {:?}", registers);
        println!("Opcode: {:X}", current_opcode);

        match current_opcode & 0xF000 {
            0x1000 => {
                println!("1NNN: Jump to the address NNN");
                program_counter = current_opcode & 0x0FFF;
            }
            0x2000 => {
                println!("2NNN: Calls subroutine at the address NNN");
                stack[stack_pointer as usize] = program_counter;
                stack_pointer = stack_pointer + 1;
                program_counter = current_opcode & 0x0FFF;
            }
            0x3000 => {
                println!("3XNN: Skips the next instruction if VX equals NN");
                if registers[((current_opcode & 0x0F00) >> 8) as usize] == ((current_opcode & 0x00FF) as u8) {
                    program_counter = program_counter + 4;
                } else {
                    program_counter = program_counter + 2;
                }
            }
            0x4000 => {
                println!("4XNN: Skips the next instruction if VX does not equal NN");
                if registers[((current_opcode & 0x0F00) >> 8) as usize] != ((current_opcode & 0x00FF) as u8) {
                    program_counter = program_counter + 4;
                } else {
                    program_counter = program_counter + 2;
                }
            }
            0x5000 => {
                println!("5XY0: Skips the next instruction if VX equals VY");
                if registers[((current_opcode & 0x0F00) >> 8) as usize] == registers[((current_opcode & 0x00F0) >> 4) as usize] {
                    program_counter = program_counter + 4;
                } else {
                    program_counter = program_counter + 2;
                }
            }
            0x6000 => {
                println!("6XNN: Sets VX to NN");
                registers[((current_opcode & 0x0F00) >> 8) as usize] = (current_opcode & 0x00FF) as u8;
                program_counter = program_counter + 2;
            }
            0x7000 => {
                println!("7XNN: Adds NN to VX. (Carry flag is not changed)");
                let new_register_value = registers[((current_opcode & 0x0F00) >> 8) as usize] + (current_opcode & 0x00FF) as u8;
                registers[((current_opcode & 0x0F00) >> 8) as usize] = new_register_value;
                program_counter = program_counter + 2;
            }
            0x8000 => {
                match current_opcode & 0x000F {
                    0x0000 => {
                        println!("8XY0: Sets VX to the value of VY");
                        registers[((current_opcode & 0x0F00) >> 8) as usize] = registers[((current_opcode & 0x00F0) >> 4) as usize];
                        program_counter = program_counter + 2;
                    }
                    0x0001 => {
                        println!("8XY1: Sets VX to VX or VY. (Bitwise OR operation)");
                        let new_register_value = registers[((current_opcode & 0x0F00) >> 8) as usize] | registers[((current_opcode & 0x00F0) >> 4) as usize];
                        registers[((current_opcode & 0x0F00) >> 8) as usize] = new_register_value;
                        program_counter = program_counter + 2;
                    }
                    0x0002 => {
                        println!("8XY2: Sets VX to VX and VY. (Bitwise AND operation)");
                        let new_register_value = registers[((current_opcode & 0x0F00) >> 8) as usize] & registers[((current_opcode & 0x00F0) >> 4) as usize];
                        registers[((current_opcode & 0x0F00) >> 8) as usize] = new_register_value;
                        program_counter = program_counter + 2;
                    }
                    0x0003 => {
                        println!("8XY3: Sets VX to VX xor VY.");
                        let new_register_value = registers[((current_opcode & 0x0F00) >> 8) as usize] ^ registers[((current_opcode & 0x00F0) >> 4) as usize];
                        registers[((current_opcode & 0x0F00) >> 8) as usize] = new_register_value;
                        program_counter = program_counter + 2;
                    }
                    0x0004 => {
                        println!("8XY4: Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't.");
                        let checked_add_value = registers[((current_opcode & 0x0F00) >> 8) as usize].checked_add(registers[((current_opcode & 0x00F0) >> 4) as usize]);
                        match checked_add_value {
                            Some(x) => registers[0xF as usize] = 0x01,
                            None => registers[0xF as usize] = 0x00
                        }
                        registers[((current_opcode & 0x0F00) >> 8) as usize] = registers[((current_opcode & 0x0F00) >> 8) as usize].wrapping_add(registers[((current_opcode & 0x00F0) >> 4) as usize]);
                        program_counter = program_counter + 2;
                    }
                    0x0005 => {
                        println!("8XY5: VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there isn't.");
                        let checked_sub_value = registers[((current_opcode & 0x0F00) >> 8) as usize].checked_sub(registers[((current_opcode & 0x00F0) >> 4) as usize]);
                        match checked_sub_value {
                            Some(x) => registers[0xF as usize] = 0x00,
                            None => registers[0xF as usize] = 0x01
                        }
                        registers[((current_opcode & 0x0F00) >> 8) as usize] = registers[((current_opcode & 0x0F00) >> 8) as usize].wrapping_sub(registers[((current_opcode & 0x00F0) >> 4) as usize]);
                        program_counter = program_counter + 2;
                    }
                    0x0006 => {
                        println!("8XY6: Shifts VY right by one and copies the result to VX. VF is set to the value of the least significant bit of VY before the shift.");
                        registers[0xF as usize] = registers[((current_opcode & 0x00F0) >> 4) as usize] & 0b0000_0001;
                        registers[((current_opcode & 0x00F0) >> 4) as usize] = registers[((current_opcode & 0x00F0) >> 4) as usize] >> 1;
                        registers[((current_opcode & 0x0F00) >> 8) as usize] = registers[((current_opcode & 0x00F0) >> 4) as usize];
                    }
                    0x0007 => {
                        println!("8XY7: Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there isn't.");
                        let checked_sub_value = registers[((current_opcode & 0x00F0) >> 8) as usize].checked_sub(registers[((current_opcode & 0x0F00) >> 4) as usize]);
                        match checked_sub_value {
                            Some(x) => registers[0xF as usize] = 0x00,
                            None => registers[0xF as usize] = 0x01
                        }
                        registers[((current_opcode & 0x0F00) >> 8) as usize] = registers[((current_opcode & 0x00F0) >> 8) as usize].wrapping_sub(registers[((current_opcode & 0x0F00) >> 4) as usize]);
                        program_counter = program_counter + 2;
                    }
                    0x000E => {
                        println!("8XYE: Shifts VY left by one and copies the result to VX. VF is set to the value of the most significant bit of VY before the shift.");
                        registers[0xF as usize] = (registers[((current_opcode & 0x00F0) >> 4) as usize] & 0b1000_0000) >> 7;
                        registers[((current_opcode & 0x00F0) >> 4) as usize] = registers[((current_opcode & 0x00F0) >> 4) as usize] << 1;
                        registers[((current_opcode & 0x0F00) >> 8) as usize] = registers[((current_opcode & 0x00F0) >> 4) as usize];
                    }
                    _ => {
                        println!("Unknown opcode: {:X}", current_opcode);
                        program_counter = program_counter + 2;
                    }
                }
            }
            0x9000 => {
                println!("9XY0: Skips the next instruction if VX doesn't equal VY. (Usually the next instruction is a jump to skip a code block");
                if registers[((current_opcode & 0x0F00) >> 8) as usize] != registers[((current_opcode & 0x00F0) >> 4) as usize] {
                    program_counter = program_counter + 4;
                } else {
                    program_counter = program_counter + 2;
                }
            }
            0xA000 => {
                println!("ANNN: Sets index register to the address NNN");
                index_register = current_opcode & 0x0FFF;
                program_counter = program_counter + 2;
            }
            0xB000 => {
                println!("BNNN: Jumps to the address NNN plus V0");
                program_counter = (current_opcode & 0x0FFF) + registers[0x0 as usize] as u16;
            }
            0xC000 => {
                println!("CXNN: Sets VX to the result of a bitwise and operation on a random number (Typically: 0 to 255) and NN");
                registers[((current_opcode & 0x0F00) >> 8) as usize] = ((current_opcode & 0x00FF) & thread_rng().gen_range(0, 255)) as u8;
                program_counter = program_counter + 2;
            }
            0xD000 => {
                println!("DXYN: Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N pixels. Each row of 8 pixels is read as bit-coded starting from memory location I; I value doesn’t change after the execution of this instruction. As described above, VF is set to 1 if any screen pixels are flipped from set to unset when the sprite is drawn, and to 0 if that doesn’t happen");
                program_counter = program_counter + 2;
            }
            0x0000 => {
                match current_opcode & 0x000F {
                    0x0004 => {
                        println!("8XY4: Calls subroutine at the address NNN");
                    }
                    _ => {
                        println!("Unknown opcode: {:X}", current_opcode);
                        program_counter = program_counter + 2;
                    }
                }
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
