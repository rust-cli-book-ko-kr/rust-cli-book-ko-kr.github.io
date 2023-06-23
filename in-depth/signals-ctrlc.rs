use std::{thread, time::Duration};

fn main() {
    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
    })
    .expect("Error setting Ctrl-C handler");

    // 다음 코드는 실제로 동작하며, Ctrl-C로 인터럽트될 수 있다.
    // 이 예시에서는 몇 초간 기다린다.
    thread::sleep(Duration::from_secs(2));
}
