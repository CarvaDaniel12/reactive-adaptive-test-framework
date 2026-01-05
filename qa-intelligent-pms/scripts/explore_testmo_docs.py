"""
Script para explorar e mapear TODOS os recursos do Testmo mencionados na documentação.
Não apenas os endpoints da API, mas também funcionalidades da plataforma.
"""

import json
from typing import Dict, List, Set

# Recursos identificados na documentação web (além dos 30 endpoints da API)

TESTMO_RESOURCES = {
    "API Endpoints (OpenAPI Schema)": {
        "Cases": ["GET", "POST", "PATCH", "DELETE", "Attachments"],
        "Projects": ["GET"],
        "Folders": ["GET", "POST", "PATCH", "DELETE"],
        "Milestones": ["GET"],
        "Runs": ["GET"],
        "Results": ["GET"],
        "Sessions": ["GET"],
        "Automation Sources": ["GET"],
        "Automation Runs": ["GET", "POST", "Append", "Complete", "Threads"],
        "Users": ["GET"],
        "Roles": ["GET"],
        "Groups": ["GET"],
    },
    
    "Platform Features (Não na API REST)": {
        "Test Case Repository": [
            "Test Case Steps",
            "BDD & Gherkin",
            "Copy & Move Cases",
            "Templates",
            "Custom Fields",
            "States & Statuses",
        ],
        "Test Execution": [
            "Test Runs",
            "Manual Testing",
            "Exploratory Sessions",
            "Screenshots",
            "Test Results",
        ],
        "Automation": [
            "Automation Jobs",
            "Automation Linking",
            "Parallel Testing",
            "CI/CD Integration",
            "Testmo CLI",
            "XML Format",
        ],
        "Planning & Organization": [
            "Milestones",
            "Folders",
            "Forecasting",
            "Requirements Coverage",
            "Traceability",
        ],
        "Reporting": [
            "Reporting Center",
            "Team Testing Workload",
            "Automation Metrics",
            "Milestone Summary & Metrics",
            "Requirements Coverage & Traceability",
        ],
        "Integrations": {
            "Issue Trackers": [
                "Jira Cloud",
                "Jira Server/DC",
                "GitHub Issues",
                "GitLab Issues",
                "Azure DevOps",
                "Linear",
                "Asana",
                "Trello",
                "Monday.com",
                "ClickUp",
                "Redmine",
                "YouTrack",
                "Trac",
                "Shortcut",
                "Custom Links",
            ],
            "CI/CD": [
                "GitHub Actions",
                "GitLab CI/CD",
                "Jenkins",
                "CircleCI",
                "Bitbucket",
                "Azure Pipelines",
                "Custom CI",
            ],
            "Automation Tools": [
                "Postman",
                "Playwright",
                "Selenium",
                "Cypress",
                "Pytest",
                "JUnit",
                "TestNG",
                "RSpec",
                "Jest",
                "Mocha",
                "Cucumber",
                "Robot Framework",
                "Appium",
                "BrowserStack",
                "Sauce Labs",
                "WebdriverIO",
                "xUnit.net",
                "NUnit",
                "PHPUnit",
                "XCTest",
                "Nightwatch.js",
                "Jasmine",
                "GoogleTest",
                "Catch",
                "Minitest",
                "Testim",
                "Ranorex",
                "More Tools",
            ],
            "Enterprise Auth": [
                "Google",
                "Azure AD",
                "Okta",
                "OneLogin",
                "Custom SAML",
            ],
        },
        "Administration": [
            "Users & Permissions",
            "Groups",
            "Roles",
            "Custom Fields",
            "Data Exports",
            "Authentication",
        ],
        "Migrations": [
            "Importing TestRail",
            "Importing CSV/Excel",
            "Migration Tips",
        ],
    },
}

def generate_summary():
    """Gera resumo completo dos recursos"""
    
    print("=" * 80)
    print("MAPEAMENTO COMPLETO DE RECURSOS DO TESTMO")
    print("=" * 80)
    print()
    
    # Contar recursos da API
    api_endpoints = 0
    for resource, methods in TESTMO_RESOURCES["API Endpoints (OpenAPI Schema)"].items():
        if isinstance(methods, list):
            api_endpoints += len(methods)
    
    print(f"API REST (OpenAPI Schema): {api_endpoints} operacoes documentadas")
    print(f"   (Mas apenas 30 endpoints HTTP reais)")
    print()
    
    # Contar features da plataforma
    platform_features = 0
    for category, features in TESTMO_RESOURCES["Platform Features (Não na API REST)"].items():
        if isinstance(features, dict):
            for subcat, items in features.items():
                platform_features += len(items) if isinstance(items, list) else 1
        elif isinstance(features, list):
            platform_features += len(features)
    
    print(f"Features da Plataforma: {platform_features}+ funcionalidades")
    print(f"   (Muitas não expostas via API REST)")
    print()
    
    print("=" * 80)
    print("RECURSOS NÃO COBERTOS PELA API REST")
    print("=" * 80)
    print()
    
    for category, features in TESTMO_RESOURCES["Platform Features (Não na API REST)"].items():
        print(f"\n[{category}]:")
        if isinstance(features, dict):
            for subcat, items in features.items():
                print(f"   - {subcat}:")
                for item in items:
                    print(f"     * {item}")
        elif isinstance(features, list):
            for item in features:
                print(f"   * {item}")
    
    print()
    print("=" * 80)
    print("CONCLUSÃO")
    print("=" * 80)
    print()
    print("A API REST do Testmo cobre apenas uma parte das funcionalidades.")
    print("Muitas features são acessíveis apenas via interface web:")
    print("  - Reporting Center")
    print("  - Forecasting")
    print("  - Data Exports")
    print("  - Custom Fields (configuração)")
    print("  - Integrações (configuração)")
    print()
    print("Para acessar essas funcionalidades, você precisaria:")
    print("  1. Usar a interface web do Testmo")
    print("  2. Usar o Testmo CLI (para automação)")
    print("  3. Fazer web scraping (não recomendado)")
    print("  4. Solicitar endpoints adicionais ao Testmo")
    print()

if __name__ == "__main__":
    generate_summary()
    
    # Salvar em JSON para referência
    with open("data/testmo-resources-complete.json", "w", encoding="utf-8") as f:
        json.dump(TESTMO_RESOURCES, f, indent=2, ensure_ascii=False)
    
    print("OK - Mapeamento salvo em: data/testmo-resources-complete.json")

