@echo off
setlocal

rem Keep the batch wrapper simple; the PowerShell script updates PATH safely.
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0install_user_path.ps1"
exit /b %ERRORLEVEL%
