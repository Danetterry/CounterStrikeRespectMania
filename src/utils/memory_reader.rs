use crate::utils::mem_utils::{get_pid, try_read_string, ModuleInfo};
use egui_render_three_d::three_d::{Matrix4, Vector3};
use process_memory::{DataMember, Memory, ProcessHandle, TryIntoProcessHandle};

// This structure contains all functions needed to read game memory (also may be ported for another games)
// Thanks to Tom Almeida for creating beautiful memory reading crate (https://crates.io/crates/process-memory)
pub struct MemoryReader {
    pub handle: ProcessHandle,
    pub module: usize,
}

unsafe impl Send for MemoryReader {}

impl MemoryReader {
    pub fn new(proccess_name: &str, module_name: &str) -> MemoryReader {
        // Getting proccess pid by name
        let pid = get_pid(proccess_name).expect("Failed to get pid");
        // Getting handle with proccess (more at https://learn.microsoft.com/en-us/windows/win32/procthread/process-handles-and-identifiers)
        let handle = pid.try_into_process_handle().expect("Failed to get handle");
        // Getting module base address
        // This is like /proc/pid/maps in linux
        let module = pid
            .get_module_base(module_name)
            .expect("Failed to get module base");

        // Return a structure
        MemoryReader { handle, module }
    }

    // Rust does not support any type on out so I just copy-paste 1 function
    pub fn read_usize(&self, offset: usize) -> usize {
        // Creating offset DataMember for reading memory
        let offset = DataMember::<usize>::new_offset(self.handle, vec![offset]);

        unsafe {
            // Reading
            offset.read().unwrap_or(0)
        }
    }

    pub fn read_i32(&self, offset: usize) -> i32 {
        let offset = DataMember::<i32>::new_offset(self.handle, vec![offset]);

        unsafe { offset.read().unwrap_or_else(|_| 0) }
    }

    pub fn read_i16(&self, offset: usize) -> i16 {
        let offset = DataMember::<i16>::new_offset(self.handle, vec![offset]);

        unsafe { offset.read().unwrap_or_else(|_| 0) }
    }

    pub fn read_u32(&self, offset: usize) -> u32 {
        let offset = DataMember::<u32>::new_offset(self.handle, vec![offset]);

        unsafe { offset.read().unwrap_or_else(|_| 0) }
    }

    pub fn read_u8(&self, offset: usize) -> u8 {
        let offset = DataMember::<u8>::new_offset(self.handle, vec![offset]);

        unsafe { offset.read().unwrap_or_else(|_| 0) }
    }

    pub fn read_bool(&self, offset: usize) -> bool {
        let offset = DataMember::<bool>::new_offset(self.handle, vec![offset]);

        unsafe { offset.read().unwrap_or_else(|_| false) }
    }

    pub fn read_vec_i32(&self, offset: usize) -> Vector3<i32> {
        let offset = DataMember::<Vector3<i32>>::new_offset(self.handle, vec![offset]);

        unsafe { offset.read().unwrap_or_else(|_| Vector3::new(0, 0, 0)) }
    }

    pub fn read_vec_f32(&self, offset: usize) -> Vector3<f32> {
        let offset = DataMember::<Vector3<f32>>::new_offset(self.handle, vec![offset]);

        unsafe {
            offset
                .read()
                .unwrap_or_else(|_| Vector3::new(0.0, 0.0, 0.0))
        }
    }

    pub fn read_matrix4_f32(&self, offset: usize) -> Matrix4<f32> {
        let offset = DataMember::<Matrix4<f32>>::new_offset(self.handle, vec![offset]);

        unsafe {
            offset.read().unwrap_or_else(|_| {
                Matrix4::new(
                    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                )
            })
        }
    }

    pub fn read_string(&self, offset: usize) -> String {
        // It's just wrapper for scary function in mem_utils.rs
        try_read_string(self.handle, vec![offset])
            .unwrap_or_else(|_| String::from("Failed to read string"))
    }
}
