use bms_sm::*;
use std::time::Duration;
use std::{thread, time};
use std::net::UdpSocket;
use tailcall::tailcall;

const TICK_SLEEP_TIME: Duration = time::Duration::from_millis(10);
const WAITING_SIM_AND_TELEMETRY_SLEEP_TIME: Duration = time::Duration::from_millis(300);

fn main() {
    let socket_port = 29778;
    let socket_host = "localhost";
    let socket: UdpSocket  = create_random_socket().expect("Can't create socket!");

    connect_to_the_telemetry_socket(&socket, socket_host, socket_port).expect("Can't connect to the telemetry's socket!");

    let flight_data = wait_for_flight_data();
    let intellivibe_data  = wait_for_intellivibe_data();

    main_loop( &socket, flight_data, intellivibe_data);
    println!("Shutting down since BMS is closed");
}

fn main_loop(socked: &UdpSocket, flight_data_file: MemoryFile<'static, FlightData>, intellivibe_data_file: MemoryFile<'static, IntellivibeData>) {
    loop {
        thread::sleep(TICK_SLEEP_TIME);
        let intellivibe_data = intellivibe_data_file.read();
        if intellivibe_data.exit_game {
            break;
        } else {
            send_flight_data_to_the_socket(socked, flight_data_file.read(), intellivibe_data)
        }
    }
}

#[tailcall]
fn wait_for_flight_data() -> MemoryFile<'static, FlightData>  {
    thread::sleep(WAITING_SIM_AND_TELEMETRY_SLEEP_TIME);
    let maybe_fligth_data = FlightData::new();
    match maybe_fligth_data {
        Ok(data) => data,
        Err(_) => wait_for_flight_data(),
    }
}

#[tailcall]
fn wait_for_intellivibe_data() -> MemoryFile<'static, IntellivibeData> {
    thread::sleep(WAITING_SIM_AND_TELEMETRY_SLEEP_TIME);
    let maybe_intellivibe_data = IntellivibeData::new();
    match maybe_intellivibe_data {
        Ok(data) => data,
        Err(_) => wait_for_intellivibe_data(),
    }
}

fn create_random_socket() -> std::io::Result<UdpSocket>  {
    UdpSocket::bind("0.0.0.0:0")
}

fn connect_to_the_telemetry_socket(socket: &UdpSocket, socket_host: &str, socket_port: i32) -> std::io:: Result<()> {
    let socket_address = format!("{socket_host}:{socket_port}");
    socket.connect(socket_address)
}

fn compute_is_on_ground(intellivible_data: &IntellivibeData) -> f32 {
    if intellivible_data.on_ground {
        1.0
    } else {
        0.0
    }
}

fn compute_actual_flight_data(flight_data: &FlightData, intellivible_data: &IntellivibeData) -> String {
    let indicated_airspeed_kmh: f32 = flight_data.kias * 1.852;
    let vertical_speed_kmh: f32 = flight_data.z_dot * 0.3048;
    let aoa: f32 = flight_data.alpha;
    let g_force: f32 = intellivible_data.g_force;
    let gear: f32 = flight_data.gear_pos;
    let airbreak: f32 = flight_data.speed_brake * 100.0;
    let flaps: f32 = 0.0; // idk where to get it
    let thrust: f32 = flight_data.rpm;
    let on_ground: f32 = compute_is_on_ground(intellivible_data);
    let result = format!(
        "bms;{:.2};{:.2};{:.2};{:.2};{:.2};{:.2};{:.2};{:.2};{:.2}\n",
        indicated_airspeed_kmh, vertical_speed_kmh, aoa, g_force, gear, airbreak, flaps, thrust, on_ground);
    result
}

fn compute_zero_data() -> String {
    let result = format!("bms;{:.2};{:.2};{:.2};{:.2};{:.2};{:.2};{:.2};{:.2};{:.2}\n", 0, 0, 0, 0, 0, 0, 0, 0, 0);
    result
}

fn send_flight_data_to_the_socket(socket: &UdpSocket, flight_data: &FlightData, intellivible_data: &IntellivibeData) {
    let data_to_socket: String = if intellivible_data.paused || intellivible_data.ejecting || intellivible_data.end_flight {
      compute_zero_data()
    } else {
      compute_actual_flight_data(flight_data, intellivible_data)
    };
    let data_as_bytes = data_to_socket.as_bytes();
    let _result = socket.send(data_as_bytes);
}