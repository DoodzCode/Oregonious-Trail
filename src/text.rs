use std::process::{Command, Child};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    
    let process_handle = Arc::new(Mutex::new(None::<Child>));

    // Thread to create the process
    let process_handle_clone = Arc::clone(&process_handle);
    let create_thread = thread::spawn(move || {
        let process = Command::new("sleep")
            .arg("5")
            .spawn()
            .expect("Failed to start process");

        let mut handle = process_handle_clone.lock().unwrap();
        *handle = Some(process);
    });

    // Give some time for the process to start
    thread::sleep(Duration::from_secs(1));

    // Thread to use the process
    let use_thread = thread::spawn(move || {
        let handle = process_handle.lock().unwrap();
        if let Some(ref mut process) = *handle {
            println!("Waiting for the process to finish...");
            let _ = process.wait().expect("Failed to wait on process");
            println!("Process finished");
        }
    });

    create_thread.join().expect("Failed to join create_thread");
    use_thread.join().expect("Failed to join use_thread");
}


/*

In this example:

The main thread creates an Arc<Mutex<Option<Child>>> to share the process handle safely between threads.

The create_thread spawns a new process (sleep 5 command) and stores the process handle in the shared Arc<Mutex>.

The use_thread waits for the process to complete using the handle stored in the shared Arc<Mutex>.

This is a basic example to demonstrate the concept. Depending on your specific needs, you might need to add error handling and other logic.

Let me know if you have any questions or need further assistance!

*/