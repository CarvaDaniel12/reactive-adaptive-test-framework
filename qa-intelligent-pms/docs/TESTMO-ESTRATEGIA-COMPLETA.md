# Estratégia Completa de Integração com Testmo

## Visão Geral

O Testmo é uma peça central do nosso framework, servindo como:
- **Repositório de Test Cases**: Armazenamento centralizado
- **Dashboard**: Visualização de métricas e tendências
- **Test Runs**: Execução e rastreamento de testes
- **Projetos**: Organização por projeto/módulo

## Três Camadas de Integração

### 1. API REST (30 endpoints)
**Quando usar**: Gerenciamento de dados, consultas, CRUD

**Recursos disponíveis:**
- Cases (CRUD completo)
- Projects, Folders, Milestones
- Runs, Results, Sessions
- Users, Roles, Groups
- Automation Sources & Runs
- Attachments

**Status**: ✅ Parcialmente implementado (Cases básico)

### 2. Testmo CLI (7 comandos)
**Quando usar**: Automação, submissão de resultados, CI/CD

**Recursos disponíveis:**
- `automation:run:submit` - Submeter test runs completos
- `automation:run:create` - Criar runs para paralelização
- `automation:run:submit-thread` - Submeter threads paralelos
- `automation:run:complete` - Completar runs
- `automation:resources:add-field` - Custom fields
- `automation:resources:add-link` - Links externos
- `automation:resources:add-artifact` - Artifacts

**Status**: ✅ Instalado e adapter criado

### 3. Interface Web (91+ funcionalidades)
**Quando usar**: Configuração, relatórios, análise visual

**Recursos disponíveis:**
- Reporting Center (métricas avançadas)
- Forecasting (previsões)
- Data Exports
- Custom Fields (configuração)
- Integrações (configuração)
- Test Case Repository (visual)
- BDD & Gherkin (editor visual)

**Status**: ⚠️ Apenas via navegação manual (não automatizado)

## Matriz de Decisão: Quando Usar Cada Camada

| Tarefa | API REST | CLI | Web Interface |
|--------|----------|-----|----------------|
| Criar/Atualizar Test Cases | ✅ | ❌ | ✅ |
| Submeter Resultados de Testes | ⚠️ | ✅ | ❌ |
| Executar Testes e Capturar Output | ❌ | ✅ | ❌ |
| Testes Paralelos | ⚠️ | ✅ | ❌ |
| Consultar Runs/Results | ✅ | ❌ | ✅ |
| Adicionar Custom Fields/Links | ⚠️ | ✅ | ✅ |
| Ver Relatórios e Métricas | ❌ | ❌ | ✅ |
| Configurar Integrações | ❌ | ❌ | ✅ |
| Exportar Dados | ❌ | ❌ | ✅ |
| Gerenciar Folders/Milestones | ✅ | ❌ | ✅ |

## Arquitetura de Integração

```
┌─────────────────────────────────────────────────────────┐
│                    Nossa Aplicação                      │
└─────────────────────────────────────────────────────────┘
                            │
        ┌───────────────────┼───────────────────┐
        │                   │                   │
        ▼                   ▼                   ▼
┌──────────────┐    ┌──────────────┐    ┌──────────────┐
│  API REST    │    │  Testmo CLI  │    │  Web (Manual)│
│  Adapter     │    │  Adapter     │    │  (Usuário)   │
└──────────────┘    └──────────────┘    └──────────────┘
        │                   │                   │
        ▼                   ▼                   ▼
┌─────────────────────────────────────────────────────────┐
│                    Testmo Platform                      │
│  - Test Cases Repository                               │
│  - Test Runs & Results                                 │
│  - Dashboards & Metrics                                │
│  - Projects, Folders, Milestones                       │
└─────────────────────────────────────────────────────────┘
```

## Fluxos de Integração

### Fluxo Reativo (Splunk → Testmo)

1. **Análise de Logs (Splunk)**
   - Processar arquivo CSV/JSON exportado
   - Identificar endpoints críticos
   - Gerar estratégia de testes

2. **Buscar no Postman**
   - Encontrar endpoints correspondentes
   - Extrair request bodies, CURL commands

3. **Sincronizar com Testmo**
   - **API REST**: Verificar se test case existe
   - **API REST**: Criar/atualizar test cases
   - **CLI**: Submeter resultados quando testes forem executados

### Fluxo Preventivo (Jira → Testmo)

1. **Análise de Tickets (Jira)**
   - Buscar tickets do sprint
   - Extrair ACs e descrições

2. **Geração de Test Cases**
   - Converter para Gherkin
   - Criar test cases estruturados

3. **Criação no Testmo**
   - **API REST**: Criar test cases
   - **API REST**: Organizar em folders
   - **API REST**: Associar a milestones

### Fluxo de Automação (Testes → Testmo)

1. **Execução de Testes**
   - Rodar testes automatizados
   - Gerar JUnit XML reports

2. **Submissão via CLI**
   - **CLI**: Submeter resultados
   - **CLI**: Adicionar contexto (build, commit, etc.)
   - **CLI**: Associar a milestones/configs

3. **Visualização**
   - **Web**: Ver resultados no dashboard
   - **Web**: Analisar métricas e tendências
   - **API REST**: Consultar resultados programaticamente

## Implementação Priorizada

### Fase 1: Core Essentials ✅
- [x] Testmo CLI instalado
- [x] TestmoCLIAdapter criado
- [x] TestmoAdapter básico (Cases CRUD)
- [x] TestmoAdapter completo (30+ endpoints: Folders, Milestones, Runs, Sessions, Automation)
- [x] Sistema de nomenclatura e estrutura
- [x] Normalizadores (Componentes, Endpoints)
- [x] TestmoStructureService
- [x] TestCaseInheritanceService
- [x] ReactiveMergeService

### Fase 2: Organização ✅
- [x] Implementar Folders completo (API REST)
- [x] Implementar Milestones completo (API REST)
- [x] Integrar CLI para submissão de resultados
- [x] Estrutura de repositórios (Base e Reativo)

### Fase 3: Automação Avançada ✅
- [x] Integração com fluxo reativo (criar test cases com herança)
- [x] Integração com fluxo preventivo (criar test cases por sprint)
- [x] Processo de merge ao final da sprint
- [x] UI para edição de nomes com validação

### Fase 4: Relatórios e Métricas
- [ ] Exportar dados via web (manual)
- [ ] Processar exports localmente
- [ ] Integrar métricas no dashboard interno

## Exemplos de Uso

### Exemplo 1: Criar Test Case e Executar

```python
from src.infrastructure.adapters.testmo_adapter import TestmoAdapter
from src.infrastructure.adapters.testmo_cli_adapter import TestmoCLIAdapter

# Criar test case via API REST
api = TestmoAdapter(config)
test_case = api.create_test_case(
    project_id=1,
    name="Login API Test",
    description="Test login endpoint",
    steps=["1. Send POST /api/login", "2. Verify response"]
)

# Executar teste e submeter resultado via CLI
cli = TestmoCLIAdapter(config)
cli.submit_test_run(
    project_id=1,
    name="Login API Test Run",
    source="api-tests",
    results_files=["reports/login-test.xml"],
    executable_command=["pytest", "tests/test_login.py", "--junitxml=reports/login-test.xml"]
)
```

### Exemplo 2: Testes Paralelos

```python
# Criar run
run_id = cli.create_test_run(
    project_id=1,
    name="E2E Tests - Parallel",
    source="e2e-tests"
)

# Cada thread executa em paralelo
threads = [
    ["tests/e2e/test_auth.py"],
    ["tests/e2e/test_booking.py"],
    ["tests/e2e/test_payment.py"]
]

for i, test_files in enumerate(threads):
    cli.submit_thread(
        run_id=run_id,
        results_files=[f"reports/thread_{i}.xml"],
        executable_command=["pytest"] + test_files + [f"--junitxml=reports/thread_{i}.xml"]
    )

# Completar run
cli.complete_run(run_id)
```

### Exemplo 3: Adicionar Contexto Completo

```python
# Preparar recursos
cli.add_resource_field("string", "Build Number", os.getenv("BUILD_NUMBER"))
cli.add_resource_field("string", "Git Commit", os.getenv("GIT_COMMIT"))
cli.add_resource_link("CI/CD Pipeline", os.getenv("CI_PIPELINE_URL"))
cli.add_resource_artifact("Coverage Report", "https://storage.example.com/coverage.html")

# Submeter com contexto
cli.submit_test_run(
    project_id=1,
    name="Full Test Suite",
    source="ci-tests",
    results_files=["reports/junit.xml"],
    resources_file="testmo-resources.json",
    tags=["ci", "automated", "full-suite"]
)
```

## Próximos Passos Imediatos

1. **Testar TestmoCLIAdapter**: Criar script de teste básico
2. **Integrar no fluxo reativo**: Usar CLI para submeter resultados após análise do Splunk
3. **Integrar no fluxo preventivo**: Usar API REST para criar test cases a partir de Jira
4. **Melhorar TestmoAdapter**: Adicionar suporte completo a paginação, filtros, expands

## Conclusão

Com a combinação de **API REST + CLI**, temos acesso a praticamente todas as funcionalidades do Testmo que podem ser automatizadas. A interface web permanece para configuração e visualização de relatórios avançados, mas o core do nosso framework pode ser totalmente automatizado.

