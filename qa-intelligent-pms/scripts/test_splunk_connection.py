#!/usr/bin/env python
"""
Teste específico de conexão com Splunk
"""

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent))


def test_splunk():
    """Testa conexão com Splunk"""
    print("=" * 50)
    print("Teste de Conexao Splunk")
    print("=" * 50)
    print()
    
    try:
        from src.infrastructure.adapters.splunk_adapter import SplunkAdapter
        from src.infrastructure.config.load_config import load_config
        
        print("[1] Carregando configuracoes...")
        config = load_config()
        splunk_config = config.splunk
        
        print(f"    Host: {splunk_config.get('host', 'NAO CONFIGURADO')}")
        print(f"    Port: {splunk_config.get('port', 'NAO CONFIGURADO')}")
        print(f"    Index: {splunk_config.get('default_index', 'NAO CONFIGURADO')}")
        
        auth = splunk_config.get('authentication', {})
        auth_type = auth.get('type', 'token')
        
        if auth_type == 'token':
            token = auth.get('token', '')
            if token:
                print(f"    Token: {'*' * 20} (configurado)")
            else:
                print("    Token: NAO CONFIGURADO")
                print("    [ERRO] Configure SPLUNK_TOKEN no arquivo .env")
                return False
        elif auth_type == 'basic':
            username = auth.get('username', '')
            password = auth.get('password', '')
            if username and password:
                print(f"    Username: {username}")
                print(f"    Password: {'*' * 20} (configurado)")
            else:
                print("    Username/Password: NAO CONFIGURADO")
                print("    [ERRO] Configure SPLUNK_USERNAME e SPLUNK_PASSWORD no arquivo .env")
                return False
        else:
            print(f"    Tipo de autenticacao: {auth_type}")
            print("    [ERRO] Tipo de autenticacao nao suportado")
            return False
        
        print()
        print("[2] Inicializando adapter...")
        adapter = SplunkAdapter(splunk_config)
        print("    [OK] Adapter inicializado")
        
        print()
        print("[3] Testando conexao...")
        print("    (Isso pode demorar alguns segundos)")
        
        # Teste simples - query que não retorna dados mas testa conexão
        test_query = "search index=* | head 1"
        results = adapter.execute_query(test_query)
        
        print(f"    [OK] Conexao estabelecida com sucesso!")
        print(f"    Resultados: {len(results)} linha(s)")
        
        print()
        print("=" * 50)
        print("[SUCESSO] Splunk conectado e funcionando!")
        print("=" * 50)
        return True
        
    except FileNotFoundError as e:
        print(f"[ERRO] Arquivo de configuracao nao encontrado: {e}")
        print("       Execute: bash scripts/setup.sh")
        return False
    except KeyError as e:
        print(f"[ERRO] Configuracao incompleta: {e}")
        print("       Verifique o arquivo configs/splunk_config.yaml")
        return False
    except Exception as e:
        print(f"[ERRO] {type(e).__name__}: {e}")
        print()
        print("Possiveis causas:")
        print("  - Host ou porta incorretos")
        print("  - Token invalido ou expirado")
        print("  - Firewall bloqueando conexao")
        print("  - Permissoes insuficientes no token")
        return False


if __name__ == "__main__":
    success = test_splunk()
    sys.exit(0 if success else 1)

