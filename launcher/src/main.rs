#![windows_subsystem = "windows"]

use std::env;
use std::ffi::OsString;
use std::path::PathBuf;
use std::process::Command;

#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

fn main() {
    if let Err(message) = run() {
        show_error(&message);
    }
}

fn run() -> Result<(), String> {
    let caller_cwd = env::current_dir().map_err(|error| format!("无法读取当前终端目录：{error}"))?;
    let app_dir = app_directory()?;
    let app_file = app_dir.join("CargoRise.pyw");

    if !app_file.exists() {
        return Err(format!("找不到 CargoRise.pyw：{}", app_file.display()));
    }

    // 保留用户传进来的参数，例如当前 main.rs 的路径。
    let forwarded_args: Vec<OsString> = env::args_os().skip(1).collect();

    // 优先使用 pythonw/pyw，它们本身就是无控制台启动器。
    // 后面的 python/py 只是兜底，同时会加 CREATE_NO_WINDOW。
    let attempts = [
        PythonAttempt::new("pythonw.exe", vec![app_file.as_os_str().to_os_string()]),
        PythonAttempt::new(
            "pyw.exe",
            vec![OsString::from("-3"), app_file.as_os_str().to_os_string()],
        ),
        PythonAttempt::new("python.exe", vec![app_file.as_os_str().to_os_string()]),
        PythonAttempt::new(
            "py.exe",
            vec![OsString::from("-3"), app_file.as_os_str().to_os_string()],
        ),
    ];

    let mut errors = Vec::new();
    for attempt in attempts {
        match spawn_python(&attempt, &forwarded_args, &app_dir, &caller_cwd) {
            Ok(()) => return Ok(()),
            Err(error) => errors.push(error),
        }
    }

    Err(format!(
        "没有找到可用的 Python 启动器，请安装 Python 3。\n{}",
        errors.join("\n")
    ))
}

struct PythonAttempt {
    program: &'static str,
    args: Vec<OsString>,
}

impl PythonAttempt {
    fn new(program: &'static str, args: Vec<OsString>) -> Self {
        Self { program, args }
    }
}

fn spawn_python(
    attempt: &PythonAttempt,
    forwarded_args: &[OsString],
    app_dir: &PathBuf,
    caller_cwd: &PathBuf,
) -> Result<(), String> {
    let mut command = Command::new(attempt.program);
    command.args(&attempt.args);
    command.args(forwarded_args);
    command.current_dir(app_dir);
    command.env("CARGORISE_CALLER_CWD", caller_cwd);
    command.env("CARGORISE_LAUNCHED_FROM", "launcher");
    hide_console_window(&mut command);

    command
        .spawn()
        .map(|_| ())
        .map_err(|error| format!("{} 启动失败：{error}", attempt.program))
}

fn app_directory() -> Result<PathBuf, String> {
    let exe_path = env::current_exe().map_err(|error| format!("无法读取启动器路径：{error}"))?;
    exe_path
        .parent()
        .map(PathBuf::from)
        .ok_or_else(|| "无法判断 CargoRise 所在目录。".to_string())
}

#[cfg(windows)]
fn hide_console_window(command: &mut Command) {
    // 这一行让兜底启动 python.exe/py.exe 时也不会额外弹出控制台窗口。
    command.creation_flags(CREATE_NO_WINDOW);
}

#[cfg(not(windows))]
fn hide_console_window(_command: &mut Command) {}

#[cfg(windows)]
fn show_error(message: &str) {
    use std::ffi::c_void;

    #[link(name = "user32")]
    extern "system" {
        fn MessageBoxW(hwnd: *mut c_void, text: *const u16, caption: *const u16, kind: u32) -> i32;
    }

    let title = wide("CargoRise");
    let text = wide(message);

    unsafe {
        MessageBoxW(std::ptr::null_mut(), text.as_ptr(), title.as_ptr(), 0x00000010);
    }
}

#[cfg(not(windows))]
fn show_error(message: &str) {
    eprintln!("{message}");
}

#[cfg(windows)]
fn wide(value: &str) -> Vec<u16> {
    value.encode_utf16().chain(Some(0)).collect()
}
