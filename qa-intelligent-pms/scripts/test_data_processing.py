#!/usr/bin/env python3
"""Testa processamento de dados do Splunk"""
import sys
from pathlib import Path

project_root = Path(__file__).parent.parent
sys.path.insert(0, str(project_root))

from src.infrastructure.adapters.splunk_file_adapter import SplunkFileAdapter

def test_data_processing():
    """Testa se os dados estão sendo processados corretamente"""
    adapter = SplunkFileAdapter()
    
    # Encontrar arquivo mais recente
    files = list(Path('data/splunk_exports').glob('*.json'))
    if not files:
        print("[ERRO] Nenhum arquivo JSON encontrado")
        return False
    
    latest = max(files, key=lambda f: f.stat().st_mtime)
    print(f"[OK] Arquivo mais recente: {latest.name}")
    print(f"[OK] Tamanho: {latest.stat().st_size / (1024*1024):.2f} MB")
    
    # Processar
    try:
        metrics = adapter.get_critical_metrics(str(latest))
        
        print(f"\n[RESULTADOS]")
        print(f"  all_endpoints: {len(metrics.get('all_endpoints', []))}")
        print(f"  most_used: {len(metrics.get('most_used_endpoints', []))}")
        print(f"  most_failed: {len(metrics.get('most_failed_endpoints', []))}")
        print(f"  critical: {len(metrics.get('critical_endpoints', []))}")
        
        overall = metrics.get('overall_metrics', {})
        print(f"\n[METRICAS GERAIS]")
        print(f"  total_requests: {overall.get('total_requests', 0):,}")
        print(f"  total_errors: {overall.get('total_errors', 0):,}")
        print(f"  error_rate: {overall.get('overall_error_rate', 0)}%")
        print(f"  unique_endpoints: {overall.get('unique_endpoints', 0)}")
        
        # Mostrar primeiros endpoints
        if metrics.get('all_endpoints'):
            print(f"\n[PRIMEIROS 3 ENDPOINTS]")
            for i, ep in enumerate(metrics['all_endpoints'][:3], 1):
                print(f"  {i}. {ep.get('endpoint', 'N/A')[:60]}")
                print(f"     Requests: {ep.get('total_requests', 0):,}, Erros: {ep.get('total_errors', 0):,}")
        
        if len(metrics.get('all_endpoints', [])) == 0:
            print("\n[ERRO] Nenhum endpoint encontrado nos dados!")
            print("[DEBUG] Verificando dados brutos...")
            data = adapter._load_data(str(latest))
            print(f"[DEBUG] Linhas carregadas: {len(data)}")
            if data:
                print(f"[DEBUG] Primeira linha: {list(data[0].keys())}")
                print(f"[DEBUG] Tem 'endpoint'? {'endpoint' in data[0]}")
        
        return len(metrics.get('all_endpoints', [])) > 0
        
    except Exception as e:
        print(f"[ERRO] Erro ao processar: {e}")
        import traceback
        traceback.print_exc()
        return False

if __name__ == "__main__":
    success = test_data_processing()
    sys.exit(0 if success else 1)
