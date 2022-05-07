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

    //fn MessageBoxA(hwnd: *const usize, caption: *const u8, title: *const u8, flags: usize) -> usize;
}

static_detour!
{
    static MessageBoxAHook: unsafe extern "system" fn(*const usize, *const u8, *const u8, usize) -> usize;
}


#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(dll_module: HINSTANCE, call_reason: DWORD, reserved: LPVOID) -> BOOL
{
    const DLL_PROCESS_ATTACH: DWORD = 1;
    const DLL_PROCESS_DETACH: DWORD = 0;

    match call_reason
    {
        DLL_PROCESS_ATTACH => init().unwrap(),
        DLL_PROCESS_DETACH => (),
        _ => (),
    }

    minwindef::TRUE
}

type FnMessageBoxA = unsafe extern "system" fn(*const usize, *const u8, *const u8, usize) -> usize;

fn init() -> Result<(), Box<dyn Error>>
{
    unsafe
    {
        let user_dll: usize = LoadLibraryA("user32\0".as_ptr());
        let msg_box_addr: usize = GetProcAddress(user_dll, "MessageBoxA\0".as_ptr());

        //let func = msg_box_addr as *const unsafe extern "system" fn(*const usize, *const u8, *const u8, usize) -> usize;

        let func: FnMessageBoxA = mem::transmute(msg_box_addr); 

        println!("injected");

        MessageBoxAHook.initialize(func, pipo)?
                       .enable()?;

        Ok(())
/*

        let func = &msg_box_addr as *const usize
                                 as *const fn(*const usize, *const u8, *const u8, usize) -> usize;
        (*func)(0x0 as *const usize, "Injected !\0".as_ptr(), "Title\0".as_ptr(), 0);

        Ok(())
*/
    }
}

fn pipo(hwnd: *const usize, text: *const u8, title: *const u8, flags: usize) -> usize
{
    unsafe
    {
        MessageBoxAHook.call(hwnd, text, "replaced\0".as_ptr(), flags)
    }
}

