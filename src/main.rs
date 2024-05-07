use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("--- 몇초로 할까요? (setting seconds) --- ");
    let mut target_seconds = String::new();
    io::stdin()
        .read_line(&mut target_seconds)
        .expect("Failed to read line");
    let mut seconds_remaining: i32 = target_seconds.trim().parse().expect("Input not an integer");

    let spinner_chars = ['/', '-', '\\', '|']; // Characters for the spinner
    let total_length = 100; // Total length of the progress bar

    let mut progress = 0; // Tracks installation progress

    // Iterate from 0 to 100
    for i in 0..=100 {
        // Calculate percentage and progress bar length
        let percentage = i;
        let progress_length = (i * total_length) / 100;

        // Create progress bar string with `progress_length` hyphens
        let progress_bar = "-".repeat(progress_length as usize);

        // Get the current spinner character
        let spinner_char = spinner_chars[progress % 4];

        // Combine progress bar and remaining time in one line
        print!(
            "\r{}% install~ 방송 준비 중.- 방송 시작: {} 초(sec) 전 Rust코딩.~{} {}",
            percentage, seconds_remaining, progress_bar, spinner_char
        );

        // Flush the output buffer to immediately display the progress bar
        io::stdout().flush().unwrap();

        // Decrement seconds remaining (no need for sleep inside the loop)
        seconds_remaining -= 1;

        // Increment progress counter
        progress += 1;

        // Sleep for one second after each iteration
        sleep(Duration::from_secs(1));
    }

    // After the loop, print the final completion message
    println!("\r100% It's all installed. 방송 시작하겠습니다.");
    println!("\r100% It's all installed. 방송 시작하겠습니다.");
    println!("\r100% It's all installed. 방송 시작하겠습니다.");
    println!("\r100% It's all installed. 방송 시작하겠습니다.");
}
