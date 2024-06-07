# uln2003
A simple crate to use a 28BYJ-48 stepper motor with and ULN2003 Driver on any hardware implementing embedded-hal

## Usage

Both esp32 examples use the wiring as shown [in this tutorial](https://randomnerdtutorials.com/esp32-stepper-motor-28byj-48-uln2003/)

### Example on an esp32 using the esp-hal (no_std)
```rust
fn main() {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);

    let clocks = ClockControl::max(system.clock_control).freeze();
    let delay = Delay::new(&clocks);

    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut motor = ULN2003::new(
        Output::new(io.pins.gpio19, Level::Low),
        Output::new(io.pins.gpio18, Level::Low),
        Output::new(io.pins.gpio5, Level::Low),
        Output::new(io.pins.gpio17, Level::Low),
        Some(delay)
    );

    loop {
        motor.step();
        delay.delay_ms(5u32);
    }
}
```

### Example using esp-idf-hal (std)

```rust
struct Delay;
impl embedded_hal::blocking::delay::DelayMs<u32> for Delay {
    fn delay_ms(&mut self, ms: u32) {
        delay::FreeRtos::delay_ms(ms);
    }
}
fn main(){
    let peripherals = Peripherals::take().unwrap();
    let mut motor = ULN2003::new(
        PinDriver::output(peripherals.pins.gpio19).unwrap(),
        PinDriver::output(peripherals.pins.gpio18).unwrap(),
        PinDriver::output(peripherals.pins.gpio5).unwrap(),
        PinDriver::output(peripherals.pins.gpio17).unwrap(),
        Some(Delay),
    );

    // run for 100 steps with 5 ms between steps
    motor.step_for(100, 5).unwrap();

    loop {
        motor.step();
        delay::FreeRtos::delay_ms(5);
    }
}

```
