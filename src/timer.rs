use std::time::SystemTime;

static mut STARTED_TIME: SystemTime = SystemTime::UNIX_EPOCH;
static mut IS_STARTED: bool = false;

pub fn start() {
    unsafe {
        STARTED_TIME = SystemTime::now();
        IS_STARTED = true;
    }
}

pub fn stop(message: &str) {
    unsafe {
        if !IS_STARTED {
            println!("lol - stopping un-started timer");

            return;
        }
    }

    let now = SystemTime::now();

    unsafe {
        IS_STARTED = false;
        println!("{} | {:?}ms",
                 message,
                 now
                     .duration_since(STARTED_TIME)
                     .expect("wtf 4981276")
                     .as_millis());
    }
}
