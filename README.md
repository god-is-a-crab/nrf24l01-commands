# nRF24L01 Commands

This crate provides:
- Bitfield definitions for nRF24L01 registers
- A friendly API for generating SPI byte sequences for nRF24L01 commands

## Examples

### Write CONFIG register
```rust
use nrf24l01_commands::{registers, commands};

let config = registers::Config::new()
    .with_mask_rx_dr(true)
    .with_mask_tx_ds(false)
    .with_mask_max_rt(false)
    .with_en_crc(false)
    .with_crco(true)
    .with_pwr_up(true)
    .with_prim_rx(false);
let write_command = commands::WRegister(config);
let spi_bytes = write_command.bytes();
assert_eq!(spi_bytes, [0b0010_0000, 0b0100_0110]);
```
### Read FIFO_STATUS register
```rust
use nrf24l01_commands::{registers, commands};

let bytes = commands::RRegister::<registers::FifoStatus>::bytes();
assert_eq!(bytes, [0 | 0x17, 0]);
```
### Write TX payload
```rust
use nrf24l01_commands::commands;

let payload = [1, 2, 3, 4, 5, 6, 7, 8, 9];
let bytes = commands::WTxPayload(payload).bytes();
assert_eq!(bytes, [0b1010_0000, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
```
