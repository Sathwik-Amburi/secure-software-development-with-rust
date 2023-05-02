use std::alloc::{alloc_zeroed, dealloc, Layout};
use std::mem::{align_of, size_of};
use std::str;

struct SensitiveData {
    data: [u8; 64],
}

impl SensitiveData {
    fn new() -> Self {
        SensitiveData {
            data: b"Sensitive information: secret_key=ABC123!\0".pad_to_align(),
        }
    }

    // Mimic sensitive data clean up
    fn zeroize(&mut self) {
        self.data = [0; 64];
    }

    // Simulate usage of the sensitive data
    fn use_sensitive_data(&self) {
        if let Ok(sensitive_data_str) = str::from_utf8(&self.data) {
            println!("Using sensitive data: {}", sensitive_data_str);
        }
    }
}

trait PadToAlign {
    fn pad_to_align(self) -> [u8; 64];
}

impl PadToAlign for &[u8] {
    fn pad_to_align(self) -> [u8; 64] {
        let mut array = [0; 64];
        array[..self.len()].copy_from_slice(self);
        array
    }
}

fn main() {
    let layout =
        Layout::from_size_align(size_of::<SensitiveData>(), align_of::<SensitiveData>()).unwrap();
    let sensitive_data_ptr = unsafe { alloc_zeroed(layout) as *mut SensitiveData };

    if !sensitive_data_ptr.is_null() {
        unsafe {
            // Initialize the sensitive data
            sensitive_data_ptr.write(SensitiveData::new());

            // Perform some operations with the sensitive data
            (*sensitive_data_ptr).use_sensitive_data();

            // Intentionally not zeroizing the sensitive data before deallocating the memory
            // zeriozing the sensitive data before deallocating the memory
            // (*sensitive_data_ptr).zeroize();
            dealloc(sensitive_data_ptr as *mut u8, layout);
        }
    }
}
