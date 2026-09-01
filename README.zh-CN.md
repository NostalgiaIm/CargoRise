# CargoRise

![Windows](https://img.shields.io/badge/platform-Windows-blue)
![Rust](https://img.shields.io/badge/backend-Rust-orange)
![Python](https://img.shields.io/badge/gui-Python%20%2B%20Tkinter-yellow)
![Portable](https://img.shields.io/badge/build-portable-brightgreen)
![Release](https://img.shields.io/badge/release-v0.2.0-2ea44f)

English README: [README.md](README.md)

CargoRise 是一个面向 Windows 的 Rust 项目创建工具，适合 RustRover、VS Code 和终端工作流。它的目标很简单：你写完 `main.rs` 之后，可以更快创建下一个项目，减少切换窗口和输入路径的次数。

当前版本：`v0.2.0`

## 特性

- 终端快速创建 Rust 项目
- GUI 窗口创建项目
- 创建后可自动打开 IDE
- 支持 RustRover 和 VS Code
- 自动记住保存路径
- 可以把当前 `main.rs` 复制到新项目
- 便携式 Windows 发布包，可直接放入 `PATH`

## 包结构

```text
CargoRise/
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
  README.zh-CN.md
```

## 快速开始

### 1. 最快的终端方式

```powershell
cargorise-new hello_world
```

指定保存目录：

```powershell
cargorise-new hello_world C:\Projects\Rust
```

### 2. 打开已有项目

```powershell
cargorise-open C:\Projects\Rust\hello_world
```

强制指定 IDE：

```powershell
cargorise-open C:\Projects\Rust\hello_world --ide vscode
cargorise-open C:\Projects\Rust\hello_world --ide rustrover
```

### 3. 窗口模式

```powershell
cargorise
```

窗口里可以填写：

- 项目名
- 保存路径
- 是否创建后直接打开 IDE
- IDE 模式：`auto` / `rustrover` / `vscode`

## 安装

推荐直接运行：

```powershell
.\install_windowsapps.cmd
```

如果你想加入用户 `PATH`：

```powershell
.\install_user_path.cmd
```

如果你习惯 PowerShell Profile：

```powershell
.\install_powershell_profile.cmd
```

安装后，重启 RustRover 或 VS Code，然后在新终端里测试：

```powershell
where.exe cargorise
cargorise
```

## 说明

- `cargorise` 是窗口模式。
- `cargorise-new` 是最快的创建命令。
- `cargorise-open` 用于创建后直接打开项目。
- RustRover 和 VS Code 通常不需要每个项目都额外配置 `.json`。
- 如果 IDE 没有识别到，请确认 `rustrover` 或 `code` 已经在 `PATH` 中。

## 运行要求

- Windows 10 / 11
- Python 3
- 如果要重新编译源码，需要 Rust 工具链

## 从源码构建

```powershell
.\build_all.cmd
```

会重新生成：

```text
cargorise.exe
cargo_rise_core.exe
```

## 版本说明

- `v0.2.0`：新增 `cargorise-open`，支持 GUI 创建后直接打开 IDE，并改为更快的项目骨架生成方式。
