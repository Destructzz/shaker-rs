use std::time::Duration;
use thiserror::Error;

use serialport::{
    DataBits, Error, FlowControl, Parity, SerialPort, SerialPortType, StopBits, available_ports,
};

use crate::errors::ShakerError;
pub struct Shaker {
    baude: u32,
    serial_port: Option<Box<dyn SerialPort>>,
}

type Result<T> = std::result::Result<T, ShakerError>;

const CONTROLLER_RATE: i32 = 120;

impl Shaker {
    fn new(baude: u32) -> Shaker {
        Shaker {
            baude: baude,
            serial_port: None,
        }
    }

    fn init(&mut self) -> Result<()> {
        let ports = available_ports()?;

        let leonardo_device = self.find_leonardo_port(ports);

        let port = match leonardo_device {
            Some(port) => {
                println!("Порт в использовании: {}", port.port_name);
                port
            }
            None => return Err(ShakerError::NoLeonardo),
        };

        self.serial_port = match self.create_serial_port(port.port_name.as_str()) {
            Err(e) => {
                return Err(e);
            }
            Ok(serial_port) => Some(serial_port),
        };

        Ok(())
    }

    fn find_leonardo_port(
        &self,
        ports: Vec<serialport::SerialPortInfo>,
    ) -> Option<serialport::SerialPortInfo> {
        for port in &ports {
            if let SerialPortType::UsbPort(ref usb_info) = port.port_type {
                println!(
                    "USB: {} VID:{:04x} PID:{:04x}",
                    port.port_name, usb_info.vid, usb_info.pid
                );
                if usb_info.vid == 0x2341 && usb_info.pid == 0x8036 {
                    println!("Найден Arduino Leonardo!");
                    Some(port.clone());
                }
            }
        }
        None
    }

    fn create_serial_port(&self, port_name: &str) -> Result<Box<dyn SerialPort>> {
        let port = serialport::new(port_name, self.baude)
            .timeout(Duration::from_millis(100))
            .data_bits(DataBits::Eight)
            .flow_control(FlowControl::None)
            .parity(Parity::None)
            .stop_bits(StopBits::One)
            .open()?;
        Ok(port)
    }

    fn process(&mut self, dx: i32, dy: i32) -> Result<()> {
        if dx > 5000 || dy > 5000 {
            return Err(ShakerError::PositionNormalOverflow);
        }
        let dx_count: i32 = CONTROLLER_RATE / dx;
        let dy_count: i32 = CONTROLLER_RATE / dy;

        if dx_count == dy_count{
            for i in 0..dx_count{
                self.send_move(120, 120)?;
            }
            self.send_move(CONTROLLER_RATE % dx, CONTROLLER_RATE % dy)?;
        }else{
            let lowest : i32; 
            if dx < dy{
                lowest = dx;
            }else {
                lowest = dy;
            }
        }


        return Ok(());
    }

    fn send_move(&mut self, dx: i32, dy: i32) -> Result<()> {
        if dx > 120 || dy > 120{
            return Err(ShakerError::PositionBytesOverflow);
        }

        match &mut self.serial_port {
            Some(s_port) => {
                s_port.write_all(format!("MOVE {} {}\r\n", dx, dy).as_bytes())?;
            }
            None => {
                return Err(ShakerError::NoSerialPort);
            }
        }
        return Ok(());
    }
}
