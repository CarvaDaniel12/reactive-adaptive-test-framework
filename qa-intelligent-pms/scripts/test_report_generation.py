#!/usr/bin/env python3
"""Testa geração de relatório HTML"""
import sys
from pathlib import Path

project_root = Path(__file__).parent.parent
sys.path.insert(0, str(project_root))

from src.infrastructure.repositories.file_metrics_repository import FileMetricsRepository
from src.application.reativo.reactive_service import LogAnalysis
from datetime import timedelta
from scripts.generate_html_report import generate_html

def test_report_generation():
    """Testa se consegue gerar relatório do snapshot mais recente"""
    repo = FileMetricsRepository()
    latest = repo.get_latest_snapshot()
    
    if not latest:
        print("[ERRO] Nenhum snapshot encontrado")
        return False
    
    print(f"[OK] Snapshot encontrado: {latest.snapshot_id}")
    
    # Criar análise
    analysis = LogAnalysis(timedelta(hours=6))
    analysis.metrics_snapshot = latest
    analysis.total_errors = latest.get_total_errors()
    analysis.error_rate = latest.get_overall_error_rate() / 100.0
    analysis.prioritized_recommendations = []
    analysis.trends = []
    analysis.coverage_gaps = []
    analysis.regression_risks = []
    
    # Gerar HTML
    reports_dir = project_root / 'reports'
    reports_dir.mkdir(exist_ok=True)
    html_file = reports_dir / f'report_{latest.snapshot_id}.html'
    
    try:
        generate_html(analysis, html_file)
        if html_file.exists():
            print(f"[OK] HTML gerado: {html_file}")
            print(f"[OK] Tamanho: {html_file.stat().st_size} bytes")
            return True
        else:
            print(f"[ERRO] Arquivo não foi criado")
            return False
    except Exception as e:
        print(f"[ERRO] Erro ao gerar: {e}")
        import traceback
        traceback.print_exc()
        return False

if __name__ == "__main__":
    success = test_report_generation()
    sys.exit(0 if success else 1)
