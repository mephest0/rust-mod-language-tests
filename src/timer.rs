use std::time::SystemTime;

static mut STARTED_TIME: SystemTime = SystemTime::UNIX_EPOCH;
static mut IS_STARTED: bool = false;

pub fn start() {
    unsafe {
        STARTED_TIME = SystemTime::now();
        IS_STARTED = true;
    }
}

pub fn stop(message: &str) -> i32 {
    unsafe {
        if !IS_STARTED {
            println!("lol - stopping un-started timer");

            return -1;
        }
    }

    let now = SystemTime::now();

    unsafe {
        let dt = now
            .duration_since(STARTED_TIME)
            .expect("wtf 4981276")
            .as_millis();

        IS_STARTED = false;
        println!("{} | {:?}ms",
                 message,
                 dt);

        dt as i32
    }
}
