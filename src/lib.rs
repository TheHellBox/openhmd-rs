extern crate openhmd_rs_sys;

use openhmd_rs_sys::*;

pub use openhmd_rs_sys::{ohmd_float_value, ohmd_int_value, ohmd_string_value};

pub struct Context {
    context: &'static ohmd_context,
}

pub struct Device {
    device: &'static ohmd_device,
}

pub const SHADER_DISTORTION_VERT: &'static str = r#"
version 120
void main(void)
{
	gl_TexCoord[0] = gl_MultiTexCoord0;
    gl_Position = gl_ProjectionMatrix * gl_ModelViewMatrix * gl_Vertex;
}
"#;

pub const SHADER_SIMPLE_FRAG: &'static str = r#"
version 120

//per eye texture to warp for lens distortion
uniform sampler2D warpTexture;

//Position of lens center in m (usually eye_w/2, eye_h/2)
uniform vec2 LensCenter;
//Scale from texture co-ords to m (usually eye_w, eye_h)
uniform vec2 ViewportScale;
//Distortion overall scale in m (usually ~eye_w/2)
uniform float WarpScale;
//Distoriton coefficients (PanoTools model) [a,b,c,d]
uniform vec4 HmdWarpParam;

//chromatic distortion post scaling
uniform vec3 aberr;

void main()
{
    //output_loc is the fragment location on screen from [0,1]x[0,1]
    vec2 output_loc = vec2(gl_TexCoord[0].s, gl_TexCoord[0].t);
    //Compute fragment location in lens-centered co-ordinates at world scale
	vec2 r = output_loc * ViewportScale - LensCenter;
    //scale for distortion model
    //distortion model has r=1 being the largest circle inscribed (e.g. eye_w/2)
    r /= WarpScale;

    //|r|**2
    float r_mag = length(r);
    //offset for which fragment is sourced
    vec2 r_displaced = r * (HmdWarpParam.w + HmdWarpParam.z * r_mag +
		HmdWarpParam.y * r_mag * r_mag +
		HmdWarpParam.x * r_mag * r_mag * r_mag);
    //back to world scale
    r_displaced *= WarpScale;
    //back to viewport co-ord
    vec2 tc_r = (LensCenter + aberr.r * r_displaced) / ViewportScale;
    vec2 tc_g = (LensCenter + aberr.g * r_displaced) / ViewportScale;
    vec2 tc_b = (LensCenter + aberr.b * r_displaced) / ViewportScale;

    float red = texture2D(warpTexture, tc_r).r;
    float green = texture2D(warpTexture, tc_g).g;
    float blue = texture2D(warpTexture, tc_b).b;
    //Black edges off the texture
    gl_FragColor = ((tc_g.x < 0.0) || (tc_g.x > 1.0) || (tc_g.y < 0.0) || (tc_g.y > 1.0)) ? vec4(0.0, 0.0, 0.0, 1.0) : vec4(red, green, blue, 1.0);
}
"#;

impl Context {
    pub fn new() -> Context {
        unsafe {
            Context {
                context: ohmd_ctx_create(),
            }
        }
    }

    pub fn probe(&self) -> i32 {
        unsafe { ohmd_ctx_probe(self.context) as i32 }
    }

    pub fn update(&self) {
        unsafe {
            ohmd_ctx_update(self.context);
        }
    }

    pub fn list_open_device(&self, index: i32) -> Device {
        unsafe {
            Device {
                device: ohmd_list_open_device(self.context, index),
            }
        }
    }

    pub fn get_error(&self) -> i32 {
        unsafe { ohmd_ctx_get_error(self.context) as i32 }
    }

    pub fn destroy(&self) {
        unsafe {
            ohmd_ctx_destroy(self.context);
        }
    }

    pub fn list_gets(&self, index: i32, otype: ohmd_string_value) -> &str {
        use std::ffi::CStr;
        unsafe {
            let raw = ohmd_list_gets(self.context, index, otype);
            CStr::from_ptr(raw).to_str().unwrap()
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        self.destroy();
    }
}

impl Device {
    pub fn getf(&self, otype: ohmd_float_value) -> Option<[f32; 16]> {
        let mut out: [f32; 16] = [0.0; 16];
        unsafe {
            match ohmd_device_getf(self.device, otype, &mut out) {
                0 => return Some(out),
                _ => return None,
            };
        }
    }

    pub fn get_rotation_quat(&self) -> [f32; 4] {
        let ohmd_orient = match self.getf(ohmd_float_value::OHMD_ROTATION_QUAT) {
            Some(x) => [x[0], x[1], x[2], x[3]],
            _ => [0.0; 4],
        };
        ohmd_orient
    }

    pub fn get_position_vec(&self) -> [f32; 3] {
        let ohmd_position = match self.getf(ohmd_float_value::OHMD_POSITION_VECTOR) {
            Some(x) => [x[0], x[1], x[2]],
            _ => [0.0; 3],
        };
        ohmd_position
    }

    pub fn get_view_matrix_l(&self) -> [f32; 16] {
        let view_left = match self.getf(ohmd_float_value::OHMD_LEFT_EYE_GL_MODELVIEW_MATRIX) {
            Some(x) => x,
            None => [0.0; 16],
        };
        view_left
    }

    pub fn get_view_matrix_r(&self) -> [f32; 16] {
        let view_right = match self.getf(ohmd_float_value::OHMD_RIGHT_EYE_GL_MODELVIEW_MATRIX) {
            Some(x) => x,
            None => [0.0; 16],
        };
        view_right
    }

    pub fn get_proj_matrix_l(&self) -> [f32; 16] {
        let oproj = match self.getf(ohmd_float_value::OHMD_LEFT_EYE_GL_PROJECTION_MATRIX) {
            Some(x) => x,
            _ => [0.0; 16],
        };
        oproj
    }

    pub fn get_proj_matrix_r(&self) -> [f32; 16] {
        let oproj = match self.getf(ohmd_float_value::OHMD_RIGHT_EYE_GL_PROJECTION_MATRIX) {
            Some(x) => x,
            _ => [0.0; 16],
        };
        oproj
    }

    pub fn get_scr_size_w(&self) -> f32 {
        let scr_size_w = match self.getf(ohmd_float_value::OHMD_SCREEN_HORIZONTAL_SIZE) {
            Some(x) => x[0],
            _ => 0.149760,
        };
        scr_size_w
    }

    pub fn get_scr_size_h(&self) -> f32 {
        let scr_size_h = match self.getf(ohmd_float_value::OHMD_SCREEN_VERTICAL_SIZE) {
            Some(x) => x[0],
            _ => 0.093600,
        };
        scr_size_h
    }

    pub fn get_scr_res_w(&self) -> u32 {
        let scrw = match self.geti(ohmd_int_value::OHMD_SCREEN_HORIZONTAL_RESOLUTION) {
            Some(x) => x,
            _ => 1280,
        } as u32;
        scrw
    }

    pub fn get_scr_res_h(&self) -> u32 {
        let scrh = match self.geti(ohmd_int_value::OHMD_SCREEN_VERTICAL_RESOLUTION) {
            Some(x) => x,
            _ => 800,
        } as u32;
        scrh
    }

    pub fn get_distortion_k(&self) -> [f32; 4] {
        let distortion_k = match self.getf(ohmd_float_value::OHMD_UNIVERSAL_DISTORTION_K) {
            Some(x) => [x[0], x[1], x[2], x[3]],
            _ => [0.0, 0.0, 0.0, 1.0],
        };
        distortion_k
    }

    pub fn get_aberration_k(&self) -> [f32; 3] {
        let aberration_k = match self.getf(ohmd_float_value::OHMD_UNIVERSAL_ABERRATION_K) {
            Some(x) => [x[0], x[1], x[2]],
            _ => [0.0, 0.0, 1.0],
        };
        aberration_k
    }

    pub fn setf(&self, otype: ohmd_float_value, value: &mut [f32; 16]) -> Option<bool> {
        unsafe {
            match ohmd_device_setf(self.device, otype, value) {
                0 => return Some(true),
                _ => return None,
            };
        }
    }

    pub fn geti(&self, otype: ohmd_int_value) -> Option<i32> {
        let mut out: [i32; 1] = [0];
        unsafe {
            match ohmd_device_geti(self.device, otype, &mut out) {
                0 => return Some(out[0]),
                _ => return None,
            };
        }
    }
    fn close(&self) -> i32 {
        unsafe { ohmd_close_device(self.device) as i32 }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        self.close();
    }
}
