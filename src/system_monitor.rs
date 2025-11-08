use sysinfo::{System, Networks};

// this calculates transfer rate
pub fn bytes_per_sec(prev: u64, now: u64, dt_s: f32) -> f32 {
    if now >= prev {
        (now - prev) as f32 / dt_s
    } else {
        0.0 // handle counter reset
    }
}

// sums the total network traffic (received, transmitted)
pub fn total_net_bytes(nets: &Networks) -> (u64, u64) {
    let mut rx = 0;
    let mut tx = 0;
    for (_name, data) in nets.iter() {
        rx += data.total_received();
        tx += data.total_transmitted();
    }
    (rx, tx)
}

// total bytes read/write from disk
pub fn total_disk_bytes(sys: &System) -> (u64, u64) {
    let mut r = 0;
    let mut w = 0;
    for (_pid, process) in sys.processes() {
        let io = process.disk_usage();
        r += io.total_read_bytes;
        w += io.total_written_bytes;
    }
    (r, w)
}