#!/usr/bin/env python
"""
Script para testar se é possível criar um projeto no Testmo via API
"""

import sys
from pathlib import Path

project_root = Path(__file__).parent.parent
sys.path.insert(0, str(project_root))

from src.infrastructure.config.load_config import load_config
from src.infrastructure.adapters.testmo_adapter import TestmoAdapter

def test_create_project():
    """Testa criar um projeto no Testmo"""
    print("=" * 70)
    print("Teste: Criar Projeto no Testmo via API")
    print("=" * 70)
    print()
    
    try:
        config = load_config()
        testmo = TestmoAdapter(config.testmo)
        
        print("[1] Verificando projetos existentes...")
        existing_projects = testmo.get_projects()
        print(f"    Projetos existentes: {len(existing_projects)}")
        for p in existing_projects[:3]:
            print(f"      - {p.get('name')} (ID: {p.get('id')})")
        print()
        
        print("[2] Tentando criar projeto de teste...")
        print("    Nome: 'Projeto Teste API'")
        print("    Nota: 'Teste de criação via API'")
        print()
        
        # Tentar criar projeto
        project_data = {
            'name': 'Projeto Teste API',
            'note': 'Teste de criação via API'
        }
        
        try:
            response = testmo._request('POST', '/projects', json=project_data)
            if response:
                print("    [SUCESSO] Projeto criado!")
                print(f"    Resposta: {response}")
                return True
            else:
                print("    [FALHOU] Resposta vazia")
                return False
        except Exception as e:
            error_str = str(e)
            print(f"    [ERRO] {type(e).__name__}: {error_str}")
            
            if '405' in error_str or 'Method Not Allowed' in error_str:
                print()
                print("    [CONFIRMADO] API não suporta criação de projetos")
                print("    Método POST não é permitido em /projects")
            elif '403' in error_str or 'Forbidden' in error_str:
                print()
                print("    [AVISO] Permissão negada")
                print("    API Key pode não ter permissão para criar projetos")
            elif '422' in error_str:
                print()
                print("    [AVISO] Dados inválidos")
                print("    Formato da requisição pode estar incorreto")
            else:
                print()
                print("    [INFO] Erro desconhecido")
            
            return False
        
    except Exception as e:
        print(f"[ERRO] {type(e).__name__}: {e}")
        import traceback
        traceback.print_exc()
        return False

def main():
    """Executa teste"""
    result = test_create_project()
    
    print()
    print("=" * 70)
    if result:
        print("[CONCLUSÃO] É possível criar projetos via API")
    else:
        print("[CONCLUSÃO] NÃO é possível criar projetos via API")
        print()
        print("Solução: Criar projetos manualmente via interface web do Testmo")
        print("         Depois usar o project_id nos scripts")
    print("=" * 70)
    
    return result

if __name__ == "__main__":
    success = main()
    sys.exit(0 if success else 1)
