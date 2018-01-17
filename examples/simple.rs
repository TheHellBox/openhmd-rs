extern crate openhmd_rs;
use openhmd_rs::*;

fn main(){
    let context = Context::new();
    println!("{}", context.probe());
    let device = context.list_open_device(0);
    context.update();
    println!("OHMD_ROTATION_QUAT {:?}", device.getf(ohmd_float_value::OHMD_ROTATION_QUAT));
    println!("OHMD_SCREEN_HORIZONTAL_RESOLUTION {:?}", device.geti(ohmd_int_value::OHMD_SCREEN_HORIZONTAL_RESOLUTION));
    println!("OHMD_SCREEN_VERTICAL_RESOLUTION {:?}", device.geti(ohmd_int_value::OHMD_SCREEN_VERTICAL_RESOLUTION));
}
