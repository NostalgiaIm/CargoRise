@echo off
setlocal EnableExtensions

set "TOOL_DIR=%~dp0"
if "%TOOL_DIR:~-1%"=="\" set "TOOL_DIR=%TOOL_DIR:~0,-1%"

set "LAUNCHER_PATH=%TOOL_DIR%\cargorise.exe"
set "CORE_PATH=%TOOL_DIR%\cargo_rise_core.exe"

set "PROFILE_DIR_51=%USERPROFILE%\Documents\WindowsPowerShell"
set "PROFILE_DIR_7=%USERPROFILE%\Documents\PowerShell"
set "PROFILE_51=%PROFILE_DIR_51%\Microsoft.PowerShell_profile.ps1"
set "PROFILE_7=%PROFILE_DIR_7%\Microsoft.PowerShell_profile.ps1"

if not exist "%LAUNCHER_PATH%" (
    echo Cannot find cargorise.exe:
    echo %LAUNCHER_PATH%
    exit /b 1
)

call :install_profile "%PROFILE_DIR_51%" "%PROFILE_51%"
call :install_profile "%PROFILE_DIR_7%" "%PROFILE_7%"

echo CargoRise PowerShell command installed.
echo Restart RustRover or VS Code, then run:
echo   Get-Command cargorise
echo   Get-Command cargorise-new
echo   Get-Command cargorise-open
exit /b 0

:install_profile
set "PROFILE_DIR=%~1"
set "PROFILE_FILE=%~2"

if not exist "%PROFILE_DIR%" mkdir "%PROFILE_DIR%"
if not exist "%PROFILE_FILE%" type nul > "%PROFILE_FILE%"

findstr /C:"# CargoRise profile shim" "%PROFILE_FILE%" >nul 2>nul
if not errorlevel 1 exit /b 0

>> "%PROFILE_FILE%" echo.
>> "%PROFILE_FILE%" echo # CargoRise profile shim
>> "%PROFILE_FILE%" echo function cargorise {
>> "%PROFILE_FILE%" echo     ^& '%LAUNCHER_PATH%' @args
>> "%PROFILE_FILE%" echo }
>> "%PROFILE_FILE%" echo function cargorise-new {
>> "%PROFILE_FILE%" echo     if ($args.Count -eq 0^) {
>> "%PROFILE_FILE%" echo         Write-Host 'Usage: cargorise-new project_name [save_path]'
>> "%PROFILE_FILE%" echo         return
>> "%PROFILE_FILE%" echo     }
>> "%PROFILE_FILE%" echo     $savePath = if ($args.Count -ge 2^) { $args[1] } else { (Get-Location^).Path }
>> "%PROFILE_FILE%" echo     ^& '%CORE_PATH%' create --name $args[0] --path $savePath
>> "%PROFILE_FILE%" echo }
>> "%PROFILE_FILE%" echo function cargorise-open {
>> "%PROFILE_FILE%" echo     if ($args.Count -eq 0^) {
>> "%PROFILE_FILE%" echo         Write-Host 'Usage: cargorise-open project_path [--ide auto^|vscode^|rustrover]'
>> "%PROFILE_FILE%" echo         return
>> "%PROFILE_FILE%" echo     }
>> "%PROFILE_FILE%" echo     $ide = 'auto'
>> "%PROFILE_FILE%" echo     if ($args.Count -ge 3 -and $args[1] -eq '--ide'^) { $ide = $args[2] }
>> "%PROFILE_FILE%" echo     ^& '%CORE_PATH%' open --path $args[0] --ide $ide
>> "%PROFILE_FILE%" echo }
exit /b 0
