use libc::c_char;

// Opaque C structures
pub enum DisirInstance {}
pub enum DisirConfig {}

// The status definitions from the c library
#[derive(PartialEq)]
#[derive(Debug)]
#[repr(C)]
pub enum Status {
    OK = 0,
}

#[link(name = "disirc")]
extern {
    pub fn disir_instance_create(filepath: *const c_char,
                                 config: *const DisirConfig,
                                 instance: *mut *mut DisirInstance) -> Status;
    pub fn disir_instance_destroy(instance: *mut *mut DisirInstance) -> Status;
}

