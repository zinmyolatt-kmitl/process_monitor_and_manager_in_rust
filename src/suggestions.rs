use crate::models::{ProcRow, Suggestion};
use crate::util::fmt_bytes;

pub fn make_suggestions(rows: &[ProcRow], total_cpu: f32, mem_pct: f32) -> Vec<Suggestion> {
    let mut out = Vec::new();
    
    if total_cpu > 10.0 {
        if let Some(top) = rows.iter().max_by(|a, b| a.cpu.total_cmp(&b.cpu)) {
            out.push(Suggestion {
                title: format!("High CPU: {} at {:.1}%", top.name, top.cpu),
                detail: format!(
                    "Consider suspending or killing PID {} if it's misbehaving.",
                    top.pid
                ),
            });
        }
    }
    
    if mem_pct > 20.0 {
        if let Some(top) = rows.iter().max_by_key(|p| p.mem_bytes) {
            out.push(Suggestion {
                title: format!(
                    "Memory pressure: {} using {}",
                    top.name,
                    fmt_bytes(top.mem_bytes)
                ),
                detail: format!(
                    "Close unused apps or lower priority of PID {}.",
                    top.pid
                ),
            });
        }
    }
    
    for p in rows.iter().filter(|p| {
        p.cpu < 0.5 && (p.read_bps + p.write_bps) < 1024 && p.mem_bytes > 500 * 1024 * 1024
    }) {
        out.push(Suggestion {
            title: format!("Idle hog: {} holding {}", p.name, fmt_bytes(p.mem_bytes)),
            detail: format!("You could lower its priority or close it. PID {}", p.pid),
        });
    }
    
    out
}