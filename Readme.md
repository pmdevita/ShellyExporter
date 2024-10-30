# Shelly Plus Plug US Prometheus Exporter

Exports power usage info from the Shelly Plus Plug US to Prometheus. 
Written in Rust (with a lot of help from Claude) as a learning exercise.


## Usage

Set the following env vars (.env file works too)

- `HOST` - The network interface to bind the webserver to (default: 127.0.0.1)
- `PORT` - The port to bind the webserver to (default: 9000)
- `SHELLY_HOST` - The address of the Shelly Plug to request (required)
- `SHELLY_DEVICE_ID` - The device ID to request stats for (default: 0)

