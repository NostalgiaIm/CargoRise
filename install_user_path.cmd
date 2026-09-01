@echo off
setlocal EnableExtensions

set "TOOL_DIR=%~dp0"
if "%TOOL_DIR:~-1%"=="\" set "TOOL_DIR=%TOOL_DIR:~0,-1%"

set "BIN_DIR=%USERPROFILE%\Documents\Codex\Tools\bin"
set "SHIM_PATH=%BIN_DIR%\cargorise.cmd"
set "NEW_SHIM_PATH=%BIN_DIR%\cargorise-new.cmd"
set "OPEN_SHIM_PATH=%BIN_DIR%\cargorise-open.cmd"
set "LAUNCHER_PATH=%TOOL_DIR%\cargorise.exe"
set "CORE_PATH=%TOOL_DIR%\cargo_rise_core.exe"

if not exist "%LAUNCHER_PATH%" (
    echo Cannot find cargorise.exe:
    echo %LAUNCHER_PATH%
    exit /b 1
)

if not exist "%BIN_DIR%" mkdir "%BIN_DIR%"
if errorlevel 1 (
    echo Failed to create:
    echo %BIN_DIR%
    exit /b 1
)

(
    echo @echo off
    echo setlocal
    echo set "CARGORISE_CALLER_CWD=%%CD%%"
    echo start "" "%LAUNCHER_PATH%" %%*
) > "%SHIM_PATH%"
if errorlevel 1 (
    echo Failed to write:
    echo %SHIM_PATH%
    exit /b 1
)

(
    echo @echo off
    echo setlocal
    echo if "%%~1"=="" ^(
    echo     echo Usage: cargorise-new project_name [save_path]
    echo     exit /b 1
    echo ^)
    echo set "SAVE_PATH=%%~2"
    echo if "%%SAVE_PATH%%"=="" set "SAVE_PATH=%%CD%%"
    echo "%CORE_PATH%" create --name "%%~1" --path "%%SAVE_PATH%%"
) > "%NEW_SHIM_PATH%"
if errorlevel 1 (
    echo Failed to write:
    echo %NEW_SHIM_PATH%
    exit /b 1
)

(
    echo @echo off
    echo setlocal
    echo if "%%~1"=="" ^(
    echo     echo Usage: cargorise-open project_path [--ide auto^|vscode^|rustrover]
    echo     exit /b 1
    echo ^)
    echo set "IDE=auto"
    echo if /I "%%~2"=="--ide" set "IDE=%%~3"
    echo "%CORE_PATH%" open --path "%%~1" --ide "%%IDE%%"
) > "%OPEN_SHIM_PATH%"
if errorlevel 1 (
    echo Failed to write:
    echo %OPEN_SHIM_PATH%
    exit /b 1
)

set "USER_PATH="
for /f "tokens=2,* skip=2" %%A in ('reg query HKCU\Environment /v Path 2^>nul') do (
    set "USER_PATH=%%B"
)

if not defined USER_PATH (
    reg add HKCU\Environment /v Path /t REG_EXPAND_SZ /d "%BIN_DIR%" /f >nul
    if errorlevel 1 (
        echo Failed to update user PATH.
        echo You can still run CargoRise directly:
        echo "%TOOL_DIR%\cargorise.exe"
        exit /b 1
    )
) else (
    echo ;%USER_PATH%; | find /I ";%BIN_DIR%;" >nul
    if errorlevel 1 (
        reg add HKCU\Environment /v Path /t REG_EXPAND_SZ /d "%USER_PATH%;%BIN_DIR%" /f >nul
        if errorlevel 1 (
            echo Failed to update user PATH.
            echo You can still run CargoRise directly:
            echo "%TOOL_DIR%\cargorise.exe"
            exit /b 1
        )
    )
)

set "PATH=%PATH%;%BIN_DIR%"

echo CargoRise command shim created:
echo %SHIM_PATH%
echo.
echo User PATH now contains:
echo %BIN_DIR%
echo.
echo Test it now:
echo where.exe cargorise
echo cargorise
echo cargorise-new hello_world
echo cargorise-open D:\RustProjects\hello_world
echo.
echo If you use RustRover or VS Code, fully exit and reopen the editor.
