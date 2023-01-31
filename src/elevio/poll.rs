use crossbeam_channel as cbc;
use std::thread;
use std::time;
use std::convert::TryFrom;

use super::elev;
use super::elev::Call;

#[derive(Debug)]
pub struct CallButton {
    pub floor: u8,
    pub call: Call,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Message {
    call_up(u8),
    call_down(u8),
    cab(u8),
    floor_sensor(u8),
    stop_button(bool),
    obstruction(bool)
}

pub fn call_buttons(elev: elev::Elevator, ch: cbc::Sender<Message>, period: time::Duration) {
    let mut prev = vec![[false; 3]; elev.num_floors.into()];
    loop {
        for floor in 0..elev.num_floors { // Check all the floors
            // Aligned with Elev::Call
            let calls = [Message::call_up(floor),Message::call_down(floor), Message::cab(floor)];

            for (idx, c) in calls.iter().enumerate() { // Check all the buttons..

                let buttonPressed = elev.call_button(floor, Call::try_from(idx as u8 ).unwrap());
                if buttonPressed && buttonPressed != prev[floor as usize][idx] {
                    ch.send(calls[idx].clone()).unwrap();
                }
                prev[floor as usize][idx] = buttonPressed;
            }
        }
        thread::sleep(period)
    }
}

pub fn floor_sensor(elev: elev::Elevator, ch: cbc::Sender<Message>, period: time::Duration) {
    let mut prev = u8::MAX;
    loop {
        if let Some(f) = elev.floor_sensor() {
            if f != prev {
                ch.send(Message::floor_sensor(f)).unwrap();
                prev = f;
            }
        }
        thread::sleep(period)
    }
}

pub fn stop_button(elev: elev::Elevator, ch: cbc::Sender<Message>, period: time::Duration) {
    let mut prev = false;
    loop {
        let v = elev.stop_button();
        if prev != v {
            ch.send(Message::stop_button(v)).unwrap();
            prev = v;
        }
        thread::sleep(period)
    }
}

pub fn obstruction(elev: elev::Elevator, ch: cbc::Sender<Message>, period: time::Duration) {
    let mut prev = false;
    loop {
        let v = elev.obstruction();
        if prev != v {
            ch.send(Message::obstruction(v)).unwrap();
            prev = v;
        }
        thread::sleep(period)
    }
}
