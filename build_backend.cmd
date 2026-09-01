@echo off
setlocal
set "SCRIPT_DIR=%~dp0"
cargo build --release --manifest-path "%SCRIPT_DIR%backend\Cargo.toml"
if %ERRORLEVEL%==0 goto COPY_EXE

echo.
echo Normal cargo build failed. Trying cargo +stable build...
cargo +stable build --release --manifest-path "%SCRIPT_DIR%backend\Cargo.toml"
if not %ERRORLEVEL%==0 exit /b %ERRORLEVEL%

:COPY_EXE
copy /Y "%SCRIPT_DIR%backend\target\release\cargo_rise_core.exe" "%SCRIPT_DIR%cargo_rise_core.exe" >nul
echo Built "%SCRIPT_DIR%cargo_rise_core.exe"
