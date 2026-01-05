#!/usr/bin/env python
"""
Script para verificar credenciais do Testmo e listar projetos disponíveis
"""

import sys
from pathlib import Path

project_root = Path(__file__).parent.parent
sys.path.insert(0, str(project_root))

from src.infrastructure.config.load_config import load_config
from src.infrastructure.adapters.testmo_adapter import TestmoAdapter

def main():
    print("=" * 70)
    print("Verificação de Credenciais Testmo")
    print("=" * 70)
    print()
    
    try:
        config = load_config()
        testmo_config = config.testmo
        
        print("[1] Verificando configuração...")
        print(f"    Base URL: {testmo_config.get('base_url', 'NÃO CONFIGURADO')}")
        print(f"    API Key: {'Configurado' if testmo_config.get('api_key') else 'NÃO CONFIGURADO'}")
        print(f"    Default Project ID: {testmo_config.get('default_project_id', 'Não configurado')}")
        print()
        
        if not testmo_config.get('api_key'):
            print("[ERRO] API Key não configurada!")
            return False
        
        print("[2] Conectando ao Testmo...")
        testmo = TestmoAdapter(testmo_config)
        print("    [OK] Adapter criado")
        print()
        
        print("[3] Buscando projetos...")
        try:
            # Tentar fazer requisição direta para ver resposta
            raw_response = testmo._request('GET', '/projects')
            print(f"    Resposta bruta da API: {raw_response}")
            print()
            
            projects = testmo.get_projects()
            
            if not projects:
                print("    [AVISO] Nenhum projeto encontrado")
                print()
                print("    Possíveis causas:")
                print("    - API Key não tem permissão para listar projetos")
                print("    - Não há projetos no Testmo")
                print("    - Base URL incorreta")
                print("    - Formato da resposta diferente do esperado")
                print()
                print("    Solução: Configure 'default_project_id' manualmente no testmo_config.yaml")
                print()
                print("    Para encontrar o Project ID:")
                print("    1. Acesse o Testmo no navegador")
                print("    2. Vá para um projeto")
                print("    3. O ID aparece na URL: https://hostfully-pmp.testmo.net/projects/12345")
                print("    4. Configure 'default_project_id: 12345' no testmo_config.yaml")
                return False
        except Exception as e:
            print(f"    [ERRO] Erro ao buscar projetos: {e}")
            print()
            print("    Solução: Configure 'default_project_id' manualmente no testmo_config.yaml")
            return False
        
        print(f"    [OK] {len(projects)} projeto(s) encontrado(s):")
        print()
        for i, project in enumerate(projects, 1):
            project_id = project.get('id')
            project_name = project.get('name', 'Sem nome')
            print(f"    {i}. ID: {project_id}, Nome: {project_name}")
        
        print()
        print("[4] Recomendação:")
        if projects:
            first_project_id = projects[0].get('id')
            print(f"    Configure 'default_project_id: {first_project_id}' no testmo_config.yaml")
            print(f"    Ou use o projeto ID {first_project_id} nas requisições")
        
        print()
        print("=" * 70)
        print("[SUCESSO] Credenciais válidas!")
        print("=" * 70)
        return True
        
    except Exception as e:
        print()
        print(f"[ERRO] {type(e).__name__}: {e}")
        import traceback
        traceback.print_exc()
        return False

if __name__ == "__main__":
    success = main()
    sys.exit(0 if success else 1)
