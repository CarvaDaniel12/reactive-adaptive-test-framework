#!/usr/bin/env python3
"""
Teste de Conexão Splunk via REST API Alternativa
"""

import sys
from pathlib import Path

# Adicionar diretório raiz ao path
project_root = Path(__file__).parent.parent
sys.path.insert(0, str(project_root))

def test_splunk_rest_api():
    """Testa conexão Splunk usando REST API direta"""
    print("=" * 50)
    print("Teste de Conexao Splunk (REST API Alternativa)")
    print("=" * 50)
    print()
    
    try:
        from src.infrastructure.adapters.splunk_adapter import SplunkAdapter
        from src.infrastructure.config.load_config import load_config
        
        print("[1] Carregando configuracoes...")
        config = load_config()
        splunk_config = config.splunk
        
        # Ativar REST API
        splunk_config['use_rest_api'] = True
        
        print(f"    Host: {splunk_config.get('host', 'NAO CONFIGURADO')}")
        print(f"    Port: {splunk_config.get('port', 'NAO CONFIGURADO')}")
        print(f"    Modo: REST API (alternativa)")
        print(f"    Index: {splunk_config.get('default_index', 'NAO CONFIGURADO')}")
        
        auth = splunk_config.get('authentication', {})
        auth_type = auth.get('type', 'basic')
        
        if auth_type == 'basic':
            username = auth.get('username', '')
            if username:
                print(f"    Username: {username}")
                print(f"    Password: {'*' * 20} (configurado)")
            else:
                print("    [ERRO] Configure SPLUNK_USERNAME e SPLUNK_PASSWORD no arquivo .env")
                return False
        
        print()
        print("[2] Inicializando adapter (REST API)...")
        adapter = SplunkAdapter(splunk_config)
        print("    [OK] Adapter inicializado")
        
        print()
        print("[3] Testando conexao via REST API...")
        print("    (Isso pode demorar alguns segundos)")
        
        # Teste simples - query que não retorna dados mas testa conexão
        test_query = "search index=* | head 1"
        
        try:
            results = adapter.execute_query(test_query)
            print(f"    [OK] Conexao estabelecida com sucesso!")
            print(f"    Resultados encontrados: {len(results)}")
            
            if results:
                print()
                print("    Primeiro resultado:")
                for key, value in list(results[0].items())[:5]:
                    print(f"      {key}: {value}")
            
            print()
            print("=" * 50)
            print("[SUCESSO] Splunk conectado via REST API!")
            print("=" * 50)
            print()
            print("Agora voce pode usar o Splunk normalmente.")
            print("Configure use_rest_api: true em configs/splunk_config.yaml")
            
            return True
            
        except Exception as e:
            print(f"    [ERRO] {type(e).__name__}: {e}")
            print()
            print("Possiveis causas:")
            print("  - Host ou porta incorretos")
            print("  - Username/password invalidos")
            print("  - Firewall bloqueando conexao")
            print("  - Permissoes insuficientes")
            return False
        
    except Exception as e:
        print(f"[ERRO] {type(e).__name__}: {e}")
        import traceback
        traceback.print_exc()
        return False


if __name__ == "__main__":
    success = test_splunk_rest_api()
    exit(0 if success else 1)

