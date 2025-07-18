// syscall.rs
use crate::functional_simulator::memory::Memory;
use crate::functional_simulator::registers::Registers;
use log::{error, info};
use std::io::Read;

/// Handles MIPS system calls using the ABI conventions.
/// Returns Some(address) for branches/jumps, None for regular execution.
pub fn handle_syscall(registers: &mut Registers, memory: &mut Memory) -> Option<u32> {
    let syscall_num = registers.read(2); // v0 contains syscall number

    match syscall_num {
        1 => {
            // print_int: Print integer in $a0
            let value = registers.read(4);
            println!("{}", value as i32);
            None
        },
        2 => {
            // print_float: Print float in $f12
            let value = registers.read_float(12);
            println!("{}", value);
            None
        },
        3 => {
            // print_double: Print double in $f12 (treated as float for simplicity)
            let value = registers.read_float(12);
            println!("{}", value);
            None
        },
        4 => {
            // print_string: Print null-terminated string at address in $a0
            let addr = registers.read(4) as usize;
            let mut string = String::new();
            let mut current = addr;

            // Read bytes until null terminator or memory boundary
            while let Some(byte) = memory.read_byte(current) {
                if byte == 0 {
                    break;
                }
                string.push(byte as char);
                current += 1;
            }

            println!("{}", string);
            None
        },
        5 => {
            // read_int: Read integer from stdin into $v0
            let mut input = String::new();
            if let Err(e) = std::io::stdin().read_line(&mut input) {
                error!("Failed to read from stdin: {}", e);
                return None;
            }

            let value = input.trim().parse::<i32>().unwrap_or(0);
            registers.write(2, value as u32);
            None
        },
        6 => {
            // read_float: Read float from stdin into $f0
            let mut input = String::new();
            if let Err(e) = std::io::stdin().read_line(&mut input) {
                error!("Failed to read from stdin: {}", e);
                return None;
            }

            let value = input.trim().parse::<f32>().unwrap_or(0.0);
            registers.write_float(0, value);
            None
        },
        7 => {
            // read_double: Read double from stdin into $f0 (treated as float)
            let mut input = String::new();
            if let Err(e) = std::io::stdin().read_line(&mut input) {
                error!("Failed to read from stdin: {}", e);
                return None;
            }

            let value = input.trim().parse::<f32>().unwrap_or(0.0);
            registers.write_float(0, value);
            None
        },
        8 => {
            // read_string: Read string from stdin into memory at address in $a0
            let addr = registers.read(4) as usize;
            let max_length = registers.read(5) as usize;

            let mut input = String::new();
            if let Err(e) = std::io::stdin().read_line(&mut input) {
                error!("Failed to read from stdin: {}", e);
                return None;
            }

            // Truncate input to max_length-1 (leave room for null terminator)
            let input = if input.len() > max_length - 1 {
                input[..max_length - 1].to_string()
            } else {
                input
            };

            // Write string to memory
            for (i, byte) in input.bytes().enumerate() {
                if i >= max_length - 1 {
                    break;
                }
                memory.write_byte(addr + i, byte);
            }

            // Add null terminator
            memory.write_byte(addr + std::cmp::min(input.len(), max_length - 1), 0);
            None
        },
        9 => {
            // sbrk: Allocate heap memory
            // This is a simplified implementation that just returns a pointer
            // to memory past the current program
            let amount = registers.read(4) as usize;
            let current_heap_end = memory.heap_end();

            if current_heap_end + amount < memory.size {
                let new_heap_end = current_heap_end + amount;
                memory.set_heap_end(new_heap_end);
                registers.write(2, current_heap_end as u32);
            } else {
                // Out of memory
                registers.write(2, 0);
            }
            None
        },
        10 => {
            // exit: End program
            Some(0xFFFFFFFF) // Special value to indicate program termination
        },
        11 => {
            // print_char: Print character in $a0
            let value = registers.read(4) as u8;
            print!("{}", value as char);
            None
        },
        12 => {
            // read_char: Read character from stdin into $v0
            let mut buffer = [0; 1];
            if let Err(e) = std::io::stdin().read_exact(&mut buffer) {
                error!("Failed to read character from stdin: {}", e);
                registers.write(2, 0);
                return None;
            }
            registers.write(2, buffer[0] as u32);
            None
        },
        13 => {
            // open: Open file
            let filename_addr = registers.read(4) as usize;
            let flags = registers.read(5);
            let mode = registers.read(6);

            // Read filename from memory as null-terminated string
            let mut filename = String::new();
            let mut current = filename_addr;

            while let Some(byte) = memory.read_byte(current) {
                if byte == 0 {
                    break;
                }
                filename.push(byte as char);
                current += 1;
            }

            info!(
                "Syscall 13 (open): Opening file '{}' with flags {}, mode {}",
                filename, flags, mode
            );

            // For now, return a dummy file descriptor
            // In a real implementation, this would maintain a file descriptor table
            registers.write(2, 3); // Return fd 3 (after stdin, stdout, stderr)
            None
        },
        14 => {
            // read: Read from file
            let fd = registers.read(4);
            let buffer_addr = registers.read(5) as usize;
            let count = registers.read(6) as usize;

            info!(
                "Syscall 14 (read): Reading {} bytes from fd {} into address 0x{:x}",
                count, fd, buffer_addr
            );

            // Simulate successful read with random data for testing
            // In a real implementation, this would read from the actual file
            let bytes_read = count.min(128); // Simulate reading at most 128 bytes

            for i in 0..bytes_read {
                // Fill with placeholder data
                memory.write_byte(buffer_addr + i, (i % 256) as u8);
            }

            registers.write(2, bytes_read as u32); // Return number of bytes read
            None
        },
        15 => {
            // write: Write to file
            let fd = registers.read(4);
            let buffer_addr = registers.read(5) as usize;
            let count = registers.read(6) as usize;

            info!(
                "Syscall 15 (write): Writing {} bytes from address 0x{:x} to fd {}",
                count, buffer_addr, fd
            );

            // For console output (fd=1), actually print to console
            if fd == 1 || fd == 2 {
                // stdout or stderr
                let mut output = String::new();
                for i in 0..count {
                    if let Some(byte) = memory.read_byte(buffer_addr + i) {
                        output.push(byte as char);
                    }
                }
                print!("{}", output);
            }

            // Simulate successful write
            registers.write(2, count as u32); // Return number of bytes written
            None
        },
        16 => {
            // close: Close file
            let fd = registers.read(4);

            info!("Syscall 16 (close): Closing fd {}", fd);

            // Simulate successful close
            registers.write(2, 0); // Success
            None
        },
        17 => {
            // exit2: Exit with return value
            let return_code = registers.read(4);
            println!("Program terminated with exit code {}", return_code);
            Some(0xFFFFFFFF) // Special value to indicate program termination
        },
        30 => {
            // Syscall 30: Get system time
            use std::time::{SystemTime, UNIX_EPOCH};
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis() as u32;

            registers.write(2, now);
            None
        },
        31 => {
            // Syscall 31: Sleep for milliseconds in $a0
            let ms = registers.read(4);
            std::thread::sleep(std::time::Duration::from_millis(ms as u64));
            None
        },
        34 => {
            // print_hex: Print integer in $a0 as hex
            let value = registers.read(4);
            println!("0x{:x}", value);
            None
        },
        35 => {
            // print_bin: Print integer in $a0 as binary
            let value = registers.read(4);
            println!("0b{:b}", value);
            None
        },
        36 => {
            // print_uint: Print integer in $a0 as unsigned
            let value = registers.read(4);
            println!("{}", value);
            None
        },
        // Add more syscalls as needed
        _ => {
            println!("Unimplemented syscall: {}", syscall_num);
            None
        },
    }
}
