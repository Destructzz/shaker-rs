use std::{thread::{self}, time::Duration};
use std::sync::{Arc, Mutex};
use rdev::{listen, Button, Event, EventType};

mod shaker;
use shaker::Shaker;
mod config;
pub use config::Config;
mod errors;

const BAUDE_RATE: u32 = 2_000_000;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let position = config::Config::init()?;
    let mut sh = Shaker::new(BAUDE_RATE);
    sh.init()?;

    // Флаги состояния кнопок
    let left_pressed = Arc::new(Mutex::new(false));
    let right_pressed = Arc::new(Mutex::new(false));
    let both_pressed = Arc::new(Mutex::new(false));

    let left_clone = left_pressed.clone();
    let right_clone = right_pressed.clone();
    let both_clone = both_pressed.clone();

    // Запускаем слушатель в отдельном потоке
    thread::spawn(move || {
        if let Err(error) = listen(move |event: Event| {
            match event.event_type {
                EventType::ButtonPress(Button::Left) => {
                    *left_clone.lock().unwrap() = true;
                    check_both(&left_clone, &right_clone, &both_clone);
                }
                EventType::ButtonRelease(Button::Left) => {
                    *left_clone.lock().unwrap() = false;
                    *both_clone.lock().unwrap() = false;
                }
                EventType::ButtonPress(Button::Right) => {
                    *right_clone.lock().unwrap() = true;
                    check_both(&left_clone, &right_clone, &both_clone);
                }
                EventType::ButtonRelease(Button::Right) => {
                    *right_clone.lock().unwrap() = false;
                    *both_clone.lock().unwrap() = false;
                }
                _ => {}
            }
        }) {
            eprintln!("Ошибка слушателя: {:?}", error);
        }
    });

    // Основной цикл
    loop {
        let is_both = *both_pressed.lock().unwrap();
        
        if is_both {
            if let Err(e) = sh.process(-position.x, position.y) {
                println!("Something happened: {:?}", e);
            }
            if let Err(e) = sh.process(position.x, -position.y) {
                println!("Something happened: {:?}", e);
            }
        }
        
        thread::sleep(Duration::from_millis(25));
    }
}

fn check_both(
    left: &Arc<Mutex<bool>>, 
    right: &Arc<Mutex<bool>>, 
    both: &Arc<Mutex<bool>>
) {
    let left_val = *left.lock().unwrap();
    let right_val = *right.lock().unwrap();
    
    if left_val && right_val {
        *both.lock().unwrap() = true;
        println!("Обе кнопки нажаты!");
    }
}
