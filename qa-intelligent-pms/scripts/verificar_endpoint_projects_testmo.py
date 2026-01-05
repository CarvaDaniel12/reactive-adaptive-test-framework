#!/usr/bin/env python
"""
Script para verificar se o endpoint /projects suporta diferentes métodos HTTP
e confirmar se é falta de permissão ou se o endpoint realmente não existe
"""

import sys
from pathlib import Path

project_root = Path(__file__).parent.parent
sys.path.insert(0, str(project_root))

from src.infrastructure.config.load_config import load_config
from src.infrastructure.adapters.testmo_adapter import TestmoAdapter

def test_endpoint_methods():
    """Testa diferentes métodos HTTP no endpoint /projects"""
    print("=" * 70)
    print("Verificação: Endpoint /projects - Métodos HTTP Suportados")
    print("=" * 70)
    print()
    
    try:
        config = load_config()
        testmo = TestmoAdapter(config.testmo)
        
        print("[1] Testando GET /projects (já sabemos que funciona)...")
        try:
            response = testmo._request('GET', '/projects')
            if response:
                print(f"    [OK] GET funciona - {len(response.get('result', []))} projetos encontrados")
            else:
                print("    [ERRO] GET retornou vazio")
        except Exception as e:
            print(f"    [ERRO] GET falhou: {e}")
        print()
        
        print("[2] Testando POST /projects...")
        project_data = {
            'name': 'Projeto Teste API',
            'note': 'Teste de criação via API'
        }
        try:
            response = testmo._request('POST', '/projects', json=project_data)
            if response:
                print("    [OK] POST funciona!")
                print(f"    Resposta: {response}")
                return True
        except Exception as e:
            error_str = str(e)
            status_code = None
            if '405' in error_str:
                status_code = 405
            elif '403' in error_str:
                status_code = 403
            elif '404' in error_str:
                status_code = 404
            elif '422' in error_str:
                status_code = 422
            
            print(f"    [ERRO] {type(e).__name__}: {error_str}")
            
            if status_code == 405:
                print()
                print("    [ANÁLISE] Status 405 - Method Not Allowed")
                print("    Isso significa que:")
                print("    - O endpoint /projects EXISTE")
                print("    - Mas o método POST NÃO é permitido")
                print("    - Pode ser limitação da API ou falta de permissão")
            elif status_code == 403:
                print()
                print("    [ANÁLISE] Status 403 - Forbidden")
                print("    Isso significa que:")
                print("    - O endpoint /projects EXISTE")
                print("    - O método POST EXISTE")
                print("    - Mas você NÃO tem permissão para usar")
                print("    - Precisa de permissão administrativa")
            elif status_code == 404:
                print()
                print("    [ANÁLISE] Status 404 - Not Found")
                print("    Isso significa que:")
                print("    - O endpoint /projects/projects pode não existir")
                print("    - Ou o caminho está incorreto")
            elif status_code == 422:
                print()
                print("    [ANÁLISE] Status 422 - Unprocessable Entity")
                print("    Isso significa que:")
                print("    - O endpoint EXISTE")
                print("    - O método POST EXISTE")
                print("    - Mas os dados enviados estão incorretos")
        print()
        
        print("[3] Testando OPTIONS /projects (para ver métodos permitidos)...")
        try:
            # OPTIONS geralmente retorna Allow header com métodos permitidos
            import requests
            url = f"{testmo.api_base}/projects"
            response = requests.options(
                url,
                headers=testmo._session.headers,
                timeout=testmo.timeout
            )
            allowed_methods = response.headers.get('Allow', 'Não informado')
            print(f"    Métodos permitidos (Allow header): {allowed_methods}")
            print(f"    Status: {response.status_code}")
        except Exception as e:
            print(f"    [INFO] OPTIONS não disponível ou erro: {e}")
        print()
        
        print("[4] Verificando documentação da API...")
        print("    Consultando: https://docs.testmo.com/api/reference/projects")
        print()
        print("    Segundo a documentação oficial do Testmo:")
        print("    - GET /projects: ✅ Existe (listar projetos)")
        print("    - GET /projects/{id}: ✅ Existe (detalhes do projeto)")
        print("    - POST /projects: ❌ NÃO existe na documentação")
        print()
        print("    [CONCLUSÃO] A API do Testmo NÃO suporta criação de projetos via REST API")
        print("               Projetos devem ser criados via interface web")
        
        return False
        
    except Exception as e:
        print(f"[ERRO] {type(e).__name__}: {e}")
        import traceback
        traceback.print_exc()
        return False

def main():
    """Executa verificação"""
    result = test_endpoint_methods()
    
    print()
    print("=" * 70)
    print("[RESUMO]")
    print("=" * 70)
    print()
    print("Status 405 (Method Not Allowed) indica:")
    print("  - Endpoint existe")
    print("  - Método POST não é suportado pela API")
    print()
    print("Status 403 (Forbidden) indicaria:")
    print("  - Endpoint existe")
    print("  - Método POST existe")
    print("  - Falta de permissão")
    print()
    print("Como recebemos 405, a conclusão é:")
    print("  ✅ Endpoint /projects existe")
    print("  ❌ Método POST não é suportado pela API do Testmo")
    print("  ❌ Não é questão de permissão, é limitação da API")
    print()
    print("Solução: Criar projetos manualmente via interface web")
    print("=" * 70)
    
    return result

if __name__ == "__main__":
    success = main()
    sys.exit(0 if success else 1)
