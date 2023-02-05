use evdev_rs::Device;
use evdev_rs::ReadFlag;

fn main() {
    let d = Device::new_from_path("/dev/input/event4").unwrap();

    loop {
        let ev = d.next_event(ReadFlag::NORMAL).map(|val| val.1);
        match ev {
            Ok(ev) => println!("Event: time {}.{}, {} {}",
                              ev.time.tv_sec,
                              ev.time.tv_usec,
                              ev.event_type().map(|ev_type| format!("{}", ev_type)).unwrap_or("".to_owned()),
                              ev.value),
            Err(_e) => (),
        }
    }
}