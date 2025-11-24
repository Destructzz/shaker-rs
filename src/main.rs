use std::{thread, time::Duration};
use serialport::{DataBits, FlowControl, Parity, SerialPortType, StopBits, available_ports};

mod shaker;
use shaker::Shaker;


const BAUDE_RATE: u32 = 2_000_000;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ports = available_ports()?;


    let mut leonardo_device: Option<serialport::SerialPortInfo> = None;

    for port in &ports{
        if let SerialPortType::UsbPort(ref usb_info) = port.port_type{
            println!("USB: {} VID:{:04x} PID:{:04x}", port.port_name, usb_info.vid, usb_info.pid);
            if usb_info.vid == 0x2341 && usb_info.pid == 0x8036{
                println!("Найден Arduino Leonardo!");
                leonardo_device = Some(port.clone());
                break;
            }
        }
    }

    let port = match leonardo_device {
        Some(port) => {
            println!("Порт в использовании: {}", port.port_name);
            port
        },
        None => {
            println!("Arduino Leonardo не найден!");
            return Ok(());  // Или exit(1)
        }
    };

    let mut serial_port = serialport::new(port.port_name, BAUDE_RATE).timeout(Duration::from_millis(100))
            .data_bits(DataBits::Eight)
            .flow_control(FlowControl::None)
            .parity(Parity::None)
            .stop_bits(StopBits::One)
            .open()?;


    // thread::sleep(Duration::from_millis(2000));
    // for _ in 1..10{
    //     serial_port.write_all(b"MOVE 50 0\r\n")?;
    //     thread::sleep(Duration::from_millis(500));
    // }

    let obj: Shaker;
    
    Ok(())


    // главный rolling poll который проверяет нажатие клавиш
    // сервис который определяет что нажато а что нет (нажатие клавиши / нажатие кнопок контроллера)
    //сервис работы с контроллером если логика будет слишком сложная для одного if 
    // сам метод джитера, само испольнение либо COM либо winapi вызов(можно не делать).
    //паттерн что будет делать если сервис определения даст добро. можно хард код
}
