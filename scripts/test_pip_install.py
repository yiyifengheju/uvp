#!/usr/bin/env python3
"""测试 pip 安装流程"""

import os
import sys
import subprocess
import tempfile
import shutil
from pathlib import Path


def run_command(cmd, cwd=None):
    """运行命令并返回结果"""
    print(f"运行: {' '.join(cmd)}")
    result = subprocess.run(
        cmd,
        cwd=cwd,
        capture_output=True,
        text=True,
        shell=(os.name == 'nt')
    )
    if result.returncode != 0:
        print(f"错误: {result.stderr}")
        return False
    if result.stdout:
        print(result.stdout)
    return True


def test_pip_install():
    """测试 pip 安装"""
    print("=" * 60)
    print("测试 pip 安装流程")
    print("=" * 60)
    
    # 获取项目根目录
    project_root = Path(__file__).parent.parent
    print(f"项目根目录: {project_root}")
    
    # 创建临时测试目录
    with tempfile.TemporaryDirectory() as tmpdir:
        test_dir = Path(tmpdir) / "test-project"
        test_dir.mkdir()
        print(f"测试目录: {test_dir}")
        
        # 1. 初始化 uv 项目
        print("\n[1/5] 初始化 uv 项目...")
        if not run_command(["uv", "init"], cwd=test_dir):
            print("✗ uv init 失败")
            return False
        print("✓ uv init 成功")
        
        # 2. 构建 Python 包
        print("\n[2/5] 构建 Python 包...")
        if not run_command(["uv", "build"], cwd=project_root):
            print("✗ uv build 失败")
            return False
        print("✓ uv build 成功")
        
        # 3. 查找构建的 wheel 文件
        print("\n[3/5] 查找 wheel 文件...")
        dist_dir = project_root / "dist"
        wheels = list(dist_dir.glob("uv_plus-*.whl"))
        if not wheels:
            print("✗ 未找到 wheel 文件")
            return False
        wheel_file = wheels[0]
        print(f"找到 wheel: {wheel_file.name}")
        
        # 4. 安装 wheel
        print("\n[4/5] 安装 wheel...")
        if not run_command(["uv", "add", str(wheel_file)], cwd=test_dir):
            print("✗ uv add 失败")
            return False
        print("✓ uv add 成功")
        
        # 5. 测试 uvp 命令
        print("\n[5/5] 测试 uvp 命令...")
        if not run_command(["uvp", "--version"], cwd=test_dir):
            print("✗ uvp --version 失败")
            return False
        print("✓ uvp 命令可用")
        
        print("\n" + "=" * 60)
        print("✓ 所有测试通过！")
        print("=" * 60)
        return True


if __name__ == "__main__":
    success = test_pip_install()
    sys.exit(0 if success else 1)
