mod injector;
mod observator;

use std::thread;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use clap::{App, Arg};

use crate::injector::needle;
use crate::observator::watcher;

fn main()
{
    let matches = App::new("procject")
                       .version("0.666")
                       .author("Hrafnskogr <hrafnskogr@pm.me>")
                       .about("Userland EDR PoC")
                       .arg(Arg::with_name("dll")
                                .help("Path of the dll to inject in processes")
                                .short("d")
                                .long("dll")
                                .required(true))
                       .arg(Arg::with_name("proc_filter")
                                .help("Name of procs to observe and inject")
                                .short("p")
                                .long("procs")
                                .multiple(true)
                                .required(true))
                       .get_matches();

    let proc_filter: Vec<&str> = matches.values_of("proc_filter").unwrap().collect();
    
    init_thread_and_launch(matches.value_of("dll").unwrap(), proc_filter);
}

fn init_thread_and_launch(dll_path: &str, proc_filter: Vec<&str>) 
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
    let filter: Vec<String> = proc_filter.iter().map(|&x| String::from(x)).collect();
    let th = thread::spawn(move || { needle::injecter(dll_str, filter, stop, rx); } );
    threads.push(th);

    // Start threads
    for th in threads
    {
        th.join().expect("thread panicked");
    }

    println!("Exiting main");
}


