// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the thread spawned on line 19 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. If you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :)

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let data = Arc::new(Mutex::new(JobStatus{jobs_completed: 0}));

    {
        let workerdata = data.clone();
        thread::spawn(move || {
            for i in 0..10 {
                println!("Iteration:{}, Sleeping...", i);
                thread::sleep(Duration::from_millis(250));
                println!("Iteration:{}, Woke...", i);

                let mut data = workerdata.lock().unwrap();
                data.jobs_completed += 1;

                println!("Iteration:{}, Completed", i);
            }
        });
    }

    let mut m: u8 = 0;
    while data.lock().unwrap().jobs_completed < 10 {
        m += 1;
        println!("{} Monitor Waiting ... ", m);
        thread::sleep(Duration::from_millis(500));
    }
}
