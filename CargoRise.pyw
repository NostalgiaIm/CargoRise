import os
import runpy
from pathlib import Path


# 保存调用者原本所在目录。
# 比如你在 RustRover 终端的某个项目里输入 cargorise，这里会记住那个项目目录。
os.environ.setdefault("CARGORISE_CALLER_CWD", os.getcwd())

# .pyw 会使用 pythonw.exe 打开，不会自带控制台窗口。
app_file = Path(__file__).resolve().with_suffix(".py")
os.chdir(app_file.parent)
runpy.run_path(str(app_file), run_name="__main__")
