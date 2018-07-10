extern crate openhmd_rs;
use openhmd_rs::*;

fn main(){
    let context = Context::new();
    println!("{}", context.probe());
    let device = context.list_open_device(0);
    context.update();
    println!("OHMD_ROTATION_QUAT {:?}", device.get_rotation_quat());
    println!("OHMD_SCREEN_HORIZONTAL_RESOLUTION {:?}", device.get_scr_res_w());
    println!("OHMD_SCREEN_VERTICAL_RESOLUTION {:?}", device.get_scr_res_h());
}
