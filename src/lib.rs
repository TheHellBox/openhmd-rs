extern crate openhmd_rs_sys;

use openhmd_rs_sys::*;

pub use openhmd_rs_sys::{ohmd_float_value, ohmd_string_value, ohmd_int_value};

pub struct Context{
    context: &'static ohmd_context
}

pub struct OpenHMDDevice{
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

    pub fn destroy(&self){
        unsafe{
            ohmd_ctx_destroy(self.context);
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

    pub fn list_open_device(&self, index: i32) -> OpenHMDDevice{
        unsafe{
            OpenHMDDevice{
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

impl OpenHMDDevice{
    pub fn getf(&self, otype: ohmd_float_value) -> [f32; 16]{
        let mut out: [f32; 16] = [0.0, 0.0, 0.0, 0.0,0.0, 0.0, 0.0, 0.0,0.0, 0.0, 0.0, 0.0,0.0, 0.0, 0.0, 0.0];
        unsafe{
            ohmd_device_getf(self.device, otype, &mut out);
        }
        out
    }

    pub fn geti(&self, otype: ohmd_int_value) -> [i32; 1]{
        let mut out: [i32; 1] = [0];
        unsafe{
            ohmd_device_geti(self.device, otype, &mut out);
        }
        out
    }
}
