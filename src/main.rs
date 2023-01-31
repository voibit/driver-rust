use std::collections::HashSet;
use std::thread::*;
use std::time::*;

use crossbeam_channel as cbc;

use driver_rust::elevio::{elev, elev::Call, poll};
use driver_rust::elevio::elev as e;
use std::convert::TryFrom;
use driver_rust::elevio::poll::Message;


fn main() -> () {
    let elev_num_floors = 4;
    let elevator = e::Elevator::init("localhost:15657", elev_num_floors).unwrap();
    println!("Elevator started:\n{:#?}", elevator);
    let poll_period = Duration::from_millis(25);

    let (tx, rx) = cbc::unbounded::<Message>();

    let pools = [poll::call_buttons, poll::floor_sensor, poll::stop_button, poll::obstruction];
    for poll in pools {
        let elevator = elevator.clone();
        let tx = tx.clone();
        spawn(move || poll(elevator, tx, poll_period));
    }


    let mut dirn = e::Direction::Down;
    if elevator.floor_sensor().is_none() {
        elevator.motor_direction(dirn.clone());
    }

    let mut stops_downward : HashSet<u8> = HashSet::with_capacity(elev_num_floors as usize -1);
    let mut stops_upward : HashSet<u8> = HashSet::with_capacity(elev_num_floors as usize -1);

    loop {
        match rx.recv() {
            Ok(Message::call_up(f)) => {
                stops_upward.insert(f);
                println!("Call UP from floor {}!",f);
            },
            Ok(Message::call_down(f)) => {
                stops_downward.insert(f);
                println!("Call Down from floor {}!",f);
            },
            /*
            Message::call_down(f) =>,
            Message::cab(f) =>,
            Message::floor_sensor(f) =>,
            Message::obstruction(b) =>,
            Message::stop_button(b) =>,
            */
            _ => ()
        }

        // Do stuff.
    }
    /*
        cbc::select! {
            recv(call_button_rx) -> a => {
                let call_button = a.unwrap();
                println!("{:#?}", call_button);
                elevator.call_button_light(call_button.floor, call_button.call, true);
            },
            recv(floor_sensor_rx) -> a => {
                let floor = a.unwrap();
                println!("Floor: {:#?}", floor);
                dirn =
                    if floor == 0 {
                        e::Direction::Up
                    } else if floor == elev_num_floors-1 {
                        e::Direction::Down
                    } else {
                        dirn
                    };
                elevator.motor_direction(dirn.clone());
            },
            recv(stop_button_rx) -> a => {
                let stop = a.unwrap();
                println!("Stop button: {:#?}", stop);
                for floor in 0..elev_num_floors {
                    for call in [Call::Up, Call::Down, Call::Cab] {
                        elevator.call_button_light(floor, call , false);
                    }
                }
            },
            recv(obstruction_rx) -> a => {
                let obstr = a.unwrap();
                println!("Obstruction: {:#?}", obstr);
                elevator.motor_direction(if obstr { e::Direction::Stop } else { dirn.clone() });
            },
        }
    }

     */
}
