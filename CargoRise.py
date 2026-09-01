import argparse
import json
import os
import shutil
import subprocess
import threading
from pathlib import Path
from typing import List, Optional
import tkinter as tk
from tkinter import filedialog, messagebox, ttk


APP_NAME = "CargoRise"
APP_DIR = Path(__file__).resolve().parent
CONFIG_DIR = Path(os.environ.get("APPDATA", Path.home())) / APP_NAME
CONFIG_FILE = CONFIG_DIR / "config.json"
BACKEND_MANIFEST = APP_DIR / "backend" / "Cargo.toml"
CALLER_CWD_ENV = "CARGORISE_CALLER_CWD"
DEFAULT_IDE = "auto"


def load_config() -> dict:
    """读取上一次保存的路径，让下次打开软件时不用重新选择目录。"""
    if not CONFIG_FILE.exists():
        return {}
    try:
        return json.loads(CONFIG_FILE.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return {}


def save_config(data: dict) -> None:
    """把常用保存路径写入当前用户的 AppData 配置目录。"""
    CONFIG_DIR.mkdir(parents=True, exist_ok=True)
    CONFIG_FILE.write_text(json.dumps(data, ensure_ascii=False, indent=2), encoding="utf-8")


def caller_directory() -> Path:
    """读取用户在哪个终端目录里调用 CargoRise，例如 RustRover 的项目终端。"""
    raw = os.environ.get(CALLER_CWD_ENV)
    if raw and Path(raw).exists():
        return Path(raw).resolve()
    return Path.cwd().resolve()


def find_cargo_root(start_dir: Path) -> Optional[Path]:
    """从当前目录一路向上找 Cargo.toml，用来判断当前是不是在 Rust 项目里。"""
    current = start_dir if start_dir.is_dir() else start_dir.parent
    for directory in [current, *current.parents]:
        if (directory / "Cargo.toml").exists():
            return directory
    return None


def discover_main_file(explicit_main_file: Optional[Path], caller_dir: Path) -> Optional[Path]:
    """
    自动寻找当前项目里的 main.rs。
    这样在 RustRover、VS Code 或普通终端里直接输入 cargorise，也能检测到当前 main。
    """
    if explicit_main_file and explicit_main_file.exists():
        return explicit_main_file.resolve()

    candidates = [
        caller_dir / "main.rs",
        caller_dir / "src" / "main.rs",
    ]

    cargo_root = find_cargo_root(caller_dir)
    if cargo_root:
        candidates.append(cargo_root / "src" / "main.rs")

    if caller_dir.name == "src":
        candidates.append(caller_dir / "main.rs")

    for candidate in candidates:
        if candidate.exists():
            return candidate.resolve()

    return None


def default_directory(config: dict, caller_dir: Path) -> Path:
    """优先使用记住的保存路径；没有配置时使用当前 Cargo 项目的上一级目录。"""
    saved = config.get("save_dir")
    if saved and Path(saved).exists():
        return Path(saved)

    cargo_root = find_cargo_root(caller_dir)
    if cargo_root and cargo_root.parent.exists():
        return cargo_root.parent

    return caller_dir if caller_dir.exists() else Path.cwd()


def backend_command(args: List[str]) -> List[str]:
    """
    找到 Rust 后端的启动方式。
    如果已经编译出 exe，就直接运行 exe；否则用 cargo run 临时编译并运行。
    """
    exe_name = "cargo_rise_core.exe" if os.name == "nt" else "cargo_rise_core"
    bundled_exe = APP_DIR / exe_name
    release_exe = APP_DIR / "backend" / "target" / "release" / exe_name
    debug_exe = APP_DIR / "backend" / "target" / "debug" / exe_name

    if bundled_exe.exists():
        return [str(bundled_exe), *args]
    if release_exe.exists():
        return [str(release_exe), *args]
    if debug_exe.exists():
        return [str(debug_exe), *args]
    return ["cargo", "run", "--quiet", "--manifest-path", str(BACKEND_MANIFEST), "--", *args]


def hidden_subprocess_options() -> dict:
    """
    Windows 下从 GUI 程序启动 cargo/Rust 后端时，避免额外弹出控制台窗口。
    其他系统不需要这个参数。
    """
    if os.name == "nt" and hasattr(subprocess, "CREATE_NO_WINDOW"):
        return {"creationflags": subprocess.CREATE_NO_WINDOW}
    return {}


class CargoRiseApp(tk.Tk):
    def __init__(self, main_file: Optional[Path], caller_dir: Path):
        super().__init__()

        # Tkinter 是 Python 标准库自带的 GUI，适合做这种小工具窗口。
        self.title(APP_NAME)
        self.geometry("620x300")
        self.minsize(560, 280)

        self.config_data = load_config()
        self.caller_dir = caller_dir
        self.main_file = main_file if main_file and main_file.exists() else None

        self.project_name = tk.StringVar()
        self.save_dir = tk.StringVar(value=str(default_directory(self.config_data, self.caller_dir)))
        self.copy_main = tk.BooleanVar(value=self.main_file is not None)
        self.open_after = tk.BooleanVar(value=True)
        self.ide_choice = tk.StringVar(value=DEFAULT_IDE)
        self.status = tk.StringVar(value="准备就绪")
        self.create_button: Optional[ttk.Button] = None
        self.open_path_button: Optional[ttk.Button] = None

        self._build_ui()

    def _build_ui(self) -> None:
        """创建窗口中的输入框、按钮和状态提示。"""
        root = ttk.Frame(self, padding=18)
        root.pack(fill="both", expand=True)
        root.columnconfigure(1, weight=1)

        ttk.Label(root, text="项目名称").grid(row=0, column=0, sticky="w", pady=(0, 8))
        name_entry = ttk.Entry(root, textvariable=self.project_name)
        name_entry.grid(row=0, column=1, columnspan=2, sticky="ew", pady=(0, 8))
        name_entry.focus()

        ttk.Label(root, text="保存路径").grid(row=1, column=0, sticky="w", pady=(0, 8))
        ttk.Entry(root, textvariable=self.save_dir).grid(row=1, column=1, sticky="ew", pady=(0, 8))
        ttk.Button(root, text="选择", command=self.pick_directory).grid(row=1, column=2, padx=(8, 0), pady=(0, 8))

        if self.main_file:
            text = f"把检测到的 {self.main_file.name} 复制到新项目的 src/main.rs"
            ttk.Checkbutton(root, text=text, variable=self.copy_main).grid(
                row=2, column=1, columnspan=2, sticky="w", pady=(2, 8)
            )

        caller_text = f"调用目录：{self.caller_dir}"
        ttk.Label(root, text=caller_text).grid(row=3, column=1, columnspan=2, sticky="w", pady=(0, 8))

        options = ttk.Frame(root)
        options.grid(row=4, column=1, columnspan=2, sticky="ew", pady=(6, 8))
        self.create_button = ttk.Button(options, text="创建 Cargo 项目", command=self.create_project)
        self.create_button.pack(side="left")
        self.open_path_button = ttk.Button(options, text="打开保存路径", command=self.open_save_path)
        self.open_path_button.pack(side="left", padx=(8, 0))

        mode_row = ttk.Frame(root)
        mode_row.grid(row=5, column=0, columnspan=3, sticky="ew", pady=(4, 2))
        ttk.Checkbutton(mode_row, text="创建后直接打开 IDE", variable=self.open_after).pack(side="left")
        ttk.Label(mode_row, text="IDE").pack(side="left", padx=(16, 6))
        ttk.Combobox(
            mode_row,
            textvariable=self.ide_choice,
            values=("auto", "rustrover", "vscode"),
            width=12,
            state="readonly",
        ).pack(side="left")

        ttk.Separator(root).grid(row=6, column=0, columnspan=3, sticky="ew", pady=(14, 10))
        ttk.Label(root, textvariable=self.status).grid(row=7, column=0, columnspan=3, sticky="w")

    def pick_directory(self) -> None:
        """弹出文件夹选择窗口，选择 Cargo 项目要保存到哪个上级目录。"""
        selected = filedialog.askdirectory(initialdir=self.save_dir.get() or str(Path.cwd()))
        if selected:
            self.save_dir.set(selected)

    def open_save_path(self) -> None:
        """用 Windows 文件管理器打开当前保存路径。"""
        path = Path(self.save_dir.get()).expanduser()
        if path.exists():
            os.startfile(path)
        else:
            messagebox.showwarning(APP_NAME, "保存路径不存在。")

    def create_project(self) -> None:
        """把界面中的输入交给 Rust 后端，让 Rust 后端执行 cargo new。"""
        name = self.project_name.get().strip()
        parent = Path(self.save_dir.get()).expanduser()

        if not name:
            messagebox.showerror(APP_NAME, "请先输入项目名称。")
            return

        if not parent.exists():
            messagebox.showerror(APP_NAME, "保存路径不存在。")
            return

        if not BACKEND_MANIFEST.exists():
            messagebox.showerror(APP_NAME, f"找不到 Rust 后端项目：\n{BACKEND_MANIFEST}")
            return

        command_args = ["create", "--name", name, "--path", str(parent)]

        # 如果用户从 main.rs 调出了窗口，可以选择把当前 main.rs 放进新项目。
        if self.main_file and self.copy_main.get():
            command_args.extend(["--main-file", str(self.main_file)])

        if self.open_after.get():
            command_args.append("--open")
            command_args.extend(["--ide", self.ide_choice.get().strip() or DEFAULT_IDE])

        command = backend_command(command_args)
        if command[0] == "cargo" and shutil.which("cargo") is None:
            messagebox.showerror(APP_NAME, "找不到 cargo。请先安装 Rust，或确认 cargo 已经加入 PATH。")
            return

        self.status.set("正在创建项目...")
        self._set_controls_enabled(False)
        threading.Thread(
            target=self._create_project_worker,
            args=(command, parent, name),
            daemon=True,
        ).start()

    def _create_project_worker(self, command: List[str], parent: Path, name: str) -> None:
        try:
            result = subprocess.run(
                command,
                text=True,
                capture_output=True,
                check=False,
                **hidden_subprocess_options(),
            )
        except OSError as exc:
            self.after(0, lambda: self._create_failed(str(exc)))
            return

        if result.returncode != 0:
            error_text = result.stderr.strip() or result.stdout.strip() or "Rust 后端执行失败。"
            self.after(0, lambda: self._create_failed(error_text))
            return

        created_path = result.stdout.strip() or str(parent / name)
        self.after(0, lambda: self._create_succeeded(parent, created_path))

    def _create_failed(self, message: str) -> None:
        messagebox.showerror(APP_NAME, message)
        self.status.set("创建失败")
        self._set_controls_enabled(True)

    def _create_succeeded(self, parent: Path, created_path: str) -> None:
        self.config_data["save_dir"] = str(parent)
        save_config(self.config_data)
        self.status.set(f"已创建：{created_path}")
        self._set_controls_enabled(True)

        if not self.open_after.get() and messagebox.askyesno(APP_NAME, "项目已创建。是否在文件管理器中打开？"):
            os.startfile(created_path)

    def _set_controls_enabled(self, enabled: bool) -> None:
        state = "normal" if enabled else "disabled"
        if self.create_button is not None:
            self.create_button.configure(state=state)
        if self.open_path_button is not None:
            self.open_path_button.configure(state=state)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="CargoRise GUI")
    parser.add_argument("main_file", nargs="?", help="可选：当前正在编辑的 main.rs 路径")
    return parser.parse_args()


def main() -> None:
    args = parse_args()
    caller_dir = caller_directory()
    explicit_main_file = Path(args.main_file).resolve() if args.main_file else None
    main_file = discover_main_file(explicit_main_file, caller_dir)
    app = CargoRiseApp(main_file, caller_dir)
    app.mainloop()


if __name__ == "__main__":
    main()
