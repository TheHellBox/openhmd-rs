extern crate openhmd_rs_sys;

use openhmd_rs_sys::*;

pub use openhmd_rs_sys::{ohmd_float_value, ohmd_string_value, ohmd_int_value};

pub struct Context{
    context: &'static ohmd_context
}

pub struct Device{
    device: &'static ohmd_device
}

impl Context{
    pub fn new() -> Context{
        unsafe{
            Context{
                context: ohmd_ctx_create()
            }
        }
    }

    pub fn probe(&self) -> i32{
        unsafe{
            ohmd_ctx_probe(self.context) as i32
        }
    }

    pub fn update(&self){
        unsafe{
            ohmd_ctx_update(self.context);
        }
    }

    pub fn list_open_device(&self, index: i32) -> Device{
        unsafe{
            Device{
                device: ohmd_list_open_device(self.context, index)
            }
        }
    }

    pub fn get_error(&self) -> i32{
        unsafe{
            ohmd_ctx_get_error(self.context) as i32
        }
    }
}

impl Drop for Context{
    fn drop(&mut self){
        unsafe{
            ohmd_ctx_destroy(self.context);
        }
    }
}

impl Device{
    pub fn getf(&self, otype: ohmd_float_value) -> [f32; 16]{
        let mut out: [f32; 16] = [0.0, 0.0, 0.0, 0.0,0.0, 0.0, 0.0, 0.0,0.0, 0.0, 0.0, 0.0,0.0, 0.0, 0.0, 0.0];
        unsafe{
            ohmd_device_getf(self.device, otype, &mut out);
        }
        out
    }

    pub fn geti(&self, otype: ohmd_int_value) -> i32{
        let mut out: [i32; 1] = [0];
        unsafe{
            ohmd_device_geti(self.device, otype, &mut out);
        }
        out[0]
    }
}

impl Drop for Device{
    fn drop(&mut self){
        unsafe{
            ohmd_close_device(self.device);
        }
    }
}
