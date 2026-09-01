# CargoRise 安装说明

## 推荐顺序

1. 打开 `CargoRise_Portable`
2. 运行：

```powershell
.\install_windowsapps.cmd
```

3. 关闭并重新打开 RustRover 或 VS Code
4. 在新终端里输入：

```powershell
cargorise
```

## 其他方式

### 用户 PATH

```powershell
.\install_user_path.cmd
```

### PowerShell Profile

```powershell
.\install_powershell_profile.cmd
```

## 常用命令

```powershell
cargorise
cargorise-new hello_world
cargorise-open D:\RustProjects\hello_world
```

## 常见问题

### 终端里找不到命令

先执行：

```powershell
where.exe cargorise
```

如果刚修改 PATH，请重新打开编辑器和终端。

### IDE 没有自动打开

确认以下命令在 PATH 中至少有一个：

```powershell
code
rustrover
```

## 备注

- 这个版本是便携包。
- 不需要在项目里额外添加 `.json` 配置。
- `cargorise-new` 是最快路径，`cargorise` 是窗口模式。

