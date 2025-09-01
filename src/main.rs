use windows::Win32::System::ProcessStatus::EnumProcesses;

fn main() {
    let mut lpidprocess: [u32; 1024] = [0; 1024];
    let cb: u32 = (lpidprocess.len() * std::mem::size_of::<u32>()) as u32;
    let mut lpcbneeded: u32 = 0;

    unsafe {
        match EnumProcesses(lpidprocess.as_mut_ptr(), cb, &mut lpcbneeded) {
            Ok(()) => {
                let num_processes = lpcbneeded as usize / std::mem::size_of::<u32>();

                for pid in lpidprocess.iter().take(num_processes) {
                    println!("Process ID: {}", pid);
                }
            }
            Err(e) => {
                eprintln!("EnumProcesses failed: {:?}", e);
            }
        }
    }
}
