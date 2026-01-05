#!/bin/bash
# Script de setup inicial

echo "Configurando ambiente..."

# Criar diretórios necessários
mkdir -p data/tickets
mkdir -p data/test_cases
mkdir -p data/patterns
mkdir -p data/sessions
mkdir -p screenshots
mkdir -p videos
mkdir -p logs
mkdir -p cache

# Copiar arquivos de configuração exemplo
if [ ! -f .env ]; then
    cp .env.example .env
    echo "Arquivo .env criado. Por favor, edite com suas credenciais."
fi

if [ ! -f configs/jira_config.yaml ]; then
    cp configs/jira_config.yaml.example configs/jira_config.yaml
    echo "Arquivo jira_config.yaml criado. Por favor, edite com suas configurações."
fi

if [ ! -f configs/splunk_config.yaml ]; then
    cp configs/splunk_config.yaml.example configs/splunk_config.yaml
    echo "Arquivo splunk_config.yaml criado. Por favor, edite com suas configurações."
fi

if [ ! -f configs/postman_config.yaml ]; then
    cp configs/postman_config.yaml.example configs/postman_config.yaml
    echo "Arquivo postman_config.yaml criado. Por favor, edite com suas configurações."
fi

echo "Setup concluído!"

