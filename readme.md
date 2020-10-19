![](https://img.shields.io/crates/v/fk-lis3dsh.svg)
# embedded-hal driver for LIS3DSH 

## Usage

Include the library in Your Cargo.toml
```
[dependencies.fk-lis3dsh]
version = "0.1.0"
```


Use embedded-hal to create spi and cs and create accelerometer: 
 
Create accelerometer with default configuration, only SPIBus implemented for now.

```
use fk_lis3dsh::{LIS3DSH, RawAccelerometer};

let mut acc =
            LIS3DSH::new_with_interface(lis3dsh::commbus::SPIBus::new(spi, cs), &mut delay).unwrap();
```

Access accelerometer data 
```
if acc.has_data().unwrap() {
    let accel = acc.accel_raw().unwrap();
    rprintln!("{}\t{}\t{}",
                accel.x,
                accel.y,
                accel.z,
    );
}
```
