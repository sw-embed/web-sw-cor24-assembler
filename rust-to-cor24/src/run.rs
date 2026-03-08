//! cor24-run: COR24 assembler and emulator CLI
//!
//! Usage:
//!   cor24-run --demo                              Run built-in LED demo
//!   cor24-run --demo --speed 50000 --time 10      Run at 50k IPS for 10 seconds
//!   cor24-run --run <file.s>                      Assemble and run
//!   cor24-run --assemble <in.s> <out.bin> <out.lst>  Assemble to binary + listing

use cor24_emulator::assembler::Assembler;
use cor24_emulator::emulator::EmulatorCore;
use std::env;
use std::fs;
use std::io::Write;
use std::thread;
use std::time::{Duration, Instant};

/// Default emulation speed (instructions per second)
const DEFAULT_SPEED: u64 = 100_000;

/// Default time limit in seconds
const DEFAULT_TIME_LIMIT: f64 = 10.0;

fn print_leds(leds: u8) {
    print!("\rLEDs: ");
    for i in (0..8).rev() {
        if (leds >> i) & 1 == 1 { print!("\x1b[91m●\x1b[0m"); }
        else { print!("○"); }
    }
    print!("  (0x{:02X})  ", leds);
    std::io::stdout().flush().ok();
}

/// Run emulator with timing control
fn run_with_timing(emu: &mut EmulatorCore, speed: u64, time_limit: f64) -> u64 {
    let start = Instant::now();
    let time_limit_duration = Duration::from_secs_f64(time_limit);

    let batch_size: u64 = if speed == 0 { 10000 } else { (speed / 100).max(1) };
    let batch_duration = if speed == 0 {
        Duration::ZERO
    } else {
        Duration::from_secs_f64(batch_size as f64 / speed as f64)
    };

    let mut total_instructions: u64 = 0;
    let mut batch_start = Instant::now();
    let mut prev_led = emu.get_led();

    emu.resume();

    loop {
        if start.elapsed() >= time_limit_duration {
            break;
        }

        let result = emu.run_batch(batch_size);
        total_instructions += result.instructions_run;

        // Check for LED changes
        let led = emu.get_led();
        if led != prev_led {
            print_leds(led);
            prev_led = led;
        }

        // Print any UART output
        let output = emu.get_uart_output();
        if !output.is_empty() {
            // EmulatorCore accumulates; we print new chars
            // For simplicity, just track that we printed
        }

        if result.instructions_run == 0 {
            break; // halted or paused
        }

        if speed > 0 {
            let elapsed = batch_start.elapsed();
            if elapsed < batch_duration {
                thread::sleep(batch_duration - elapsed);
            }
            batch_start = Instant::now();
        }
    }

    total_instructions
}

/// Load assembled bytes into emulator at their correct addresses
fn load_assembled(emu: &mut EmulatorCore, result: &cor24_emulator::assembler::AssemblyResult) {
    for line in &result.lines {
        if !line.bytes.is_empty() {
            for (i, &b) in line.bytes.iter().enumerate() {
                emu.write_byte(line.address + i as u32, b);
            }
        }
    }
}

/// LED counter demo with spin loop delay
const DEMO_SOURCE: &str = r#"
; LED Counter Demo with Spin Loop Delay
; Counts 0-255 on LEDs, loops forever

        push    fp
        mov     fp, sp
        add     sp, -3

        la      r1, -65536
        lc      r0, 0
        sw      r0, 0(fp)

main_loop:
        lw      r0, 0(fp)
        sb      r0, 0(r1)

        la      r2, 15000
delay:
        lc      r0, 1
        sub     r2, r0
        brt     delay

        lw      r0, 0(fp)
        lc      r2, 1
        add     r0, r2
        sw      r0, 0(fp)

        bra     main_loop
"#;

fn parse_args() -> (String, u64, f64, Option<String>) {
    let args: Vec<String> = env::args().collect();
    let mut command = String::new();
    let mut speed = DEFAULT_SPEED;
    let mut time_limit = DEFAULT_TIME_LIMIT;
    let mut file = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--demo" => command = "demo".to_string(),
            "--run" => {
                command = "run".to_string();
                if i + 1 < args.len() {
                    file = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "--assemble" => {
                command = "assemble".to_string();
            }
            "--speed" | "-s" => {
                if i + 1 < args.len() {
                    speed = args[i + 1].parse().unwrap_or(DEFAULT_SPEED);
                    i += 1;
                }
            }
            "--time" | "-t" => {
                if i + 1 < args.len() {
                    time_limit = args[i + 1].parse().unwrap_or(DEFAULT_TIME_LIMIT);
                    i += 1;
                }
            }
            _ => {
                if command.is_empty() && !args[i].starts_with('-') {
                    file = Some(args[i].clone());
                }
            }
        }
        i += 1;
    }

    (command, speed, time_limit, file)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("cor24-run: COR24 assembler and emulator\n");
        println!("Usage:");
        println!("  cor24-run --demo [options]        Run built-in LED demo");
        println!("  cor24-run --run <file.s> [opts]   Assemble and run");
        println!("  cor24-run --assemble <in.s> <out.bin> <out.lst>");
        println!();
        println!("Options:");
        println!("  --speed, -s <ips>    Instructions per second (default: {})", DEFAULT_SPEED);
        println!("  --time, -t <secs>    Time limit in seconds (default: {})", DEFAULT_TIME_LIMIT);
        println!();
        println!("Example:");
        println!("  cor24-run --demo --speed 100000 --time 10");
        return;
    }

    let (command, speed, time_limit, file) = parse_args();

    match command.as_str() {
        "demo" => {
            println!("=== COR24 LED Demo ===\n");
            println!("Binary counter 0-255 on LEDs with spin loop delay");
            println!("Speed: {} instructions/sec, Time limit: {}s\n", speed, time_limit);

            let mut asm = Assembler::new();
            let result = asm.assemble(DEMO_SOURCE);
            if !result.errors.is_empty() {
                eprintln!("Assembly error: {}", result.errors.join("\n"));
                return;
            }

            println!("Program listing:");
            for line in &result.lines {
                if !line.bytes.is_empty() {
                    let bytes: String = line.bytes.iter().map(|b| format!("{:02X} ", b)).collect();
                    println!("{:04X}: {:14} {}", line.address, bytes.trim(), line.source);
                }
            }
            println!();

            let mut emu = EmulatorCore::new();
            load_assembled(&mut emu, &result);

            println!("Running (Ctrl+C to stop)...\n");
            let instructions = run_with_timing(&mut emu, speed, time_limit);

            println!("\n\nExecuted {} instructions in {:.1}s", instructions, time_limit);
            println!("Effective speed: {:.0} IPS", instructions as f64 / time_limit);
        }

        "run" => {
            let filename = match file {
                Some(f) => f,
                None => {
                    eprintln!("Usage: cor24-run --run <file.s>");
                    return;
                }
            };

            let source = fs::read_to_string(&filename).expect("Cannot read file");
            let mut asm = Assembler::new();
            let result = asm.assemble(&source);
            if !result.errors.is_empty() {
                eprintln!("Assembly error: {}", result.errors.join("\n"));
                return;
            }

            let byte_count: usize = result.lines.iter().map(|l| l.bytes.len()).sum();
            println!("Assembled {} bytes\n", byte_count);
            println!("Listing:");
            for line in &result.lines {
                if !line.bytes.is_empty() {
                    let bytes: String = line.bytes.iter().map(|b| format!("{:02X} ", b)).collect();
                    println!("{:04X}: {:14} {}", line.address, bytes.trim(), line.source);
                }
            }
            println!("\nRunning (speed: {} IPS, time limit: {}s)...\n", speed, time_limit);

            let mut emu = EmulatorCore::new();
            load_assembled(&mut emu, &result);

            let instructions = run_with_timing(&mut emu, speed, time_limit);

            // Print UART output if any
            let uart = emu.get_uart_output();
            if !uart.is_empty() {
                println!("\nUART output: {}", uart);
            }

            println!("\nExecuted {} instructions", instructions);
        }

        "assemble" => {
            if args.len() < 5 {
                eprintln!("Usage: cor24-run --assemble <in.s> <out.bin> <out.lst>");
                return;
            }
            let source = fs::read_to_string(&args[2]).expect("Cannot read file");
            let mut asm = Assembler::new();
            let result = asm.assemble(&source);
            if !result.errors.is_empty() {
                eprintln!("Assembly error: {}", result.errors.join("\n"));
                return;
            }

            // Collect machine code bytes
            let machine_code: Vec<u8> = result.lines.iter()
                .flat_map(|line| line.bytes.iter().copied())
                .collect();

            fs::write(&args[3], &machine_code).expect("Cannot write .bin");
            let mut lst_file = fs::File::create(&args[4]).expect("Cannot write .lst");
            for line in &result.lines {
                if !line.bytes.is_empty() {
                    let bytes: String = line.bytes.iter().map(|b| format!("{:02X} ", b)).collect();
                    writeln!(lst_file, "{:04X}: {:14} {}", line.address, bytes.trim(), line.source).ok();
                } else if !line.source.is_empty() {
                    writeln!(lst_file, "                    {}", line.source).ok();
                }
            }
            println!("Wrote {} bytes to {}", machine_code.len(), args[3]);
            println!("Wrote listing to {}", args[4]);
        }

        _ => {
            eprintln!("Unknown command. Use --demo, --run, or --assemble");
        }
    }
}
