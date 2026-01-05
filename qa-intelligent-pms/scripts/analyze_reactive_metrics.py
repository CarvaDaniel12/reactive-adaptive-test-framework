#!/usr/bin/env python3
"""
Script: analyze_reactive_metrics.py
Análise completa de métricas reativas com histórico e recomendações.
"""

import argparse
import sys
from pathlib import Path
from datetime import timedelta, datetime

# Adicionar raiz do projeto ao path
project_root = Path(__file__).parent.parent
sys.path.insert(0, str(project_root))

from src.application.reativo.reactive_service import ReactiveService
from src.infrastructure.repositories.file_metrics_repository import FileMetricsRepository


def print_analysis_report(analysis):
    """Imprime relatório completo da análise"""
    print("\n" + "=" * 70)
    print("ANALISE REATIVA DE METRICAS - RELATORIO COMPLETO")
    print("=" * 70)
    
    # Métricas gerais
    print("\n[METRICAS GERAIS]")
    print(f"  Periodo analisado: {analysis.time_window}")
    print(f"  Total de requisicoes: {analysis.total_errors + (analysis.metrics_snapshot.get_total_requests() - analysis.total_errors) if analysis.metrics_snapshot else 'N/A'}")
    print(f"  Total de erros: {analysis.total_errors}")
    print(f"  Taxa de erro geral: {analysis.error_rate * 100:.2f}%")
    if analysis.metrics_snapshot:
        print(f"  Endpoints unicos: {analysis.metrics_snapshot.get_unique_endpoints()}")
        print(f"  Endpoints criticos: {analysis.metrics_snapshot.get_critical_count()}")
    
    # Tendências
    if analysis.trends:
        print("\n[TENDENCIAS]")
        degrading = [t for t in analysis.trends if t.is_degrading]
        improving = [t for t in analysis.trends if t.is_improving]
        
        if degrading:
            print(f"  Endpoints degradando: {len(degrading)}")
            print("  Top 5 mais degradados:")
            for trend in sorted(degrading, key=lambda t: t.change_percentage, reverse=True)[:5]:
                print(f"    - {trend.endpoint}: {trend.previous_error_rate:.2f}% -> {trend.current_error_rate:.2f}% "
                      f"(+{trend.change_percentage:.1f}%)")
        
        if improving:
            print(f"  Endpoints melhorando: {len(improving)}")
            print("  Top 5 mais melhorados:")
            for trend in sorted(improving, key=lambda t: t.change_percentage)[:5]:
                print(f"    - {trend.endpoint}: {trend.previous_error_rate:.2f}% -> {trend.current_error_rate:.2f}% "
                      f"({trend.change_percentage:.1f}%)")
    else:
        print("\n[TENDENCIAS]")
        print("  Nenhum snapshot anterior encontrado para comparacao")
    
    # Endpoints prioritários
    if analysis.prioritized_recommendations:
        print("\n[ENDPOINTS PRIORITARIOS PARA TESTE]")
        for i, rec in enumerate(analysis.prioritized_recommendations[:10], 1):
            priority = rec.get('priority', 'medium').upper()
            score = rec.get('priority_score', 0)
            print(f"  {i}. {rec.get('endpoint', 'N/A')} [{priority}] (Score: {score:.1f})")
            print(f"     {rec.get('description', '')}")
    
    # Riscos de regressão
    if analysis.regression_risks:
        print("\n[RISCOS DE REGRESSAO]")
        for risk in analysis.regression_risks:
            print(f"  [{risk.get('priority', 'medium').upper()}] {risk.get('endpoint', 'N/A')}")
            print(f"     {risk.get('description', '')}")
            print(f"     Acao sugerida: {risk.get('suggested_action', '')}")
    
    # Gaps de cobertura
    if analysis.coverage_gaps:
        print("\n[GAPS DE COBERTURA DE TESTES]")
        for gap in analysis.coverage_gaps[:10]:
            print(f"  [{gap.get('priority', 'medium').upper()}] {gap.get('endpoint', 'N/A')}")
            print(f"     {gap.get('description', '')}")
            print(f"     {gap.get('suggested_action', '')}")
    
    # Alertas
    if analysis.alerts:
        print("\n[ALERTAS]")
        for alert in analysis.alerts[:10]:
            severity = alert.get('severity', 'medium').upper()
            print(f"  [{severity}] {alert.get('title', alert.get('message', 'N/A'))}")
    
    # Recomendações acionáveis
    if analysis.prioritized_recommendations:
        print("\n[RECOMENDACOES ACIONAVEIS]")
        print("  Priorizar testes nos seguintes endpoints:")
        for rec in analysis.prioritized_recommendations[:5]:
            endpoint = rec.get('endpoint', 'N/A')
            tests = rec.get('suggested_tests', [])
            print(f"  - {endpoint}:")
            for test in tests[:3]:  # Top 3 testes sugeridos
                print(f"      * {test}")
    
    print("\n" + "=" * 70)
    print("[SUCESSO] Analise completa gerada!")
    print("=" * 70 + "\n")


def main():
    """Função principal"""
    parser = argparse.ArgumentParser(
        description="Analisa metricas reativas do Splunk com historico e recomendacoes"
    )
    parser.add_argument(
        'file_path',
        nargs='?',
        help="Caminho para arquivo CSV exportado do Splunk. Se omitido, procura o mais recente em data/splunk_exports/"
    )
    parser.add_argument(
        '--time-window',
        type=int,
        default=6,
        help="Janela de tempo em horas (padrao: 6)"
    )
    parser.add_argument(
        '--save-snapshot',
        action='store_true',
        help="Salvar snapshot de metricas (padrao: sempre salva)"
    )
    
    args = parser.parse_args()
    
    # Determinar arquivo a processar
    if args.file_path:
        input_path = Path(args.file_path)
        if not input_path.exists():
            print(f"[ERRO] Arquivo nao encontrado: {input_path}")
            sys.exit(1)
        file_path = str(input_path)
    else:
        # Procurar arquivo mais recente
        exports_dir = project_root / "data" / "splunk_exports"
        csv_files = list(exports_dir.glob("*.csv"))
        if not csv_files:
            print(f"[ERRO] Nenhum arquivo CSV encontrado em {exports_dir}")
            print("       Exporte um arquivo do Splunk primeiro.")
            sys.exit(1)
        
        latest_file = max(csv_files, key=lambda f: f.stat().st_mtime)
        file_path = str(latest_file)
        print(f"[INFO] Usando arquivo mais recente: {latest_file.name}")
    
    # Criar serviço
    time_window = timedelta(hours=args.time_window)
    service = ReactiveService()
    
    print(f"[INFO] Processando metricas do arquivo: {file_path}")
    print(f"[INFO] Janela de tempo: {time_window}")
    
    try:
        # Executar análise
        analysis = service.analyze_production_logs(
            time_window=time_window,
            file_path=file_path
        )
        
        # Imprimir relatório
        print_analysis_report(analysis)
        
        # Informações adicionais
        if analysis.metrics_snapshot:
            snapshot_id = analysis.metrics_snapshot.snapshot_id
            print(f"[INFO] Snapshot salvo: {snapshot_id}")
            print(f"[INFO] Use este snapshot para comparacao futura")
            print(f"[INFO] Para gerar relatorio HTML: python scripts/generate_html_report.py {snapshot_id}")
        
        return 0
        
    except Exception as e:
        print(f"[ERRO] Falha ao processar metricas: {e}")
        import traceback
        traceback.print_exc()
        return 1


if __name__ == "__main__":
    sys.exit(main())

