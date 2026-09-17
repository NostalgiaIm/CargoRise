$ErrorActionPreference = "Stop"

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$launcherPath = Join-Path $scriptDir "cargorise.exe"
$corePath = Join-Path $scriptDir "cargo_rise_core.exe"
$commandPaths = @(
    (Join-Path $scriptDir "cargorise.cmd"),
    (Join-Path $scriptDir "cargorise-new.cmd"),
    (Join-Path $scriptDir "cargorise-open.cmd")
)

if (-not (Test-Path -LiteralPath $launcherPath)) {
    Write-Error "Cannot find cargorise.exe: $launcherPath"
    exit 1
}

if (-not (Test-Path -LiteralPath $corePath)) {
    Write-Error "Cannot find cargo_rise_core.exe: $corePath"
    exit 1
}

foreach ($commandPath in $commandPaths) {
    if (-not (Test-Path -LiteralPath $commandPath)) {
        Write-Error "Cannot find command wrapper: $commandPath"
        exit 1
    }
}

$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
$pathParts = @()
if ($userPath) {
    $pathParts = @(
        $userPath -split ";" |
            Where-Object { $_ -and ($_ -ine $scriptDir) }
    )
}

$newUserPath = (@($scriptDir) + $pathParts) -join ";"
try {
    [Environment]::SetEnvironmentVariable("Path", $newUserPath, "User")
} catch {
    Write-Error "Failed to update the user PATH. Open a normal PowerShell window from the latest CargoRise folder and run .\install_user_path.cmd again. Details: $($_.Exception.Message)"
    exit 1
}

$env:Path = "$scriptDir;$env:Path"

try {
    Add-Type -ErrorAction Stop -TypeDefinition @"
using System;
using System.Runtime.InteropServices;

public static class CargoRiseEnvironment {
    [DllImport("user32.dll", SetLastError = true, CharSet = CharSet.Auto)]
    public static extern IntPtr SendMessageTimeout(
        IntPtr hWnd,
        uint Msg,
        UIntPtr wParam,
        string lParam,
        uint fuFlags,
        uint uTimeout,
        out UIntPtr lpdwResult);
}
"@

    $result = [UIntPtr]::Zero
    [CargoRiseEnvironment]::SendMessageTimeout([IntPtr]0xffff, 0x001a, [UIntPtr]::Zero, "Environment", 0x0002, 5000, [ref]$result) | Out-Null
} catch {
    Write-Host "PATH was saved. If an already-open app still uses the old command, restart that app."
}

Write-Host "CargoRise folder installed at the front of your user PATH:"
Write-Host "  $scriptDir"
Write-Host ""
Write-Host "RustRover and VS Code read PATH when they start."
Write-Host "Fully restart RustRover or VS Code, then open a new terminal."
Write-Host ""
Write-Host "Test with:"
Write-Host "  where.exe cargorise"
Write-Host "  cargorise"
