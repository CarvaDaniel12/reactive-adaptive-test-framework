# Humano no Loop: Design de Interação

## Visão Geral

O sistema de QA Inteligente foi projetado para **assistir** o QA, não substituí-lo. O "humano no loop" é essencial para:
- **Validação** de recomendações geradas
- **Contexto de negócio** que a máquina não tem
- **Decisões estratégicas** sobre priorização
- **Ajustes finos** baseados em conhecimento tácito

**IMPORTANTE**: Este é um **framework** usado por QAs que não têm acesso a Cursor/VSCode. O fluxo completo considera:
- ✅ Exportação manual do Splunk (humano no Splunk web)
- ✅ Upload do arquivo via interface web (ou colocar em pasta e usar terminal)
- ✅ Processamento automático (sistema processa)
- ✅ Interpretação e decisão (humano lê resultados)

Veja [GUIA-USUARIO-FINAL.md](GUIA-USUARIO-FINAL.md) para o fluxo passo a passo do usuário final.

## Pontos de Interação Humana

### 1. Análise Reativa - Validação de Prioridades

**Quando**: Após processar métricas do Splunk

**O que o sistema faz**:
- Processa métricas automaticamente
- Gera lista de endpoints prioritários
- Calcula scores de prioridade
- Identifica tendências e riscos

**O que o humano faz**:
- **Revisa** a lista de prioridades
- **Ajusta** scores baseado em contexto de negócio
- **Remove** falsos positivos (ex: endpoints legados que não precisam de teste)
- **Adiciona** endpoints importantes que o sistema não detectou

**Interface proposta**:
```bash
# Sistema gera relatório
python scripts/analyze_reactive_metrics.py data/splunk_exports/complete_6h.csv

# Humano revisa e ajusta
python scripts/interactive_priority_review.py --snapshot-id 2025-12-14_13-00-00
```

### 2. Geração de Testes - Aprovação e Edição

**Quando**: Sistema sugere criar testes para endpoints críticos

**O que o sistema faz**:
- Identifica endpoints que precisam de testes
- Gera sugestões de casos de teste
- Cria estrutura básica de collection Postman

**O que o humano faz**:
- **Aprova** ou **rejeita** sugestões
- **Edita** casos de teste gerados
- **Adiciona** casos específicos que o sistema não pensou
- **Ajusta** dados de teste (ex: IDs específicos, tokens)

**Interface proposta**:
```bash
# Sistema gera sugestões
python scripts/generate_test_suggestions.py --endpoint "/api/v3/quotes"

# Humano revisa interativamente
python scripts/interactive_test_generator.py --collection-id abc123
```

### 3. Análise de Tendências - Interpretação

**Quando**: Sistema detecta degradação ou melhoria

**O que o sistema faz**:
- Compara períodos
- Identifica mudanças significativas
- Gera alertas automáticos

**O que o humano faz**:
- **Interpreta** o contexto (ex: "degradação esperada após deploy")
- **Investiga** causas raiz
- **Decide** se ação é necessária
- **Documenta** decisões e aprendizados

**Interface proposta**:
```bash
# Sistema mostra tendências
python scripts/analyze_reactive_metrics.py --compare

# Humano adiciona contexto
python scripts/add_trend_context.py --trend-id TREND-001 --note "Deploy realizado às 14h"
```

### 4. Integração com Postman - Revisão de Collections

**Quando**: Sistema cria/atualiza collections automaticamente

**O que o sistema faz**:
- Cria collection com testes sugeridos
- Organiza por prioridade
- Adiciona assertions básicas

**O que o humano faz**:
- **Revisa** collection no Postman
- **Ajusta** assertions
- **Adiciona** variáveis de ambiente
- **Executa** testes manualmente primeiro
- **Aprova** para automação contínua

**Fluxo**:
1. Sistema cria collection `"Reactive Tests - 2025-12-14"`
2. Humano recebe notificação/relatório
3. Humano abre no Postman e revisa
4. Humano marca como "aprovado" ou "precisa ajuste"
5. Sistema pode re-gerar baseado em feedback

### 5. Decisões de Cobertura - Estratégia

**Quando**: Sistema identifica gaps de cobertura

**O que o sistema faz**:
- Lista endpoints sem testes
- Prioriza por criticidade
- Sugere tipos de teste

**O que o humano faz**:
- **Decide** quais gaps são aceitáveis (ex: endpoints internos)
- **Prioriza** baseado em roadmap
- **Planeja** sprints de cobertura
- **Balanceia** esforço vs. valor

**Interface proposta**:
```bash
# Sistema mostra gaps
python scripts/analyze_coverage_gaps.py

# Humano marca decisões
python scripts/mark_coverage_decisions.py --endpoint "/api/internal/*" --decision "skip" --reason "Endpoints internos, baixa prioridade"
```

## Padrões de Interação

### Modo Interativo vs. Automático

**Modo Automático** (atual):
- Sistema processa e gera relatórios
- Humano lê e decide ações manualmente
- Sem feedback loop

**Modo Interativo** (proposto):
- Sistema gera sugestões
- Humano revisa e aprova/rejeita
- Sistema aprende com feedback
- Próximas sugestões melhoram

### Feedback Loop

```
Sistema Gera → Humano Revisa → Humano Ajusta → Sistema Aprende → Próxima Geração Melhor
```

**Exemplos de feedback**:
- "Este endpoint não é crítico" → Sistema reduz prioridade
- "Faltou testar este cenário" → Sistema adiciona ao template
- "Este alerta é falso positivo" → Sistema ajusta threshold

## Implementação Futura

### Fase 1: Revisão Interativa (Próxima)
- Scripts interativos para revisar prioridades
- Aprovação/rejeição de sugestões
- Salvar decisões humanas

### Fase 2: Feedback Loop
- Sistema aprende com decisões humanas
- Melhora sugestões ao longo do tempo
- Personaliza para contexto do projeto

### Fase 3: Interface Web (Futuro)
- Dashboard para revisar métricas
- Interface drag-and-drop para priorização
- Visualização de tendências
- Aprovação de testes em lote

## Exemplos de Uso

### Cenário 1: Revisão Semanal de Métricas

```bash
# 1. Sistema processa métricas
python scripts/analyze_reactive_metrics.py data/splunk_exports/weekly.csv

# 2. Humano revisa prioridades
python scripts/review_priorities.py --interactive

# 3. Sistema gera testes aprovados
python scripts/generate_approved_tests.py --priority-threshold 70
```

### Cenário 2: Investigação de Degradação

```bash
# 1. Sistema detecta degradação
python scripts/analyze_reactive_metrics.py --compare

# 2. Humano investiga e documenta
python scripts/investigate_trend.py --trend-id TREND-001
# [Abre interface para adicionar notas, screenshots, links para tickets]

# 3. Sistema cria ticket no Jira (se aprovado)
python scripts/create_investigation_ticket.py --trend-id TREND-001 --approved
```

### Cenário 3: Planejamento de Cobertura

```bash
# 1. Sistema mostra gaps
python scripts/analyze_coverage_gaps.py

# 2. Humano planeja sprint
python scripts/plan_coverage_sprint.py --interactive
# [Interface para selecionar endpoints, estimar esforço, criar tickets]

# 3. Sistema gera ACs e testes
python scripts/generate_sprint_tests.py --sprint-id SPRINT-123
```

## Princípios de Design

1. **Transparência**: Sistema sempre mostra **por que** sugeriu algo
2. **Reversibilidade**: Humano pode **desfazer** qualquer ação automática
3. **Controle**: Humano tem **controle total** sobre o que é automatizado
4. **Aprendizado**: Sistema **melhora** com feedback humano
5. **Eficiência**: Automação **acelera** trabalho humano, não substitui

## Próximos Passos

1. ✅ Fluxo básico funcionando (análise reativa)
2. 🔄 **Implementar revisão interativa de prioridades**
3. 🔄 **Criar interface para aprovação de testes**
4. 🔄 **Adicionar feedback loop básico**
5. 🔄 **Integrar com Jira para criar tickets aprovados**

