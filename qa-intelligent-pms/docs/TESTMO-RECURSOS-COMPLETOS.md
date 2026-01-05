# Recursos Completos do Testmo - Mapeamento e Implementação

## Estratégia de Exploração

Como o Testmo não tem MCP confiável e a indexação do Cursor não está funcionando, vamos usar uma abordagem sistemática:

1. **Schema OpenAPI**: Baixar e analisar o schema completo
2. **Documentação Web**: Explorar todos os endpoints documentados
3. **Exploração via API**: Testar endpoints diretamente
4. **Mapeamento Completo**: Documentar todos os recursos disponíveis

## Recursos Identificados na Documentação

### 1. Core Resources (Já Implementado Parcialmente)

#### Cases (Test Cases)
- ✅ `GET /api/v1/projects/{project_id}/cases` - Listar
- ✅ `POST /api/v1/projects/{project_id}/cases` - Criar (até 100)
- ✅ `PATCH /api/v1/projects/{project_id}/cases` - Atualizar em massa
- ✅ `DELETE /api/v1/projects/{project_id}/cases` - Deletar
- ⚠️ **Faltando**: Expands, filtros avançados, custom fields

#### Projects
- ✅ `GET /api/v1/projects` - Listar
- ⚠️ **Faltando**: Criar, atualizar, deletar, detalhes

#### Suites (Folders)
- ✅ `GET /api/v1/projects/{project_id}/suites` - Listar
- ✅ `POST /api/v1/suites` - Criar
- ⚠️ **Faltando**: Atualizar, deletar, casos dentro de suites

### 2. Recursos NÃO Implementados (Críticos)

#### Folders
- ❌ `GET /api/v1/projects/{project_id}/folders` - Listar
- ❌ `POST /api/v1/projects/{project_id}/folders` - Criar
- ❌ `PATCH /api/v1/projects/{project_id}/folders/{folder_id}` - Atualizar
- ❌ `DELETE /api/v1/projects/{project_id}/folders/{folder_id}` - Deletar

#### Milestones
- ❌ `GET /api/v1/projects/{project_id}/milestones` - Listar
- ❌ `POST /api/v1/projects/{project_id}/milestones` - Criar
- ❌ `PATCH /api/v1/milestones/{milestone_id}` - Atualizar
- ❌ `DELETE /api/v1/milestones/{milestone_id}` - Deletar

#### Runs (Test Runs)
- ❌ `GET /api/v1/projects/{project_id}/runs` - Listar
- ❌ `POST /api/v1/projects/{project_id}/runs` - Criar
- ❌ `GET /api/v1/runs/{run_id}` - Detalhes
- ❌ `PATCH /api/v1/runs/{run_id}` - Atualizar
- ❌ `DELETE /api/v1/runs/{run_id}` - Deletar

#### Results (Test Results)
- ❌ `GET /api/v1/runs/{run_id}/results` - Listar resultados
- ❌ `POST /api/v1/runs/{run_id}/results` - Criar resultado
- ❌ `PATCH /api/v1/results/{result_id}` - Atualizar resultado
- ❌ `DELETE /api/v1/results/{result_id}` - Deletar resultado

#### Sessions (Exploratory Testing)
- ❌ `GET /api/v1/projects/{project_id}/sessions` - Listar sessões
- ❌ `POST /api/v1/projects/{project_id}/sessions` - Criar sessão
- ❌ `GET /api/v1/sessions/{session_id}` - Detalhes
- ❌ `PATCH /api/v1/sessions/{session_id}` - Atualizar
- ❌ `DELETE /api/v1/sessions/{session_id}` - Deletar

#### Automation Sources
- ❌ `GET /api/v1/projects/{project_id}/automation/sources` - Listar
- ❌ `POST /api/v1/projects/{project_id}/automation/sources` - Criar
- ❌ `GET /api/v1/automation/sources/{source_id}` - Detalhes
- ❌ `PATCH /api/v1/automation/sources/{source_id}` - Atualizar
- ❌ `DELETE /api/v1/automation/sources/{source_id}` - Deletar

#### Automation Runs
- ❌ `GET /api/v1/automation/sources/{source_id}/runs` - Listar
- ❌ `POST /api/v1/automation/sources/{source_id}/runs` - Criar
- ❌ `GET /api/v1/automation/runs/{run_id}` - Detalhes
- ❌ `PATCH /api/v1/automation/runs/{run_id}` - Atualizar

#### Attachments
- ❌ `GET /api/v1/cases/{case_id}/attachments` - Listar
- ❌ `POST /api/v1/cases/{case_id}/attachments` - Upload
- ❌ `GET /api/v1/attachments/{attachment_id}` - Download
- ❌ `DELETE /api/v1/attachments/{attachment_id}` - Deletar

#### Users & Roles
- ❌ `GET /api/v1/users` - Listar usuários
- ❌ `GET /api/v1/users/{user_id}` - Detalhes
- ❌ `GET /api/v1/roles` - Listar roles
- ❌ `GET /api/v1/user` - Usuário atual

#### Groups
- ❌ `GET /api/v1/groups` - Listar grupos
- ❌ `POST /api/v1/groups` - Criar grupo
- ❌ `GET /api/v1/groups/{group_id}` - Detalhes

### 3. Funcionalidades Avançadas

#### Custom Fields
- ⚠️ **Parcial**: Usamos `custom_priority`, `custom_description`, `custom_steps`
- ❌ **Faltando**: Listar campos customizados disponíveis
- ❌ **Faltando**: Criar/atualizar campos customizados

#### Expands (Relacionamentos)
- ⚠️ **Parcial**: Sabemos que existem (automation_links, comments, folders, history, users, tags, templates)
- ❌ **Faltando**: Implementar uso de expands em todas as queries

#### Pagination
- ⚠️ **Parcial**: Sabemos que existe (page, per_page)
- ❌ **Faltando**: Implementar paginação automática em todas as listagens

#### Filtros Avançados
- ⚠️ **Parcial**: Usamos `project_id`, `suite_id`
- ❌ **Faltando**: `folder_id`, `template_id`, `created_by`, `created_after`, `created_before`

#### Ordenação
- ❌ **Faltando**: `sort`, `order` (asc/desc)

## Próximos Passos

1. **Baixar Schema OpenAPI**: Tentar baixar do Google Drive
2. **Explorar Endpoints**: Testar cada endpoint documentado
3. **Estender TestmoAdapter**: Implementar todos os recursos
4. **Criar Serviços Especializados**: 
   - `TestRunService` - Para gerenciar test runs
   - `TestResultService` - Para resultados
   - `SessionService` - Para sessões exploratórias
   - `AutomationService` - Para automação
   - `FolderService` - Para organização
   - `MilestoneService` - Para planejamento

## Schema OpenAPI

Link: https://drive.google.com/file/d/1t9JfZzagR77hrAbEi872Qe7FpXc2qlKs/view?usp=sharing

**Ação**: Baixar e analisar o schema completo para mapear TODOS os endpoints disponíveis.

