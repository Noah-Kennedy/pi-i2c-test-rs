use i2cdev::linux::LinuxI2CDevice;
use i2cdev::core::I2CDevice;

const I2C_TX_MESSAGE: [u8; 4] = [0xde, 0xad, 0xbe, 0xef];

fn main() {
    let mut bus = LinuxI2CDevice::new("/dev/i2c-1", 7).unwrap();

    bus.set_slave_address(7).unwrap();

    let mut rx_buffer: [u8; 32] = [0; 32];

    loop {
        bus.write(&I2C_TX_MESSAGE).unwrap();
        println!("TX: {:x?}", I2C_TX_MESSAGE);

        bus.read(&mut rx_buffer).unwrap();
        println!("RX: {:x?}", rx_buffer);
    }
}
