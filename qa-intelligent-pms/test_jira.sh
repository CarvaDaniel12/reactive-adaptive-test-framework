#!/bin/bash
# Teste de conexão Jira - precisa de JIRA_URL e JIRA_EMAIL

JIRA_URL="${JIRA_URL:-https://EMPRESA.atlassian.net}"
JIRA_EMAIL="${JIRA_EMAIL:-email@empresa.com}"
JIRA_TOKEN="ATATT3xFfGF082vNLxCQzdZnhG6VF0Mb-x6xzWx2TmAoIyIVBXeaHqnqhF9fZejYqwtjxdfb_A9BATrASq2mM1KI4WDH6d3hU9xrWx1tYl3zRJIv5eWDbiANBSAK2RBPWQIY5LxbaUx-pGETPBEtpytu9g1FMGhV7n-t1PardH72V7uZQjeq6VA=5DE7FCAA"

echo "Testing Jira connection..."
echo "URL: $JIRA_URL"
echo "Email: $JIRA_EMAIL"
echo ""

curl -s -w "\nHTTP Status: %{http_code}\n" \
  --url "${JIRA_URL}/rest/api/3/myself" \
  --user "${JIRA_EMAIL}:${JIRA_TOKEN}" \
  --header 'Accept: application/json'
