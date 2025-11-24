use serialport::{Error, SerialPortType, available_ports};
pub struct Shaker{}

impl Shaker{
    fn check_ports() -> Option<serialport::SerialPortInfo>{
        let ports = available_ports();
        match ports{
            Err(e) => return None,
            Some(vec) => {
                return Some(vec)
            }
        } 
    }

    fn open_serial_port(port_name: String) -> Result<Box<dyn SerialPort>>{

    }
    // fn process(&self);

    fn init(){
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
        let obj: Shaker;
        Ok(())
    }
}