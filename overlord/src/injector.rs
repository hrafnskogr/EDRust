pub mod needle
{
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    use std::time;
    use std::sync::mpsc::Receiver;

    extern "system"
    {
        fn OpenProcess(dwDesiredAccess: u32,
                       bInheritHandle: bool,
                       dwProcessId: u32) -> usize;

        fn VirtualAllocEx(hProcess: usize,
                          lpAddress: *const usize,
                          dwSize: u32,
                          flAllocationType: u32,
                          flProtect: u32) -> usize;

        fn WriteProcessMemory(hProcess: usize,
                              lpBaseAddress: *const usize,
                              lpBuffer: *const u8,
                              nSize: usize,
                              lpNumberOfBytesWritten: *mut u32) -> bool;

        fn CreateRemoteThread(hProcess: usize,
                              lpSecurityAttributes: usize,
                              dwStackSize: usize,
                              lpStartAddress: *const usize,
                              lpParameter: *const usize,
                              dwCreationFlag: u32,
                              lpThreadId: u32);

        fn LoadLibraryA(lpLibFileName: *const u8) -> usize;
        fn GetProcAddress(hModule: *const usize, lpProcName: *const u8) -> usize;
        fn GetModuleHandleA(lpModuleName: *const u8) -> usize;  
    }

    const PROCESS_ALL_ACCESS:       u32     = 0x001F0FFF;
    const MEM_COMMIT:               u32     = 0x00001000;
    const MEM_RESERVED:             u32     = 0x00002000;
    const PAGE_EXECUTE_READWRITE:   u32     = 0x40;

    pub fn injecter(dll_path: String, proc_filter: Vec<String>, stop: Arc<AtomicBool>, rx: Receiver<(String, usize)>)
    {
        while !stop.load(Ordering::SeqCst)
        {
            match rx.recv_timeout(time::Duration::from_millis(10))
            {
                Ok((proc_name, pid)) => 
                {
                    println!("Watching {}", proc_name); 
                    if proc_name == "tester.exe"
                    {
                        inject(pid, &dll_path);
                        println!("Injecting process: {} | pid: {}", proc_name, pid)
                    }
                },
                Err(_) => (),
            }
        }

        println!("Exiting injector");
    }

    fn inject(pid: usize, dll_path: &String)
    {
        unsafe
        {
            let mod_h = GetModuleHandleA("kernel32.dll\0".as_ptr());
            let load_lib_addr = GetProcAddress(mod_h as *const usize, "LoadLibraryA\0".as_ptr());

            println!("LoadLibraryA Addr: {:#x}", load_lib_addr);

            let proc_handle = OpenProcess(PROCESS_ALL_ACCESS, true, pid as u32);
            let mem_addr = VirtualAllocEx(proc_handle, 0x0 as *const usize, dll_path.len() as u32, MEM_COMMIT | MEM_RESERVED, PAGE_EXECUTE_READWRITE);
   
            let mut bytes_written: u32 = 0;
            WriteProcessMemory(proc_handle, mem_addr as *const usize, dll_path.as_bytes().as_ptr(), dll_path.len(), &mut bytes_written);
            
            CreateRemoteThread(proc_handle, 0x0, 0x0, load_lib_addr as *const usize, mem_addr as *const usize, 0x0, 0x0);
        }
    }
}
