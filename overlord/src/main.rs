mod injector;
mod observator;

use std::thread;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::injector::needle;
use crate::observator::watcher;

use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about = "-- Userland EDR PoC --", long_about = None)]
struct Args
{
    /// Path of the dll to be injected in the processes
    #[clap(short, long)]
    dll: String,

    /// [Multiple] Name of the processes (including file extension) to be observed and injected.
    /// Example : WindowsTerminal.exe
    #[clap(short='p', long="procs")]
    proc_filter: Vec<String>,
}


fn main()
{
    let args = Args::parse();
    init_thread_and_launch(args.dll, args.proc_filter);
}

fn init_thread_and_launch(dll_path: String, proc_filter: Vec<String>) 
{
    // Define an atomic bool as a flag to know whether to stop or not
    let should_stop = Arc::new(AtomicBool::new(false));
    let stop = should_stop.clone();

    // Define a custom handler for the ctr-c signal
    ctrlc::set_handler(move || {
        stop.store(true, Ordering::SeqCst);
    }).expect("Error overloading Ctrl-C");

    // Define a channel for communication between the observer and the injector
    let (tx, rx): (Sender<(String, usize)>, Receiver<(String, usize)>) = mpsc::channel();

    // Init thread list
    let mut threads = Vec::new();

    // Init and spawn the observer thread
    let stop = should_stop.clone();
    let obs_tx = tx.clone();
    let th = thread::spawn(move || { watcher::process_monitor(stop, obs_tx); } );
    threads.push(th);

    // Init and spawn the injector thread
    let stop = should_stop.clone();
    let dll_str = String::from(dll_path);
    let filter: Vec<String> = proc_filter.clone(); 
    let th = thread::spawn(move || { needle::injecter(dll_str, filter, stop, rx); } );
    threads.push(th);

    // Start threads
    for th in threads
    {
        th.join().expect("thread panicked");
    }

    println!("Exiting main");
}


