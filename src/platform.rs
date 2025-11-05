#[cfg(target_family = "unix")]
use std::process::Command;

#[cfg(target_family = "windows")]
use windows_sys::Win32::{
    Foundation::{CloseHandle, HANDLE},
    System::{
        Diagnostics::ToolHelp::{
            CreateToolhelp32Snapshot, Thread32First, Thread32Next, THREADENTRY32, TH32CS_SNAPTHREAD,
        },
        Threading::{
            OpenThread, SuspendThread, ResumeThread, SetPriorityClass, PROCESS_SET_INFORMATION,
            THREAD_SUSPEND_RESUME, BELOW_NORMAL_PRIORITY_CLASS, ABOVE_NORMAL_PRIORITY_CLASS,
        },
    },
};

#[cfg(target_family = "windows")]
fn get_threads_in_process(pid: i32) -> std::io::Result<Vec<u32>> {
    use windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE;
    
    let snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0) };
    if snapshot == INVALID_HANDLE_VALUE {
        return Err(std::io::Error::last_os_error());
    }

    let mut threads = Vec::new();
    let mut entry: THREADENTRY32 = unsafe { std::mem::zeroed() };
    entry.dwSize = std::mem::size_of::<THREADENTRY32>() as u32;

    unsafe {
        if Thread32First(snapshot, &mut entry) == 1 {
            loop {
                if entry.th32OwnerProcessID == pid as u32 {
                    threads.push(entry.th32ThreadID);
                }
                if Thread32Next(snapshot, &mut entry) == 0 {
                    break;
                }
            }
        }
        CloseHandle(snapshot);
    }

    Ok(threads)
}

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
pub fn priority_boost(pid: i32) -> std::io::Result<()> {
    use libc::{getpriority, setpriority, PRIO_PROCESS};
    use std::io;

    unsafe {
        let current = getpriority(PRIO_PROCESS, pid as u32);
        let mut new_nice = current - 1; 
        if new_nice < -20 {
            new_nice = -20;
        }

        if setpriority(PRIO_PROCESS, pid as u32, new_nice) != 0 {
            return Err(io::Error::last_os_error());
        }
    }

    Ok(())
}

#[cfg(target_family = "unix")]
pub fn priority_lower(pid: i32) -> std::io::Result<()> {
    use libc::{getpriority, setpriority, PRIO_PROCESS};
    use std::io;

    unsafe {
        let current = getpriority(PRIO_PROCESS, pid as u32);
        let mut new_nice = current + 1;
        if new_nice > 19 {
            new_nice = 19;
        }

        if setpriority(PRIO_PROCESS, pid as u32, new_nice) != 0 {
            return Err(io::Error::last_os_error());
        }
    }

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

#[cfg(target_family = "windows")]
pub fn suspend(pid: i32) -> std::io::Result<()> {
    let threads = get_threads_in_process(pid)?;
    for tid in threads {
        unsafe {
            let h_thread: HANDLE = OpenThread(THREAD_SUSPEND_RESUME, 0, tid);
            if h_thread != std::ptr::null_mut() {
                SuspendThread(h_thread);
                CloseHandle(h_thread);
            }
        }
    }
    Ok(())
}

#[cfg(target_family = "windows")]
pub fn resume(pid: i32) -> std::io::Result<()> {
    let threads = get_threads_in_process(pid)?;
    for tid in threads {
        unsafe {
            let h_thread: HANDLE = OpenThread(THREAD_SUSPEND_RESUME, 0, tid);
            if h_thread != std::ptr::null_mut() {
                ResumeThread(h_thread);
                CloseHandle(h_thread);
            }
        }
    }
    Ok(())
}

#[cfg(target_family = "windows")]
pub fn priority_boost(pid: i32) -> std::io::Result<()> {
    use windows_sys::Win32::System::Threading::OpenProcess;

    unsafe {
        let handle = OpenProcess(PROCESS_SET_INFORMATION, 0, pid as u32);
        if handle == std::ptr::null_mut() {
            return Err(std::io::Error::last_os_error());
        }
        let ok = SetPriorityClass(handle, ABOVE_NORMAL_PRIORITY_CLASS);
        CloseHandle(handle);
        if ok == 0 {
            return Err(std::io::Error::last_os_error());
        }
    }
    Ok(())
}


#[cfg(target_family = "windows")]
pub fn priority_lower(pid: i32) -> std::io::Result<()> {
    use windows_sys::Win32::System::Threading::OpenProcess;

    unsafe {
        let handle = OpenProcess(PROCESS_SET_INFORMATION, 0, pid as u32);
        if handle == std::ptr::null_mut() {
            return Err(std::io::Error::last_os_error());
        }
        let ok = SetPriorityClass(handle, BELOW_NORMAL_PRIORITY_CLASS);
        CloseHandle(handle);
        if ok == 0 {
            return Err(std::io::Error::last_os_error());
        }
    }
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