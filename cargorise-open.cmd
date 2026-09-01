@echo off
setlocal EnableExtensions

if "%~1"=="" (
    echo Usage: cargorise-open project_path [--ide auto^|vscode^|rustrover]
    echo Example: cargorise-open D:\RustProjects\hello_world
    exit /b 1
)

set "SCRIPT_DIR=%~dp0"
set "PROJECT_PATH=%~1"
set "IDE=auto"

if /I "%~2"=="--ide" (
    set "IDE=%~3"
)

"%SCRIPT_DIR%cargo_rise_core.exe" open --path "%PROJECT_PATH%" --ide "%IDE%"
