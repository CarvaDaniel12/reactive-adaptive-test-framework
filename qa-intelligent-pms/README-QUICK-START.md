# Quick Start - Para QAs (Sem Terminal)

## 🚀 Começar em 3 Passos

### 1. Exportar do Splunk
- Acesse Splunk web
- Execute query (veja [GUIA-EXPORTACAO-SPLUNK.md](docs/GUIA-EXPORTACAO-SPLUNK.md))
- Exporte como CSV (Events + Verbose Mode)

### 2. Colocar Arquivo
- Copie o CSV para: `data/splunk_exports/`

### 3. Processar (Escolha uma opção)

**⭐ Opção A - Duplo Clique (Mais Fácil)**:
- **Windows**: Duplo clique em `scripts/processar_metricas.bat`
- **Mac/Linux**: Duplo clique em `scripts/processar_metricas.sh`

**⭐ Opção B - Interface Web (Mais Amigável)**:
- Duplo clique em `scripts/iniciar_interface_web.bat` (Windows) ou `.sh` (Mac/Linux)
- Arraste o CSV para a área de upload
- Clique em "Processar Métricas"

**Opção C - Terminal (Se preferir)**:
```bash
python scripts/analyze_reactive_metrics.py
```

## ✅ Pronto!

O sistema vai:
- Processar métricas automaticamente
- Gerar relatório HTML (abre no navegador)
- Salvar snapshot para comparação futura

## 📚 Documentação Completa

- **Usuários finais**: [GUIA-USUARIO-FINAL.md](docs/GUIA-USUARIO-FINAL.md)
- **Sem terminal**: [INTERFACE-SEM-TERMINAL.md](docs/INTERFACE-SEM-TERMINAL.md)
- **Fluxo completo**: [FLUXO-COMPLETO-USUARIO.md](docs/FLUXO-COMPLETO-USUARIO.md)
- **Exportar Splunk**: [GUIA-EXPORTACAO-SPLUNK.md](docs/GUIA-EXPORTACAO-SPLUNK.md)

