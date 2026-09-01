@echo off
setlocal EnableExtensions

if "%~1"=="" (
    echo Usage: cargorise-new project_name [save_path]
    echo Example: cargorise-new hello_world
    exit /b 1
)

set "SCRIPT_DIR=%~dp0"
set "PROJECT_NAME=%~1"
set "SAVE_PATH=%~2"
if "%SAVE_PATH%"=="" set "SAVE_PATH=%CD%"

"%SCRIPT_DIR%cargo_rise_core.exe" create --name "%PROJECT_NAME%" --path "%SAVE_PATH%"
