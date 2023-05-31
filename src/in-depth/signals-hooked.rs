use signal_hook::{consts::SIGINT, iterator::Signals};
use std::{error::Error, thread, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
    let mut signals = Signals::new(&[SIGINT])?;

    thread::spawn(move || {
        for sig in signals.forever() {
            println!("Received signal {:?}", sig);
        }
    });

    // 다음 코드는 실제로 동작하며, Ctrl-C로 인터럽트될 수 있다.
    // 이 예시에서는 몇 초간 기다린다.
    thread::sleep(Duration::from_secs(2));

    Ok(())
}
