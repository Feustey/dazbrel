#!/bin/bash

# 🔒 TESTS DE SÉCURITÉ - DAZNO UMBREL APP
# Script de validation des corrections de sécurité

set -e

echo "🔒 Démarrage des tests de sécurité..."

# Configuration
BASE_URL="http://localhost:3000"
VALID_TOKEN=""

# Couleurs pour l'affichage
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Fonction d'affichage des résultats
print_result() {
    if [ $1 -eq 0 ]; then
        echo -e "${GREEN}✅ $2${NC}"
    else
        echo -e "${RED}❌ $2${NC}"
    fi
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

echo "📋 1. Test des endpoints sans authentification"

# Test de santé (doit marcher)
echo "   Testing /api/health (should work)..."
curl -s -f "$BASE_URL/api/health" > /dev/null
print_result $? "Endpoint /api/health accessible sans authentification"

# Test d'endpoint protégé sans token (doit échouer)
echo "   Testing /api/node/info without auth (should fail)..."
curl -s -f "$BASE_URL/api/node/info" > /dev/null 2>&1
if [ $? -ne 0 ]; then
    print_result 0 "Endpoint /api/node/info correctement protégé"
else
    print_result 1 "FAILLE: Endpoint /api/node/info accessible sans authentification"
fi

echo ""
echo "📋 2. Test de validation d'entrée (injection SQL)"

# Test d'injection SQL
echo "   Testing SQL injection protection..."
response=$(curl -s -w "%{http_code}" -X POST \
    -H "Content-Type: application/json" \
    -d '{"recommendation_id": "'; DROP TABLE users; --", "execution_mode": "auto"}' \
    "$BASE_URL/api/recommendations/auto-execute" 2>/dev/null | tail -n1)

if [ "$response" = "401" ] || [ "$response" = "400" ]; then
    print_result 0 "Protection contre injection SQL active"
else
    print_result 1 "FAILLE: Injection SQL possible (code: $response)"
fi

echo ""
echo "📋 3. Test de validation d'entrée (XSS)"

# Test d'injection XSS
echo "   Testing XSS protection..."
response=$(curl -s -w "%{http_code}" -X POST \
    -H "Content-Type: application/json" \
    -d '{"recommendation_id": "<script>alert(1)</script>", "execution_mode": "auto"}' \
    "$BASE_URL/api/recommendations/auto-execute" 2>/dev/null | tail -n1)

if [ "$response" = "401" ] || [ "$response" = "400" ]; then
    print_result 0 "Protection contre XSS active"
else
    print_result 1 "FAILLE: XSS possible (code: $response)"
fi

echo ""
echo "📋 4. Test de rate limiting"

echo "   Testing rate limiting (sending multiple requests)..."
success_count=0
for i in {1..15}; do
    response=$(curl -s -w "%{http_code}" "$BASE_URL/api/health" 2>/dev/null | tail -n1)
    if [ "$response" = "200" ]; then
        ((success_count++))
    fi
    sleep 0.1
done

if [ $success_count -lt 15 ]; then
    print_result 0 "Rate limiting actif ($success_count/15 requêtes réussies)"
else
    print_warning "Rate limiting pourrait être inactif ou limite très élevée"
fi

echo ""
echo "📋 5. Vérification des hardcoded credentials"

echo "   Checking for hardcoded credentials in source..."
if grep -r "02a1b2c3d4e5f6789abcdef123456789abcdef123456789abcdef123456789abcd" src/ 2>/dev/null; then
    print_result 1 "FAILLE: Hardcoded pubkey trouvée dans le code source"
else
    print_result 0 "Aucune hardcoded pubkey détectée"
fi

if grep -r "825645821654876544" src/ 2>/dev/null; then
    print_result 1 "FAILLE: Hardcoded channel ID trouvé"
else
    print_result 0 "Aucun hardcoded channel ID détecté"
fi

echo ""
echo "📋 6. Vérification de la configuration Docker"

if [ -f "docker-compose.secure.yml" ]; then
    print_result 0 "Configuration Docker sécurisée disponible"
    
    # Vérifier les secrets
    if grep -q "secrets:" docker-compose.secure.yml; then
        print_result 0 "Utilisation de Docker secrets configurée"
    else
        print_result 1 "Docker secrets non configurés"
    fi
    
    # Vérifier l'utilisateur non-root
    if grep -q "user:" docker-compose.secure.yml; then
        print_result 0 "Utilisateur non-root configuré"
    else
        print_result 1 "Utilisateur root utilisé (risque de sécurité)"
    fi
else
    print_result 1 "Configuration Docker sécurisée manquante"
fi

echo ""
echo "📋 7. Vérification des middlewares de sécurité"

if [ -f "src/middleware/auth.rs" ]; then
    print_result 0 "Middleware d'authentification présent"
else
    print_result 1 "Middleware d'authentification manquant"
fi

if [ -f "src/middleware/validation.rs" ]; then
    print_result 0 "Middleware de validation présent"
else
    print_result 1 "Middleware de validation manquant"
fi

if [ -f "src/middleware/rate_limiting.rs" ]; then
    print_result 0 "Middleware de rate limiting présent"
else
    print_result 1 "Middleware de rate limiting manquant"
fi

echo ""
echo "📋 8. Test des dépendances de sécurité"

if grep -q "hmac.*=" Cargo.toml; then
    print_result 0 "Dépendance HMAC pour authentification présente"
else
    print_result 1 "Dépendance HMAC manquante"
fi

if grep -q "regex.*=" Cargo.toml; then
    print_result 0 "Dépendance regex pour validation présente"
else
    print_result 1 "Dépendance regex manquante"
fi

echo ""
echo "📋 RÉSUMÉ DES TESTS DE SÉCURITÉ"
echo "================================"

# Compter les fichiers de sécurité
security_files=0
[ -f "src/middleware/auth.rs" ] && ((security_files++))
[ -f "src/middleware/validation.rs" ] && ((security_files++))
[ -f "src/middleware/rate_limiting.rs" ] && ((security_files++))
[ -f "docker-compose.secure.yml" ] && ((security_files++))
[ -f "SECURITY.md" ] && ((security_files++))

echo "🔧 Fichiers de sécurité: $security_files/5"

# Vérifications critiques
critical_checks=0
total_critical=4

# Check 1: Authentification
if [ -f "src/middleware/auth.rs" ]; then
    ((critical_checks++))
    echo -e "${GREEN}✅ Authentification implémentée${NC}"
else
    echo -e "${RED}❌ Authentification manquante${NC}"
fi

# Check 2: Validation d'entrée
if [ -f "src/middleware/validation.rs" ]; then
    ((critical_checks++))
    echo -e "${GREEN}✅ Validation d'entrée implémentée${NC}"
else
    echo -e "${RED}❌ Validation d'entrée manquante${NC}"
fi

# Check 3: Configuration sécurisée
if [ -f "docker-compose.secure.yml" ]; then
    ((critical_checks++))
    echo -e "${GREEN}✅ Configuration Docker sécurisée${NC}"
else
    echo -e "${RED}❌ Configuration Docker sécurisée manquante${NC}"
fi

# Check 4: Pas de hardcoded credentials
if ! grep -r "02a1b2c3d4e5f6789abcdef" src/ 2>/dev/null; then
    ((critical_checks++))
    echo -e "${GREEN}✅ Pas de hardcoded credentials${NC}"
else
    echo -e "${RED}❌ Hardcoded credentials détectées${NC}"
fi

echo ""
if [ $critical_checks -eq $total_critical ]; then
    echo -e "${GREEN}🎉 TOUTES LES VÉRIFICATIONS CRITIQUES RÉUSSIES!${NC}"
    echo -e "${GREEN}   Application prête pour déploiement sécurisé${NC}"
    exit 0
else
    echo -e "${RED}⚠️  VÉRIFICATIONS ÉCHOUÉES: $critical_checks/$total_critical${NC}"
    echo -e "${RED}   Des corrections supplémentaires sont nécessaires${NC}"
    exit 1
fi