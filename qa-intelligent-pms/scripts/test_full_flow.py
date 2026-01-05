#!/usr/bin/env python3
"""Testa fluxo completo: processar arquivo -> criar snapshot -> gerar HTML"""
import sys
from pathlib import Path
from datetime import timedelta

project_root = Path(__file__).parent.parent
sys.path.insert(0, str(project_root))

from src.infrastructure.adapters.splunk_file_adapter import SplunkFileAdapter
from src.application.reativo.reactive_service import ReactiveService
from scripts.generate_html_report import generate_html

def test_full_flow():
    """Testa fluxo completo"""
    # Encontrar arquivo mais recente
    adapter = SplunkFileAdapter()
    files = list(adapter.export_dir.glob('*.json'))
    if not files:
        print("[ERRO] Nenhum arquivo JSON encontrado")
        return False
    
    latest_file = max(files, key=lambda f: f.stat().st_mtime)
    print(f"[OK] Processando: {latest_file.name}")
    
    # Processar via ReactiveService (como no web_app)
    service = ReactiveService()
    analysis = service.analyze_production_logs(
        time_window=timedelta(hours=6),
        file_path=str(latest_file)
    )
    
    print(f"[OK] Análise criada")
    print(f"  Snapshot ID: {analysis.metrics_snapshot.snapshot_id}")
    print(f"  Total endpoints: {len(analysis.metrics_snapshot.endpoints)}")
    print(f"  Critical: {len(analysis.metrics_snapshot.critical)}")
    print(f"  Most used: {len(analysis.metrics_snapshot.most_used)}")
    print(f"  Total requests: {analysis.metrics_snapshot.get_total_requests():,}")
    print(f"  Total errors: {analysis.metrics_snapshot.get_total_errors():,}")
    
    # Gerar HTML
    reports_dir = project_root / 'reports'
    reports_dir.mkdir(exist_ok=True)
    html_file = reports_dir / f"report_{analysis.metrics_snapshot.snapshot_id}.html"
    
    try:
        generate_html(analysis, html_file)
        if html_file.exists():
            size = html_file.stat().st_size
            content = html_file.read_text(encoding='utf-8')
            print(f"\n[OK] HTML gerado: {html_file.name} ({size} bytes)")
            print(f"  Tem 'Endpoints Críticos'? {'Endpoints Críticos' in content}")
            print(f"  Tem 'Top Endpoints'? {'Top Endpoints' in content}")
            print(f"  Contagem de '<tr>' (linhas de tabela): {content.count('<tr>')}")
            return True
        else:
            print(f"[ERRO] Arquivo não foi criado")
            return False
    except Exception as e:
        print(f"[ERRO] Erro: {e}")
        import traceback
        traceback.print_exc()
        return False

if __name__ == "__main__":
    success = test_full_flow()
    sys.exit(0 if success else 1)
