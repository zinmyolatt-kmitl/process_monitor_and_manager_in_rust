#[cfg(target_family = "unix")]
use std::process::Command;

#[cfg(target_family = "unix")]
pub fn kill(pid: i32) -> std::io::Result<()> {
    nix::sys::signal::kill(
        nix::unistd::Pid::from_raw(pid),
        nix::sys::signal::Signal::SIGKILL,
    )
    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}

#[cfg(target_family = "unix")]
pub fn suspend(pid: i32) -> std::io::Result<()> {
    nix::sys::signal::kill(
        nix::unistd::Pid::from_raw(pid),
        nix::sys::signal::Signal::SIGSTOP,
    )
    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}

#[cfg(target_family = "unix")]
pub fn resume(pid: i32) -> std::io::Result<()> {
    nix::sys::signal::kill(
        nix::unistd::Pid::from_raw(pid),
        nix::sys::signal::Signal::SIGCONT,
    )
    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}

#[cfg(target_family = "unix")]
pub fn priority_boost(_pid: i32) -> std::io::Result<()> {
    // TODO: implement nice/renice
    Ok(())
}

#[cfg(target_family = "unix")]
pub fn priority_lower(_pid: i32) -> std::io::Result<()> {
    // TODO: implement nice/renice
    Ok(())
}

#[cfg(target_family = "unix")]
pub fn start(cmd: &str) -> std::io::Result<()> {
    if cmd.trim().is_empty() {
        return Ok(());
    }
    Command::new("sh").arg("-c").arg(cmd).spawn()?;
    Ok(())
}

#[cfg(target_family = "windows")]
pub fn kill(pid: i32) -> std::io::Result<()> {
    use windows_sys::Win32::Foundation::{CloseHandle, HANDLE};
    use windows_sys::Win32::System::Threading::{OpenProcess, TerminateProcess, PROCESS_TERMINATE};

    unsafe {
        let handle: HANDLE = OpenProcess(PROCESS_TERMINATE, 0, pid as u32);
        if handle == std::ptr::null_mut() {
            return Err(std::io::Error::last_os_error());
        }
        
        let result = TerminateProcess(handle, 1);
        CloseHandle(handle);
        
        if result == 0 {
            return Err(std::io::Error::last_os_error());
        }
    }
    Ok(())
}

// Windows stubs for now - can be implemented later
#[cfg(target_family = "windows")]
pub fn suspend(_pid: i32) -> std::io::Result<()> { 
    // TODO: Implement using SuspendThread on all threads of the process
    Ok(()) 
}

#[cfg(target_family = "windows")]
pub fn resume(_pid: i32) -> std::io::Result<()> { 
    // TODO: Implement using ResumeThread on all threads of the process
    Ok(()) 
}

#[cfg(target_family = "windows")]
pub fn priority_boost(_pid: i32) -> std::io::Result<()> { 
    // TODO: Implement using SetPriorityClass
    Ok(()) 
}

#[cfg(target_family = "windows")]
pub fn priority_lower(_pid: i32) -> std::io::Result<()> { 
    // TODO: Implement using SetPriorityClass
    Ok(()) 
}

#[cfg(target_family = "windows")]
pub fn start(cmd: &str) -> std::io::Result<()> {
    if cmd.trim().is_empty() { 
        return Ok(()); 
    }
    std::process::Command::new("cmd").arg("/C").arg(cmd).spawn()?;
    Ok(())
}