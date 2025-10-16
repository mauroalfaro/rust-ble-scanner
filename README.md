# rust-ble-scanner

Small cross‑platform BLE scanner to quickly list nearby devices and watch RSSI for a specific address.

## Local Setup
- Prereqs: recent Rust toolchain (`rustup`)
- Windows 10+: enable Bluetooth and grant the terminal Bluetooth access if prompted
- Linux: ensure Bluetooth is enabled; add your user to the `bluetooth` group or run with the right capabilities
- macOS: enable Bluetooth in System Settings

Run examples:
```
cargo run -- scan --duration 10
cargo run -- watch AA:BB:CC:DD:EE:FF --interval 1
```

## Commands
- `scan` – active scan for a few seconds, then print `address  rssi  name`
- `watch` – find a device by address and print RSSI periodically

## JSON Examples for CLI Output
These examples reflect the CLI output represented as JSON for reference.

`scan`:
```
[
  { "address": "AA:BB:CC:DD:EE:FF", "rssi": -54, "name": "SensorTag" },
  { "address": "11:22:33:44:55:66", "rssi": -72, "name": "Beacon-01" }
]
```

`watch AA:BB:CC:DD:EE:FF --interval 1`:
```
[
  { "rssi": -55 },
  { "rssi": -54 },
  { "rssi": -56 }
]
```

## Notes
- Keep `--duration` short and rerun scans when needed
- On Linux, verify `bluetoothd` is running and the adapter is powered (`bluetoothctl`) if scans return nothing
