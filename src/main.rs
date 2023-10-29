use enigo::*;
use std::sync::Arc;
use std::sync::Mutex;

#[cfg(feature = "unstable_grab")]
use rdev::{grab, Event, EventType, Key};

const LEFT_START: i32 = 1296;
const UP_START: i32 = 614;
const CELL_SIZE: i32 = 53;
pub const VK_F3: u32 = 0x72;
pub const VK_F4: u32 = 0x73;

fn main() {
    println!("running clicker-rs...");
    let enigo = Enigo::new();
    let enigo_builder: Arc<Mutex<Enigo>> = Arc::new(Mutex::new(enigo));
    let furz = enigo_builder.clone();

    let do_shit2 = move || {
        let mut enigo = furz.lock().unwrap();
        println!("Hotkey Triggered!");
        let old_pos = enigo.mouse_location();

        if (1260, 580) > old_pos || old_pos > (1920, 890) {
            println!("Cursor out of safe range.. Ignoring");
            return;
        }
        enigo.mouse_move_to(LEFT_START, UP_START);
        std::thread::sleep(core::time::Duration::from_millis(50));
        enigo.key_down(Key::Control);
        for i in 0..12 {
            for j in 0..5 {
                enigo.mouse_click(MouseButton::Left);
                std::thread::sleep(core::time::Duration::from_millis(10));
                enigo.mouse_move_to(LEFT_START + CELL_SIZE * i, UP_START + CELL_SIZE * j);
                std::thread::sleep(core::time::Duration::from_millis(30));
                let new_pos = enigo.mouse_location();

                if new_pos.0 + (CELL_SIZE / 2) > old_pos.0
                    && new_pos.1 + (CELL_SIZE / 2) > old_pos.1
                {
                    enigo.mouse_click(MouseButton::Left);
                    std::thread::sleep(core::time::Duration::from_millis(10));
                    enigo.key_up(Key::Control);
                    println!("Hotkey DONE!");
                    enigo.mouse_move_to(old_pos.0, old_pos.1);
                    return;
                }
            }
        }
    };

    let mut hk = hotkey::Listener::new();
    hk.register_hotkey(0, VK_F4, do_shit2).unwrap();

    let furz = enigo_builder.clone();
    hk.register_hotkey(0, VK_F3, move || {
        let enigo = furz.lock().unwrap();
        println!("{:?}", enigo.mouse_location());
    })
    .unwrap();

    ctrlc::set_handler(move || {
        println!("bye bye");
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    hk.listen();

    // #[cfg(feature = "unstable_grab")]
    // let callback = |event: Event| -> Option<Event> {
    //     if let EventType::KeyPress(Key::CapsLock) = event.event_type {
    //         println!("Consuming and cancelling CapsLock");
    //         None // CapsLock is now effectively disabled
    //     } else {
    //         Some(event)
    //     }
    // };
    // // This will block.
    // #[cfg(feature = "unstable_grab")]
    // if let Err(error) = grab(callback) {
    //     println!("Error: {:?}", error)
    // }
}
