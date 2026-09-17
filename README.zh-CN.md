<div align="center">

<img src="./assets/logo_dark.png" width="110" height="110" alt="CargoRise Logo" />

# CargoRise

### 面向 Windows 的快捷 Rust 项目创建工具

**轻量、快速、便携的 Rust 项目启动器**

[![版本](https://img.shields.io/badge/Release-v0.2.2-2563EB.svg?style=flat-square&logo=github)](https://github.com/NostalgiaIm/CargoRise/releases)
[![平台](https://img.shields.io/badge/Platform-Windows%2010%20%7C%2011-0078D4.svg?style=flat-square&logo=windows)](https://www.microsoft.com/windows)
[![后端](https://img.shields.io/badge/Backend-Rust-orange.svg?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![界面](https://img.shields.io/badge/GUI-Python%20%2B%20Tkinter-F7DF1E.svg?style=flat-square&logo=python)](https://www.python.org/)

<br />

**[简体中文](README.zh-CN.md)** • **[English](README.md)**

<br />

</div>

CargoRise 是一个面向 Windows 的轻量工具，用于快速创建 Rust 项目，并在创建后直接用 IDE 打开。

它适合 RustRover、VS Code 以及以终端为主的开发流程。写完一个 `main.rs` 后，可以更快开始下一个项目，减少窗口切换和路径输入。

> 英文 README 是项目的主说明文档，中文版本请查看 [README.zh-CN.md](README.zh-CN.md)。

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
  assets/
    logo_dark.png
    CargoRise.ico
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
.\install_user_path.cmd
```

旧版 WindowsApps 安装脚本会作为兼容入口保留，现在也会使用同样的 PATH 优先安装方式：

```powershell
.\install_windowsapps.cmd
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
- 如果编辑器仍然启动旧版 CargoRise，请在最新版 CargoRise 文件夹里重新运行 `install_user_path.cmd`，然后完全退出并重新打开编辑器，让它重新读取最新的 `PATH`。

## 运行要求

- Windows 10 / 11
- Python 3
- 如果要重新编译 Windows 启动器源码，需要 Rust 工具链和 MinGW `windres`

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

- `v0.2.2`：更新安装逻辑，把最新版 portable CargoRise 文件夹放到用户 `PATH` 最前面，避免 RustRover、VS Code 或 WindowsApps 继续命中旧版转发脚本。
- `v0.2.1`：新增 CargoRise 默认图标，窗口模式会使用该图标，同时将图标嵌入 Windows 启动器可执行文件。Rust 构建脚本现在会自动完成图标资源嵌入。
- `v0.2.0`：新增 `cargorise-open`，支持 GUI 创建后直接打开 IDE，并改为更快的项目骨架生成方式。
