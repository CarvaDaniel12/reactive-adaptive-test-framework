#!/usr/bin/env python3
"""
Script: generate_html_report.py
Gera relatório HTML a partir de análise de métricas (mais fácil de visualizar e compartilhar).
"""

import sys
import json
from pathlib import Path
from datetime import datetime

# Adicionar raiz do projeto ao path
project_root = Path(__file__).parent.parent
sys.path.insert(0, str(project_root))

from src.application.reativo.reactive_service import ReactiveService
from src.infrastructure.repositories.file_metrics_repository import (
    FileMetricsRepository,
)
from datetime import timedelta


def generate_html(analysis, output_path: Path):
    """Gera relatório HTML a partir da análise"""

    html = f"""<!DOCTYPE html>
<html lang="pt-BR">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Análise Reativa de Métricas - {datetime.now().strftime('%d/%m/%Y %H:%M')}</title>
    <link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Marcellus&family=Josefin+Sans:wght@300;400;600;700&display=swap">
    <style>
        :root {{
            --color-obsidian: #0A0A0A;
            --color-champagne: #F2F0E4;
            --color-gold: #D4AF37;
            --color-gold-light: #F2E8C4;
            --color-charcoal: #141414;
            --color-midnight-blue: #1E3D59;
            --color-pewter: #888888;
            --color-white: #FFFFFF;
            --spacing-xs: 4px;
            --spacing-sm: 8px;
            --spacing-md: 16px;
            --spacing-lg: 24px;
            --spacing-xl: 32px;
            --spacing-2xl: 48px;
            --font-display: 'Marcellus', serif;
            --font-body: 'Josefin Sans', sans-serif;
            --glow-gold-subtle: 0 0 15px rgba(212, 175, 55, 0.2);
            --glow-gold-medium: 0 0 20px rgba(212, 175, 55, 0.3);
        }}
        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}
        body {{
            font-family: var(--font-body);
            background-color: var(--color-obsidian);
            color: var(--color-champagne);
            line-height: 1.6;
            padding: var(--spacing-md);
            background-image: 
                repeating-linear-gradient(
                    45deg,
                    transparent,
                    transparent 10px,
                    rgba(212, 175, 55, 0.03) 10px,
                    rgba(212, 175, 55, 0.03) 20px
                ),
                repeating-linear-gradient(
                    -45deg,
                    transparent,
                    transparent 10px,
                    rgba(212, 175, 55, 0.03) 10px,
                    rgba(212, 175, 55, 0.03) 20px
                );
        }}
        .container {{
            max-width: 1200px;
            margin: 0 auto;
        }}
        .header {{
            background: var(--color-charcoal);
            border: 3px solid var(--color-gold);
            padding: var(--spacing-2xl);
            margin-bottom: var(--spacing-xl);
            box-shadow: var(--glow-gold-medium);
        }}
        .header h1 {{
            font-family: var(--font-display);
            color: var(--color-gold);
            margin: 0 0 var(--spacing-sm) 0;
            font-size: 2rem;
            text-transform: uppercase;
            letter-spacing: 2px;
        }}
        .header p {{
            color: var(--color-pewter);
            font-size: 0.875rem;
        }}
        .section {{
            background: var(--color-charcoal);
            border: 2px solid var(--color-gold);
            padding: var(--spacing-xl);
            margin-bottom: var(--spacing-lg);
            box-shadow: var(--glow-gold-subtle);
        }}
        .section h2 {{
            font-family: var(--font-display);
            color: var(--color-gold);
            border-bottom: 3px solid var(--color-gold);
            padding-bottom: var(--spacing-sm);
            margin: 0 0 var(--spacing-lg) 0;
            font-size: 1.5rem;
            text-transform: uppercase;
            letter-spacing: 1px;
        }}
        .section h3 {{
            font-family: var(--font-display);
            color: var(--color-champagne);
            margin: var(--spacing-lg) 0 var(--spacing-md) 0;
            font-size: 1.25rem;
        }}
        .metric-card {{
            display: inline-block;
            background: var(--color-obsidian);
            border: 2px solid var(--color-gold);
            padding: var(--spacing-md) var(--spacing-lg);
            margin: var(--spacing-sm);
            box-shadow: var(--glow-gold-subtle);
        }}
        .metric-card .label {{
            font-size: 0.75rem;
            color: var(--color-pewter);
            text-transform: uppercase;
            letter-spacing: 1px;
            margin-bottom: var(--spacing-xs);
        }}
        .metric-card .value {{
            font-size: 1.5rem;
            font-weight: 700;
            color: var(--color-gold);
            font-family: var(--font-display);
        }}
        table {{
            width: 100%;
            border-collapse: collapse;
            margin-top: var(--spacing-md);
            border: 2px solid var(--color-gold);
        }}
        th {{
            background: var(--color-charcoal);
            color: var(--color-gold);
            padding: var(--spacing-md);
            text-align: left;
            font-weight: 600;
            text-transform: uppercase;
            font-size: 0.875rem;
            letter-spacing: 1px;
            border-bottom: 2px solid var(--color-gold);
        }}
        td {{
            padding: var(--spacing-md);
            border-bottom: 1px solid rgba(212, 175, 55, 0.2);
            color: var(--color-champagne);
        }}
        tr:hover {{
            background: rgba(212, 175, 55, 0.05);
        }}
        .badge {{
            display: inline-block;
            padding: var(--spacing-xs) var(--spacing-sm);
            border: 1px solid;
            font-size: 0.6875rem;
            font-weight: 600;
            text-transform: uppercase;
            letter-spacing: 0.5px;
        }}
        .badge-critical {{
            background: rgba(220, 53, 69, 0.1);
            color: #dc3545;
            border-color: #dc3545;
        }}
        .badge-high {{
            background: rgba(253, 126, 20, 0.1);
            color: #fd7e14;
            border-color: #fd7e14;
        }}
        .badge-medium {{
            background: rgba(212, 175, 55, 0.1);
            color: var(--color-gold);
            border-color: var(--color-gold);
        }}
        .badge-improving {{
            background: rgba(212, 175, 55, 0.1);
            color: var(--color-gold);
            border-color: var(--color-gold);
        }}
        .badge-degrading {{
            background: rgba(220, 53, 69, 0.1);
            color: #dc3545;
            border-color: #dc3545;
        }}
        .badge-stable {{
            background: rgba(136, 136, 136, 0.1);
            color: var(--color-pewter);
            border-color: var(--color-pewter);
        }}
        .endpoint {{
            font-family: 'Courier New', monospace;
            font-size: 0.8125rem;
            color: var(--color-champagne);
        }}
        .footer {{
            text-align: center;
            color: var(--color-pewter);
            margin-top: var(--spacing-2xl);
            padding-top: var(--spacing-lg);
            border-top: 2px solid var(--color-gold);
            font-size: 0.875rem;
        }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1><span style="color: var(--color-gold);">■</span> Análise Reativa de Métricas</h1>
            <p>Gerado em {datetime.now().strftime('%d/%m/%Y às %H:%M:%S')}</p>
        </div>
    
    <div class="section">
        <h2><span style="color: var(--color-gold);">▲</span> Métricas Gerais</h2>
        <div class="metric-card">
            <div class="label">Total de Requisições</div>
            <div class="value">{(analysis.metrics_snapshot.get_total_requests() if analysis.metrics_snapshot else 0):,}</div>
        </div>
        <div class="metric-card">
            <div class="label">Total de Erros</div>
            <div class="value">{analysis.total_errors:,}</div>
        </div>
        <div class="metric-card">
            <div class="label">Taxa de Erro</div>
            <div class="value">{analysis.error_rate * 100:.2f}%</div>
        </div>
        <div class="metric-card">
            <div class="label">Endpoints Únicos</div>
            <div class="value">{analysis.metrics_snapshot.get_unique_endpoints() if analysis.metrics_snapshot else 0}</div>
        </div>
        <div class="metric-card">
            <div class="label">Endpoints Críticos</div>
            <div class="value">{analysis.metrics_snapshot.get_critical_count() if analysis.metrics_snapshot else 0}</div>
        </div>
    </div>
"""

    # Tendências
    if analysis.trends:
        html += """
    <div class="section">
        <h2><span style="color: var(--color-gold);">▼</span> Tendências</h2>
"""
        degrading = [t for t in analysis.trends if t.is_degrading]
        improving = [t for t in analysis.trends if t.is_improving]

        if degrading:
            html += f"""
        <h3><span style="color: #dc3545;">▼</span> Endpoints Degradando ({len(degrading)})</h3>
        <table>
            <thead>
                <tr>
                    <th>Endpoint</th>
                    <th>Taxa Anterior</th>
                    <th>Taxa Atual</th>
                    <th>Mudança</th>
                    <th>Status</th>
                </tr>
            </thead>
            <tbody>
"""
            for trend in sorted(
                degrading, key=lambda t: t.change_percentage, reverse=True
            )[:10]:
                html += f"""
                <tr>
                    <td class="endpoint">{trend.endpoint[:80]}</td>
                    <td>{trend.previous_error_rate:.2f}%</td>
                    <td>{trend.current_error_rate:.2f}%</td>
                    <td>+{trend.change_percentage:.1f}%</td>
                    <td><span class="badge badge-degrading">Degradando</span></td>
                </tr>
"""
            html += """
            </tbody>
        </table>
"""

        if improving:
            html += f"""
        <h3><span style="color: var(--color-gold);">▲</span> Endpoints Melhorando ({len(improving)})</h3>
        <table>
            <thead>
                <tr>
                    <th>Endpoint</th>
                    <th>Taxa Anterior</th>
                    <th>Taxa Atual</th>
                    <th>Mudança</th>
                    <th>Status</th>
                </tr>
            </thead>
            <tbody>
"""
            for trend in sorted(improving, key=lambda t: t.change_percentage)[:10]:
                html += f"""
                <tr>
                    <td class="endpoint">{trend.endpoint[:80]}</td>
                    <td>{trend.previous_error_rate:.2f}%</td>
                    <td>{trend.current_error_rate:.2f}%</td>
                    <td>{trend.change_percentage:.1f}%</td>
                    <td><span class="badge badge-improving">Melhorando</span></td>
                </tr>
"""
            html += """
            </tbody>
        </table>
"""
        html += """
    </div>
"""
    else:
        html += """
    <div class="section">
        <h2><span style="color: var(--color-gold);">▼</span> Tendências</h2>
        <p style="color: var(--color-pewter);">Nenhum snapshot anterior encontrado para comparação. Execute novamente após processar outro período para ver tendências.</p>
    </div>
"""

    # Endpoints Prioritários
    if analysis.prioritized_recommendations:
        html += """
    <div class="section">
        <h2><span style="color: var(--color-gold);">◆</span> Endpoints Prioritários para Teste</h2>
        <table>
            <thead>
                <tr>
                    <th>#</th>
                    <th>Endpoint</th>
                    <th>Prioridade</th>
                    <th>Score</th>
                    <th>Recomendação</th>
                </tr>
            </thead>
            <tbody>
"""
        for i, rec in enumerate(analysis.prioritized_recommendations[:20], 1):
            priority = rec.get("priority", "medium").upper()
            badge_class = (
                f"badge-{priority.lower()}"
                if priority.lower() in ["critical", "high", "medium"]
                else "badge-medium"
            )
            html += f"""
                <tr>
                    <td>{i}</td>
                    <td class="endpoint">{rec.get('endpoint', 'N/A')[:60]}</td>
                    <td><span class="badge {badge_class}">{priority}</span></td>
                    <td>{rec.get('priority_score', 0):.1f}</td>
                    <td>{rec.get('description', '')[:100]}</td>
                </tr>
"""
        html += """
            </tbody>
        </table>
    </div>
"""

    # Gaps de Cobertura
    if analysis.coverage_gaps:
        html += f"""
    <div class="section">
        <h2><span style="color: var(--color-gold);">◉</span> Gaps de Cobertura de Testes ({len(analysis.coverage_gaps)})</h2>
        <table>
            <thead>
                <tr>
                    <th>Endpoint</th>
                    <th>Prioridade</th>
                    <th>Razão</th>
                    <th>Ação Sugerida</th>
                </tr>
            </thead>
            <tbody>
"""
        for gap in analysis.coverage_gaps[:20]:
            priority = gap.get("priority", "medium").upper()
            badge_class = (
                f"badge-{priority.lower()}"
                if priority.lower() in ["critical", "high", "medium"]
                else "badge-medium"
            )
            html += f"""
                <tr>
                    <td class="endpoint">{gap.get('endpoint', 'N/A')[:60]}</td>
                    <td><span class="badge {badge_class}">{priority}</span></td>
                    <td>{gap.get('reason', '')[:80]}</td>
                    <td>{gap.get('suggested_action', '')[:100]}</td>
                </tr>
"""
        html += """
            </tbody>
        </table>
    </div>
"""

    # Endpoints Críticos (do snapshot)
    if analysis.metrics_snapshot and analysis.metrics_snapshot.critical:
        html += f"""
    <div class="section">
        <h2><span style="color: #dc3545;">✗</span> Endpoints Críticos ({len(analysis.metrics_snapshot.critical)})</h2>
        <table>
            <thead>
                <tr>
                    <th>Endpoint</th>
                    <th>Requisições</th>
                    <th>Erros</th>
                    <th>Taxa de Erro</th>
                    <th>Erros 4XX</th>
                    <th>Erros 5XX</th>
                </tr>
            </thead>
            <tbody>
"""
        for endpoint in analysis.metrics_snapshot.critical[:50]:
            html += f"""
                <tr>
                    <td class="endpoint">{endpoint.endpoint[:80]}</td>
                    <td>{endpoint.total_requests:,}</td>
                    <td>{endpoint.total_errors:,}</td>
                    <td>{endpoint.error_rate:.2f}%</td>
                    <td>{endpoint.client_errors_4xx:,}</td>
                    <td>{endpoint.server_errors_5xx:,}</td>
                </tr>
"""
        html += """
            </tbody>
        </table>
    </div>
"""

    # Endpoints Mais Usados (do snapshot)
    if analysis.metrics_snapshot and analysis.metrics_snapshot.most_used:
        html += f"""
    <div class="section">
        <h2><span style="color: var(--color-gold);">■</span> Top Endpoints Mais Usados ({len(analysis.metrics_snapshot.most_used)})</h2>
        <table>
            <thead>
                <tr>
                    <th>Endpoint</th>
                    <th>Requisições</th>
                    <th>Erros</th>
                    <th>Taxa de Erro</th>
                    <th>Clientes Únicos</th>
                </tr>
            </thead>
            <tbody>
"""
        for endpoint in analysis.metrics_snapshot.most_used[:20]:
            html += f"""
                <tr>
                    <td class="endpoint">{endpoint.endpoint[:80]}</td>
                    <td>{endpoint.total_requests:,}</td>
                    <td>{endpoint.total_errors:,}</td>
                    <td>{endpoint.error_rate:.2f}%</td>
                    <td>{endpoint.unique_clients:,}</td>
                </tr>
"""
        html += """
            </tbody>
        </table>
    </div>
"""

    # Alertas
    if analysis.alerts:
        html += f"""
    <div class="section">
        <h2><span style="color: #dc3545;">⚠</span> Alertas ({len(analysis.alerts)})</h2>
        <table>
            <thead>
                <tr>
                    <th>Severidade</th>
                    <th>Endpoint</th>
                    <th>Mensagem</th>
                </tr>
            </thead>
            <tbody>
"""
        for alert in analysis.alerts[:20]:
            severity = alert.get("severity", "medium").upper()
            badge_class = (
                f"badge-{severity.lower()}"
                if severity.lower() in ["critical", "high", "medium"]
                else "badge-medium"
            )
            html += f"""
                <tr>
                    <td><span class="badge {badge_class}">{severity}</span></td>
                    <td class="endpoint">{alert.get('endpoint', alert.get('title', 'N/A'))[:60]}</td>
                    <td>{alert.get('message', alert.get('title', ''))[:100]}</td>
                </tr>
"""
        html += """
            </tbody>
        </table>
    </div>
"""

    html += f"""
        <div class="footer">
            <p>Framework de QA Inteligente - Gerado automaticamente</p>
            <p>Para mais informações, consulte a documentação em docs/</p>
        </div>
    </div>
</body>
</html>
"""

    with open(output_path, "w", encoding="utf-8") as f:
        f.write(html)

    return output_path


def main():
    """Função principal"""
    import argparse

    parser = argparse.ArgumentParser(
        description="Gera relatório HTML a partir de análise de métricas"
    )
    parser.add_argument(
        "snapshot_id",
        nargs="?",
        help="ID do snapshot (ex: 2025-12-14_13-00-00). Se omitido, usa o mais recente.",
    )
    parser.add_argument(
        "--output",
        "-o",
        help="Caminho do arquivo HTML de saída (padrão: reports/report_YYYY-MM-DD_HH-MM-SS.html)",
    )

    args = parser.parse_args()

    # Criar diretório de relatórios
    reports_dir = project_root / "reports"
    reports_dir.mkdir(exist_ok=True)

    # Carregar snapshot
    repo = FileMetricsRepository()

    if args.snapshot_id:
        # Buscar snapshot específico
        from datetime import datetime

        try:
            snapshot_date = datetime.strptime(args.snapshot_id, "%Y-%m-%d_%H-%M-%S")
            snapshot = repo.get_snapshot_by_date(snapshot_date)
        except ValueError:
            print(f"[ERRO] Formato de snapshot_id inválido. Use: YYYY-MM-DD_HH-MM-SS")
            return 1
    else:
        # Usar mais recente
        snapshot = repo.get_latest_snapshot()

    if not snapshot:
        print(
            "[ERRO] Nenhum snapshot encontrado. Execute analyze_reactive_metrics.py primeiro."
        )
        return 1

    # Criar análise a partir do snapshot
    # (Simplificado - em produção, carregaria análise completa)
    from src.application.reativo.reactive_service import LogAnalysis

    analysis = LogAnalysis(snapshot.time_window)
    analysis.metrics_snapshot = snapshot
    analysis.total_errors = snapshot.get_total_errors()
    analysis.error_rate = snapshot.get_overall_error_rate() / 100.0

    # Buscar snapshot anterior para tendências
    previous = repo.get_snapshots_in_range(
        snapshot.timestamp - timedelta(days=1), snapshot.timestamp
    )
    if previous:
        from src.application.reativo.metrics_analyzer import MetricsAnalyzer

        analyzer = MetricsAnalyzer()
        analysis.trends = analyzer.compare_snapshots(previous[0], snapshot)

    # Gerar recomendações
    from src.application.reativo.recommendation_engine import RecommendationEngine

    engine = RecommendationEngine()
    analysis.prioritized_recommendations = engine.generate_prioritized_recommendations(
        snapshot, analysis.trends if hasattr(analysis, "trends") else None
    )
    analysis.coverage_gaps = engine.suggest_test_coverage_gaps(snapshot, [])
    analysis.alerts = []  # Simplificado

    # Gerar HTML
    if args.output:
        output_path = Path(args.output)
    else:
        output_path = reports_dir / f"report_{snapshot.snapshot_id}.html"

    generate_html(analysis, output_path)

    print(f"[SUCESSO] Relatório HTML gerado: {output_path}")
    print(f"[INFO] Abrindo no navegador...")

    # Abrir automaticamente no navegador
    import webbrowser
    import os

    file_url = f"file://{output_path.absolute()}"
    webbrowser.open(file_url)

    return 0


if __name__ == "__main__":
    sys.exit(main())
