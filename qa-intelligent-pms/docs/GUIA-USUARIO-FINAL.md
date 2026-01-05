# Guia do Usuário Final - Framework de QA Inteligente

## Visão Geral

Este framework ajuda QAs a priorizar testes baseado em métricas reais de produção. Você não precisa ser desenvolvedor para usar - tudo é feito através de uma interface web simples e visual.

## Como Funciona

1. **Você exporta dados do Splunk** (manual, no navegador do Splunk)
2. **Faz upload do arquivo CSV** na interface web
3. **Clica em "Processar Métricas"** e aguarda
4. **Visualiza resultados completos** diretamente na interface
5. **Decide quais endpoints testar** baseado nas recomendações

**Tudo sem usar terminal ou comandos!**

## Passo a Passo Completo

### Passo 1: Exportar Dados do Splunk

1. **Acesse o Splunk** no seu navegador
2. **Execute a query padrão** (copie do guia abaixo)
3. **Exporte como CSV**:
   - Clique em "Export" → "CSV"
   - **IMPORTANTE**: Escolha:
     - Format: **Events** (não Statistics)
     - Mode: **Verbose Mode** (não Smart Mode)
   - Salve o arquivo no seu computador

**Query Padrão** (copie e cole no Splunk):
```
index=main earliest=-6h@h latest=now() sourcetype=logs source=*localhost_access_log*
| rex field=_raw "\"(?<method>[A-Z]+)\s+(?<path>[^\s\"]+)\s+HTTP/1\.1\"\s+(?<status>\d+)" 
| eval is_4xx = if(status>=400 AND status<500, 1, 0)
| eval is_5xx = if(status>=500, 1, 0)
| eval has_error_keyword = if(
    match(_raw, "(?i)\\b(ERROR|FAIL|FAILED|EXCEPTION|TIMEOUT|FATAL|CRITICAL|UNAVAILABLE|REJECTED|DENIED|FORBIDDEN|UNAUTHORIZED|BAD_REQUEST|INTERNAL_ERROR|SERVICE_UNAVAILABLE|GATEWAY_TIMEOUT|CONNECTION_REFUSED|OUT_OF_MEMORY|STACK_TRACE|THROWABLE)\\b"), 1, 0)
| eval is_error = if(is_4xx=1 OR is_5xx=1 OR has_error_keyword=1, 1, 0)
| stats count as total_requests,
         sum(is_error) as total_errors,
         sum(is_4xx) as client_errors_4xx,
         sum(is_5xx) as server_errors_5xx,
         sum(has_error_keyword) as keyword_errors,
         avg(response_time) as avg_response_time,
         max(response_time) as max_response_time,
         min(response_time) as min_response_time,
         dc(client_ip) as unique_clients
         by path
| rename path as endpoint
| eval error_rate = round((total_errors/total_requests)*100, 2)
| eval importance_score = (total_requests * 0.4) + (total_errors * 0.6)
| eval is_critical = if((error_rate > 5 AND total_requests > 100) OR server_errors_5xx > 0, 1, 0)
| sort -importance_score
```

**Veja guia completo**: [GUIA-EXPORTACAO-SPLUNK.md](GUIA-EXPORTACAO-SPLUNK.md)

### Passo 2: Iniciar a Interface Web

**Primeira vez (configuração inicial)**:
- Alguém com acesso técnico precisa iniciar o servidor uma vez
- Depois disso, a interface fica disponível para todos

**Como iniciar** (apenas uma vez, ou quando o servidor parar):
- Peça para alguém técnico executar: `python src/presentation/web_app.py`
- Ou configure para iniciar automaticamente (futuro)

**Acessar a interface**:
- Abra seu navegador
- Acesse: `http://localhost:5000`
- A interface web aparece automaticamente

### Passo 3: Fazer Upload e Processar

1. **Na interface web, você verá**:
   - Uma área grande com bordas tracejadas
   - Texto: "Clique ou arraste o arquivo CSV aqui"

2. **Fazer upload do arquivo**:
   - **Opção A**: Arraste o arquivo CSV da pasta do seu computador e solte na área tracejada
   - **Opção B**: Clique na área tracejada e escolha o arquivo no explorador de arquivos

3. **Após selecionar o arquivo**:
   - O nome do arquivo aparece na área
   - Um botão "Processar Métricas" aparece abaixo

4. **Clicar em "Processar Métricas"**:
   - Uma barra de progresso aparece mostrando o progresso
   - Você verá mensagens como "Enviando arquivo...", "Processando métricas...", "Concluído!"
   - Aguarde até a barra chegar a 100%

### Passo 4: Ver Resultados na Interface

Após o processamento, a interface mostra automaticamente:

#### 4.1. Métricas Gerais (Cards no Topo)
- **Total de Requisições**: Quantas requisições foram analisadas
- **Total de Erros**: Quantos erros foram encontrados
- **Taxa de Erro**: Porcentagem de requisições com erro
- **Endpoints Únicos**: Quantos endpoints diferentes foram encontrados
- **Endpoints Críticos**: Quantos endpoints precisam atenção urgente

#### 4.2. Endpoints Prioritários para Teste (Tabela)
Uma tabela mostrando:
- **Endpoint**: Caminho da API (ex: `/api/v3/quotes`)
- **Método**: HTTP method (GET, POST, etc.)
- **Prioridade**: Score de 0-100 (quanto maior, mais importante testar)
- **Taxa de Erro**: Porcentagem de erros neste endpoint
- **Recomendação**: O que fazer (ex: "Reproduzir erros", "Investigar degradação")

**Como usar**: Foque nos endpoints com maior prioridade (score alto) e alta taxa de erro.

#### 4.3. Tendências (Se Houver Análise Anterior)
Se você já processou um arquivo antes, a interface mostra:
- **Endpoints Degradando** (vermelho): Pioraram desde a última análise
- **Endpoints Melhorando** (verde): Melhoraram desde a última análise
- **Mudança Percentual**: Quanto piorou/melhorou

**Como usar**: Priorize investigar endpoints que estão degradando.

#### 4.4. Gaps de Cobertura
Lista de endpoints críticos que ainda não têm testes identificados.

**Como usar**: Use para planejar quais testes criar na próxima sprint.

#### 4.5. Riscos de Regressão
Endpoints que podem quebrar em futuras mudanças.

**Como usar**: Adicione estes endpoints aos testes de regressão.

#### 4.6. Link para Relatório HTML Completo
No topo dos resultados, há um link "Ver Relatório HTML Completo".

**Como usar**: Clique para abrir um relatório detalhado em nova aba, útil para compartilhar com o time ou salvar.

#### 4.7. Buscar Matches no Postman
Após processar, você pode buscar endpoints correspondentes no Postman:

1. **Clique no botão "🔍 Buscar Matches no Postman"**
2. **Aguarde a busca** (pode levar alguns segundos)
3. **Veja os matches encontrados**:
   - ✅ **Verde (Idêntico)**: Test case já existe no Testmo e é idêntico
   - ⚠️ **Amarelo (Diferente)**: Test case existe mas tem diferenças
   - ℹ️ **Azul (Novo)**: Test case não existe ainda

**Para cada match, você verá**:
- Nome do request no Postman
- Método e endpoint
- Collection do Postman
- CURL command (clique em "Ver CURL")
- Comparação com Testmo (se existir)

#### 4.8. Editar Nomes dos Test Cases
Antes de sincronizar com Testmo, você pode editar os nomes sugeridos:

1. **Selecione os test cases** que deseja sincronizar (checkboxes)
2. **Seção "✏️ Editar Nomes dos Test Cases" aparece automaticamente**
3. **Edite os nomes** se necessário:
   - Nome sugerido aparece automaticamente
   - Formato: `METHOD_TestType_Description` (ex: `POST_CreateQuote_ValidRequest`)
   - Validação em tempo real (verde = válido, vermelho = inválido)
4. **Clique em "🔄 Restaurar nome sugerido"** se quiser voltar ao padrão

**Regras de nomenclatura**:
- Deve seguir formato: `METHOD_TestType_Description`
- Exemplos válidos: `POST_CreateQuote_ValidRequest`, `GET_GetQuote_NotFound`
- Exemplos inválidos: `Create Quote Test`, `POST-Create-Quote`

#### 4.9. Sincronizar com Testmo
Após selecionar e editar nomes, sincronize com Testmo:

1. **Selecione os test cases** desejados (checkboxes)
2. **Edite nomes** se necessário (seção aparece automaticamente)
3. **Informe o Project ID do Testmo** (número do projeto)
4. **Clique em "✅ Sincronizar com Testmo"**
5. **Aguarde o processamento**

**O que acontece**:
- **Novos**: Test cases são criados no Testmo
- **Idênticos**: São reutilizados (nada acontece)
- **Diferentes**: Você pode escolher atualizar ou apenas reutilizar
- **Herança**: Se existe caso similar no Base, ele é herdado para Reativo

**Resultado**:
- Estatísticas mostram quantos foram criados, atualizados, reutilizados
- Informações para teste manual (CURL, endpoints, etc.)
- Links para test cases no Testmo

**Estrutura criada no Testmo**:
- **Repositório Reativo**: `Reativo/{Data}_{Prioridade}_{Tendência}/{Endpoint}/`
- **Repositório Base**: `Base/{Componente}/{Endpoint}/` (se herdado)

### Passo 5: Processar Arquivos Já Enviados

Se você já fez upload de arquivos antes:

1. **Na interface, você verá uma seção "Arquivos Disponíveis"**
2. **Lista de arquivos CSV** que já estão no sistema
3. **Botão "Processar"** ao lado de cada arquivo
4. **Clique em "Processar"** para reprocessar qualquer arquivo

**Útil para**: Comparar diferentes períodos ou reprocessar com novas análises.

### Passo 6: Interpretar e Decidir (Humano no Loop)

Após ver os resultados, você decide:

1. **Quais endpoints testar primeiro?**
   - Use a tabela de prioridades como guia
   - Endpoints com score alto (80+) e alta taxa de erro são os mais importantes
   - Ajuste baseado no seu conhecimento de negócio

2. **Quais são falsos positivos?**
   - Endpoints legados que não precisam de teste
   - Endpoints internos de baixa prioridade
   - Anote mentalmente ou em um documento

3. **O que fazer com as recomendações?**
   - **Criar testes no Postman?** → Use a lista de prioridades
   - **Criar tickets no Jira?** → Foque em endpoints críticos degradando
   - **Investigar erros?** → Priorize endpoints com 5XX (erros de servidor)
   - **Compartilhar com time?** → Use o relatório HTML

## Exemplo de Uso Real

### Segunda-feira - Primeira Análise

1. Exportei do Splunk: `metricas_segunda_6h.csv`
2. Abri interface web: `http://localhost:5000`
3. Arrastei o arquivo para a área de upload
4. Cliquei em "Processar Métricas"
5. Aguardei barra de progresso chegar a 100%
6. **Resultado**: Interface mostrou 218 endpoints críticos
7. **Decisão**: Vou focar nos top 5 desta semana

### Quinta-feira - Segunda Análise (Comparação)

1. Exportei do Splunk: `metricas_quinta_6h.csv`
2. Fiz upload na interface web
3. Processei o arquivo
4. **Resultado**: 
   - Interface mostrou automaticamente comparação com segunda-feira
   - `/api/v3/quotes` melhorou 15% (verde)
   - `/channels/google/query` degradou 8% (vermelho)
5. **Decisão**: Vou investigar o endpoint que degradou

### Sexta-feira - Relatório Semanal

1. Processei arquivo da semana
2. Cliquei em "Ver Relatório HTML Completo"
3. Salvei o HTML
4. Enviei para o time
5. Discutimos prioridades na reunião

## Perguntas Frequentes

### "Onde encontro a query do Splunk?"
Veja: [GUIA-EXPORTACAO-SPLUNK.md](GUIA-EXPORTACAO-SPLUNK.md) - seção "Query Padrão Validada"

### "Preciso instalar algo?"
Apenas uma vez (configuração inicial):
1. Python 3.9+ instalado
2. Execute: `pip install -r requirements.txt`
3. Configure `.env` com credenciais (se usar Jira/Postman)

**Depois disso, você só usa a interface web!**

### "Posso usar em Windows/Mac/Linux?"
Sim! A interface web funciona em qualquer sistema operacional e navegador.

### "Como compartilhar resultados com o time?"
1. Clique em "Ver Relatório HTML Completo" na interface
2. Salve o HTML ou envie por email
3. Ou tire screenshots das seções importantes

### "E se eu não souber usar terminal?"
**Você não precisa!** Tudo é feito na interface web:
- Upload: arrastar e soltar arquivo
- Processar: clicar em botão
- Ver resultados: tudo aparece na tela
- Relatório: clicar em link

### "Como processar o mesmo arquivo novamente?"
Na seção "Arquivos Disponíveis", clique no botão "Processar" ao lado do arquivo desejado.

### "A interface não abre, o que fazer?"
1. Verifique se o servidor está rodando (peça ajuda técnica)
2. Verifique se está acessando `http://localhost:5000`
3. Tente atualizar a página (F5)

## Funcionalidades Avançadas

### Integração com Postman

O sistema busca automaticamente endpoints no Postman e sugere test cases:

1. **Busca automática**: Compara endpoints do Splunk com requests no Postman
2. **Informações completas**: Extrai request body, headers, CURL commands
3. **Sugestões inteligentes**: Gera test cases baseados nos requests encontrados

### Integração com Testmo

O sistema gerencia test cases no Testmo com estrutura organizada:

#### Estrutura de Repositórios

**Repositório Base** (Reutilizável):
- Organizado por componente
- Estrutura: `Base/{Componente}/{METHOD}_{Endpoint}/{METHOD}_{TestType}_{Description}`
- Exemplo: `Base/Booking/POST_api-v3-quotes/POST_CreateQuote_ValidRequest`

**Repositório Reativo** (Sprint-based):
- Organizado por criticidade/tendência
- Estrutura: `Reativo/{Data}_{Prioridade}_{Tendência}/{METHOD}_{Endpoint}/`
- Exemplo: `Reativo/2025-01-15_Critical_Degrading/POST_api-v3-quotes/`

**Herança de Casos**:
- Casos do Reativo podem herdar estrutura de casos do Base
- Mantém link entre casos base e reativos
- Adiciona contexto específico (data, prioridade, métricas)

**Merge ao Final da Sprint**:
- Casos úteis do Reativo são migrados para Base
- Estrutura reativa é deletada após merge
- Base é atualizado com novos conhecimentos

**Documentação completa**: Veja [TESTMO-NOMENCLATURA-ESTRUTURA.md](TESTMO-NOMENCLATURA-ESTRUTURA.md)

### Validação e Normalização

O sistema normaliza automaticamente:
- **Componentes**: "Booking Service" → "Booking"
- **Endpoints**: `/api/v3/quotes/` → `/api/v3/quotes`
- **Nomes**: Gera nomes seguindo convenções parseáveis

### Cache Persistente

Análises são salvas automaticamente:
- Cache em arquivo JSON
- Recuperação automática ao reiniciar
- Histórico de análises processadas

## Suporte

Se tiver dúvidas:
1. Consulte este guia
2. Veja [GUIA-EXPORTACAO-SPLUNK.md](GUIA-EXPORTACAO-SPLUNK.md) para detalhes do Splunk
3. Peça ajuda para alguém técnico se a interface não funcionar
