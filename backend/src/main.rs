#![windows_subsystem = "windows"]

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

fn main() {
    if let Err(message) = run() {
        eprintln!("{message}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);
    let command = args.next().ok_or_else(usage)?;

    match command.as_str() {
        "create" => {
            let options = parse_create_options(args)?;
            let project_dir = create_project(&options)?;

            if options.open_after {
                if let Err(error) = open_project(&project_dir, options.ide.as_deref()) {
                    eprintln!("{error}");
                }
            }

            println!("{}", project_dir.display());
            Ok(())
        }
        "open" => {
            let options = parse_open_options(args)?;
            open_project(&options.project_path, options.ide.as_deref())?;
            println!("{}", options.project_path.display());
            Ok(())
        }
        "-h" | "--help" | "help" => {
            println!("{}", usage());
            Ok(())
        }
        other => Err(format!("{other}\n\n{}", usage())),
    }
}

struct CreateOptions {
    name: String,
    parent_dir: PathBuf,
    main_file: Option<PathBuf>,
    open_after: bool,
    ide: Option<String>,
}

struct OpenOptions {
    project_path: PathBuf,
    ide: Option<String>,
}

fn parse_create_options(args: impl Iterator<Item = String>) -> Result<CreateOptions, String> {
    let mut name: Option<String> = None;
    let mut parent_dir: Option<PathBuf> = None;
    let mut main_file: Option<PathBuf> = None;
    let mut open_after = false;
    let mut ide: Option<String> = None;
    let mut positionals: Vec<String> = Vec::new();

    let mut args = args.peekable();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--name" => name = Some(next_value(&mut args, "--name")?),
            "--path" => parent_dir = Some(PathBuf::from(next_value(&mut args, "--path")?)),
            "--main-file" => main_file = Some(PathBuf::from(next_value(&mut args, "--main-file")?)),
            "--ide" => ide = Some(next_value(&mut args, "--ide")?),
            "--open" => open_after = true,
            "-h" | "--help" => return Err(create_usage()),
            _ if arg.starts_with('-') => return Err(format!("未知参数：{arg}\n\n{}", create_usage())),
            _ => positionals.push(arg),
        }
    }

    if name.is_none() {
        name = positionals.get(0).cloned();
    }
    if parent_dir.is_none() {
        parent_dir = positionals.get(1).map(PathBuf::from);
    }
    if positionals.len() > 2 {
        return Err(format!("create 只接受 project_name 和可选的 path。\n\n{}", create_usage()));
    }

    Ok(CreateOptions {
        name: name.ok_or_else(create_usage)?,
        parent_dir: parent_dir.unwrap_or_else(current_dir_fallback),
        main_file,
        open_after,
        ide,
    })
}

fn parse_open_options(args: impl Iterator<Item = String>) -> Result<OpenOptions, String> {
    let mut project_path: Option<PathBuf> = None;
    let mut ide: Option<String> = None;
    let mut positionals: Vec<String> = Vec::new();

    let mut args = args.peekable();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--path" => project_path = Some(PathBuf::from(next_value(&mut args, "--path")?)),
            "--ide" => ide = Some(next_value(&mut args, "--ide")?),
            "-h" | "--help" => return Err(open_usage()),
            _ if arg.starts_with('-') => return Err(format!("未知参数：{arg}\n\n{}", open_usage())),
            _ => positionals.push(arg),
        }
    }

    if project_path.is_none() {
        project_path = positionals.get(0).map(PathBuf::from);
    }
    if positionals.len() > 1 {
        return Err(format!("open 只接受一个路径参数。\n\n{}", open_usage()));
    }

    Ok(OpenOptions {
        project_path: project_path.unwrap_or_else(current_dir_fallback),
        ide,
    })
}

fn create_project(options: &CreateOptions) -> Result<PathBuf, String> {
    validate_project_name(&options.name)?;

    let parent_dir = normalize_existing_dir(&options.parent_dir)?;
    let project_dir = parent_dir.join(&options.name);

    if project_dir.exists() {
        return Err(format!("目标目录已存在：{}", project_dir.display()));
    }

    fs::create_dir_all(project_dir.join("src"))
        .map_err(|error| format!("无法创建项目目录：{error}"))?;

    let cargo_toml = format!(
        "[package]\nname = \"{}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\n",
        escape_toml_string(&options.name)
    );
    fs::write(project_dir.join("Cargo.toml"), cargo_toml)
        .map_err(|error| format!("无法写入 Cargo.toml：{error}"))?;

    let main_rs = match &options.main_file {
        Some(source) if source.exists() => {
            fs::read_to_string(source).map_err(|error| format!("无法读取 main.rs：{error}"))?
        }
        _ => default_main_rs(),
    };
    fs::write(project_dir.join("src").join("main.rs"), main_rs)
        .map_err(|error| format!("无法写入 src/main.rs：{error}"))?;

    fs::write(project_dir.join(".gitignore"), "/target\n")
        .map_err(|error| format!("无法写入 .gitignore：{error}"))?;

    Ok(project_dir)
}

fn open_project(project_path: &Path, ide: Option<&str>) -> Result<(), String> {
    let target = resolve_project_root(project_path)?;
    let launcher = find_ide_launcher(ide)?;

    let mut command = if is_batch_script(&launcher.program) {
        let mut shell = Command::new("cmd");
        shell.arg("/C").arg(&launcher.program);
        shell
    } else {
        Command::new(&launcher.program)
    };
    command.args(&launcher.args);
    command.arg(&target);
    command.current_dir(&target);
    hide_console_window(&mut command);

    command
        .spawn()
        .map(|_| ())
        .map_err(|error| format!("无法启动 {}：{}", launcher.label, error))
}

struct IdeLauncher {
    label: &'static str,
    program: String,
    args: Vec<String>,
}

fn find_ide_launcher(ide: Option<&str>) -> Result<IdeLauncher, String> {
    let choice = ide.unwrap_or("auto").to_ascii_lowercase();

    match choice.as_str() {
        "vscode" | "code" => find_vscode_launcher().ok_or_else(|| ide_not_found_error("VS Code")),
        "rustrover" | "rust" => {
            find_rustrover_launcher().ok_or_else(|| ide_not_found_error("RustRover"))
        }
        "auto" | "" => find_rustrover_launcher()
            .or_else(find_vscode_launcher)
            .ok_or_else(|| ide_not_found_error("RustRover 或 VS Code")),
        other => Err(format!("未知 IDE 选项：{other}\n\n{}", open_usage())),
    }
}

fn find_vscode_launcher() -> Option<IdeLauncher> {
    find_program(&["code.cmd", "code.exe", "code"]).map(|program| IdeLauncher {
        label: "VS Code",
        program,
        args: vec!["-n".to_string()],
    })
}

fn find_rustrover_launcher() -> Option<IdeLauncher> {
    find_program(&["rustrover.exe", "rustrover.bat", "rustrover"]).map(|program| IdeLauncher {
        label: "RustRover",
        program,
        args: Vec::new(),
    })
}

fn find_program(candidates: &[&str]) -> Option<String> {
    let path = env::var_os("PATH")?;
    for directory in env::split_paths(&path) {
        for candidate in candidates {
            let full = directory.join(candidate);
            if full.exists() {
                return Some(full.to_string_lossy().into_owned());
            }
        }
    }
    None
}

fn resolve_project_root(path: &Path) -> Result<PathBuf, String> {
    let existing = if path.exists() {
        path.to_path_buf()
    } else {
        return Err(format!("路径不存在：{}", path.display()));
    };

    if existing.is_file() {
        if existing
            .file_name()
            .and_then(|value| value.to_str())
            .map(|value| value.eq_ignore_ascii_case("Cargo.toml"))
            .unwrap_or(false)
        {
            return existing
                .parent()
                .map(PathBuf::from)
                .ok_or_else(|| format!("无法读取项目根目录：{}", existing.display()));
        }

        let start_dir = existing
            .parent()
            .ok_or_else(|| format!("无法读取父目录：{}", existing.display()))?;
        return Ok(find_cargo_root(start_dir).unwrap_or_else(|| start_dir.to_path_buf()));
    }

    if existing.join("Cargo.toml").exists() {
        return Ok(existing);
    }

    Ok(find_cargo_root(&existing).unwrap_or(existing))
}

fn find_cargo_root(start_dir: &Path) -> Option<PathBuf> {
    let mut current = Some(start_dir);
    while let Some(dir) = current {
        if dir.join("Cargo.toml").exists() {
            return Some(dir.to_path_buf());
        }
        current = dir.parent();
    }
    None
}

fn normalize_existing_dir(path: &Path) -> Result<PathBuf, String> {
    if path.exists() && path.is_dir() {
        Ok(path.to_path_buf())
    } else if path.exists() {
        Err(format!("保存路径不是文件夹：{}", path.display()))
    } else {
        Err(format!("保存路径不存在：{}", path.display()))
    }
}

fn validate_project_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("项目名称不能为空。".to_string());
    }
    if name.starts_with('-') {
        return Err("项目名称不能以 - 开头。".to_string());
    }
    if !name
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '-')
    {
        return Err("项目名称只允许字母、数字、下划线和连字符。".to_string());
    }

    let upper = name.to_ascii_uppercase();
    let reserved = [
        "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7",
        "COM8", "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
    ];
    if reserved.contains(&upper.as_str()) {
        return Err("项目名称是 Windows 保留设备名，请换一个名字。".to_string());
    }

    Ok(())
}

fn escape_toml_string(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

fn default_main_rs() -> String {
    "fn main() {\n    println!(\"Hello, world!\");\n}\n".to_string()
}

fn next_value<I>(args: &mut std::iter::Peekable<I>, flag: &str) -> Result<String, String>
where
    I: Iterator<Item = String>,
{
    args.next().ok_or_else(|| format!("{flag} 后面需要一个值。"))
}

fn current_dir_fallback() -> PathBuf {
    env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

fn is_batch_script(path: &str) -> bool {
    let lower = path.to_ascii_lowercase();
    lower.ends_with(".cmd") || lower.ends_with(".bat")
}

fn hide_console_window(command: &mut Command) {
    #[cfg(windows)]
    {
        command.creation_flags(CREATE_NO_WINDOW);
    }
}

fn ide_not_found_error(preferred: &str) -> String {
    format!(
        "找不到可用的 {preferred} 命令。请确认对应启动器已经加入 PATH。"
    )
}

fn usage() -> String {
    format!(
        "{create}\n\n{open}\n\n常用示例：\n  cargo_rise_core create --name hello_world --path D:\\\\RustProjects\n  cargo_rise_core create hello_world D:\\\\RustProjects\n  cargo_rise_core create --name hello_world --path D:\\\\RustProjects --open --ide vscode\n  cargo_rise_core open --path D:\\\\RustProjects\\\\hello_world --ide vscode\n",
        create = create_usage(),
        open = open_usage()
    )
}

fn create_usage() -> String {
    "create --name <project_name> --path <save_path> [--main-file <main.rs>] [--open] [--ide auto|vscode|rustrover]\n也支持：create <project_name> [save_path]".to_string()
}

fn open_usage() -> String {
    "open --path <project_path> [--ide auto|vscode|rustrover]\n也支持：open <project_path>".to_string()
}
