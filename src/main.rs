use std::io::{self, Write};

use windows::Win32::Foundation::{CloseHandle, HANDLE, HMODULE, MAX_PATH};
use windows::Win32::System::ProcessStatus::{
    EnumProcessModules, EnumProcesses, GetModuleBaseNameW,
};
use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};

fn main() {
    list_processes();

    print!("Enter the process name: ");
    io::stdout().flush().unwrap();

    let mut target_name = String::new();
    io::stdin()
        .read_line(&mut target_name)
        .expect("Failed to read line");

    let target_name = target_name.trim();

    match get_remote_process_handle(target_name) {
        Some((handle, pid)) => println!(
            "Found process '{}' with pid {} and handle {:?}",
            target_name, pid, handle
        ),
        None => println!(
            "Process '{}' not found or insufficient permissions",
            target_name
        ),
    }
}

fn list_processes() {
    let mut lpidprocess: [u32; 1024] = [0; 1024];
    let cb: u32 = (lpidprocess.len() * std::mem::size_of::<u32>()) as u32;
    let mut lpcbneeded: u32 = 0;

    unsafe {
        EnumProcesses(lpidprocess.as_mut_ptr(), cb, &mut lpcbneeded).expect("EnumProcesses failed")
    };
    let num_processes: usize = lpcbneeded as usize / std::mem::size_of::<u32>();

    for pid in lpidprocess.iter().take(num_processes) {
        let handle: HANDLE = match unsafe {
            OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, *pid)
        } {
            Ok(h) => h,
            Err(_e) => continue,
        };

        let mut hmodule: HMODULE = HMODULE::default();
        let mut cb_needed: u32 = 0;

        if unsafe {
            EnumProcessModules(
                handle,
                &mut hmodule,
                std::mem::size_of_val(&hmodule) as u32,
                &mut cb_needed,
            )
            .ok()
            .is_none()
        } {
            continue;
        }

        let mut process_name = [0u16; MAX_PATH as usize];

        let len = unsafe { GetModuleBaseNameW(handle, Some(hmodule), &mut process_name) };

        if len > 0 {
            let name = String::from_utf16_lossy(&process_name[..len as usize]);
            println!("Process {} with id: {}", name, pid);
        }

        unsafe { CloseHandle(handle).ok() };
    }

    println!("Number of processes detected: {}", num_processes);
}

fn get_remote_process_handle(target_process_name: &str) -> Option<(HANDLE, u32)> {
    let mut lpidprocess: [u32; 1024] = [0; 1024];
    let cb: u32 = (lpidprocess.len() * std::mem::size_of::<u32>()) as u32;
    let mut lpcbneeded: u32 = 0;

    unsafe {
        EnumProcesses(lpidprocess.as_mut_ptr(), cb, &mut lpcbneeded).expect("EnumProcesses failed")
    };
    let num_processes: usize = lpcbneeded as usize / std::mem::size_of::<u32>();

    for pid in lpidprocess.iter().take(num_processes) {
        let handle: HANDLE = match unsafe {
            OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, *pid)
        } {
            Ok(h) => h,
            Err(_e) => continue,
        };

        let mut hmodule: HMODULE = HMODULE::default();
        let mut cb_needed: u32 = 0;

        if unsafe {
            EnumProcessModules(
                handle,
                &mut hmodule,
                std::mem::size_of_val(&hmodule) as u32,
                &mut cb_needed,
            )
            .ok()
            .is_none()
        } {
            continue;
        }

        let mut process_name = [0u16; MAX_PATH as usize];

        let len = unsafe { GetModuleBaseNameW(handle, Some(hmodule), &mut process_name) };

        if len > 0 {
            let name = String::from_utf16_lossy(&process_name[..len as usize]);

            if name == target_process_name.to_lowercase() {
                return Some((handle, *pid));
            }
        }

        unsafe { CloseHandle(handle).ok() };
    }

    None
}
