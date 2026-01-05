#!/usr/bin/env python
"""
Scripts de teste básicos para integrações
Testa cada integração individualmente
"""

import sys
from pathlib import Path

# Adicionar src ao path
sys.path.insert(0, str(Path(__file__).parent.parent))


def test_jira():
    """Testa conexão com Jira"""
    print("[TEST] Testando Jira...")
    try:
        from src.infrastructure.adapters.jira_adapter import JiraAdapter
        from src.infrastructure.config.load_config import load_config
        
        config = load_config()
        adapter = JiraAdapter(config.jira)
        
        # Teste simples - buscar um ticket (substitua PMS-1 por um ticket real)
        print("  [INFO] Configure um ticket_key válido no código para testar")
        print("  [OK] Jira adapter inicializado com sucesso")
        return True
    except Exception as e:
        print(f"  [ERRO] {e}")
        return False


def test_splunk():
    """Testa conexão com Splunk"""
    print("[TEST] Testando Splunk...")
    try:
        from src.infrastructure.adapters.splunk_adapter import SplunkAdapter
        from src.infrastructure.config.load_config import load_config
        
        config = load_config()
        adapter = SplunkAdapter(config.splunk)
        
        print("  [OK] Splunk adapter inicializado com sucesso")
        print("  [INFO] Execute uma query real para testar completamente")
        return True
    except Exception as e:
        print(f"  [ERRO] {e}")
        return False


def test_postman():
    """Testa conexão com Postman"""
    print("[TEST] Testando Postman...")
    try:
        from src.infrastructure.adapters.postman_adapter import PostmanAdapter
        from src.infrastructure.config.load_config import load_config
        
        config = load_config()
        adapter = PostmanAdapter(config.postman)
        
        print("  [OK] Postman adapter inicializado com sucesso")
        print("  [INFO] Tente listar collections para testar completamente")
        return True
    except Exception as e:
        print(f"  [ERRO] {e}")
        return False


def test_playwright():
    """Testa Playwright"""
    print("[TEST] Testando Playwright...")
    try:
        from playwright.sync_api import sync_playwright
        
        with sync_playwright() as p:
            browser = p.chromium.launch(headless=True)
            page = browser.new_page()
            page.goto("https://example.com")
            title = page.title()
            browser.close()
            print(f"  [OK] Playwright funcionando! Titulo: {title}")
            return True
    except Exception as e:
        print(f"  [ERRO] {e}")
        print("  [INFO] Execute: playwright install chromium")
        return False


def main():
    """Executa todos os testes"""
    print("=" * 50)
    print("Testes de Integração")
    print("=" * 50)
    print()
    
    results = {
        'Jira': test_jira(),
        'Splunk': test_splunk(),
        'Postman': test_postman(),
        'Playwright': test_playwright()
    }
    
    print()
    print("=" * 50)
    print("Resumo:")
    print("=" * 50)
    for name, result in results.items():
        status = "[OK]" if result else "[FALHOU]"
        print(f"  {name}: {status}")
    
    all_ok = all(results.values())
    if all_ok:
        print("\n[OK] Todos os testes basicos passaram!")
    else:
        print("\n[AVISO] Alguns testes falharam. Verifique configuracoes.")


if __name__ == "__main__":
    main()

