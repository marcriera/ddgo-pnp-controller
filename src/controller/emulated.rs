use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use crate::controller::physical::ControllerState;

mod dgoc44u;
mod slph00051;
mod sotp031201_p4b2b7;
mod sotp031201_p4b7;
mod sotp031201_p5b5;
mod sotp031201_p5b7;
mod tcpp20009;
mod tcpp20011;
mod zkns001;

const FFS_MOUNT: &str = "/tmp/ffs";
const ENDPOINT0: &str = "/tmp/ffs/ep0";
const ENDPOINT1: &str = "/tmp/ffs/ep1";
const ANDROID_GADGET: &str = "/sys/class/android_usb/android0";

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ControllerModel {
    DGOC44U,
    TCPP20009,
    TCPP20011,
    SOTP031201P4B7,
    SOTP031201P4B2B7,
    SOTP031201P5B5,
    SOTP031201P5B7,
    ZKNS001,
    SLPH00051,
}

pub struct DeviceDescriptor {
    b_device_class: u8,
    b_device_sub_class: u8,
    id_vendor: u16,
    id_product: u16,
    bcd_device: u16,
    i_manufacturer: &'static str,
    i_product: &'static str,
    i_serial_number: &'static str,
}

pub fn set_model(state: &ControllerState) -> Option<ControllerModel> {
    let model;
    let model_name;
    let descriptors: (&DeviceDescriptor, &[u8], &[u8]);
    if state.button_right {
        model_name = "DGOC44-U";
        model = ControllerModel::DGOC44U;
        descriptors = (
            &dgoc44u::DEVICE_DESCRIPTOR,
            &dgoc44u::DESCRIPTORS,
            &dgoc44u::STRINGS,
        );
    } else if state.button_up {
        model_name = "ZKNS-001";
        model = ControllerModel::ZKNS001;
        descriptors = (
            &zkns001::DEVICE_DESCRIPTOR,
            &zkns001::DESCRIPTORS,
            &zkns001::STRINGS,
        );
    } else if state.button_down && state.power == 0 {
        model_name = "SLPH-00051";
        model = ControllerModel::SLPH00051;
        descriptors = (
            &slph00051::DEVICE_DESCRIPTOR,
            &slph00051::DESCRIPTORS,
            &slph00051::STRINGS,
        );
    } else if state.button_d {
        model_name = "TCPP-20009";
        model = ControllerModel::TCPP20009;
        descriptors = (
            &tcpp20009::DEVICE_DESCRIPTOR,
            &tcpp20009::DESCRIPTORS,
            &tcpp20009::STRINGS,
        );
    } else if state.button_b {
        model_name = "TCPP-20011";
        model = ControllerModel::TCPP20011;
        descriptors = (
            &tcpp20011::DEVICE_DESCRIPTOR,
            &tcpp20011::DESCRIPTORS,
            &tcpp20011::STRINGS,
        );
    } else if state.button_c && state.power == 0 {
        model_name = "SOTP-031201 (P4/B7 mode)";
        model = ControllerModel::SOTP031201P4B7;
        descriptors = (
            &sotp031201_p4b7::DEVICE_DESCRIPTOR,
            &sotp031201_p4b7::DESCRIPTORS,
            &sotp031201_p4b7::STRINGS,
        );
    } else if state.button_c && state.power == 1 {
        model_name = "SOTP-031201 (P4/B2-B7 mode)";
        model = ControllerModel::SOTP031201P4B2B7;
        descriptors = (
            &sotp031201_p4b2b7::DEVICE_DESCRIPTOR,
            &sotp031201_p4b2b7::DESCRIPTORS,
            &sotp031201_p4b2b7::STRINGS,
        );
    } else if state.button_c && state.power == 2 {
        model_name = "SOTP-031201 (P5/B5 mode)";
        model = ControllerModel::SOTP031201P5B5;
        descriptors = (
            &sotp031201_p5b5::DEVICE_DESCRIPTOR,
            &sotp031201_p5b5::DESCRIPTORS,
            &sotp031201_p5b5::STRINGS,
        );
    } else if state.button_c && state.power == 3 {
        model_name = "SOTP-031201 (P5/B7 mode)";
        model = ControllerModel::SOTP031201P5B7;
        descriptors = (
            &sotp031201_p5b7::DEVICE_DESCRIPTOR,
            &sotp031201_p5b7::DESCRIPTORS,
            &sotp031201_p5b7::STRINGS,
        );
    } else {
        println!("ddgo-pnp-controller: No controller selected, starting RNDIS gadget.");
        Command::new("rndis-gadget.sh").output().ok();
        return None;
    }
    println!("ddgo-pnp-controller: Selected controller {}.", model_name);
    init_gadget(&model, descriptors);
    return Some(model);
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
        ControllerModel::ZKNS001 => {
            zkns001::update_gadget(state);
        }
        ControllerModel::SLPH00051 => {
            slph00051::update_gadget(state);
        }
    }
}

pub fn handle_ctrl_transfer(model: ControllerModel, data: &[u8]) {
    println!("ddgo-pnp-controller: CTRL REQ: {:?}", data);
    if data[1] == 6 && data[3] == 34 {
        // Get HID report descriptor
        let report: Option<&[u8]>;
        match model {
            ControllerModel::DGOC44U => {
                report = Some(&dgoc44u::HID_REPORT_DESCRIPTOR);
            }
            ControllerModel::ZKNS001 => {
                report = Some(&zkns001::HID_REPORT_DESCRIPTOR);
            }
            ControllerModel::SLPH00051 => {
                report = Some(&slph00051::HID_REPORT_DESCRIPTOR);
            }
            _ => {
                report = None;
            }
        }
        match report {
            Some(rep) => {
                if let Ok(mut file) = File::create(ENDPOINT0) {
                    file.write(rep).ok();
                }
            }
            None => (),
        }
    } else {
        match model {
            ControllerModel::SLPH00051 => {
                slph00051::handle_ctrl_transfer(data);
            }
            _ => (),
        }
    }
}

fn init_gadget(
    model: &ControllerModel,
    (device, descriptors, strings): (&DeviceDescriptor, &[u8], &[u8]),
) {
    // Init g_ffs kernel module
    Command::new("modprobe")
        .arg("g_ffs")
        .arg(String::from("bDeviceClass=") + &device.b_device_class.to_string())
        .arg(String::from("bDeviceSubClass=") + &device.b_device_sub_class.to_string())
        .arg(String::from("idVendor=") + &device.id_vendor.to_string())
        .arg(String::from("idProduct=") + &device.id_product.to_string())
        .arg(String::from("bcdDevice=") + &device.bcd_device.to_string())
        .arg(String::from("iManufacturer=") + device.i_manufacturer)
        .arg(String::from("iProduct=") + device.i_product)
        .arg(String::from("iSerialNumber=") + device.i_serial_number)
        .output()
        .ok();
    Command::new("mkdir").args(["-p", &FFS_MOUNT]).output().ok();
    Command::new("mount")
        .args(["-t", "functionfs", "ffs", &FFS_MOUNT])
        .output()
        .ok();

    let controller_model = model.clone();

    thread::spawn(move || {
        if let Ok(mut ep0) = File::open(&ENDPOINT0) {
            let mut buffer = [0; 12];
            loop {
                if let Ok(_result) = ep0.read(&mut buffer) {
                    if buffer[8] == 0x4 {
                        // Control transfer received
                        handle_ctrl_transfer(controller_model, &buffer[0..8]);
                    }
                }
                // Wait between cycles
                sleep(Duration::from_millis(10));
            }
        }
    });
    if let Ok(mut ep0) = File::create(&ENDPOINT0) {
        ep0.write_all(descriptors).ok();
        println!("ddgo-pnp-controller: Descriptors written to EP0");
        ep0.write_all(strings).ok();
        println!("ddgo-pnp-controller: Strings written to EP0");
    }

    // Init Android Gadget for old 3.4 kernel
    let gadget = Path::new(&ANDROID_GADGET);
    if gadget.is_dir() {
        fs::write(
            gadget.join(Path::new("bDeviceClass")),
            &device.b_device_class.to_string(),
        )
        .ok();
        fs::write(
            gadget.join(Path::new("bDeviceSubClass")),
            &device.b_device_sub_class.to_string(),
        )
        .ok();
        fs::write(
            gadget.join(Path::new("idVendor")),
            format!("{:x}", &device.id_vendor),
        )
        .ok();
        fs::write(
            gadget.join(Path::new("idProduct")),
            format!("{:x}", &device.id_product),
        )
        .ok();
        fs::write(
            gadget.join(Path::new("bcdDevice")),
            format!("{:x}", &device.bcd_device),
        )
        .ok();
        fs::write(
            gadget.join(Path::new("iManufacturer")),
            &device.i_manufacturer.to_string(),
        )
        .ok();
        fs::write(
            gadget.join(Path::new("iProduct")),
            &device.i_product.to_string(),
        )
        .ok();
        fs::write(
            gadget.join(Path::new("iSerial")),
            &device.i_serial_number.to_string(),
        )
        .ok();
        fs::write(gadget.join(Path::new("functions")), "ffs").ok();
        fs::write(gadget.join(Path::new("f_ffs/aliases")), "ffs").ok();
        fs::write(gadget.join(Path::new("enable")), "1").ok();
    }
}
