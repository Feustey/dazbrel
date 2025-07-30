# 🚀 GUIDE DE DÉPLOIEMENT SÉCURISÉ

## 🔒 PRÉ-REQUIS DE SÉCURITÉ

Avant de déployer cette application Lightning Network, assurez-vous d'avoir :

- [ ] Docker avec support des secrets
- [ ] Accès aux certificats LND 
- [ ] Variables d'environnement configurées
- [ ] Firewall configuré
- [ ] SSL/TLS certificats (pour production)

## 📋 ÉTAPES DE DÉPLOIEMENT

### 1. 🔑 Configuration des secrets

```bash
# Créer le dossier des secrets
mkdir -p secrets
chmod 700 secrets

# Générer une clé secrète pour JWT
openssl rand -base64 32 > secrets/auth_secret_key.txt

# Ajouter les credentials Bitcoin RPC
echo "your_bitcoin_rpc_user" > secrets/bitcoin_rpc_user.txt
echo "$(openssl rand -base64 16)" > secrets/bitcoin_rpc_pass.txt

# Sécuriser les fichiers
chmod 600 secrets/*
```

### 2. 🐳 Configuration Docker

```bash
# Utiliser la configuration sécurisée
cp docker-compose.secure.yml docker-compose.yml

# Variables d'environnement
export LND_HOST="umbrel.local"
export ENABLE_RATE_LIMITING="true"
export MAX_REQUESTS_PER_MINUTE="30"
export AUTH_TOKEN_TTL_SECONDS="1800"
```

### 3. 🔧 Construction et démarrage

```bash
# Construction de l'image
docker-compose build

# Vérification de la configuration
docker-compose config

# Démarrage sécurisé
docker-compose up -d
```

### 4. ✅ Vérification de sécurité

```bash
# Exécuter les tests de sécurité
./security_tests.sh

# Vérifier les logs
docker-compose logs web | grep -i "security\|auth\|error"

# Health check
curl http://localhost:3000/api/health
```

## 🔐 GÉNÉRATION DE TOKENS D'AUTHENTIFICATION

### Méthode 1: Via l'API interne
```rust
// Dans le code Rust
use crate::middleware::generate_auth_token;
let token = generate_auth_token();
println!("Token: {}", token);
```

### Méthode 2: Script bash
```bash
#!/bin/bash
# generate_token.sh

timestamp=$(date +%s)
secret="your-secret-key-here"
signature=$(echo -n "${timestamp}" | openssl dgst -sha256 -hmac "${secret}" -binary | xxd -p)
token_data="${timestamp}:${signature}"
token=$(echo -n "${token_data}" | base64)
echo "Token: ${token}"
```

## 🌐 UTILISATION DE L'API SÉCURISÉE

### Authentification requise
```bash
# Obtenir des informations sur le nœud
curl -H "Authorization: Bearer YOUR_TOKEN_HERE" \
     http://localhost:3000/api/node/info

# Exécuter une recommandation
curl -X POST \
     -H "Authorization: Bearer YOUR_TOKEN_HERE" \
     -H "Content-Type: application/json" \
     -d '{"recommendation_id": "rec_001", "execution_mode": "auto"}' \
     http://localhost:3000/api/recommendations/auto-execute
```

### Endpoints publics (sans auth)
```bash
# Health check
curl http://localhost:3000/api/health
```

## 🚨 MONITORING ET ALERTES

### 1. Logs de sécurité
```bash
# Surveiller les tentatives d'authentification échouées
docker-compose logs web | grep "Authentication failed"

# Surveiller le rate limiting
docker-compose logs web | grep "Rate limit exceeded"

# Surveiller les tentatives d'injection
docker-compose logs web | grep "SQL injection\|XSS attempt"
```

### 2. Métriques importantes
- Nombre de requêtes authentifiées vs non-authentifiées
- Taux d'échec d'authentification
- Déclenchements de rate limiting
- Tentatives d'injection détectées

### 3. Alertes recommandées
```bash
# Script d'alerte simple
#!/bin/bash
failed_auth=$(docker-compose logs web --since=1m | grep -c "Authentication failed")
if [ $failed_auth -gt 10 ]; then
    echo "ALERTE: $failed_auth tentatives d'authentification échouées dans la dernière minute"
    # Envoyer notification (email, Slack, etc.)
fi
```

## 🔧 CONFIGURATION DE PRODUCTION

### Variables d'environnement recommandées
```bash
# Sécurité renforcée
export RUST_LOG="warn"
export ENABLE_RATE_LIMITING="true" 
export MAX_REQUESTS_PER_MINUTE="10"
export AUTH_TOKEN_TTL_SECONDS="900"  # 15 minutes

# Logging de sécurité
export ENABLE_SECURITY_LOGGING="true"
export LOG_FAILED_AUTH="true"
export LOG_RATE_LIMIT="true"

# Performance
export TOKIO_WORKER_THREADS="4"
```

### Configuration firewall
```bash
# Autoriser uniquement les ports nécessaires
ufw allow 3000/tcp  # Application
ufw allow 22/tcp    # SSH admin
ufw deny by default incoming
ufw allow outgoing
ufw enable
```

### SSL/TLS (obligatoire en production)
```bash
# Avec nginx proxy
server {
    listen 443 ssl;
    server_name your-domain.com;
    
    ssl_certificate /path/to/cert.pem;
    ssl_certificate_key /path/to/key.pem;
    
    location / {
        proxy_pass http://localhost:3000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

## 📊 TESTS DE CHARGE ET SÉCURITÉ

### Test de rate limiting
```bash
# Test avec Apache Bench
ab -n 1000 -c 10 http://localhost:3000/api/health

# Test avec curl en boucle
for i in {1..50}; do
    curl -w "%{http_code}\n" -o /dev/null -s http://localhost:3000/api/health
done
```

### Test d'authentification
```bash
# Test sans token (doit échouer)
curl -w "%{http_code}\n" http://localhost:3000/api/node/info

# Test avec token invalide
curl -H "Authorization: Bearer invalid_token" \
     -w "%{http_code}\n" http://localhost:3000/api/node/info
```

## 🔄 MAINTENANCE DE SÉCURITÉ

### Rotation des secrets (mensuel)
```bash
# Générer nouvelle clé
openssl rand -base64 32 > secrets/auth_secret_key_new.txt

# Mettre à jour la configuration
# Redémarrer l'application
docker-compose restart web

# Supprimer l'ancienne clé
rm secrets/auth_secret_key.txt
mv secrets/auth_secret_key_new.txt secrets/auth_secret_key.txt
```

### Mise à jour des dépendances (hebdomadaire)
```bash
# Vérifier les vulnérabilités
cargo audit

# Mettre à jour les dépendances
cargo update

# Rebuilder l'image
docker-compose build --no-cache
```

### Backup de sécurité
```bash
# Sauvegarder la configuration
tar -czf backup-$(date +%Y%m%d).tar.gz \
    docker-compose.yml \
    secrets/ \
    src/middleware/

# Chiffrer le backup  
gpg -c backup-$(date +%Y%m%d).tar.gz
```

## ⚠️ INCIDENT DE SÉCURITÉ

### En cas de compromission suspectée :

1. **Isolation immédiate**
   ```bash
   docker-compose down
   ufw deny 3000/tcp
   ```

2. **Investigation**
   ```bash
   # Analyser les logs
   docker-compose logs web > security_incident.log
   grep -i "failed\|error\|attack" security_incident.log
   ```

3. **Récupération**
   ```bash
   # Regénérer tous les secrets
   ./regenerate_secrets.sh
   
   # Redéployer avec nouvelle configuration
   docker-compose up -d
   ```

## 📞 CONTACTS DE SÉCURITÉ

- **Équipe de sécurité** : security@dazno.de
- **Incidents** : incident-response@dazno.de  
- **Vulnérabilités** : vulnerability@dazno.de

---

**⚠️ IMPORTANT**: Cette application gère des fonds Lightning Network. Toujours tester en environnement de développement avant la production et effectuer des audits de sécurité réguliers.