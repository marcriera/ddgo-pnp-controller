use std::thread;
use std::fs::File;
use std::io::{Read, Write};
use std::process::Command;

use crate::controller::physical::ControllerState;
mod dgoc44u;
mod tcpp20009;
mod vok00106;

#[derive(PartialEq)]
pub enum ControllerModel {
    DGOC44U,
    TCPP20009,
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
    else if state.button_up {
        println!("Selected controller TCPP-20009.");
        init_gadget(&tcpp20009::DEVICE_DESCRIPTOR, &tcpp20009::DESCRIPTORS, &tcpp20009::STRINGS);
        return Some(ControllerModel::TCPP20009);
    }
    else if state.button_a {
        println!("Selected controller VOK-00106.");
        return Some(ControllerModel::VOK00106);
    }
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
        ControllerModel::VOK00106 => {
            vok00106::update_gadget(state);
        }
    }
}

fn init_gadget(device: &DeviceDescriptor, descriptors: &[u8], strings: &[u8]) {
    Command::new("modprobe").arg("g_ffs")
    .arg(String::from("bDeviceClass=")+&device.b_device_class.to_string())
    .arg(String::from("bDeviceSubclass=")+&device.b_device_sub_class.to_string())
    .arg(String::from("idVendor=")+&device.id_vendor.to_string())
    .arg(String::from("idProduct=")+&device.id_product.to_string())
    .arg(String::from("iManufacturer=")+device.i_manufacturer)
    .arg(String::from("iProduct=")+device.i_product)
    .arg(String::from("iSerialNumber=")+device.i_serial_number)
    .output().ok();
    Command::new("mkdir").args(["-p","/tmp/ffs-mascon"]).output().ok();
    Command::new("mount").args(["-t","functionfs","mascon","/tmp/ffs-mascon"]).output().ok();

    thread::spawn(move || {
        if let Ok(mut ep0) = File::open("/tmp/ffs-mascon/ep0") {
            let mut buffer = [0; 100];
            loop {
                ep0.read(&mut buffer).ok();
            }
        }
    });
    if let Ok(mut ep0) = File::create("/tmp/ffs-mascon/ep0") {
        ep0.write(descriptors).ok();
        println!("USB Gadget: Descriptors written to EP0");
        ep0.write(strings).ok();
        println!("USB Gadget: Strings written to EP0");
    }
}
