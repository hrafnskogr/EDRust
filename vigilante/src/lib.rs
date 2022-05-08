#![cfg(windows)]

use winapi::shared::minwindef;
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID};

use detour::static_detour;

use std::mem;
use std::error::Error;

#[link(name = "User32")]
extern "system"
{
    fn LoadLibraryA(lpLibFilename: *const u8) -> usize;
    fn GetProcAddress(hModule: usize, lpProcName: *const u8) -> usize;

    fn MessageBoxA(hwnd: *const usize, caption: *const u8, title: *const u8, flags: usize) -> usize;
}

static_detour!
{
    static OpenProcessHook: unsafe extern "system" fn(u32, bool, u32) -> usize;
}

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(dll_module: HINSTANCE, call_reason: DWORD, reserved: LPVOID) -> BOOL
{
    const DLL_PROCESS_ATTACH: DWORD = 1;
    const DLL_PROCESS_DETACH: DWORD = 0;

    match call_reason
    {
        DLL_PROCESS_ATTACH => 
        {
            match init()
            {
                Ok(_) => (),
                Err(x) =>
                {
                    println!("Error : {:?}", x);
                    ()
                },
            }
        },
        DLL_PROCESS_DETACH => (),
        _ => (),
    }

    minwindef::TRUE
}

type OpenProc = unsafe extern "system" fn(u32, bool, u32) -> usize;

fn init() -> Result<(), Box<dyn Error>>
{
    unsafe
    {
        let kernel32_dll: usize = LoadLibraryA("kernel32\0".as_ptr());
        let open_process_addr: usize = GetProcAddress(kernel32_dll, "OpenProcess\0".as_ptr());

        let func: OpenProc = mem::transmute(open_process_addr);
        OpenProcessHook.initialize(func, pipo)?
                       .enable()?;
        
        Ok(())

/*
 * Reminder: 
        let func = &msg_box_addr as *const usize
                                 as *const fn(*const usize, *const u8, *const u8, usize) -> usize;
        (*func)(0x0 as *const usize, "Injected !\0".as_ptr(), "Title\0".as_ptr(), 0);

        Ok(())
*/
    }
}

#[allow(unused_variables)]
fn pipo(desired_access: u32, inherit_handle: bool, pid: u32) -> usize
{
    unsafe
    {
        MessageBoxA(0x0 as *const usize, "No!\0".as_ptr(), "Computer says...\0".as_ptr(), 0x0);

    }

    usize::MAX
}

