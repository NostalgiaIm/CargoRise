# CargoRise Installation

## Recommended Setup

1. Open the extracted CargoRise folder, for example `C:\Tools\CargoRise`.
2. Run:

```powershell
.\install_user_path.cmd
```

3. Restart RustRover or VS Code.
4. Open a new terminal and run:

```powershell
cargorise
```

## Other Options

### User PATH

```powershell
.\install_user_path.cmd
```

### WindowsApps Compatibility Wrapper

```powershell
.\install_windowsapps.cmd
```

This wrapper uses the same PATH-first installer. It is kept for users who installed older CargoRise builds through `install_windowsapps.cmd`.

### PowerShell Profile

```powershell
.\install_powershell_profile.cmd
```

## Common Commands

```powershell
cargorise
cargorise-new hello_world
cargorise-open C:\Projects\Rust\hello_world
```

## Troubleshooting

### Command not found

Run:

```powershell
where.exe cargorise
```

If you just changed `PATH`, reopen your editor and terminal.

If CargoRise still opens an older copy, run `install_user_path.cmd` from the latest CargoRise folder. The installer puts that portable folder first in the user `PATH`; then fully restart RustRover or VS Code.

### IDE does not open automatically

Make sure at least one of these commands is available on `PATH`:

```powershell
code
rustrover
```

## Notes

- This release is portable.
- No extra `.json` files are needed inside each project.
- `cargorise-new` is the fastest creation path, and `cargorise` is the GUI mode.
- `v0.2.3` makes the `cargorise` command prefer the Rust launcher, so Windows can use the embedded CargoRise icon more reliably.
- `v0.2.2` puts the latest portable CargoRise folder first in the user `PATH`, so editors do not keep launching older copies after reinstalling.
- `v0.2.1` uses `assets/CargoRise.ico` as the default GUI and Windows launcher icon.
