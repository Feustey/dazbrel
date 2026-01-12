# 🚀 Dazno Umbrel - Guide de Déploiement

## Configuration pour Umbrel Local (umbrel.local)

### Phase 1 : Préparation de l'environnement

#### 1. Accès aux certificats et macaroons LND

Sur votre Umbrel (192.168.0.29), les fichiers nécessaires sont situés à :

```bash
# Certificat TLS LND
/home/umbrel/umbrel/app-data/lightning/data/lnd/tls.cert

# Macaroon admin LND  
/home/umbrel/umbrel/app-data/lightning/data/lnd/data/chain/bitcoin/mainnet/admin.macaroon

# Alternative pour le macaroon readonly (recommandé pour la sécurité)
/home/umbrel/umbrel/app-data/lightning/data/lnd/data/chain/bitcoin/mainnet/readonly.macaroon
```

#### 2. Copie des certificats sur votre machine de développement

```bash
# Créer le dossier pour les certificats
mkdir -p ./lnd-credentials

# Copier le certificat TLS
scp umbrey2@192.168.0.29:/home/umbrel/umbrel/app-data/lightning/data/lnd/tls.cert ./lnd-credentials/

# Copier le macaroon admin (ou readonly pour plus de sécurité)
scp umbrey2@192.168.0.29:/home/umbrel/umbrel/app-data/lightning/data/lnd/data/chain/bitcoin/mainnet/admin.macaroon ./lnd-credentials/
```

### Phase 2 : Configuration de l'application

#### 1. Variables d'environnement

Créez un fichier `.env` :

```bash
# Configuration LND Umbrel
LND_GRPC_URI=https://192.168.0.29:10009
LND_TLS_CERT_PATH=./lnd-credentials/tls.cert
LND_MACAROON_PATH=./lnd-credentials/admin.macaroon

# Configuration MCP API
MCP_API_URL=https://api.dazno.de

# Optionnel : Clé API MCP
# MCP_API_KEY=your_api_key_here
```

#### 2. Mise à jour du docker-compose.yml

```yaml
version: "3.7"

services:
  app_proxy:
    environment:
      APP_HOST: dazno_web_1
      APP_PORT: 3000

  web:
    build: .
    restart: on-failure
    environment:
      # Configuration LND
      - LND_GRPC_URI=https://192.168.0.29:10009
      - LND_TLS_CERT_PATH=/lnd-credentials/tls.cert
      - LND_MACAROON_PATH=/lnd-credentials/admin.macaroon
      
      # Configuration MCP
      - MCP_API_URL=https://api.dazno.de
      
    volumes:
      # Monter les certificats LND
      - ./lnd-credentials:/lnd-credentials:ro
      - ./data:/app/data
    networks:
      default:
        ipv4_address: $APP_DAZNO_IP
    ports:
      - "3000:3000"
```

### Phase 3 : Test de la connexion

#### 1. Test en mode développement

```bash
# Charger les variables d'environnement
source .env

# Lancer l'application
cargo run
```

#### 2. Vérification des endpoints

```bash
# Test de santé de l'application
curl http://localhost:3000/api/health

# Information du nœud Lightning
curl http://localhost:3000/api/node/info

# Liste des canaux
curl http://localhost:3000/api/node/channels

# Interface utilisateur avancée
open http://localhost:3000/superior
```

### Phase 4 : Intégration dans Umbrel

#### 1. Structure des fichiers pour Umbrel App Store

```
dazno/
├── umbrel-app.yml          # Métadonnées de l'app
├── docker-compose.yml      # Configuration Docker
├── Dockerfile             # Image de l'application
└── data/                  # Données persistantes
```

#### 2. Configuration pour l'App Store Umbrel

Le fichier `umbrel-app.yml` est déjà configuré pour :
- Dépendances : `lightning-node`, `lightning-terminal`, `electrs`, `bitcoin`
- Port : 3000
- Catégorie : Lightning

## Configuration Portainer (installation locale)

Le fichier `docker-compose.portainer.yml` est prêt pour un déploiement via Portainer.

### 1. Variables à renseigner dans Portainer

Dans l’onglet **Env** (ou via un fichier `.env`), définissez au minimum :

```bash
# Chemins hôte vers vos données Umbrel/Bitcoin
LND_DATA_DIR=/home/umbrel/umbrel/app-data/lightning/data/lnd
BITCOIN_DATA_DIR=/home/umbrel/umbrel/app-data/bitcoin/data/bitcoin

# Identifiants RPC Bitcoin (si requis par votre nœud)
BITCOIN_RPC_USER=bitcoinrpc
BITCOIN_RPC_PASS=change_me

# Optionnel
MCP_API_URL=https://api.dazno.de
MCP_API_KEY=
LND_HOST=umbrel.local
```

### 2. Stack Portainer

Copiez-collez le contenu de `docker-compose.portainer.yml` dans **Stacks** puis déployez.

L’application sera disponible sur :

```
http://<ip_du_serveur>:3000
```

### Phase 5 : Sécurité et bonnes pratiques

#### 1. Utilisation du macaroon readonly (recommandé)

Pour un déploiement en production, utilisez le macaroon readonly :

```bash
LND_MACAROON_PATH=./lnd-credentials/readonly.macaroon
```

#### 2. Restrictions réseau

L'application est configurée pour :
- Accès local uniquement aux données Umbrel
- Communications externes limitées à l'API MCP
- Pas d'exposition de données sensibles

### Phase 6 : Monitoring et logs

#### 1. Logs de l'application

```bash
# Suivre les logs en temps réel
docker-compose logs -f web

# Logs spécifiques à la connexion LND
grep "Lightning" logs/app.log
```

#### 2. Métriques disponibles

- Status de connexion LND : `/api/health`
- Informations du nœud : `/api/node/info`
- Données des canaux : `/api/node/channels`
- WebSocket temps réel : `ws://localhost:3000/ws/realtime`

## 🎯 Fonctionnalités activées

### ✅ Intégration locale Umbrel
- Connexion directe au LND local
- Lecture des données Lightning Terminal
- Accès aux données Electrs et Bitcoin Core
- Performance optimale (pas de latence réseau)

### ✅ Sécurité maximale
- Exécution 100% locale
- Pas d'exposition des macaroons
- Communications chiffrées TLS

### ✅ Interface utilisateur avancée
- Dashboard temps réel
- Analyse de performance
- Recommandations IA
- Comparaison avec Amboss

### ✅ API complète
- RESTful endpoints
- WebSocket pour temps réel
- Intégration MCP pour recommandations

## 🚨 Dépannage

### Problème : "TLS certificate not found"
- Vérifiez le chemin du certificat
- Assurez-vous que le fichier est lisible
- Vérifiez les permissions du fichier

### Problème : "Macaroon not found"  
- Vérifiez le chemin du macaroon
- Utilisez le bon macaroon (admin ou readonly)
- Vérifiez les permissions

### Problème : "Connection refused"
- Vérifiez que LND est en cours d'exécution
- Vérifiez l'adresse IP et le port (10009)
- Testez la connectivité réseau

## 📞 Support

Pour toute question ou problème :
1. Vérifiez les logs de l'application
2. Testez la connectivité LND avec `lncli getinfo`
3. Consultez la documentation Umbrel

L'application est maintenant prête pour le déploiement sur votre Umbrel local ! 🎉
