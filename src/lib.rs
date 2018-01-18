extern crate openhmd_rs_sys;

use openhmd_rs_sys::*;

pub use openhmd_rs_sys::{ohmd_float_value, ohmd_string_value, ohmd_int_value};

pub struct Context{
    context: &'static ohmd_context
}

pub struct Device{
    device: &'static ohmd_device
}

pub const shader_distortion_vert: &'static str  = r#"
version 120
void main(void)
{
	gl_TexCoord[0] = gl_MultiTexCoord0;
    gl_Position = gl_ProjectionMatrix * gl_ModelViewMatrix * gl_Vertex;
}
"#;

pub const shader_distortion_frag: &'static str = r#"
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

    pub fn destroy(&self){
        unsafe{
            ohmd_ctx_destroy(self.context);
        }
    }

    pub fn list_gets(&self, index: i32, otype: ohmd_string_value) -> i32{
        unsafe{
            ohmd_list_gets(self.context, index, otype) as i32
        }
    }
}

impl Drop for Context{
    fn drop(&mut self){
        self.destroy();
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
    fn close(&self) -> i32{
        unsafe{
            ohmd_close_device(self.device) as i32
        }
    }
}

impl Drop for Device{
    fn drop(&mut self){
        self.close();
    }
}
