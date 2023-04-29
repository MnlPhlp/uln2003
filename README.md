# uln2003
A simple crate to use a 28BYJ-48 stepper motor with and ULN2003 Driver on any hardware implementing embedded-hal

## Usage

Both esp32 examples use the wiring as shown [in this tutorial](https://randomnerdtutorials.com/esp32-stepper-motor-28byj-48-uln2003/)

### Example on an esp32 using the esp-hal (no_std)
```rust
let peripherals = Peripherals::take();
let system = peripherals.DPORT.split();
let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
let mut delay = Delay::new(&clocks);
let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
let mut motor = ULN2003::new(
    io.pins.gpio19.into_push_pull_output(),
    io.pins.gpio18.into_push_pull_output(),
    io.pins.gpio5.into_push_pull_output(),
    io.pins.gpio17.into_push_pull_output(),
);

loop {
    motor.step();
    delay.delay_ms(5u32);
}
```

### Example using esp-idf-hal (std)
```rust
let peripherals = Peripherals::take().unwrap();
    let mut motor = ULN2003::new(
        PinDriver::output(peripherals.pins.gpio19).unwrap(),
        PinDriver::output(peripherals.pins.gpio18).unwrap(),
        PinDriver::output(peripherals.pins.gpio5).unwrap(),
        PinDriver::output(peripherals.pins.gpio17).unwrap(),
    );

    loop {
        motor.step();
        delay::FreeRtos::delay_ms(5);
    }

```
