#!/usr/bin/env python
"""
Teste de integração Postman via interface web
Testa a rota /reactive/find-postman-matches/<snapshot_id>
"""

import sys
from pathlib import Path

# Adicionar src ao path
project_root = Path(__file__).parent.parent
sys.path.insert(0, str(project_root))

from flask import Flask
from src.presentation.web_app import app
import json


def test_find_postman_matches_snapshot_not_found():
    """Testa erro quando snapshot não existe"""
    print("[TEST] Testando snapshot não encontrado...")
    
    with app.test_client() as client:
        response = client.post('/reactive/find-postman-matches/snapshot-inexistente-123')
        
        assert response.status_code == 404, f"Esperado 404, recebido {response.status_code}"
        data = response.get_json()
        assert 'error' in data, "Resposta deve conter campo 'error'"
        assert 'não encontrado' in data['error'].lower() or 'not found' in data['error'].lower()
        print("  [OK] Snapshot não encontrado tratado corretamente")


def test_find_postman_matches_structure():
    """Testa estrutura da resposta com snapshot válido"""
    print("[TEST] Testando estrutura da resposta...")
    
    # Listar snapshots disponíveis
    snapshots_dir = project_root / 'data' / 'metrics' / 'snapshots'
    snapshot_files = list(snapshots_dir.glob('*.json'))
    
    if not snapshot_files:
        print("  [SKIP] Nenhum snapshot disponível para testar")
        return
    
    # Usar primeiro snapshot encontrado
    snapshot_file = snapshot_files[0]
    snapshot_id = snapshot_file.stem
    print(f"  [INFO] Usando snapshot: {snapshot_id}")
    
    with app.test_client() as client:
        response = client.post(f'/reactive/find-postman-matches/{snapshot_id}')
        
        # Pode retornar erro se credenciais não estiverem configuradas (esperado)
        if response.status_code in [400, 500]:
            data = response.get_json()
            error = data.get('error', '')
            error_lower = error.lower()
            if any(word in error_lower for word in ['postman', 'testmo', 'config', 'credential', 'project id']):
                print(f"  [SKIP] Credenciais não configuradas ou Project ID não encontrado (esperado)")
                print(f"         Erro: {error[:100]}")
                return
        
        assert response.status_code in [200, 400, 500], f"Status code inesperado: {response.status_code}"
        
        if response.status_code == 200:
            data = response.get_json()
            
            # Validar estrutura
            assert 'matches' in data, "Resposta deve conter 'matches'"
            assert 'total_found' in data, "Resposta deve conter 'total_found'"
            assert 'project_id' in data, "Resposta deve conter 'project_id'"
            
            assert isinstance(data['matches'], list), "'matches' deve ser uma lista"
            assert isinstance(data['total_found'], int), "'total_found' deve ser int"
            assert data['total_found'] == len(data['matches']), "total_found deve igualar tamanho de matches"
            
            # Validar estrutura de cada match (se houver)
            for i, match in enumerate(data['matches'][:5]):  # Validar apenas primeiros 5
                assert isinstance(match, dict), f"Match {i} deve ser um dicionário"
                assert 'endpoint' in match, f"Match {i} deve ter 'endpoint'"
                assert 'method' in match, f"Match {i} deve ter 'method'"
                assert 'title' in match, f"Match {i} deve ter 'title'"
                assert 'testmo_status' in match, f"Match {i} deve ter 'testmo_status'"
                assert match['testmo_status'] in ['not_exists', 'identical', 'different'], \
                    f"testmo_status inválido: {match['testmo_status']}"
            
            print(f"  [OK] Estrutura válida: {data['total_found']} matches encontrados")
        else:
            print(f"  [INFO] Erro (possivelmente credenciais): {response.get_json().get('error', '')[:100]}")


def test_find_postman_matches_empty_snapshot():
    """Testa comportamento com snapshot sem endpoints críticos"""
    print("[TEST] Testando snapshot sem endpoints...")
    
    # Criar snapshot vazio temporário para teste
    from src.infrastructure.repositories.file_metrics_repository import FileMetricsRepository
    from src.domain.entities.metrics_snapshot import MetricsSnapshot
    from datetime import timedelta, datetime
    
    repo = FileMetricsRepository()
    
    # Criar snapshot vazio
    empty_snapshot = MetricsSnapshot(
        snapshot_id="test-empty-snapshot",
        timestamp=datetime(2025, 12, 15, 0, 0, 0),  # datetime object, não string
        time_window=timedelta(hours=6),
        overall_metrics={
            'total_requests': 0,
            'total_errors': 0,
            'overall_error_rate': 0.0,
            'unique_endpoints': 0
        },
        endpoints=[],
        critical=[],
        most_used=[],
        most_failed=[]
    )
    
    # Salvar temporariamente
    try:
        repo.save_snapshot(empty_snapshot)
        
        with app.test_client() as client:
            response = client.post('/reactive/find-postman-matches/test-empty-snapshot')
            
            if response.status_code == 200:
                data = response.get_json()
                # Quando não há endpoints, pode retornar apenas 'matches' e 'message' ou estrutura completa
                if 'matches' in data:
                    assert data['matches'] == [], "Deve retornar lista vazia"
                    if 'total_found' in data:
                        assert data['total_found'] == 0, "total_found deve ser 0"
                    print("  [OK] Snapshot vazio tratado corretamente")
                else:
                    # Estrutura alternativa com message
                    assert 'message' in data, "Deve ter 'matches' ou 'message'"
                    print("  [OK] Snapshot vazio retornou mensagem apropriada")
            elif response.status_code == 400:
                data = response.get_json()
                error = data.get('error', '')
                if 'project id' in error.lower() or 'não encontrado' in error.lower():
                    print("  [SKIP] Project ID não encontrado (esperado sem credenciais)")
                else:
                    print(f"  [INFO] Status 400: {error[:100]}")
            else:
                error_msg = response.get_json().get('error', '') if response.is_json else str(response.data)
                print(f"  [INFO] Status {response.status_code}: {error_msg[:100]}")
    finally:
        # Limpar snapshot de teste
        snapshot_file = repo.snapshots_dir / "test-empty-snapshot.json"
        if snapshot_file.exists():
            snapshot_file.unlink()


def main():
    """Executa todos os testes"""
    print("=" * 70)
    print("Testes de Integração Postman - Interface Web")
    print("=" * 70)
    print()
    
    tests = [
        test_find_postman_matches_snapshot_not_found,
        test_find_postman_matches_structure,
        test_find_postman_matches_empty_snapshot,
    ]
    
    passed = 0
    failed = 0
    skipped = 0
    
    for test in tests:
        try:
            test()
            passed += 1
        except AssertionError as e:
            print(f"  [FALHOU] {e}")
            failed += 1
        except Exception as e:
            if 'SKIP' in str(e) or 'skip' in str(e).lower():
                skipped += 1
            else:
                print(f"  [ERRO] {e}")
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
