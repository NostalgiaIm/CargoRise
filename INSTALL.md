# CargoRise Installation

## Recommended Setup

1. Open the extracted CargoRise folder, for example `C:\Tools\CargoRise`.
2. 运行：

```powershell
.\install_windowsapps.cmd
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
- `v0.2.1` uses `assets/CargoRise.ico` as the default GUI and Windows launcher icon.
