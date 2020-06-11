use nannou_osc as osc;
use rppal as rpi;
use std::{thread, time};

// The amount of time to wait in between each data update
const SLEEPMS: u64 = 50;

// Which bcm pin are we going to read from?
const BUTPIN: u8 = 23;

// Data model for the program
struct Model {
    target_addr: String,
    sender: osc::Sender<osc::Connected>,
    gpio: rpi::gpio::Gpio,
}

// Populate data model
fn model() -> Model {
    let port = 1234;
    let target_addr = format!("{}:{}", "127.0.0.1", port);
    let sender = osc::sender()
        .expect("Could not bind to default socket")
        .connect(&target_addr)
        .expect("Could not connect to socket at address");

    let gpio = rpi::gpio::Gpio::new().expect("Could not get gpio access");

    Model {
        target_addr,
        sender,
        gpio,
    }
}

fn make_packet_from(value: i32, osc_addr: String) -> (String, Vec<nannou_osc::Type>) {
    let args = vec![osc::Type::Int(value)];
    let packet = (osc_addr, args);

    packet
}

fn wait() {
    let sleeptime = time::Duration::from_millis(SLEEPMS);
    thread::sleep(sleeptime);
}

fn main() {
    let model = model();
    let mut state = false;
    let pin = model
        .gpio
        .get(BUTPIN)
        .expect("Could not get pin")
        .into_input();

    println!("Sending OSC messages to {}", model.target_addr);

    loop {
        // 1. Poll gpio pins
        let polledstate = pin.is_high();

        // 2. If change to state, send new state
        if polledstate != state {
            // Update state
            state = polledstate;

            // Print new value
            print!("State changed: {}", state);

            // Target message address
            let osc_addr = "/gpio/button/1".to_string();

            // Create network packet
            let packet = make_packet_from(state as i32, osc_addr);

            // Send network packet if possible
            model.sender.send(packet).ok();
        }

        // 3. Wait
        wait();
    }
}
