# IoT Device Simulator
IoT Device Simulator， Support DLT645、ModBus、OPC UA

| Device Protocol      | State       |
|----------------------|-------------|
| `DLT645-07 over tcp` | in progress |
| `DLT645-97 over tcp` | in progress |
| `Modbus-TCP`         |             |
| `OPC-UA`             |             |

Build
```bash
cargo build --release  --target x86_64-unknown-linux-gnu
cargo build --release  --target x86_64-pc-windows-gnu
```

