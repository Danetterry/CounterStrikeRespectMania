use process_memory::{CopyAddress, Pid};
use winapi::um::{handleapi::CloseHandle, tlhelp32};
use winapi::{shared::minwindef, um::handleapi::INVALID_HANDLE_VALUE};

// These things were stolen from https://github.com/Tommoa/rs-process-memory/pull/4
// Thank you, Kiiyya!

/// A helper function to turn a `c_char` array to a String
fn utf8_to_string(bytes: &[i8]) -> String {
    use std::ffi::CStr;
    unsafe {
        CStr::from_ptr(bytes.as_ptr())
            .to_string_lossy()
            .into_owned()
    }
}

/// Get the process ID of some process by name. For example,"MyGame.exe".
/// If you want to get the PID of your own process, use `std::process:id() as Pid` instead.
///
/// # Errors
/// If no process exists of that name, returns an `std::io::Error` with kind `std::io::ErrorKind::NotFound`.
/// If something went very wrong with the windows API, returns last OS error.
#[allow(clippy::cast_possible_truncation)]
pub fn get_pid(process_name: &str) -> std::io::Result<Pid> {
    let mut entry = tlhelp32::PROCESSENTRY32 {
        dwSize: size_of::<tlhelp32::PROCESSENTRY32>() as u32,
        cntUsage: 0,
        th32ProcessID: 0,
        th32DefaultHeapID: 0,
        th32ModuleID: 0,
        cntThreads: 0,
        th32ParentProcessID: 0,
        pcPriClassBase: 0,
        dwFlags: 0,
        szExeFile: [0; minwindef::MAX_PATH],
    };

    let snapshot: winapi::um::winnt::HANDLE;
    unsafe {
        snapshot = tlhelp32::CreateToolhelp32Snapshot(tlhelp32::TH32CS_SNAPPROCESS, 0);
        if snapshot == INVALID_HANDLE_VALUE {
            return Err(std::io::Error::last_os_error());
        }

        if tlhelp32::Process32First(snapshot, &mut entry) == minwindef::TRUE {
            // makeshift do-while
            loop {
                // println!("Have process: {}", utf8_to_string(&entry.szExeFile));
                if utf8_to_string(&entry.szExeFile) == process_name {
                    let pid = entry.th32ProcessID;
                    if CloseHandle(snapshot) == minwindef::FALSE {
                        panic!("Could not close handle")
                    };
                    return Ok(pid);
                }

                if tlhelp32::Process32Next(snapshot, &mut entry) == minwindef::FALSE {
                    break;
                }
            }
        }

        if CloseHandle(snapshot) == minwindef::FALSE {
            panic!("Could not close handle")
        };
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Could not find Process ID of \"{}\".", process_name),
        ))
    }
}

/// Handling modules (e.g. DLLs) in a process.
pub trait ModuleInfo {
    /// Gets the base address of a module in a process. For example, "GameAssembly.dll" when on Windows.
    /// You can then use the address in the `base` parameter of [`set_offset`] for example.
    ///
    /// # Errors
    /// Returns `std::io::ErrorKind::NotFound` when no such module name exists.
    /// Returns OS Error when something else went wrong.
    ///
    /// # Panics
    /// Panics when closing the handle fails (e.g. double close).
    ///
    /// [`set_offset`]: trait.Memory.html#tymethod.set_offset
    fn get_module_base(&self, name: &str) -> std::io::Result<usize>;
}

#[allow(clippy::cast_possible_truncation)] // for size_of as u32
impl ModuleInfo for Pid {
    fn get_module_base(&self, name: &str) -> std::io::Result<usize> {
        // taken from https://stackoverflow.com/questions/41552466/how-do-i-get-the-physical-baseaddress-of-an-dll-used-in-a-process
        let mut module_entry = tlhelp32::MODULEENTRY32 {
            dwSize: 0,
            th32ModuleID: 0,
            th32ProcessID: 0,
            GlblcntUsage: 0,
            ProccntUsage: 0,
            modBaseAddr: std::ptr::null_mut(), // yikes
            modBaseSize: 0,
            hModule: std::ptr::null_mut(), // yikes
            szModule: [0; tlhelp32::MAX_MODULE_NAME32 + 1],
            szExePath: [0; minwindef::MAX_PATH],
        };

        unsafe {
            module_entry.dwSize = size_of::<tlhelp32::MODULEENTRY32>() as u32;

            let snapshot = tlhelp32::CreateToolhelp32Snapshot(
                tlhelp32::TH32CS_SNAPMODULE | tlhelp32::TH32CS_SNAPMODULE32,
                *self,
            );
            if snapshot == INVALID_HANDLE_VALUE {
                return Err(std::io::Error::last_os_error());
            }

            if tlhelp32::Module32First(snapshot, &mut module_entry) == minwindef::TRUE {
                // makeshift do-while
                loop {
                    if utf8_to_string(&module_entry.szModule) == name {
                        let addr = module_entry.modBaseAddr as usize;
                        if CloseHandle(snapshot) == minwindef::FALSE {
                            panic!("Could not close handle")
                        };
                        return Ok(addr);
                    }

                    if tlhelp32::Module32Next(snapshot, &mut module_entry) == minwindef::FALSE {
                        break;
                    }
                }
            }

            // We searched everything, nothing found
            if CloseHandle(snapshot) == minwindef::FALSE {
                panic!("Could not close handle")
            };
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!(
                    "Process PID#{} contains no module named \"{}\".",
                    *self, name
                ),
            ))
        }
    }
}

// This was pasted from https://github.com/Tommoa/rs-process-memory/issues/3
pub fn try_read_string(
    handle: process_memory::ProcessHandle,
    starting_offsets: Vec<usize>,
) -> Result<String, std::io::Error> {
    let mut offset = handle.get_offset(&starting_offsets)?;
    let mut parts = Vec::<u8>::new();
    let mut byte = [0u8; 1];
    loop {
        handle.copy_address(offset, &mut byte)?;
        if byte[0] == 0 {
            break;
        }
        offset += 1;
        parts.push(byte[0]);
    }
    String::from_utf8(parts).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}

// pub fn try_read_string(
//     handle: process_memory::ProcessHandle,
//     starting_offsets: Vec<usize>,
// ) -> Result<String, std::io::Error> {
//     let mut offset = handle.get_offset(&starting_offsets)?;
//     let mut parts = Vec::<u8>::new();
//     let mut byte = [0u8; 1];
//     let max_bytes = 64; // Max bytes to read
//     let mut bytes_read = 0;
//
//     loop {
//         if bytes_read >= max_bytes {
//             break;
//         }
//         handle.copy_address(offset, &mut byte)?;
//         if byte[0] == 0 {
//             break;
//         }
//         offset += 1;
//         parts.push(byte[0]);
//         bytes_read += 1;
//     }
//     String::from_utf8(parts).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
// }
