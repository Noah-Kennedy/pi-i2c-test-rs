use rppal::i2c;

const I2C_TX_MESSAGE: [u8; 4] = [0xde, 0xad, 0xbe, 0xef];

fn main() {
    let mut bus = i2c::I2c::new().unwrap();

    bus.set_slave_address(7).unwrap();

    let mut rx_buffer: [u8; 32] = [0; 32];

    loop {
        let tx_length = bus.write(&I2C_TX_MESSAGE).unwrap();
        println!("TX: {:x?}", I2C_TX_MESSAGE);
        println!("Wrote {} bytes", tx_length);

        let rx_length = bus.read(&mut rx_buffer).unwrap();
        println!("RX: {:x?}", rx_buffer);
        println!("Read {} bytes", rx_length);
    }
}
