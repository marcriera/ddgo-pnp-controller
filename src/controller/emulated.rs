use std::thread;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::process::Command;
use std::path::Path;

use crate::controller::physical::ControllerState;
mod dgoc44u;
mod tcpp20009;
mod tcpp20011;
mod sotp031201_p4b7;
mod sotp031201_p4b2b7;
mod sotp031201_p5b5;
mod sotp031201_p5b7;
mod vok00106;

const FFS_MOUNT: &str = "/tmp/ffs";
const ENDPOINT0: &str = "/tmp/ffs/ep0";
const ENDPOINT1: &str = "/tmp/ffs/ep1";
const ANDROID_GADGET: &str = "/sys/class/android_usb/android0";

#[derive(PartialEq)]
pub enum ControllerModel {
    DGOC44U,
    TCPP20009,
    TCPP20011,
    SOTP031201P4B7,
    SOTP031201P4B2B7,
    SOTP031201P5B5,
    SOTP031201P5B7,
    VOK00106,
}

pub struct DeviceDescriptor {
    b_device_class: u8,
    b_device_sub_class: u8,
    id_vendor: u16,
    id_product: u16,
    i_manufacturer: &'static str,
    i_product: &'static str,
    i_serial_number: &'static str,
}

pub fn set_model(state: &ControllerState) -> Option<ControllerModel> {
    if state.button_right {
        println!("Selected controller DGOC44-U.");
        init_gadget(&dgoc44u::DEVICE_DESCRIPTOR, &dgoc44u::DESCRIPTORS, &dgoc44u::STRINGS);
        return Some(ControllerModel::DGOC44U);
    }
    else if state.button_d {
        println!("Selected controller TCPP-20009.");
        init_gadget(&tcpp20009::DEVICE_DESCRIPTOR, &tcpp20009::DESCRIPTORS, &tcpp20009::STRINGS);
        return Some(ControllerModel::TCPP20009);
    }
    else if state.button_b {
        println!("Selected controller TCPP-20011.");
        init_gadget(&tcpp20011::DEVICE_DESCRIPTOR, &tcpp20011::DESCRIPTORS, &tcpp20011::STRINGS);
        return Some(ControllerModel::TCPP20011);
    }
    else if state.button_c && state.power == 0 {
        println!("Selected controller SOTP-031201 (P4/B7 mode).");
        init_gadget(&sotp031201_p4b7::DEVICE_DESCRIPTOR, &sotp031201_p4b7::DESCRIPTORS, &sotp031201_p4b7::STRINGS);
        return Some(ControllerModel::SOTP031201P4B7);
    }
    else if state.button_c && state.power == 1 {
        println!("Selected controller SOTP-031201 (P4/B2-B7 mode).");
        init_gadget(&sotp031201_p4b2b7::DEVICE_DESCRIPTOR, &sotp031201_p4b2b7::DESCRIPTORS, &sotp031201_p4b2b7::STRINGS);
        return Some(ControllerModel::SOTP031201P4B2B7);
    }
    else if state.button_c && state.power == 2 {
        println!("Selected controller SOTP-031201 (P5/B5 mode).");
        init_gadget(&sotp031201_p5b5::DEVICE_DESCRIPTOR, &sotp031201_p5b5::DESCRIPTORS, &sotp031201_p5b5::STRINGS);
        return Some(ControllerModel::SOTP031201P5B5);
    }
    else if state.button_c && state.power == 3 {
        println!("Selected controller SOTP-031201 (P5/B7 mode).");
        init_gadget(&sotp031201_p5b7::DEVICE_DESCRIPTOR, &sotp031201_p5b7::DESCRIPTORS, &sotp031201_p5b7::STRINGS);
        return Some(ControllerModel::SOTP031201P5B7);
    }
/*     else if state.button_a {
        println!("Selected controller VOK-00106.");
        return Some(ControllerModel::VOK00106);
    } */
    else {
        println!("No controller selected.");
        return None;
    }
}

pub fn set_state(state: &mut ControllerState, model: &ControllerModel) {
    match model {
        ControllerModel::DGOC44U => {
            dgoc44u::update_gadget(state);
        }
        ControllerModel::TCPP20009 => {
            tcpp20009::update_gadget(state);
        }
        ControllerModel::TCPP20011 => {
            tcpp20011::update_gadget(state);
        }
        ControllerModel::SOTP031201P4B7 => {
            sotp031201_p4b7::update_gadget(state);
        }
        ControllerModel::SOTP031201P4B2B7 => {
            sotp031201_p4b2b7::update_gadget(state);
        }
        ControllerModel::SOTP031201P5B5 => {
            sotp031201_p5b5::update_gadget(state);
        }
        ControllerModel::SOTP031201P5B7 => {
            sotp031201_p5b7::update_gadget(state);
        }
        ControllerModel::VOK00106 => {
            vok00106::update_gadget(state);
        }
    }
}

fn init_gadget(device: &DeviceDescriptor, descriptors: &[u8], strings: &[u8]) {
    // Init g_ffs kernel module
    Command::new("modprobe").arg("g_ffs")
    .arg(String::from("bDeviceClass=")+&device.b_device_class.to_string())
    .arg(String::from("bDeviceSubClass=")+&device.b_device_sub_class.to_string())
    .arg(String::from("idVendor=")+&device.id_vendor.to_string())
    .arg(String::from("idProduct=")+&device.id_product.to_string())
    .arg(String::from("iManufacturer=")+device.i_manufacturer)
    .arg(String::from("iProduct=")+device.i_product)
    .arg(String::from("iSerialNumber=")+device.i_serial_number)
    .output().ok();
    Command::new("mkdir").args(["-p",&FFS_MOUNT]).output().ok();
    Command::new("mount").args(["-t","functionfs","ffs",&FFS_MOUNT]).output().ok();

    thread::spawn(move || {
        if let Ok(mut ep0) = File::open(&ENDPOINT0) {
            let mut buffer = [0; 4];
            loop {
                ep0.read(&mut buffer).ok();
            }
        }
    });
    if let Ok(mut ep0) = File::create(&ENDPOINT0) {
        ep0.write_all(descriptors).ok();
        println!("USB Gadget: Descriptors written to EP0");
        ep0.write_all(strings).ok();
        println!("USB Gadget: Strings written to EP0");
    }

    // Init Android Gadget for old 3.4 kernel
    let gadget = Path::new(&ANDROID_GADGET);
    if gadget.is_dir() {
        fs::write(gadget.join(Path::new("bDeviceClass")), &device.b_device_class.to_string()).ok();
        fs::write(gadget.join(Path::new("bDeviceSubClass")), &device.b_device_sub_class.to_string()).ok();
        fs::write(gadget.join(Path::new("idVendor")), format!("{:x}", &device.id_vendor)).ok();
        fs::write(gadget.join(Path::new("idProduct")), format!("{:x}", &device.id_product)).ok();
        fs::write(gadget.join(Path::new("iManufacturer")), &device.i_manufacturer.to_string()).ok();
        fs::write(gadget.join(Path::new("iProduct")), &device.i_product.to_string()).ok();
        fs::write(gadget.join(Path::new("iSerial")), &device.i_serial_number.to_string()).ok();
        fs::write(gadget.join(Path::new("functions")), "ffs").ok();
        fs::write(gadget.join(Path::new("f_ffs/aliases")), "ffs").ok();
        fs::write(gadget.join(Path::new("enable")), "1").ok();
    }
}
