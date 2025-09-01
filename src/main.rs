use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::System::ProcessStatus::EnumProcesses;
use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};

fn main() {
    let mut lpidprocess: [u32; 1024] = [0; 1024];
    let cb: u32 = (lpidprocess.len() * std::mem::size_of::<u32>()) as u32;
    let mut lpcbneeded: u32 = 0;

    unsafe {
        EnumProcesses(lpidprocess.as_mut_ptr(), cb, &mut lpcbneeded).expect("EnumProcesses failed");
        let num_processes: usize = lpcbneeded as usize / std::mem::size_of::<u32>();
        println!("Number of processes detected: {}", num_processes);

        for &pid in &lpidprocess[..num_processes] {
            let handle: HANDLE =
                match OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid) {
                    Ok(h) => h,
                    Err(_e) => continue,
                };

            println!("Process ID: {}", pid);

            CloseHandle(handle).ok();
        }
    }
}
