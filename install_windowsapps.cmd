@echo off
setlocal

rem WindowsApps can keep stale app execution aliases. Put this portable folder first in user PATH.
echo Installing this CargoRise folder at the front of the current user's PATH...
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0install_user_path.ps1"
exit /b %ERRORLEVEL%
