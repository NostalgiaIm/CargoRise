Set FileSystem = CreateObject("Scripting.FileSystemObject")
Set Shell = CreateObject("WScript.Shell")

ScriptDir = FileSystem.GetParentFolderName(WScript.ScriptFullName)
AppFile = ScriptDir & "\CargoRise.pyw"
LauncherFile = ScriptDir & "\cargorise.exe"

Args = ""
For Each Arg In WScript.Arguments
    Args = Args & " " & Quote(Arg)
Next

' 0 表示隐藏窗口；False 表示不等待 Python 窗口关闭。
If FileSystem.FileExists(LauncherFile) Then
    Shell.Run Quote(LauncherFile) & Args, 0, False
Else
    Shell.Run "pythonw.exe " & Quote(AppFile) & Args, 0, False
End If

Function Quote(Value)
    Quote = Chr(34) & Replace(Value, Chr(34), Chr(34) & Chr(34)) & Chr(34)
End Function
