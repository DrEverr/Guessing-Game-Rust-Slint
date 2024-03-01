slint::include_modules!();
use rand::Rng;
use std::cmp::Ordering;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_start({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let secret_number = rand::thread_rng().gen_range(1..=100);
            ui.set_secret_number(secret_number);
        }
    });

    ui.on_guess({
        let ui_handle = ui.as_weak();
        move |guess| {
            let ui = ui_handle.unwrap();
            let secret_number = ui.get_secret_number();

            let guess: i32 = match guess.parse() {
                Ok(num) => num,
                Err(_) => {
                    ui.set_hint("Invalid number".to_string().into());
                    return;
                }
            };

            match guess.cmp(&secret_number) {
                Ordering::Less => {
                    ui.set_hint("Too small".to_string().into());
                    let lives = ui.get_lives() - 1;
                    ui.set_lives(lives);
                    if lives == 0 {
                        ui.set_is_running(false);
                        ui.set_hint("You lose!".to_string().into());
                    }
                }
                Ordering::Greater => {
                    ui.set_hint("Too big".to_string().into());
                    let lives = ui.get_lives() - 1;
                    ui.set_lives(lives);
                    if lives == 0 {
                        ui.set_is_running(false);
                        ui.set_hint("You lose!".to_string().into());
                    }
                }
                Ordering::Equal => {
                    ui.set_is_running(false);
                    ui.set_hint("You win!".to_string().into());
                }
            }
        }
    });

    ui.run()
}
