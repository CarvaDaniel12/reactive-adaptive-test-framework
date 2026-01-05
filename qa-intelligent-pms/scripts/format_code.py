#!/usr/bin/env python3
"""
Formata código automaticamente com ruff
Roda antes de commitar para manter código limpo
"""
import subprocess
import sys
from pathlib import Path

project_root = Path(__file__).parent.parent

def format_code():
    """Formata código com ruff"""
    print("[FORMAT] Formatando codigo...")
    result = subprocess.run(
        ["ruff", "format", "src/", "scripts/"],
        cwd=project_root
    )
    if result.returncode == 0:
        print("[OK] Codigo formatado")
    else:
        print("[ERRO] Erro ao formatar")
        sys.exit(1)

def fix_imports():
    """Organiza imports com ruff"""
    print("[IMPORTS] Organizando imports...")
    result = subprocess.run(
        ["ruff", "check", "--fix", "src/", "scripts/"],
        cwd=project_root
    )
    if result.returncode == 0:
        print("[OK] Imports organizados")
    else:
        print("[AVISO] Alguns imports nao puderam ser corrigidos automaticamente")

if __name__ == "__main__":
    format_code()
    fix_imports()
    print("\n[OK] Pronto! Codigo formatado e organizado.")
