#!/usr/bin/env python3
"""
Script para verificar permissões do usuário no Splunk
"""

import sys
from pathlib import Path

# Adicionar diretório raiz ao path
project_root = Path(__file__).parent.parent
sys.path.insert(0, str(project_root))

def check_permissions():
    """Verifica permissões do usuário no Splunk"""
    print("=" * 60)
    print("Verificando Permissoes do Usuario no Splunk")
    print("=" * 60)
    print()
    
    try:
        from src.infrastructure.adapters.splunk_adapter import SplunkAdapter, SplunkConnectionError
        from src.infrastructure.config.load_config import load_config
        import requests
        
        print("[1] Carregando configuracoes...")
        config = load_config()
        splunk_config = config.splunk
        
        # Ativar REST API para este teste
        splunk_config['use_rest_api'] = True
        
        host = splunk_config.get('host', 'NAO CONFIGURADO')
        port = splunk_config.get('port', 'NAO CONFIGURADO')
        auth = splunk_config.get('authentication', {})
        username = auth.get('username', '')
        
        print(f"    Host: {host}")
        print(f"    Port: {port}")
        print(f"    Username: {username}")
        print()
        
        print("[2] Tentando autenticar...")
        adapter = SplunkAdapter(splunk_config)
        
        # Tentar obter sessão primeiro
        try:
            session_token = adapter._get_rest_api_session()
            print(f"    [OK] Autenticacao bem-sucedida!")
            print(f"    Token de sessao obtido: {session_token[:20]}...")
            print()
        except SplunkConnectionError as e:
            print(f"    [ERRO] Falha na autenticacao: {e}")
            print()
            print("Nao foi possivel autenticar. Verifique:")
            print("  - Username e password corretos")
            print("  - Porta 8089 aberta (pode precisar abrir via support case)")
            print("  - Conta nao e free trial (free trial nao tem acesso REST API)")
            return False
        
        print("[3] Verificando informacoes do usuario...")
        print("    (Testando varios endpoints da API)")
        print()
        
        # Tentar diferentes endpoints para verificar permissões
        endpoints_to_test = [
            ("Contexto Atual", "/services/auth/current-context"),
            ("Usuario Atual", "/services/authentication/users/current"),
            ("Roles", "/services/authorization/roles"),
            ("Capabilities", "/services/authorization/capabilities"),
            ("Info do Servidor", "/services/server/info"),
        ]
        
        effective_port = adapter.port
        base_path = getattr(adapter, '_rest_api_path', '')
        headers = {'Authorization': f'Splunk {session_token}'}
        
        results = {}
        
        for name, endpoint in endpoints_to_test:
            try:
                url = f"https://{host}:{effective_port}{base_path}{endpoint}"
                print(f"    Testando: {name} ({endpoint})")
                
                response = requests.get(
                    url,
                    headers=headers,
                    verify=True,
                    timeout=30
                )
                
                if response.status_code == 200:
                    print(f"      [OK] Status 200 - Acesso permitido")
                    
                    # Tentar parsear resposta
                    try:
                        import xml.etree.ElementTree as ET
                        root = ET.fromstring(response.text)
                        
                        # Extrair informações relevantes
                        info = {}
                        for elem in root.iter():
                            if elem.text and elem.tag not in ['response', 'messages', 'meta']:
                                tag = elem.tag
                                if tag not in info:
                                    info[tag] = []
                                info[tag].append(elem.text)
                        
                        # Simplificar (pegar primeiro valor de cada tag)
                        simplified = {k: v[0] if len(v) == 1 else v for k, v in info.items()}
                        results[name] = simplified
                        
                        # Mostrar algumas informações principais
                        if 'username' in simplified:
                            print(f"      Username: {simplified['username']}")
                        if 'roles' in simplified:
                            print(f"      Roles: {simplified['roles']}")
                        if 'capabilities' in simplified:
                            caps = simplified['capabilities']
                            if isinstance(caps, list):
                                print(f"      Capabilities: {len(caps)} encontradas")
                            else:
                                print(f"      Capabilities: {caps}")
                        
                    except ET.ParseError:
                        # Tentar como JSON
                        try:
                            data = response.json()
                            results[name] = data
                            print(f"      Resposta JSON recebida")
                        except:
                            results[name] = {"raw": response.text[:200]}
                            print(f"      Resposta recebida (formato desconhecido)")
                    
                elif response.status_code == 403:
                    print(f"      [AVISO] Status 403 - Acesso negado (sem permissao)")
                    results[name] = {"error": "403 Forbidden"}
                elif response.status_code == 404:
                    print(f"      [AVISO] Status 404 - Endpoint nao encontrado")
                    results[name] = {"error": "404 Not Found"}
                else:
                    print(f"      [AVISO] Status {response.status_code}")
                    results[name] = {"error": f"HTTP {response.status_code}"}
                
                print()
                
            except requests.exceptions.ConnectionError as e:
                print(f"      [ERRO] Erro de conexao: {e}")
                results[name] = {"error": str(e)}
                print()
            except Exception as e:
                print(f"      [ERRO] Erro inesperado: {e}")
                results[name] = {"error": str(e)}
                print()
        
        print("=" * 60)
        print("Resumo das Permissoes")
        print("=" * 60)
        print()
        
        accessible = [name for name, data in results.items() if "error" not in data or data.get("error") != "403 Forbidden"]
        forbidden = [name for name, data in results.items() if data.get("error") == "403 Forbidden"]
        not_found = [name for name, data in results.items() if data.get("error") == "404 Not Found"]
        
        if accessible:
            print("Endpoints acessiveis:")
            for name in accessible:
                print(f"  [OK] {name}")
            print()
        
        if forbidden:
            print("Endpoints sem permissao (403):")
            for name in forbidden:
                print(f"  [X] {name}")
            print()
        
        if not_found:
            print("Endpoints nao encontrados (404):")
            for name in not_found:
                print(f"  [?] {name}")
            print()
        
        # Tentar usar método do adapter se disponível
        print("[4] Usando metodo do adapter para obter informacoes...")
        try:
            user_info = adapter.get_current_user_info()
            if user_info:
                print("    [OK] Informacoes obtidas via adapter:")
                for key, value in user_info.items():
                    if isinstance(value, list):
                        print(f"      {key}: {', '.join(map(str, value[:5]))}")
                    else:
                        print(f"      {key}: {value}")
        except Exception as e:
            print(f"    [AVISO] Metodo do adapter nao funcionou: {e}")
        
        print()
        print("=" * 60)
        print("Conclusao")
        print("=" * 60)
        print()
        
        if accessible:
            print("[SUCESSO] Voce tem acesso a alguns endpoints da API REST do Splunk!")
            print("Isso indica que:")
            print("  - Autenticacao esta funcionando")
            print("  - Voce tem pelo menos algumas permissoes")
            print()
            print("Se ainda nao consegue executar queries, pode ser:")
            print("  - Permissao especifica para executar searches")
            print("  - Permissao para acessar os indices necessarios")
        else:
            print("[AVISO] Nenhum endpoint acessivel encontrado.")
            print("Isso pode indicar:")
            print("  - Permissoes muito restritas")
            print("  - Conta free trial (sem acesso REST API)")
            print("  - Porta 8089 nao aberta")
        
        return len(accessible) > 0
        
    except Exception as e:
        print(f"[ERRO] {type(e).__name__}: {e}")
        import traceback
        traceback.print_exc()
        return False


if __name__ == "__main__":
    success = check_permissions()
    exit(0 if success else 1)

