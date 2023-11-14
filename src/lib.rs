use winapi::um::processthreadsapi::{OpenProcess, GetProcessId};
use winapi::um::tlhelp32::{CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS, MODULEENTRY32, TH32CS_SNAPMODULE, Module32First, Module32Next};
use winapi::um::memoryapi::{WriteProcessMemory, ReadProcessMemory};
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::winnt::PROCESS_ALL_ACCESS;
use winapi::ctypes::c_void;
use std::ffi::CStr;
use std::ptr;

pub mod memory {
    use super::*;

    pub struct AssaultCube {
        pub hp: usize,
        pub nades: usize,
        pub armor: usize,
    }

    pub struct AttachedProcess {
        process_handle: *mut c_void,
        use_internal: bool,
    }

    impl AttachedProcess {
        pub fn new(process_handle: *mut c_void, use_internal: bool) -> Self {
            AttachedProcess {
                process_handle,
                use_internal,
            }
        }
        

        pub fn getbase(&self, module_name: &str) -> usize {
            unsafe {
                if self.process_handle.is_null() {
                    eprintln!("Error: Process handle is not set. Call `attach_process` first.");
                    return 0;
                }

                let process_id = GetProcessId(self.process_handle);

                let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPMODULE, process_id);
                if snapshot == INVALID_HANDLE_VALUE {
                    eprintln!("Error: Unable to create tool help snapshot.");
                    return 0;
                }

                let mut module_entry = MODULEENTRY32 {
                    dwSize: std::mem::size_of::<MODULEENTRY32>() as u32,
                    ..Default::default()
                };

                if Module32First(snapshot, &mut module_entry) == 0 {
                    CloseHandle(snapshot);
                    eprintln!("Error: Unable to get the first module.");
                    return 0;
                }

                let mut base_address = 0;

                loop {
                    let current_module_name =
                        String::from_utf8_lossy(CStr::from_ptr(module_entry.szModule.as_ptr()).to_bytes());

                    if current_module_name.to_lowercase() == module_name.to_lowercase() {
                        base_address = module_entry.modBaseAddr as usize;
                        break;
                    }

                    if Module32Next(snapshot, &mut module_entry) == 0 {
                        break;
                    }
                }

                CloseHandle(snapshot);

                if base_address == 0 {
                    eprintln!("Error: Module '{}' not found.", module_name);
                }

                base_address
            }
        }

        pub fn get_ptr(&self, base: usize, offsets: &[usize]) -> usize {
            if self.use_internal {
                let mut addr: usize = base;

                for &i in offsets {
                    if offsets.contains(&i) {
                        if i != *offsets.last().unwrap() {
                            addr = unsafe { *(addr as *const usize) };
                        }
                    }
                }

                addr + *offsets.last().unwrap()
            } else {
                let mut addr: usize = self.read(base);

                for &i in offsets {
                    if offsets.contains(&i) {
                        if i != *offsets.last().unwrap() {
                            addr = self.read(addr + i);
                        }
                    }
                }

                addr + *offsets.last().unwrap()
            }
        }

        pub fn read<T: Default>(&self, address: usize) -> T {
            if self.use_internal {
                unsafe {
                    core::ptr::read::<T>(address as *const T)
                }
            } else {
                unsafe {
                    let mut value: T = Default::default();
                    let size = std::mem::size_of::<T>();

                    let success = ReadProcessMemory(
                        self.process_handle,
                        address as *const _,
                        &mut value as *mut _ as *mut _,
                        size,
                        ptr::null_mut(),
                    );

                    if success == 0 {
                        let last_error = std::io::Error::last_os_error();
                        eprintln!("Error reading process memory: {}", last_error);
                        return Default::default();
                    }

                    value
                }
            }
        }

        pub fn write<T>(&self, address: usize, value: T) -> bool {
            if self.use_internal {
                unsafe {
                    core::ptr::write(address as *mut T, value);
                }
                true
            } else {
                unsafe {
                    WriteProcessMemory(
                        self.process_handle,
                        address as *mut _,
                        &value as *const _ as *const _,
                        std::mem::size_of::<T>(),
                        ptr::null_mut(),
                    ) != 0
                }
            }
        }

        pub fn get_assault_cube(&self, base_address: usize) -> AssaultCube {
            AssaultCube {
                hp: self.get_ptr(base_address + 0x17E0A8, &[0xEC]),
                nades: self.get_ptr(base_address + 0x17E0A8, &[0x144]),
                armor: self.get_ptr(base_address + 0x17E0A8, &[0xF0]),
            }
        }
    }

    pub fn init(process_name: &str, use_internal: bool) -> Option<AttachedProcess> {
        let process_handle = get_process_handle(process_name);
        if let Some(handle) = process_handle {
            println!("Attached to process: {} (Handle: {:?})", process_name, handle);
            Some(AttachedProcess::new(handle, use_internal))
        } else {
            eprintln!("Error: Failed to attach to process: {}", process_name);
            None
        }
}

    fn get_process_handle(process_name: &str) -> Option<*mut c_void> {
        unsafe {
            let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
            if snapshot == winapi::um::handleapi::INVALID_HANDLE_VALUE {
                return None;
            }

            let mut entry: PROCESSENTRY32 = std::mem::zeroed();
            entry.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;

            if Process32First(snapshot, &mut entry) == 0 {
                CloseHandle(snapshot);
                return None;
            }

            loop {
                let process_name_in_snapshot = std::ffi::CStr::from_ptr(entry.szExeFile.as_ptr())
                    .to_string_lossy()
                    .to_lowercase();

                if process_name_in_snapshot == process_name.to_lowercase() {
                    let process_handle = OpenProcess(PROCESS_ALL_ACCESS, 0, entry.th32ProcessID);
                    CloseHandle(snapshot);
                    return Some(process_handle);
                }

                if Process32Next(snapshot, &mut entry) == 0 {
                    break;
                }
            }

            CloseHandle(snapshot);
        }

        None
    }
}
