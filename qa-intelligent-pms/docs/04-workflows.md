# Fluxos de Trabalho

Este documento descreve os fluxos de trabalho principais do sistema, incluindo diagramas de sequência detalhados.

## Fluxo Preventivo

O fluxo preventivo analisa tickets antes da Sprint começar, gerando ACs, estratégias de teste e criando test cases no Testmo organizados por sprint e componente.

### Diagrama de Sequência

```mermaid
sequenceDiagram
    participant User
    participant CLI
    participant PreventiveService
    participant JiraAdapter
    participant RiskAnalyzer
    participant ACGenerator
    participant PostmanAdapter
    participant Jira
    participant Postman
    
    User->>CLI: analyze_sprint(board_id)
    CLI->>PreventiveService: analyze_upcoming_sprint(board_id)
    
    PreventiveService->>JiraAdapter: get_sprint_tickets(board_id)
    JiraAdapter->>Jira: GET /rest/api/3/search
    Jira-->>JiraAdapter: tickets[]
    JiraAdapter-->>PreventiveService: List[Ticket]
    
    loop Para cada ticket
        PreventiveService->>RiskAnalyzer: calculate_risk(ticket)
        RiskAnalyzer->>RiskAnalyzer: analyze_history(ticket)
        RiskAnalyzer->>RiskAnalyzer: calculate_complexity(ticket)
        RiskAnalyzer-->>PreventiveService: RiskLevel
        
        alt Ticket sem ACs
            PreventiveService->>ACGenerator: generate_acs(ticket)
            ACGenerator->>ACGenerator: select_template(ticket.type)
            ACGenerator->>ACGenerator: fill_template(ticket)
            ACGenerator-->>PreventiveService: List[AC]
        end
    end
    
    PreventiveService->>PreventiveService: create_test_cases_for_sprint(analysis)
    
    loop Para cada ticket
        PreventiveService->>TestmoStructureService: ensure_sprint_structure(sprint_id, component, endpoint)
        TestmoStructureService->>TestmoAdapter: create_folder() ou find_folder()
        TestmoAdapter->>Testmo: POST /folders ou GET /folders
        Testmo-->>TestmoAdapter: folder
        
        PreventiveService->>GherkinGenerator: generate_gherkin(ticket)
        GherkinGenerator-->>PreventiveService: Gherkin
        
        PreventiveService->>TestCaseGenerator: generate_test_cases(gherkin)
        TestCaseGenerator-->>PreventiveService: test_cases[]
        
        loop Para cada test case
            PreventiveService->>TestmoAdapter: create_test_case()
            TestmoAdapter->>Testmo: POST /cases
            Testmo-->>TestmoAdapter: test_case
        end
    end
    
    PreventiveService-->>CLI: SprintAnalysis + stats
    CLI-->>User: Relatório de análise
```

### Passos Detalhados

1. **Início**: Usuário executa comando para analisar Sprint
2. **Busca Tickets**: Sistema busca todos os tickets da Sprint no Jira
3. **Análise de Risco**: Para cada ticket:
   - Analisa histórico de bugs do componente
   - Calcula complexidade baseada em descrição e tipo
   - Determina nível de risco
4. **Geração de ACs**: Para tickets sem ACs:
   - Seleciona template baseado no tipo de ticket
   - Preenche template com informações do ticket
   - Gera ACs sugeridos
5. **Normalização**: Normaliza componentes e endpoints
6. **Criação de Estrutura**: Cria estrutura no Testmo:
   - `Sprint-{ID}/{Component}/{METHOD}_{Endpoint}/`
7. **Geração de Test Cases**: Para cada ticket:
   - Converte para Gherkin
   - Gera test cases estruturados
   - Cria no Testmo com nome: `{TICKET_KEY}_{METHOD}_{TestType}_{Description}`
8. **Retorno**: Retorna análise completa com recomendações e estatísticas de criação

### Exemplo de Uso

```python
from src.application.preventivo import PreventiveService

service = PreventiveService()
analysis = service.analyze_upcoming_sprint("PMS-123")

print(f"Risco geral: {analysis.risk_assessment.overall_risk}")
print(f"Tickets de alto risco: {len(analysis.risk_assessment.high_risk_tickets)}")
print(f"Testes sugeridos: {analysis.test_strategy.total_tests_suggested}")
```

## Fluxo Reativo

O fluxo reativo analisa logs em produção para identificar padrões, gerar alertas e criar test cases no Testmo com estrutura organizada.

### Diagrama de Sequência

```mermaid
sequenceDiagram
    participant User
    participant WebApp
    participant ReactiveService
    participant SplunkFileAdapter
    participant PostmanAdapter
    participant TestCaseReuseService
    participant TestmoStructureService
    participant TestCaseInheritanceService
    participant TestmoAdapter
    participant Testmo
    
    User->>WebApp: Upload arquivo Splunk (CSV/JSON)
    WebApp->>ReactiveService: analyze_production_logs(file_path)
    
    ReactiveService->>SplunkFileAdapter: process_file(file_path)
    SplunkFileAdapter-->>ReactiveService: EndpointMetrics[]
    
    ReactiveService->>ReactiveService: generate_snapshot(metrics)
    ReactiveService->>ReactiveService: analyze_trends()
    ReactiveService->>ReactiveService: generate_recommendations()
    ReactiveService-->>WebApp: LogAnalysis
    
    User->>WebApp: Buscar matches no Postman
    WebApp->>TestCaseReuseService: find_postman_matches(endpoints)
    TestCaseReuseService->>PostmanAdapter: search_requests(endpoints)
    PostmanAdapter-->>TestCaseReuseService: matches[]
    TestCaseReuseService-->>WebApp: matches[]
    
    User->>WebApp: Selecionar test cases e sincronizar
    WebApp->>TestCaseReuseService: sync_to_testmo(selected, reactive_context)
    
    loop Para cada test case selecionado
        TestCaseReuseService->>TestCaseInheritanceService: find_base_case(component, endpoint, method)
        TestCaseInheritanceService->>TestmoAdapter: search_test_cases()
        TestmoAdapter->>Testmo: GET /cases
        Testmo-->>TestmoAdapter: cases[]
        TestmoAdapter-->>TestCaseInheritanceService: base_case ou None
        
        alt Caso base encontrado
            TestCaseReuseService->>TestmoStructureService: ensure_reactive_structure()
            TestmoStructureService->>TestmoAdapter: create_folder() ou find_folder()
            TestmoAdapter->>Testmo: POST /folders ou GET /folders
            Testmo-->>TestmoAdapter: folder
            
            TestCaseReuseService->>TestCaseInheritanceService: inherit_to_reactive()
            TestCaseInheritanceService->>TestmoAdapter: create_test_case()
            TestmoAdapter->>Testmo: POST /cases
            Testmo-->>TestmoAdapter: test_case
        else Caso base não encontrado
            TestCaseReuseService->>TestmoStructureService: ensure_reactive_structure()
            TestCaseReuseService->>TestmoAdapter: create_test_case()
            TestmoAdapter->>Testmo: POST /cases
            Testmo-->>TestmoAdapter: test_case
        end
    end
    
    TestCaseReuseService-->>WebApp: stats (created, inherited, errors)
    WebApp-->>User: Resultado da sincronização
```

### Passos Detalhados

1. **Upload de Arquivo**: Usuário faz upload de arquivo CSV/JSON exportado do Splunk
2. **Processamento**: Sistema processa arquivo e extrai métricas de endpoints
3. **Análise de Tendências**: Compara com snapshots anteriores (se existirem)
4. **Geração de Recomendações**: Identifica endpoints prioritários para teste
5. **Busca no Postman**: Busca endpoints correspondentes no Postman
6. **Sugestão de Test Cases**: Prepara sugestões com informações do Postman
7. **Verificação no Testmo**: Verifica se test cases já existem
8. **Herança de Casos**: Busca casos similares no Base para herdar
9. **Criação no Testmo**: Cria test cases no repositório Reativo:
   - Estrutura: `Reativo/{Date}_{Priority}_{Trend}/{Endpoint}/`
   - Casos herdados ou novos
   - Links para casos base (se herdados)
10. **Armazenamento**: Salva análise e snapshot para histórico
11. **Merge ao Final da Sprint**: Migra casos úteis para Base e deleta pasta reativa

### Exemplo de Uso

```python
from src.application.reativo import ReactiveService
from datetime import timedelta

service = ReactiveService()
time_window = timedelta(hours=24)
analysis = service.analyze_production_logs(time_window)

print(f"Padrões identificados: {len(analysis.patterns)}")
print(f"Alertas gerados: {len(analysis.alerts)}")
for alert in analysis.alerts:
    print(f"  - {alert.title}: {alert.severity}")
```

## Fluxo QA Agent

O fluxo do QA Agent grava ações do QA durante testes e gera automação.

### Diagrama de Sequência

```mermaid
sequenceDiagram
    participant QA
    participant QAAgent
    participant QARecorder
    participant PlaywrightEngine
    participant TestSuggester
    participant Browser
    participant Storage
    
    QA->>QAAgent: start_testing(ticket_key)
    QAAgent->>TestSuggester: suggest_test_cases(ticket_key)
    TestSuggester->>TestSuggester: load_ticket_info(ticket_key)
    TestSuggester->>TestSuggester: generate_suggestions()
    TestSuggester-->>QAAgent: List[TestSuggestion]
    QAAgent-->>QA: Sugestões de teste
    
    QA->>QAAgent: begin_recording()
    QAAgent->>QARecorder: start_session(ticket_key, qa_name)
    QARecorder->>PlaywrightEngine: launch_browser()
    PlaywrightEngine->>Browser: Launch Chromium
    Browser-->>PlaywrightEngine: Browser instance
    PlaywrightEngine-->>QARecorder: Page
    
    loop Durante teste manual
        QA->>Browser: Executa ação (click, fill, etc)
        Browser->>QARecorder: action_event
        QARecorder->>QARecorder: record_action(action)
        QARecorder->>PlaywrightEngine: take_screenshot()
        PlaywrightEngine->>Browser: Screenshot
        Browser-->>PlaywrightEngine: image
        PlaywrightEngine-->>QARecorder: screenshot_path
        QARecorder->>QARecorder: save_action(action, screenshot)
    end
    
    QA->>QAAgent: finish_testing()
    QAAgent->>QARecorder: stop_session()
    QARecorder->>QARecorder: generate_playwright_script()
    QARecorder-->>QAAgent: QASession
    
    QAAgent->>Storage: save_session(session)
    QAAgent-->>QA: Test report + Automation script
```

### Passos Detalhados

1. **Início**: QA inicia sessão de teste para um ticket
2. **Sugestões**: Sistema sugere casos de teste baseados no ticket
3. **Gravação**: QA executa testes manualmente enquanto sistema grava:
   - Todas as ações (cliques, preenchimentos, navegações)
   - Screenshots após cada ação importante
   - URLs visitadas
   - Elementos interagidos
4. **Geração de Script**: Ao finalizar:
   - Converte ações gravadas em script Playwright
   - Identifica seletores dos elementos
   - Cria validações básicas
5. **Armazenamento**: Salva sessão completa
6. **Retorno**: Retorna relatório e script de automação

### Exemplo de Uso

```python
from src.application.agent import QAAgent

agent = QAAgent()
session = agent.assist_qa_testing("PMS-456")

print(f"Sessão gravada: {session.session_id}")
print(f"Ações gravadas: {len(session.actions)}")
print(f"Script gerado: {session.automation_script is not None}")
```

## Fluxo Integrado (Preventivo + Reativo)

O sistema pode combinar análise preventiva e reativa para melhorar predições.

### Diagrama de Sequência

```mermaid
sequenceDiagram
    participant System
    participant PreventiveService
    participant ReactiveService
    participant CorrelationEngine
    participant JiraAdapter
    participant SplunkAdapter
    
    System->>PreventiveService: analyze_sprint(board_id)
    PreventiveService->>JiraAdapter: get_tickets()
    JiraAdapter-->>PreventiveService: tickets[]
    
    System->>ReactiveService: analyze_logs(time_window)
    ReactiveService->>SplunkAdapter: query_logs()
    SplunkAdapter-->>ReactiveService: logs[]
    
    System->>CorrelationEngine: correlate(preventive, reactive)
    CorrelationEngine->>CorrelationEngine: match_components(tickets, patterns)
    CorrelationEngine->>CorrelationEngine: adjust_risk_scores()
    CorrelationEngine->>CorrelationEngine: enhance_recommendations()
    CorrelationEngine-->>System: EnhancedAnalysis
    
    System->>System: generate_final_report()
```

### Benefícios da Integração

1. **Risco Mais Preciso**: Usa dados de produção para ajustar risco de tickets
2. **Testes Mais Relevantes**: Sugere testes baseados em problemas reais
3. **Priorização Melhor**: Foca em componentes com histórico de problemas
4. **Prevenção Real**: Evita problemas conhecidos antes de acontecerem

## Fluxo de Execução de Testes

Fluxo para executar testes gerados automaticamente.

### Diagrama de Sequência

```mermaid
sequenceDiagram
    participant User
    participant TestRunner
    participant PostmanAdapter
    participant PlaywrightEngine
    participant Postman
    participant Browser
    
    User->>TestRunner: run_tests(test_cases)
    
    loop Para cada teste
        alt Teste API
            TestRunner->>PostmanAdapter: execute_request(test)
            PostmanAdapter->>Postman: POST /collections/{id}/runs
            Postman->>Postman: Execute request
            Postman-->>PostmanAdapter: Result
            PostmanAdapter-->>TestRunner: TestResult
        else Teste UI
            TestRunner->>PlaywrightEngine: execute_script(test)
            PlaywrightEngine->>Browser: Run script
            Browser-->>PlaywrightEngine: Result
            PlaywrightEngine-->>TestRunner: TestResult
        end
        
        TestRunner->>TestRunner: validate_result(result)
        TestRunner->>TestRunner: save_result(result)
    end
    
    TestRunner-->>User: TestExecutionReport
```

## Fluxo de Notificações

Fluxo para enviar notificações quando alertas são gerados.

### Diagrama de Sequência

```mermaid
sequenceDiagram
    participant ReactiveService
    participant AlertManager
    participant NotificationService
    participant Slack
    participant Email
    
    ReactiveService->>AlertManager: generate_alert(pattern)
    AlertManager->>AlertManager: determine_severity(pattern)
    
    alt Severidade CRITICAL
        AlertManager->>NotificationService: notify_critical(alert)
        NotificationService->>Slack: Send message
        NotificationService->>Email: Send email
    else Severidade HIGH
        AlertManager->>NotificationService: notify_high(alert)
        NotificationService->>Slack: Send message
    else Severidade MEDIUM
        AlertManager->>NotificationService: notify_medium(alert)
        NotificationService->>NotificationService: Log only
    end
```

## Considerações de Implementação

### Tratamento de Erros

Todos os fluxos devem tratar erros graciosamente:
- Logs detalhados de erros
- Retry para falhas temporárias
- Fallback para operações críticas
- Notificações de falhas

### Performance

- Queries ao Splunk devem ter timeout
- Cache de resultados quando possível
- Processamento assíncrono para operações longas
- Limites de paginação para grandes volumes

### Detecção de Anomalias (Story 31.9)

O sistema detecta automaticamente anomalias durante a execução de workflows.

**Fluxo de Detecção:**
1. Após workflow completion, sistema carrega baseline histórico (últimas 30 execuções)
2. Calcula métricas estatísticas (média, desvio padrão) para execution time
3. Compara execução atual com baseline usando z-score
4. Detecta anomalias se:
   - Performance degradation: execution time > baseline + 2σ
   - Unusual execution time: |z-score| > 2.0
5. Gera alertas se anomalias forem detectadas
6. Armazena anomalias no banco de dados para análise posterior

**Tipos de Anomalias:**
- `PerformanceDegradation`: Tempo de execução significativamente acima do baseline
- `UnusualExecutionTime`: Tempo de execução incomum (muito rápido ou muito lento)

**Severidade:**
- `Info`: Z-score entre 2.0 e 2.5
- `Warning`: Z-score entre 2.5 e 3.0
- `Critical`: Z-score > 3.0

**Integração:**
- Detecção automática no evento `complete_workflow`
- Executado em background (non-blocking)
- Alertas enviados via sistema de alertas com rate limiting
- Dashboard UI disponível em `/anomalies`

### Segurança

- Credenciais nunca em logs
- Validação de inputs em todas as camadas
- Sanitização de dados antes de armazenar
- Auditoria de ações sensíveis

