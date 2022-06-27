use rdev::{grab, Event, EventType, Key, EventAction};

fn main() {
    // This will block.
    if let Err(error) = grab(callback) {
        println!("Error: {:?}", error)
    }
}

fn callback(event: Event) -> EventAction {
    println!("My callback {:?}", event);
    match event.event_type {
        EventType::KeyPress(Key::Tab) => {
            println!("Cancelling tab !");
            EventAction::Drop
        }
        _ => EventAction::Accept
    }
}
