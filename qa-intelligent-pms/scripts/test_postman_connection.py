#!/usr/bin/env python
"""
Teste específico de conexão com Postman
"""

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent))


def test_postman():
    """Testa conexão com Postman"""
    print("=" * 50)
    print("Teste de Conexao Postman")
    print("=" * 50)
    print()
    
    try:
        from src.infrastructure.adapters.postman_adapter import PostmanAdapter
        from src.infrastructure.config.load_config import load_config
        
        print("[1] Carregando configuracoes...")
        config = load_config()
        postman_config = config.postman
        
        api_key = postman_config.get('api_key', '')
        workspace_id = postman_config.get('workspace_id', '')
        
        if api_key:
            print(f"    API Key: {'*' * 20} (configurado)")
        else:
            print("    API Key: NAO CONFIGURADO")
            print("    [ERRO] Configure POSTMAN_API_KEY no arquivo .env")
            return False
        
        if workspace_id:
            print(f"    Workspace ID: {workspace_id}")
        else:
            print("    Workspace ID: NAO CONFIGURADO")
            print("    [AVISO] Configure workspace_id no postman_config.yaml")
        
        print()
        print("[2] Inicializando adapter...")
        adapter = PostmanAdapter(postman_config)
        print("    [OK] Adapter inicializado")
        
        print()
        print("[3] Testando conexao...")
        print("    (Listando collections do workspace)")
        
        # Teste simples - listar collections
        collections = adapter.list_collections()
        
        print(f"    [OK] Conexao estabelecida com sucesso!")
        print(f"    Collections encontradas: {len(collections)}")
        
        if collections:
            print()
            print("    Primeiras collections:")
            for i, coll in enumerate(collections[:3], 1):
                print(f"      {i}. {coll.get('name', 'Sem nome')}")
        
        print()
        print("=" * 50)
        print("[SUCESSO] Postman conectado e funcionando!")
        print("=" * 50)
        return True
        
    except FileNotFoundError as e:
        print(f"[ERRO] Arquivo de configuracao nao encontrado: {e}")
        print("       Execute: bash scripts/setup.sh")
        return False
    except KeyError as e:
        print(f"[ERRO] Configuracao incompleta: {e}")
        print("       Verifique o arquivo configs/postman_config.yaml")
        return False
    except Exception as e:
        error_str = str(e)
        print(f"[ERRO] {type(e).__name__}: {error_str}")
        print()
        
        if "401" in error_str or "Unauthorized" in error_str:
            print("Possiveis causas:")
            print("  - API Key invalida ou expirada")
            print("  - API Key nao tem permissao")
        elif "404" in error_str or "Not Found" in error_str:
            print("Possiveis causas:")
            print("  - Workspace ID incorreto")
            print("  - Workspace nao existe ou voce nao tem acesso")
        else:
            print("Possiveis causas:")
            print("  - API Key incorreta")
            print("  - Workspace ID incorreto")
            print("  - Problema de conexao")
        
        return False


if __name__ == "__main__":
    success = test_postman()
    sys.exit(0 if success else 1)

