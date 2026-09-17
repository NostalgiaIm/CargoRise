@echo off
setlocal
set "SCRIPT_DIR=%~dp0"
set "CARGORISE_CALLER_CWD=%CD%"

if exist "%SCRIPT_DIR%cargorise.exe" (
    start "" "%SCRIPT_DIR%cargorise.exe" %*
    exit /b 0
)

where pythonw >nul 2>nul
if %ERRORLEVEL%==0 (
    start "" pythonw "%SCRIPT_DIR%CargoRise.pyw" %*
    exit /b 0
)

where pyw >nul 2>nul
if %ERRORLEVEL%==0 (
    start "" pyw -3 "%SCRIPT_DIR%CargoRise.pyw" %*
    exit /b 0
)

py -3 --version >nul 2>nul
if %ERRORLEVEL%==0 (
    echo pythonw/pyw was not found, so CargoRise must start with the console Python launcher.
    py -3 "%SCRIPT_DIR%CargoRise.py" %*
    exit /b %ERRORLEVEL%
)

python --version >nul 2>nul
if %ERRORLEVEL%==0 (
    echo pythonw/pyw was not found, so CargoRise must start with console python.
    python "%SCRIPT_DIR%CargoRise.py" %*
    exit /b %ERRORLEVEL%
)

echo Python 3 was not found. Please install Python 3 or add it to PATH.
exit /b 1
