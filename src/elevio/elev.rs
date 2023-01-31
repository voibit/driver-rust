#![allow(dead_code)]

use std::convert::TryFrom;
use std::fmt;
use std::io::*;
use std::net::TcpStream;
use std::sync::*;

#[derive(Clone, Debug)]
pub struct Elevator {
    socket: Arc<Mutex<TcpStream>>,
    pub num_floors: u8,
}

#[derive(Debug, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum Direction {
    Stop = 0,
    Up = 1,
    Down = 255,
}

#[derive(Debug, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum Call {
    Up = 0,   // Outside elevator, hall
    Down = 1, // Outside elevator, hall
    Cab = 2,  // Inside elevator, command
}

impl TryFrom<u8> for Call {
    type Error = &'static str;
    fn try_from(v: u8) -> std::result::Result<Self, Self::Error> {
        match v {
            x if x == Call::Up as u8 => Ok(Call::Up),
            x if x == Call::Down as u8 => Ok(Call::Down),
            x if x == Call::Cab as u8 => Ok(Call::Cab),
            _ => Err("Not valid"),
        }
    }
}

impl Elevator {
    pub fn init(addr: &str, num_floors: u8) -> Result<Elevator> {
        Ok(Self {
            socket: Arc::new(Mutex::new(TcpStream::connect(addr)?)),
            num_floors,
        })
    }

    pub fn motor_direction(&self, dirn: Direction) {
        let buf = [1, dirn as u8, 0, 0];
        let mut sock = self.socket.lock().unwrap();
        sock.write(&buf).unwrap();
    }

    pub fn call_button_light(&self, floor: u8, call: Call, on: bool) {
        let buf = [2, call as u8, floor, on as u8];
        let mut sock = self.socket.lock().unwrap();
        sock.write(&buf).unwrap();
    }

    pub fn floor_indicator(&self, floor: u8) {
        let buf = [3, floor, 0, 0];
        let mut sock = self.socket.lock().unwrap();
        sock.write(&buf).unwrap();
    }

    pub fn door_light(&self, on: bool) {
        let buf = [4, on as u8, 0, 0];
        let mut sock = self.socket.lock().unwrap();
        sock.write(&buf).unwrap();
    }

    pub fn stop_button_light(&self, on: bool) {
        let buf = [5, on as u8, 0, 0];
        let mut sock = self.socket.lock().unwrap();
        sock.write(&buf).unwrap();
    }

    pub fn call_button(&self, floor: u8, call: Call) -> bool {
        let mut buf = [6, call as u8, floor, 0];
        let mut sock = self.socket.lock().unwrap();
        sock.write(&mut buf).unwrap();
        sock.read(&mut buf).unwrap();
        buf[1] != 0
    }
    pub fn call_button_raw(&self, floor: u8, call: u8) -> bool {
        let mut buf = [6, call, floor, 0];
        let mut sock = self.socket.lock().unwrap();
        sock.write(&mut buf).unwrap();
        sock.read(&mut buf).unwrap();
        buf[1] != 0
    }

    pub fn floor_sensor(&self) -> Option<u8> {
        let mut buf = [7, 0, 0, 0];
        let mut sock = self.socket.lock().unwrap();
        sock.write(&buf).unwrap();
        sock.read(&mut buf).unwrap();
        if buf[1] != 0 {
            Some(buf[2])
        } else {
            None
        }
    }

    pub fn stop_button(&self) -> bool {
        let mut buf = [8, 0, 0, 0];
        let mut sock = self.socket.lock().unwrap();
        sock.write(&buf).unwrap();
        sock.read(&mut buf).unwrap();
        buf[1] != 0
    }

    pub fn obstruction(&self) -> bool {
        let mut buf = [9, 0, 0, 0];
        let mut sock = self.socket.lock().unwrap();
        sock.write(&buf).unwrap();
        sock.read(&mut buf).unwrap();
        buf[1] != 0
    }
}

impl fmt::Display for Elevator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let addr = self.socket.lock().unwrap().peer_addr().unwrap();
        write!(f, "Elevator@{}({})", addr, self.num_floors)
    }
}
