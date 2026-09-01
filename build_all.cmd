@echo off
setlocal
set "SCRIPT_DIR=%~dp0"
call "%SCRIPT_DIR%build_backend.cmd"
if not %ERRORLEVEL%==0 exit /b %ERRORLEVEL%
call "%SCRIPT_DIR%build_launcher.cmd"
