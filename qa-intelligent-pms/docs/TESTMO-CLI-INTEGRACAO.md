# Integração com Testmo CLI - Guia Completo

## Visão Geral

O Testmo CLI oferece funcionalidades avançadas que complementam a API REST:

### O que o CLI oferece que a API REST não tem:

1. **Submissão de Test Runs com JUnit XML**
   - Processa automaticamente arquivos JUnit XML
   - Suporta múltiplos arquivos e busca recursiva (glob patterns)
   - Detecta e valida automaticamente formato JUnit

2. **Execução de Testes com Captura**
   - Executa comandos de teste diretamente
   - Captura console output automaticamente
   - Mede tempo de execução
   - Propaga exit codes para CI/CD

3. **Testes Paralelos (Threads)**
   - Cria runs para execução paralela
   - Submete threads individuais
   - Completa runs após todos os threads

4. **Recursos Avançados**
   - Custom fields (string, url, text, console, html)
   - Links para recursos externos
   - Artifacts (com detecção automática de size/mime type)

5. **Integração CI/CD Nativa**
   - Suporta todos os principais CI/CD tools
   - Propaga exit codes corretamente
   - Suporta execução em containers Docker

## Instalação

```bash
npm install -g @testmo/testmo-cli
```

**Nota**: Você não precisa usar Node.js/JavaScript no seu projeto. O NPM é apenas para instalar o CLI tool.

## Comandos Disponíveis

### 1. `automation:run:submit`
Submete um test run completo (cria, submete resultados e completa em um único comando).

**Uso básico:**
```bash
export TESTMO_TOKEN=seu_token
testmo automation:run:submit \
  --instance https://hostfully-pmp.testmo.net \
  --project-id 1 \
  --name "Test Run" \
  --source "backend-unit" \
  --results reports/*.xml
```

**Com execução de testes:**
```bash
testmo automation:run:submit \
  --instance https://hostfully-pmp.testmo.net \
  --project-id 1 \
  --name "Test Run" \
  --source "backend-unit" \
  --results reports/*.xml \
  -- pytest tests/
```

**Com milestone e configuração:**
```bash
testmo automation:run:submit \
  --instance https://hostfully-pmp.testmo.net \
  --project-id 1 \
  --name "Test Run" \
  --source "backend-unit" \
  --results reports/*.xml \
  --milestone "Sprint 1" \
  --config "Chrome"
```

### 2. `automation:run:create`
Cria um test run para execução paralela.

**Uso:**
```bash
RUN_ID=$(testmo automation:run:create \
  --instance https://hostfully-pmp.testmo.net \
  --project-id 1 \
  --name "Parallel Test Run" \
  --source "e2e-tests")
```

### 3. `automation:run:submit-thread`
Submete um thread para um run existente (para testes paralelos).

**Uso:**
```bash
testmo automation:run:submit-thread \
  --instance https://hostfully-pmp.testmo.net \
  --run-id $RUN_ID \
  --results reports/thread1/*.xml
```

### 4. `automation:run:complete`
Marca um test run como completo.

**Uso:**
```bash
testmo automation:run:complete \
  --instance https://hostfully-pmp.testmo.net \
  --run-id $RUN_ID
```

### 5. `automation:resources:add-field`
Adiciona um campo customizado ao arquivo de recursos.

**Uso:**
```bash
# Campo string/URL
testmo automation:resources:add-field \
  --type string \
  --name "Version" \
  --value "2.3.1-5fbcc8d0"

# Campo console/text/html (via stdin)
cat log.txt | testmo automation:resources:add-field \
  --type console \
  --name "Build Log"
```

### 6. `automation:resources:add-link`
Adiciona um link ao arquivo de recursos.

**Uso:**
```bash
testmo automation:resources:add-link \
  --name "GitHub Repository" \
  --url "https://github.com/user/repo" \
  --note "Source code repository"
```

### 7. `automation:resources:add-artifact`
Adiciona um artifact ao arquivo de recursos.

**Uso:**
```bash
testmo automation:resources:add-artifact \
  --name "Test Report" \
  --url "https://storage.example.com/report.html" \
  --file "local-report.html" \
  --note "HTML test report"
```

## Integração Python

O `TestmoCLIAdapter` fornece uma interface Python para todos os comandos do CLI:

```python
from src.infrastructure.adapters.testmo_cli_adapter import TestmoCLIAdapter
from src.infrastructure.config.settings import Settings

config = Settings().testmo
cli = TestmoCLIAdapter(config)

# Submeter test run simples
result = cli.submit_test_run(
    project_id=1,
    name="API Tests",
    source="api-tests",
    results_files=["reports/junit.xml"],
    milestone="Sprint 1"
)

# Criar run para testes paralelos
run_id = cli.create_test_run(
    project_id=1,
    name="Parallel E2E Tests",
    source="e2e-tests"
)

# Submeter threads
cli.submit_thread(run_id, ["reports/thread1.xml"])
cli.submit_thread(run_id, ["reports/thread2.xml"])

# Completar run
cli.complete_run(run_id)

# Adicionar recursos
cli.add_resource_field("string", "Version", "1.0.0")
cli.add_resource_link("CI/CD", "https://ci.example.com/build/123")
cli.add_resource_artifact("Screenshots", "https://storage.example.com/screenshots.zip")
```

## Casos de Uso

### 1. Testes Unitários Simples
```python
# Executar testes e submeter resultados
cli.submit_test_run(
    project_id=1,
    name="Unit Tests",
    source="unit-tests",
    results_files=["test-results/junit.xml"],
    executable_command=["pytest", "tests/unit/", "--junitxml=test-results/junit.xml"]
)
```

### 2. Testes Paralelos em CI/CD
```python
# Criar run
run_id = cli.create_test_run(
    project_id=1,
    name="CI Pipeline Tests",
    source="ci-tests",
    tags=["ci", "automated"]
)

# Cada job do CI submete seu thread
# Job 1:
cli.submit_thread(run_id, ["reports/job1.xml"])

# Job 2:
cli.submit_thread(run_id, ["reports/job2.xml"])

# Job final completa o run
cli.complete_run(run_id)
```

### 3. Adicionar Contexto aos Testes
```python
# Preparar recursos
cli.add_resource_field("string", "Build Number", "123")
cli.add_resource_field("string", "Git Commit", "abc123")
cli.add_resource_link("Pull Request", "https://github.com/repo/pull/456")
cli.add_resource_artifact("Coverage Report", "https://storage.example.com/coverage.html")

# Submeter com recursos
cli.submit_test_run(
    project_id=1,
    name="Test Run with Context",
    source="full-tests",
    results_files=["reports/junit.xml"],
    resources_file="testmo-resources.json"
)
```

## Vantagens do CLI vs API REST

| Funcionalidade | CLI | API REST |
|---------------|-----|----------|
| Processamento JUnit XML | ✅ Automático | ❌ Manual |
| Execução de testes | ✅ Suportado | ❌ Não |
| Captura de console | ✅ Automático | ❌ Manual |
| Medição de tempo | ✅ Automático | ❌ Manual |
| Testes paralelos | ✅ Nativo | ⚠️ Complexo |
| Custom fields | ✅ Fácil | ⚠️ Manual |
| Artifacts | ✅ Com detecção | ⚠️ Manual |
| Integração CI/CD | ✅ Nativa | ⚠️ Requer código |

## Quando Usar CLI vs API REST

### Use CLI quando:
- ✅ Submetendo resultados de testes automatizados
- ✅ Executando testes e capturando output
- ✅ Trabalhando com testes paralelos
- ✅ Integrando com CI/CD pipelines
- ✅ Adicionando contexto (fields, links, artifacts)

### Use API REST quando:
- ✅ Gerenciando test cases (CRUD)
- ✅ Consultando dados (runs, results, cases)
- ✅ Criando/atualizando folders, milestones
- ✅ Gerenciando usuários e permissões
- ✅ Integrações customizadas

## Próximos Passos

1. **Integrar CLI no fluxo reativo**: Usar CLI para submeter resultados de testes baseados em análise do Splunk
2. **Integrar CLI no fluxo preventivo**: Usar CLI para submeter testes gerados a partir de Jira tickets
3. **Criar serviço de automação**: Wrapper que escolhe automaticamente entre CLI e API REST baseado no caso de uso

