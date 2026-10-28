//! Processor model of the machine a benchmark runs on (roadmap step Q.6):
//! written next to `cpus` and `os`, so that a change of the CI runner shows
//! in the trend display. Read without further dependencies; `unknown` where
//! the platform does not tell. A failure to read never fails a benchmark.

/// Name of the processor model, whitespace collapsed, or `unknown`.
pub fn cpu_model() -> String {
    read()
        .map(|model| model.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|model| !model.is_empty())
        .unwrap_or_else(|| "unknown".to_string())
}

/// `model name` of the first processor in `/proc/cpuinfo`.
#[cfg(target_os = "linux")]
fn read() -> Option<String> {
    let info = std::fs::read_to_string("/proc/cpuinfo").ok()?;
    info.lines().find_map(|line| {
        let (key, value) = line.split_once(':')?;
        (key.trim() == "model name").then(|| value.to_string())
    })
}

#[cfg(target_os = "macos")]
fn read() -> Option<String> {
    let out = std::process::Command::new("sysctl")
        .args(["-n", "machdep.cpu.brand_string"])
        .output()
        .ok()?;
    out.status
        .success()
        .then(|| String::from_utf8_lossy(&out.stdout).into_owned())
}

#[cfg(target_os = "windows")]
fn read() -> Option<String> {
    std::env::var("PROCESSOR_IDENTIFIER").ok()
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
fn read() -> Option<String> {
    None
}
