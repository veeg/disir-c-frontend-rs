extern crate libc;
use std::mem;
use std::ptr;

// We are not exporting the C interface to the crate.
mod c;
use c::{DisirInstance, disir_instance_create, disir_instance_destroy};

// Define the public disir instace object
pub struct Instance {
    opaque: *mut c::DisirInstance,
}

impl Default for Instance {
    fn default() -> Instance {
        // QUESTION: What is the best way to initialize a c-like struct to a NULL value?
        unsafe {
            Instance { opaque : mem::uninitialized() }
        }
    }
}

impl Instance {
    pub fn create(&mut self) {
        // FIXME: Guard against re-instantiating itself.
        unsafe {
            disir_instance_create(ptr::null(), ptr::null(), &mut self.opaque);
        }
    }

    pub fn destroy (&mut self) {
        // FIXME: Guard against multiple invocation of nullptr opaque pointer.
        unsafe {
            disir_instance_destroy(&mut self.opaque);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_and_destroys () {
        let mut inst : Instance = Default::default();
        inst.create();
        inst.destroy();
    }
}
