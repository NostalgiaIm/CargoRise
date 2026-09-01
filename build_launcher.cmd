@echo off
setlocal
set "SCRIPT_DIR=%~dp0"
cargo build --release --manifest-path "%SCRIPT_DIR%launcher\Cargo.toml"
if %ERRORLEVEL%==0 goto COPY_EXE

echo.
echo Normal cargo build failed. Trying cargo +stable build...
cargo +stable build --release --manifest-path "%SCRIPT_DIR%launcher\Cargo.toml"
if not %ERRORLEVEL%==0 exit /b %ERRORLEVEL%

:COPY_EXE
copy /Y "%SCRIPT_DIR%launcher\target\release\cargorise.exe" "%SCRIPT_DIR%cargorise.exe" >nul
echo Built "%SCRIPT_DIR%cargorise.exe"
