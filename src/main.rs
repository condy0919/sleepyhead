extern crate sleepyhead;

use sleepyhead::scheduler;
use sleepyhead::io::monitor;

fn main() {
    let mut iomon = monitor::Monitor::with_size(1024);
    let mut sched = scheduler::Scheduler::new(0, &mut iomon);

    for i in 0..100 {
        sched.spawn(move || {
            println!("i = {}", i);
        });
    }

    sched.mainloop();
}
