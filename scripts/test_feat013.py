#!/usr/bin/env python3
"""
测试 FEAT-013: 跨平台支持与 pip 安装
验证 `uv add uv-plus` 安装流程
"""

import os
import sys
import subprocess
import tempfile
import shutil
from pathlib import Path


def run_command(cmd, cwd=None, check=True):
    """运行命令并返回结果"""
    print(f"\n{'='*60}")
    print(f"执行命令: {' '.join(cmd)}")
    if cwd:
        print(f"工作目录: {cwd}")
    print('='*60)
    
    result = subprocess.run(
        cmd,
        cwd=cwd,
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        shell=(os.name == 'nt')
    )
    
    if result.stdout:
        print(result.stdout)
    if result.stderr:
        print(result.stderr, file=sys.stderr)
    
    if check and result.returncode != 0:
        print(f"✗ 命令失败，退出码: {result.returncode}")
        sys.exit(1)
    
    return result


def test_build_package():
    """测试构建 Python 包"""
    print("\n" + "="*60)
    print("步骤 1: 构建 Python 包")
    print("="*60)
    
    project_root = Path(__file__).parent.parent
    
    # 清理旧的构建产物
    dist_dir = project_root / "dist"
    if dist_dir.exists():
        try:
            shutil.rmtree(dist_dir)
            print(f"已清理: {dist_dir}")
        except PermissionError:
            print(f"[WARN] 无法清理 {dist_dir}，跳过")
    
    # 构建包
    run_command(["uv", "build"], cwd=project_root)
    
    # 验证构建产物
    wheels = list(dist_dir.glob("uv_plus-*.whl"))
    if not wheels:
        print("✗ 未找到 wheel 文件")
        sys.exit(1)
    
    print(f"✓ 构建成功: {wheels[0].name}")
    return wheels[0]


def test_install_in_temp_project(wheel_path):
    """在临时项目中测试安装"""
    print("\n" + "="*60)
    print("步骤 2: 在临时项目中测试安装")
    print("="*60)
    
    with tempfile.TemporaryDirectory() as tmpdir:
        test_dir = Path(tmpdir) / "test-project"
        test_dir.mkdir()
        print(f"测试目录: {test_dir}")
        
        # 初始化 uv 项目
        print("\n初始化 uv 项目...")
        run_command(["uv", "init"], cwd=test_dir)
        
        # 安装 wheel
        print(f"\n安装 wheel: {wheel_path.name}")
        run_command(["uv", "add", str(wheel_path)], cwd=test_dir)
        
        # 验证安装
        print("\n验证安装...")
        result = run_command(["uv", "pip", "list"], cwd=test_dir, check=False)
        
        if "uv-plus" not in result.stdout:
            print("✗ uv-plus 未安装")
            sys.exit(1)
        
        print("[OK] uv-plus 已安装")
        
        # 测试 uvp 命令（使用 uv run）
        print("\n测试 uvp 命令...")
        result = run_command(["uv", "run", "uvp", "--version"], cwd=test_dir, check=False)
        
        if result.returncode != 0:
            print("✗ uvp 命令不可用")
            print("这可能是因为:")
            print("  1. Python 包装器未正确找到二进制文件")
            print("  2. 二进制文件未包含在 wheel 中")
            print("  3. PATH 环境变量未正确设置")
            sys.exit(1)
        
        print("[OK] uvp 命令可用")
        
        # 测试基本功能
        print("\n测试 uvp 基本功能...")
        result = run_command(["uv", "run", "uvp", "init", "--help"], cwd=test_dir, check=False)
        
        if result.returncode != 0:
            print("✗ uvp init --help 失败")
            sys.exit(1)
        
        print("[OK] uvp 基本功能正常")
        
        print("\n" + "="*60)
        print("[OK] 所有测试通过！")
        print("="*60)


def main():
    """主测试流程"""
    print("="*60)
    print("FEAT-013 测试: 跨平台支持与 pip 安装")
    print("="*60)
    
    # 检查 uv 是否可用
    result = run_command(["uv", "--version"], check=False)
    if result.returncode != 0:
        print("✗ uv 未安装或不可用")
        sys.exit(1)
    
    print(f"[OK] uv 版本: {result.stdout.strip()}")
    
    # 构建包
    wheel_path = test_build_package()
    
    # 在临时项目中测试安装
    test_install_in_temp_project(wheel_path)


if __name__ == "__main__":
    main()
