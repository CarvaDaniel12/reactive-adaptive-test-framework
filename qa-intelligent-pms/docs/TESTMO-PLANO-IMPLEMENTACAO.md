# Plano de Implementação Completa - Testmo

## ✅ DESCOBERTA IMPORTANTE: Testmo CLI

**Status**: Testmo CLI instalado e adapter criado!

O Testmo CLI oferece funcionalidades que complementam a API REST:
- ✅ Submissão de test runs com JUnit XML (automático)
- ✅ Execução de testes com captura de output
- ✅ Testes paralelos (threads) nativos
- ✅ Custom fields, links e artifacts
- ✅ Integração CI/CD nativa

**Adapter criado**: `TestmoCLIAdapter` em `src/infrastructure/adapters/testmo_cli_adapter.py`

## Status Atual vs. Completo

### ✅ Já Implementado (Completo)
- **Cases**: GET, POST, PATCH, DELETE, find_by_endpoint, find_by_endpoint_and_method, compare, is_identical
- **Projects**: GET (listar), GET (detalhes)
- **Suites**: GET, POST
- **Folders**: GET, POST, PATCH, DELETE (CRUD completo)
- **Milestones**: GET (listar), GET (detalhes)
- **Runs**: GET (listar), GET (detalhes), GET (results)
- **Sessions**: GET (listar), GET (detalhes)
- **Automation Sources**: GET (listar), GET (detalhes)
- **Automation Runs**: GET (listar), GET (detalhes), POST (criar)

### ⚠️ Faltando Implementar (Funcionalidades Avançadas)

## Estrutura de Implementação

### 1. Cases (Test Cases) - 5 endpoints
- ✅ `GET /api/v1/projects/{project_id}/cases` - Listar (implementado, mas falta paginação/filtros)
- ✅ `POST /api/v1/projects/{project_id}/cases` - Criar (implementado básico)
- ✅ `PATCH /api/v1/projects/{project_id}/cases` - Atualizar (implementado básico)
- ✅ `DELETE /api/v1/projects/{project_id}/cases` - Deletar (implementado básico)
- ❌ `GET /api/v1/cases/{case_id}/attachments` - Listar attachments
- ❌ `POST /api/v1/cases/{case_id}/attachments` - Criar attachment
- ❌ `POST /api/v1/cases/{case_id}/attachments/single` - Criar attachment único
- ❌ `DELETE /api/v1/cases/{case_id}/attachments` - Deletar attachments

**Melhorias Necessárias:**
- Adicionar suporte completo a paginação (page, per_page)
- Adicionar filtros (folder_id, template_id, created_by, created_after, created_before)
- Adicionar ordenação (sort, order)
- Adicionar expands (automation_links, comments, folders, history, users, tags, templates)
- Adicionar suporte a custom fields dinâmicos

### 2. Projects - 2 endpoints
- ✅ `GET /api/v1/projects` - Listar (implementado)
- ❌ `GET /api/v1/projects/{project_id}` - Detalhes de um projeto

### 3. Folders - 4 endpoints
- ❌ `GET /api/v1/projects/{project_id}/folders` - Listar folders
- ❌ `POST /api/v1/projects/{project_id}/folders` - Criar folder
- ❌ `PATCH /api/v1/projects/{project_id}/folders` - Atualizar folders
- ❌ `DELETE /api/v1/projects/{project_id}/folders` - Deletar folders

### 4. Milestones - 2 endpoints
- ❌ `GET /api/v1/projects/{project_id}/milestones` - Listar milestones
- ❌ `GET /api/v1/milestones/{milestone_id}` - Detalhes de milestone

### 5. Runs (Test Runs) - 2 endpoints
- ❌ `GET /api/v1/projects/{project_id}/runs` - Listar test runs
- ❌ `GET /api/v1/runs/{run_id}` - Detalhes de test run
- ❌ `GET /api/v1/runs/{run_id}/results` - Listar resultados de um run

### 6. Sessions (Exploratory Testing) - 2 endpoints
- ❌ `GET /api/v1/projects/{project_id}/sessions` - Listar sessões exploratórias
- ❌ `GET /api/v1/sessions/{session_id}` - Detalhes de sessão

### 7. Automation - 7 endpoints
- ❌ `GET /api/v1/projects/{project_id}/automation/sources` - Listar automation sources
- ❌ `GET /api/v1/automation/sources/{automation_source_id}` - Detalhes de automation source
- ❌ `GET /api/v1/projects/{project_id}/automation/runs` - Listar automation runs
- ❌ `POST /api/v1/projects/{project_id}/automation/runs` - Criar automation run
- ❌ `GET /api/v1/automation/runs/{automation_run_id}` - Detalhes de automation run
- ❌ `POST /api/v1/automation/runs/{automation_run_id}/append` - Adicionar recursos a run
- ❌ `POST /api/v1/automation/runs/{automation_run_id}/complete` - Completar automation run
- ❌ `POST /api/v1/automation/runs/{automation_run_id}/threads` - Criar thread
- ❌ `POST /api/v1/automation/runs/threads/{thread_id}/append` - Adicionar a thread
- ❌ `POST /api/v1/automation/runs/threads/{thread_id}/complete` - Completar thread

### 8. Users & Roles - 5 endpoints
- ❌ `GET /api/v1/user` - Usuário atual
- ❌ `GET /api/v1/users` - Listar usuários
- ❌ `GET /api/v1/users/{user_id}` - Detalhes de usuário
- ❌ `GET /api/v1/projects/{project_id}/users` - Usuários do projeto
- ❌ `GET /api/v1/roles` - Listar roles
- ❌ `GET /api/v1/roles/{role_id}` - Detalhes de role

### 9. Groups - 2 endpoints
- ❌ `GET /api/v1/groups` - Listar grupos
- ❌ `GET /api/v1/groups/{group_id}` - Detalhes de grupo

## Priorização de Implementação

### Fase 1: Core Essentials (Alta Prioridade)
1. **Folders** - Organização de test cases
2. **Milestones** - Planejamento de testes
3. **Runs & Results** - Execução e resultados
4. **Melhorias em Cases** - Paginação, filtros, expands

### Fase 2: Automation (Média Prioridade)
5. **Automation Sources & Runs** - Integração com automação
6. **Automation Threads** - Testes paralelos

### Fase 3: Advanced Features (Baixa Prioridade)
7. **Sessions** - Testes exploratórios
8. **Attachments** - Anexos em test cases
9. **Users & Roles** - Gerenciamento de usuários
10. **Groups** - Organização de usuários

## Estrutura de Código Proposta

### Opção 1: Estender TestmoAdapter (Monolítico)
- Adicionar todos os métodos no mesmo arquivo
- Prós: Simples, tudo em um lugar
- Contras: Arquivo muito grande, difícil manutenção

### Opção 2: Serviços Especializados (Recomendado)
```
src/infrastructure/adapters/testmo/
├── testmo_adapter.py (base, autenticação, request)
├── testmo_cases.py (Cases + Attachments)
├── testmo_folders.py (Folders)
├── testmo_milestones.py (Milestones)
├── testmo_runs.py (Runs + Results)
├── testmo_sessions.py (Sessions)
├── testmo_automation.py (Automation)
└── testmo_users.py (Users, Roles, Groups)
```

Cada serviço herda ou usa `TestmoAdapter` base para requests.

## Próximo Passo Imediato

**Vamos começar pela Fase 1:**
1. Melhorar Cases com paginação/filtros/expands
2. Implementar Folders completo
3. Implementar Milestones
4. Implementar Runs & Results

Qual você prefere que eu implemente primeiro?

