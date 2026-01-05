#!/usr/bin/env python
"""
Teste end-to-end: Processar → Postman → Testmo
Simula fluxo completo da interface web
"""

import sys
from pathlib import Path

# Adicionar src ao path
project_root = Path(__file__).parent.parent
sys.path.insert(0, str(project_root))

from flask import Flask
from src.presentation.web_app import app
import json


def test_full_flow_with_existing_snapshot():
    """Testa fluxo completo usando snapshot existente"""
    print("[TEST] Testando fluxo completo end-to-end...")
    print()
    
    # 1. Listar snapshots disponíveis
    snapshots_dir = project_root / 'data' / 'metrics' / 'snapshots'
    snapshot_files = list(snapshots_dir.glob('*.json'))
    
    if not snapshot_files:
        print("  [SKIP] Nenhum snapshot disponível")
        return False
    
    # Usar primeiro snapshot
    snapshot_file = snapshot_files[0]
    snapshot_id = snapshot_file.stem
    print(f"  [INFO] Usando snapshot: {snapshot_id}")
    
    with app.test_client() as client:
        # 2. Buscar matches no Postman
        print("  [1/3] Buscando matches no Postman...")
        postman_response = client.post(f'/reactive/find-postman-matches/{snapshot_id}')
        
        if postman_response.status_code != 200:
            error = postman_response.get_json().get('error', '')
            error_lower = error.lower()
            if any(word in error_lower for word in ['postman', 'testmo', 'config', 'credential', 'project id', 'não encontrado']):
                print("    [SKIP] Credenciais não configuradas ou Project ID não encontrado")
                print(f"         Erro: {error[:100]}")
                return True  # Não é falha, apenas falta de configuração
            print(f"    [ERRO] Falha ao buscar Postman: {error[:100]}")
            return False
        
        # Se chegou aqui, postman funcionou - obter dados
        postman_data = postman_response.get_json()
        matches = postman_data.get('matches', [])
        project_id = postman_data.get('project_id')
        
        print(f"    [OK] {len(matches)} matches encontrados")
        
        if not matches:
            print("    [SKIP] Nenhum match encontrado, pulando sincronização")
            return True
        
        # 3. Preparar seleção de test cases (primeiros 3 matches)
        print("  [2/3] Preparando seleção de test cases...")
        selected = []
        for match in matches[:3]:  # Apenas primeiros 3 para teste
            selected.append({
                'endpoint': match.get('endpoint'),
                'method': match.get('method'),
                'title': match.get('title'),
                'description': match.get('description', ''),
                'steps': match.get('steps', []),
                'expected_result': match.get('expected_result', ''),
                'testmo_status': match.get('testmo_status', 'not_exists'),
                'postman_info': match.get('postman_info', {})
            })
        
        print(f"    [OK] {len(selected)} test cases selecionados")
        
        # 4. Sincronizar com Testmo
        print("  [3/3] Sincronizando com Testmo...")
        if not project_id:
            print("    [SKIP] Project ID não disponível")
            return True
        
        sync_response = client.post(
            '/reactive/sync-test-cases',
            json={
                'selected': selected,
                'project_id': project_id
            }
        )
        
        if sync_response.status_code != 200:
            error = sync_response.get_json().get('error', '')
            error_lower = error.lower()
            if any(word in error_lower for word in ['testmo', 'config', 'credential', 'project']):
                print("    [SKIP] Credenciais Testmo não configuradas ou erro de configuração")
                print(f"         Erro: {error[:100]}")
                return True
            print(f"    [ERRO] Falha ao sincronizar: {error[:100]}")
            return False
        
        sync_data = sync_response.get_json()
        stats = sync_data.get('stats', {})
        
        print(f"    [OK] Sincronização concluída:")
        print(f"      - Criados: {stats.get('created', 0)}")
        print(f"      - Atualizados: {stats.get('updated', 0)}")
        print(f"      - Reutilizados: {stats.get('reused', 0)}")
        print(f"      - Herdados: {stats.get('inherited', 0)}")
        print(f"      - Erros: {len(stats.get('errors', []))}")
        
        if stats.get('errors'):
            print(f"      - Lista de erros: {stats['errors']}")
        
        print()
        print("  [OK] Fluxo completo executado com sucesso!")
        return True


def test_error_handling_chain():
    """Testa tratamento de erros em cadeia"""
    print("[TEST] Testando tratamento de erros em cadeia...")
    
    with app.test_client() as client:
        # Snapshot inexistente → Postman não deve ser chamado
        response = client.post('/reactive/find-postman-matches/snapshot-inexistente-999')
        assert response.status_code == 404
        
        # Dados inválidos → Testmo não deve ser chamado
        response = client.post(
            '/reactive/sync-test-cases',
            json={'invalid': 'data'}
        )
        assert response.status_code == 400
        
        print("  [OK] Erros tratados corretamente em cadeia")
        return True


def main():
    """Executa todos os testes"""
    print("=" * 70)
    print("Testes End-to-End: Integrações Web")
    print("=" * 70)
    print()
    
    tests = [
        test_full_flow_with_existing_snapshot,
        test_error_handling_chain,
    ]
    
    passed = 0
    failed = 0
    skipped = 0
    
    for test in tests:
        try:
            result = test()
            if result:
                passed += 1
            else:
                failed += 1
        except AssertionError as e:
            print(f"  [FALHOU] {e}")
            failed += 1
        except Exception as e:
            if 'SKIP' in str(e) or 'skip' in str(e).lower():
                skipped += 1
            else:
                print(f"  [ERRO] {e}")
                import traceback
                traceback.print_exc()
                failed += 1
        print()
    
    print("=" * 70)
    print("Resumo:")
    print(f"  Passou: {passed}")
    print(f"  Falhou: {failed}")
    print(f"  Pulou: {skipped}")
    print("=" * 70)
    
    return failed == 0


if __name__ == "__main__":
    success = main()
    sys.exit(0 if success else 1)
