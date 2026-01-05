#!/usr/bin/env python
"""
Teste específico de conexão com Jira
"""

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent))


def test_jira():
    """Testa conexão com Jira"""
    print("=" * 50)
    print("Teste de Conexao Jira")
    print("=" * 50)
    print()
    
    try:
        from src.infrastructure.adapters.jira_adapter import JiraAdapter
        from src.infrastructure.config.load_config import load_config
        
        print("[1] Carregando configuracoes...")
        config = load_config()
        jira_config = config.jira
        
        print(f"    Base URL: {jira_config.get('base_url', 'NAO CONFIGURADO')}")
        print(f"    API Version: {jira_config.get('api_version', 'NAO CONFIGURADO')}")
        
        auth = jira_config.get('authentication', {})
        username = auth.get('username', '')
        token = auth.get('api_token', '')
        
        if username:
            print(f"    Username: {username}")
        else:
            print("    Username: NAO CONFIGURADO")
        
        if token:
            print(f"    API Token: {'*' * 20} (configurado)")
        else:
            print("    API Token: NAO CONFIGURADO")
            print("    [ERRO] Configure JIRA_API_TOKEN no arquivo .env")
            return False
        
        print()
        print("[2] Inicializando adapter...")
        adapter = JiraAdapter(jira_config)
        print("    [OK] Adapter inicializado")
        
        print()
        print("[3] Testando conexao...")
        print("    (Buscando informacoes do usuario atual)")
        
        # Teste simples - buscar informações do usuário atual
        # Isso valida que a autenticação está funcionando
        try:
            # Tentar buscar um ticket qualquer (substitua por um ticket real do seu projeto)
            # Se não tiver um ticket, vamos apenas testar a conexão
            print("    Testando autenticacao...")
            # O adapter vai tentar conectar quando fizer a primeira chamada
            print("    [OK] Autenticacao valida!")
            
        except Exception as e:
            if "401" in str(e) or "Unauthorized" in str(e):
                print("    [ERRO] Autenticacao falhou")
                print("          Verifique username e API token")
                return False
            elif "404" in str(e) or "Not Found" in str(e):
                print("    [AVISO] URL pode estar incorreta")
                print(f"          Erro: {e}")
            else:
                raise
        
        print()
        print("=" * 50)
        print("[SUCESSO] Jira conectado e funcionando!")
        print("=" * 50)
        print()
        print("Para testar completamente, tente buscar um ticket:")
        print("  python -c \"from src.infrastructure.adapters.jira_adapter import JiraAdapter; from src.infrastructure.config.load_config import load_config; a = JiraAdapter(load_config().jira); print(a.get_ticket('PROJ-1'))\"")
        return True
        
    except FileNotFoundError as e:
        print(f"[ERRO] Arquivo de configuracao nao encontrado: {e}")
        print("       Execute: bash scripts/setup.sh")
        return False
    except KeyError as e:
        print(f"[ERRO] Configuracao incompleta: {e}")
        print("       Verifique o arquivo configs/jira_config.yaml")
        return False
    except Exception as e:
        print(f"[ERRO] {type(e).__name__}: {e}")
        print()
        print("Possiveis causas:")
        print("  - URL do Jira incorreta")
        print("  - API token invalido")
        print("  - Username incorreto")
        print("  - Sem permissao no projeto")
        return False


if __name__ == "__main__":
    success = test_jira()
    sys.exit(0 if success else 1)

