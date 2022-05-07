pub mod watcher
{
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::mpsc::Sender;
    use std::sync::Arc;
    use std::{time, thread};
    use std::process::{Command, Stdio};
    use std::io::Read;

    pub fn process_monitor(stop: Arc<AtomicBool>, tx: Sender<(String, usize)>)
    {
        let mut proc = Command::new("bin/proc_start_mon.exe")
                            .stdout(Stdio::piped())
                            .spawn()
                            .expect("Failed to spwan observer child process");

        loop
        {
            if stop.load(Ordering::SeqCst)
            {
                break;
            }

            thread::sleep(time::Duration::from_millis(10));
          
            // Observe command output
            // And send it to the thread responsible for injection
            let mut buf = [0;128];
            let out = proc.stdout.as_mut().unwrap(); 
            match out.read(&mut buf)
            {
                Ok(n) => 
                {
                    let proc_output = String::from_utf8_lossy(&buf[..n]);
                    let proc_infos: Vec<&str> = proc_output.split("\r\n").collect();

                    for proc_info in proc_infos
                    {
                        let infos: Vec<&str> = proc_info.split("|").collect();

                        if infos.len() == 2
                        {
                            match infos[1].parse()
                            {
                                Ok(x) => tx.send((String::from(infos[0]), x)).unwrap(),
                                Err(_) => (),
                            }
                        }
                    }
                },
                Err(_) => (),
            }
        }

        println!("Exiting Observer");
    }
}
