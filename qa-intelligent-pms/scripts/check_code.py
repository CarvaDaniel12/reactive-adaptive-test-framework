#!/usr/bin/env python3
"""
Script rápido para verificar código antes de rodar
Evita perder tempo com erros de sintaxe
"""
import subprocess
import sys
from pathlib import Path

project_root = Path(__file__).parent.parent

def run_ruff():
    """Roda ruff check - pega erros de sintaxe e estilo"""
    print("[RUFF] Verificando codigo...")
    result = subprocess.run(
        ["ruff", "check", "src/", "scripts/"],
        cwd=project_root,
        capture_output=True,
        text=True
    )
    if result.returncode != 0:
        print("[ERRO] ERROS ENCONTRADOS:")
        print(result.stdout)
        return False
    print("[OK] Ruff: Sem erros")
    return True

def run_mypy():
    """Roda mypy - verifica tipos"""
    print("\n[MYPY] Verificando tipos...")
    result = subprocess.run(
        ["mypy", "src/", "--ignore-missing-imports"],
        cwd=project_root,
        capture_output=True,
        text=True
    )
    if result.returncode != 0:
        print("[AVISO] AVISOS DE TIPO:")
        print(result.stdout)
    else:
        print("[OK] MyPy: Sem erros de tipo")
    return True  # mypy warnings não bloqueiam

def main():
    """Executa todas as verificações"""
    print("=" * 60)
    print("VERIFICACAO RAPIDA DE CODIGO")
    print("=" * 60)
    
    ruff_ok = run_ruff()
    
    if not ruff_ok:
        print("\n[ERRO] CORRIJA OS ERROS ANTES DE CONTINUAR")
        sys.exit(1)
    
    run_mypy()
    
    print("\n" + "=" * 60)
    print("[OK] VERIFICACAO CONCLUIDA")
    print("=" * 60)

if __name__ == "__main__":
    main()
