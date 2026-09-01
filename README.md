# CargoRise

![Windows](https://img.shields.io/badge/platform-Windows-blue)
![Rust](https://img.shields.io/badge/backend-Rust-orange)
![Python](https://img.shields.io/badge/gui-Python%20%2B%20Tkinter-yellow)
![Portable](https://img.shields.io/badge/build-portable-brightgreen)

CargoRise 是一个 Windows 端 Rust 项目创建工具。它面向 RustRover、VS Code 和普通终端，目标很简单：在你写完 `main.rs` 之后，少切几次窗口，少输几次路径。

当前版本：`v0.2.0`

## 特性

- 终端直接输入 `cargorise-new` 快速创建项目
- GUI 弹窗创建项目，并可在创建后直接打开 IDE 新窗口
- 支持 RustRover 和 VS Code
- 自动记住常用保存路径
- 可从当前 `main.rs` 直接复制到新项目
- 纯 Windows 可执行发布包，适合放进 PATH

## 目录

```text
CargoRise_Portable/
  CargoRise.py
  CargoRise.pyw
  CargoRise.vbs
  cargorise.exe
  cargorise.cmd
  cargorise-new.cmd
  cargorise-open.cmd
  cargo_rise_core.exe
  backend/
  launcher/
  build_all.cmd
  install_windowsapps.cmd
  install_user_path.cmd
  install_powershell_profile.cmd
```

## 使用方式

### 1. 最快的终端方式

```powershell
cargorise-new hello_world
```

指定保存路径：

```powershell
cargorise-new hello_world D:\RustProjects
```

### 2. 打开已有项目

```powershell
cargorise-open D:\RustProjects\hello_world
```

强制指定 IDE：

```powershell
cargorise-open D:\RustProjects\hello_world --ide vscode
cargorise-open D:\RustProjects\hello_world --ide rustrover
```

### 3. 弹窗模式

```powershell
cargorise
```

窗口里可以输入项目名、保存路径，并选择：

- 是否创建后直接打开 IDE
- IDE 类型：`auto` / `rustrover` / `vscode`

## 安装

推荐先运行：

```powershell
.\install_windowsapps.cmd
```

如果你更想把命令放进自己的用户 PATH：

```powershell
.\install_user_path.cmd
```

如果你习惯 PowerShell：

```powershell
.\install_powershell_profile.cmd
```

安装后重启 RustRover 或 VS Code，再在新终端里测试：

```powershell
where.exe cargorise
cargorise
```

## 说明

- `cargorise` 适合弹窗创建。
- `cargorise-new` 适合最快创建。
- `cargorise-open` 适合创建后立刻打开 IDE。
- RustRover 和 VS Code 通常不需要额外 `.json` 配置。
- 如果 IDE 没有被识别，请确认 `rustrover` 或 `code` 已经在 PATH 中。

## 运行前提

- Windows 10 / 11
- Python 3
- Rust 工具链可用时体验最好

## 注意事项

- 改完 PATH 后，编辑器要完全退出再打开。
- 在 PowerShell 里，`where.exe cargorise` 比 `where cargorise` 更可靠。
- 如果你从某个项目目录里启动 `cargorise`，窗口会优先记住那个目录。
- 当前版本以便携包为主，直接复制整个文件夹也能用。

## 构建

```powershell
.\build_all.cmd
```

会重新生成：

```text
cargorise.exe
cargo_rise_core.exe
```

## 版本说明

- `v0.2.0`：新增 `cargorise-open`，GUI 创建后可直接打开 IDE，新版后端改为快速骨架生成。
