extern crate libc;
use libc::{c_char, c_int, c_float};
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ohmd_context;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ohmd_device;

pub enum ohmd_float_value{
    OHMD_ROTATION_QUAT = 1,
    OHMD_LEFT_EYE_GL_MODELVIEW_MATRIX = 2,
    OHMD_RIGHT_EYE_GL_MODELVIEW_MATRIX = 3,
    OHMD_LEFT_EYE_GL_PROJECTION_MATRIX = 4,
    OHMD_RIGHT_EYE_GL_PROJECTION_MATRIX = 5,
    OHMD_POSITION_VECTOR = 6,
    OHMD_SCREEN_HORIZONTAL_SIZE = 7,
    OHMD_SCREEN_VERTICAL_SIZE = 8,
    OHMD_LENS_HORIZONTAL_SEPARATION = 9,
    OHMD_LENS_VERTICAL_POSITION = 10,
    OHMD_LEFT_EYE_FOV = 11,
    OHMD_LEFT_EYE_ASPECT_RATIO = 12,
    OHMD_RIGHT_EYE_FOV = 13,
    OHMD_RIGHT_EYE_ASPECT_RATIO = 14,
    OHMD_EYE_IPD = 15,
    OHMD_PROJECTION_ZFAR = 16,
    OHMD_PROJECTION_ZNEAR = 17,
    OHMD_DISTORTION_K = 18,
    OHMD_EXTERNAL_SENSOR_FUSION = 19,
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum ohmd_string_value{
    OHMD_VENDOR = 0,
    OHMD_PRODUCT = 1,
    OHMD_PATH = 2
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum ohmd_int_value{
    OHMD_SCREEN_HORIZONTAL_RESOLUTION = 0,
    OHMD_SCREEN_VERTICAL_RESOLUTION = 1,
    OHMD_DEVICE_CLASS = 2,
    OHMD_DEVICE_FLAGS = 3
}
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum ohmd_device_flags{
    OHMD_DEVICE_FLAGS_NULL_DEVICE = 1,
    OHMD_DEVICE_FLAGS_POSITIONAL_TRACKING = 2,
    OHMD_DEVICE_FLAGS_ROTATIONAL_TRACKING = 4,
    OHMD_DEVICE_FLAGS_LEFT_CONTROLLER = 8,
    OHMD_DEVICE_FLAGS_RIGHT_CONTROLLER = 16
}
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum ohmd_device_class{
    OHMD_DEVICE_CLASS_HMD = 0,
    OHMD_DEVICE_CLASS_CONTROLLER = 1,
    OHMD_DEVICE_CLASS_GENERIC_TRACKER = 2
}
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct ohmd_device_desc{
    pub driver: c_char,
    pub vendor: c_char,
    pub product: c_char,
    pub path: c_char,
    pub revision: c_int,
    pub id: c_int,
    pub device_flags: ohmd_device_flags,
    pub device_class: ohmd_device_class,
    pub driver_ptr: &'static ohmd_driver
}
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct ohmd_device_list{
    num_devices: c_int,
    devices: [ohmd_device_desc; 16]
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ohmd_driver{
    pub get_device_list: extern fn(driver: &mut ohmd_driver, list: ohmd_device_list),
    pub open_device: extern fn(driver: &ohmd_driver) -> &ohmd_device,
}
impl ohmd_driver {
    pub fn open_device(&mut self) -> &ohmd_device {
        unsafe {
            (self.open_device)(self as &ohmd_driver)
        }
    }
}
#[link(name = "openhmd")]
extern {
    pub fn ohmd_ctx_create() -> &'static mut ohmd_context;
    pub fn ohmd_ctx_destroy(ctx: &ohmd_context);
    pub fn ohmd_ctx_get_error(ctx: &ohmd_context) -> c_char;
    pub fn ohmd_ctx_probe(ctx: &ohmd_context) -> c_int;
    pub fn ohmd_ctx_update(ctx: &ohmd_context);
    pub fn ohmd_device_getf(device: &ohmd_device, otype: ohmd_float_value, out: &mut [c_float; 16]) -> c_int;
    pub fn ohmd_device_setf(device: &ohmd_device, otype: ohmd_float_value, float: &[c_float; 16]) -> c_int;
    pub fn ohmd_list_open_device(ctx: &ohmd_context, index: c_int) -> &'static ohmd_device;
    pub fn ohmd_list_gets(ctx: &ohmd_context, index: c_int, otype: ohmd_string_value) -> &[c_char];
    pub fn ohmd_device_geti(device: &ohmd_device, otype: ohmd_int_value, out: &mut [c_int; 1]) -> c_int;
    pub fn ohmd_create_external_drv(ctx: &ohmd_context) -> &'static mut ohmd_driver;
}
