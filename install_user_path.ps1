$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
& (Join-Path $ScriptDir "install_user_path.cmd")
exit $LASTEXITCODE
