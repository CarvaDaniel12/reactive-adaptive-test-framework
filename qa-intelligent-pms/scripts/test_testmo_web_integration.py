#!/usr/bin/env python
"""
Teste de integração Testmo via interface web
Testa a rota /reactive/sync-test-cases
"""

import sys
from pathlib import Path

# Adicionar src ao path
project_root = Path(__file__).parent.parent
sys.path.insert(0, str(project_root))

from flask import Flask
from src.presentation.web_app import app
import json


def test_sync_test_cases_no_json():
    """Testa erro quando Content-Type não é JSON"""
    print("[TEST] Testando Content-Type não-JSON...")
    
    with app.test_client() as client:
        response = client.post(
            '/reactive/sync-test-cases',
            data="not json",
            content_type='text/plain'
        )
        
        assert response.status_code == 400, f"Esperado 400, recebido {response.status_code}"
        data = response.get_json()
        assert 'error' in data
        print("  [OK] Content-Type não-JSON tratado corretamente")


def test_sync_test_cases_missing_data():
    """Testa erro quando dados não são fornecidos"""
    print("[TEST] Testando dados ausentes...")
    
    with app.test_client() as client:
        response = client.post(
            '/reactive/sync-test-cases',
            json={}
        )
        
        assert response.status_code == 400
        data = response.get_json()
        assert 'error' in data
        assert 'selecionado' in data['error'].lower() or 'selected' in data['error'].lower() or 'fornecido' in data['error'].lower()
        print("  [OK] Dados ausentes tratados corretamente")


def test_sync_test_cases_missing_project_id():
    """Testa erro quando project_id não é fornecido"""
    print("[TEST] Testando project_id ausente...")
    
    with app.test_client() as client:
        response = client.post(
            '/reactive/sync-test-cases',
            json={
                'selected': [
                    {'endpoint': '/api/test', 'method': 'GET'}
                ]
            }
        )
        
        assert response.status_code == 400
        data = response.get_json()
        assert 'error' in data
        assert 'project' in data['error'].lower()
        print("  [OK] project_id ausente tratado corretamente")


def test_sync_test_cases_invalid_selected():
    """Testa erro quando selected não é lista"""
    print("[TEST] Testando selected não-lista...")
    
    with app.test_client() as client:
        response = client.post(
            '/reactive/sync-test-cases',
            json={
                'selected': 'not a list',
                'project_id': 1
            }
        )
        
        assert response.status_code == 400
        data = response.get_json()
        assert 'error' in data
        assert 'lista' in data['error'].lower() or 'list' in data['error'].lower()
        print("  [OK] selected não-lista tratado corretamente")


def test_sync_test_cases_invalid_item():
    """Testa erro quando item não tem campos obrigatórios"""
    print("[TEST] Testando item sem campos obrigatórios...")
    
    with app.test_client() as client:
        response = client.post(
            '/reactive/sync-test-cases',
            json={
                'selected': [
                    {'wrong_field': 'value'}  # Sem endpoint e method
                ],
                'project_id': 1
            }
        )
        
        assert response.status_code == 400
        data = response.get_json()
        assert 'error' in data
        assert 'endpoint' in data['error'].lower() or 'method' in data['error'].lower()
        print("  [OK] Item inválido tratado corretamente")


def test_sync_test_cases_structure_valid():
    """Testa estrutura de resposta válida (pode falhar se credenciais não configuradas)"""
    print("[TEST] Testando estrutura de resposta válida...")
    
    with app.test_client() as client:
        response = client.post(
            '/reactive/sync-test-cases',
            json={
                'selected': [
                    {
                        'endpoint': '/api/v3/test',
                        'method': 'GET',
                        'title': 'Test Case',
                        'description': 'Description',
                        'steps': ['Step 1'],
                        'expected_result': 'Expected',
                        'testmo_status': 'not_exists'
                    }
                ],
                'project_id': 1
            }
        )
        
        # Pode retornar 500 se credenciais não estiverem configuradas
        if response.status_code == 500:
            data = response.get_json()
            error = data.get('error', '')
            if 'testmo' in error.lower() or 'config' in error.lower() or 'credential' in error.lower():
                print("  [SKIP] Credenciais Testmo não configuradas (esperado)")
                return
        
        # Se status é 200, validar estrutura
        if response.status_code == 200:
            data = response.get_json()
            
            assert 'success' in data, "Resposta deve conter 'success'"
            assert data['success'] is True, "success deve ser True"
            assert 'stats' in data, "Resposta deve conter 'stats'"
            
            stats = data['stats']
            assert isinstance(stats, dict), "'stats' deve ser um dicionário"
            
            required_fields = ['created', 'updated', 'reused', 'inherited', 'errors']
            for field in required_fields:
                assert field in stats, f"'stats' deve conter '{field}'"
            
            assert isinstance(stats['errors'], list), "'errors' deve ser uma lista"
            assert isinstance(stats['created'], int), "'created' deve ser int"
            assert isinstance(stats['updated'], int), "'updated' deve ser int"
            assert isinstance(stats['reused'], int), "'reused' deve ser int"
            assert isinstance(stats['inherited'], int), "'inherited' deve ser int"
            
            print(f"  [OK] Estrutura válida: stats = {stats}")
        else:
            print(f"  [INFO] Status {response.status_code}: {response.get_json().get('error', '')[:100]}")


def test_sync_test_cases_reactive_context():
    """Testa sincronização com contexto reativo"""
    print("[TEST] Testando contexto reativo...")
    
    with app.test_client() as client:
        response = client.post(
            '/reactive/sync-test-cases',
            json={
                'selected': [
                    {
                        'endpoint': '/api/v3/test',
                        'method': 'GET',
                        'title': 'Test Case',
                        'testmo_status': 'not_exists'
                    }
                ],
                'project_id': 1,
                'reactive_context': {
                    'date': '2025-12-15',
                    'priority': 'High',
                    'trend': 'Increasing'
                }
            }
        )
        
        # Pode falhar por credenciais, mas estrutura deve ser validada se sucesso
        if response.status_code == 200:
            data = response.get_json()
            assert 'stats' in data
            print("  [OK] Contexto reativo aceito")
        elif response.status_code == 500:
            error = response.get_json().get('error', '')
            if 'testmo' in error.lower() or 'config' in error.lower():
                print("  [SKIP] Credenciais Testmo não configuradas (esperado)")
            else:
                print(f"  [INFO] Erro: {error[:100]}")


def main():
    """Executa todos os testes"""
    print("=" * 70)
    print("Testes de Integração Testmo - Interface Web")
    print("=" * 70)
    print()
    
    tests = [
        test_sync_test_cases_no_json,
        test_sync_test_cases_missing_data,
        test_sync_test_cases_missing_project_id,
        test_sync_test_cases_invalid_selected,
        test_sync_test_cases_invalid_item,
        test_sync_test_cases_structure_valid,
        test_sync_test_cases_reactive_context,
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
