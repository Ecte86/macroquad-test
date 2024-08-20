use std::time::{Duration, Instant};

use anyhow::Ok as AnyOk;
use anyhow::Result as AnyResult;
use macroquad::prelude::*;

/// A test of macroquad's `draw_text` function.
#[macroquad::main("Text Test")]
async fn main() -> AnyResult<()> {
    // How long to wait between updating the text.
    let period = Duration::from_millis(250);
    // Run the text display and wait for it to finish.
    let return_result: AnyResult<()> = run("Hi there!".into(), period).await;
    // Report any errors.
    match return_result {
        Ok(_) => Ok(()),
        Err(err) => {
            println!("Error: {:?}", err);
            Err(err)
        }
    }
}

/// Execute the main loop of the program.
async fn run(text_to_display: String, update_text_time: Duration) -> AnyResult<()> {
    let current_character_index = 0;
    let text = text_to_display.to_ascii_lowercase();
    let last_update = Instant::now();

    match main_loop(text, current_character_index, last_update, update_text_time).await {
        Ok(_) => Ok(()),
        Err(err) => {
            println!("Error: {:?}", err);
            Err(err)
        }
    }
}

/// The main loop of the program.
/// Display the given text, updating the text every `text_update_delay`.
async fn main_loop(
    text: String,
    mut current_character_index: usize,
    mut last_update: Instant,
    update_text_time: Duration,
) -> AnyResult<()> {
    loop {
        // Clear the screen.
        clear_background(BLACK);
        // if char is not a letter, skip it
        if !text
            .chars()
            .nth(current_character_index)
            .unwrap()
            .is_alphabetic()
        {
            current_character_index = (current_character_index + 1) % text.len();
        }

        // Draw the text, capitalizing the current character.
        for (i, c) in text.chars().enumerate() {
            let c = if i == current_character_index {
                c.to_uppercase().next().unwrap_or(c)
            } else {
                c
            };
            draw_text(&c.to_string(), 20.0 + 20.0 * i as f32, 20.0, 30.0, WHITE);
        }

        // Update the current character index if the update interval has elapsed.
        if last_update.elapsed() >= update_text_time {
            last_update = Instant::now();
            current_character_index = (current_character_index + 1) % text.len();
        }

        if let Some(value) = handle_key_events().await {
            return value;
        }

        // Await the next frame.
        next_frame().await;
    }
}

async fn handle_key_events() -> Option<Result<(), anyhow::Error>> {
    // Check for the escape key to exit the loop.
    if let Some(key) = get_last_key_pressed() {
        if key == KeyCode::Escape {
            return Some(AnyOk(()));
        }
    }
    None
}
