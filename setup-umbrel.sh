#!/bin/bash

# 🚀 Script de configuration Dazno Umbrel
# Usage: ./setup-umbrel.sh [umbrel_ip]

set -e

UMBREL_IP=${1:-"192.168.0.29"}
UMBREL_HOST="umbrey2@${UMBREL_IP}"
SSH_OPTS="-o BatchMode=yes -o StrictHostKeyChecking=no"

echo "🎯 Configuration Dazno pour Umbrel sur ${UMBREL_IP}"
echo "=================================="

# Créer le dossier pour les credentials
echo "📁 Création du dossier lnd-credentials..."
mkdir -p ./lnd-credentials

# Test de connectivité
echo "🔍 Test de connectivité avec Umbrel..."
if ping -c 1 ${UMBREL_IP} > /dev/null 2>&1; then
    echo "✅ Umbrel accessible sur ${UMBREL_IP}"
else
    echo "❌ Impossible de joindre Umbrel sur ${UMBREL_IP}"
    echo "Vérifiez l'adresse IP et la connectivité réseau"
    exit 1
fi

check_required_apps() {
    echo "🛠 Vérification des applications Umbrel requises..."

    local missing_apps=()
    local apps=(
        "bitcoin:Bitcoin Core"
        "lightning:Lightning Node (LND)"
        "lightning-terminal:Lightning Terminal"
        "electrs:Electrs"
    )

    for app in "${apps[@]}"; do
        IFS=":" read -r folder label <<< "${app}"
        if ssh ${SSH_OPTS} ${UMBREL_HOST} "test -d /home/umbrel/umbrel/app-data/${folder}" > /dev/null 2>&1; then
            echo "✅ ${label} détecté"
        else
            echo "❌ ${label} manquant sur Umbrel"
            missing_apps+=("${label}")
        fi
    done

    if [ ${#missing_apps[@]} -gt 0 ]; then
        echo ""
        echo "Les applications suivantes doivent être installées depuis l'App Store Umbrel avant de continuer :"
        for app in "${missing_apps[@]}"; do
            echo "  - ${app}"
        done
        echo ""
        echo "Installez-les puis relancez ce script."
        exit 1
    fi

    echo "✅ Toutes les applications Umbrel requises sont présentes"
}

check_required_apps

# Fonction pour copier les certificats
copy_credentials() {
    echo "📋 Copie des certificats LND depuis Umbrel..."
    
    # Copier le certificat TLS
    echo "  → Copie du certificat TLS..."
    scp ${SSH_OPTS} ${UMBREL_HOST}:/home/umbrel/umbrel/app-data/lightning/data/lnd/tls.cert ./lnd-credentials/ || {
        echo "❌ Erreur lors de la copie du certificat TLS"
        echo "Vérifiez que vous avez accès SSH à Umbrel"
        exit 1
    }
    
    # Copier le macaroon admin
    echo "  → Copie du macaroon admin..."
    scp ${SSH_OPTS} ${UMBREL_HOST}:/home/umbrel/umbrel/app-data/lightning/data/lnd/data/chain/bitcoin/mainnet/admin.macaroon ./lnd-credentials/ || {
        echo "❌ Erreur lors de la copie du macaroon admin"
        echo "Vérifiez les chemins et permissions sur Umbrel"
        exit 1
    }
    
    # Optionnel : Copier le macaroon readonly (plus sécurisé)
    echo "  → Copie du macaroon readonly (optionnel)..."
    scp ${SSH_OPTS} ${UMBREL_HOST}:/home/umbrel/umbrel/app-data/lightning/data/lnd/data/chain/bitcoin/mainnet/readonly.macaroon ./lnd-credentials/ 2>/dev/null || {
        echo "⚠️  Macaroon readonly non trouvé (normal sur certaines versions)"
    }
    
    echo "✅ Certificats copiés avec succès"
}

# Créer le fichier .env
create_env_file() {
    echo "⚙️  Création du fichier .env..."
    
    cat > .env << EOF
# Configuration LND Umbrel
LND_GRPC_URI=https://${UMBREL_IP}:10009
LND_TLS_CERT_PATH=./lnd-credentials/tls.cert
LND_MACAROON_PATH=./lnd-credentials/admin.macaroon

# Configuration MCP API
MCP_API_URL=https://api.dazno.de

# Configuration Umbrel locale
UMBREL_HOST=${UMBREL_IP}
UMBREL_LND_HOST=${UMBREL_IP}
UMBREL_LND_PORT=10009

# Optionnel : Clé API MCP (ajoutez votre clé si disponible)
# MCP_API_KEY=your_api_key_here
EOF
    
    echo "✅ Fichier .env créé"
}

# Test de la connexion LND
test_lnd_connection() {
    echo "🔧 Test de la connexion LND..."
    
    # Charger les variables d'environnement
    source .env
    
    # Test avec lncli si disponible
    if command -v lncli &> /dev/null; then
        echo "  → Test avec lncli..."
        lncli --rpcserver=${UMBREL_IP}:10009 \
              --tlscertpath=./lnd-credentials/tls.cert \
              --macaroonpath=./lnd-credentials/admin.macaroon \
              getinfo > /dev/null 2>&1 && {
            echo "✅ Connexion LND fonctionnelle avec lncli"
        } || {
            echo "⚠️  Test lncli échoué (peut être normal)"
        }
    else
        echo "  → lncli non installé, test avec l'application..."
    fi
    
    # Test avec l'application Rust
    echo "  → Test avec l'application Dazno..."
    timeout 10s cargo run > /tmp/dazno_test.log 2>&1 &
    sleep 3
    
    # Vérifier les logs pour le succès de connexion
    if grep -q "Successfully connected to local LND" /tmp/dazno_test.log; then
        echo "✅ Connexion LND réussie avec Dazno"
    elif grep -q "Will operate in mock mode" /tmp/dazno_test.log; then
        echo "⚠️  Dazno fonctionne en mode mock (certificats non trouvés localement)"
        echo "    L'application va tenter de se connecter au démarrage"
    else
        echo "❌ Problème de connexion détecté"
        echo "Vérifiez les logs : /tmp/dazno_test.log"
    fi
    
    # Nettoyer le processus de test
    pkill -f "dazno-umbrel" 2>/dev/null || true
    rm -f /tmp/dazno_test.log
}

# Menu principal
show_menu() {
    echo ""
    echo "📋 Options disponibles :"
    echo "1) Copier les certificats depuis Umbrel"
    echo "2) Créer le fichier .env"
    echo "3) Tester la connexion LND"
    echo "4) Configuration complète (1+2+3)"
    echo "5) Lancer l'application"
    echo "6) Ouvrir l'interface web"
    echo "q) Quitter"
    echo ""
}

# Fonction pour lancer l'application
launch_app() {
    echo "🚀 Lancement de l'application Dazno..."
    echo "Interface disponible sur : http://localhost:3000"
    echo "Dashboard avancé : http://localhost:3000/superior"
    echo "Appuyez sur Ctrl+C pour arrêter"
    echo ""
    cargo run
}

# Fonction pour ouvrir l'interface web
open_web() {
    echo "🌐 Ouverture de l'interface web..."
    
    # Démarrer l'application en arrière-plan
    cargo run > /tmp/dazno_app.log 2>&1 &
    APP_PID=$!
    
    sleep 3
    
    # Ouvrir le navigateur
    if command -v open &> /dev/null; then
        open http://localhost:3000/superior
    elif command -v xdg-open &> /dev/null; then
        xdg-open http://localhost:3000/superior
    else
        echo "Ouvrez manuellement : http://localhost:3000/superior"
    fi
    
    echo "Application en cours d'exécution (PID: $APP_PID)"
    echo "Appuyez sur Entrée pour arrêter l'application..."
    read
    
    kill $APP_PID 2>/dev/null || true
    rm -f /tmp/dazno_app.log
}

# Boucle principale
while true; do
    show_menu
    read -p "Choisissez une option: " choice
    
    case $choice in
        1)
            copy_credentials
            ;;
        2)
            create_env_file
            ;;
        3)
            test_lnd_connection
            ;;
        4)
            copy_credentials
            create_env_file
            test_lnd_connection
            echo ""
            echo "🎉 Configuration complète terminée !"
            echo "Vous pouvez maintenant lancer l'application avec l'option 5"
            ;;
        5)
            launch_app
            ;;
        6)
            open_web
            ;;
        q|Q)
            echo "👋 Au revoir !"
            exit 0
            ;;
        *)
            echo "❌ Option invalide"
            ;;
    esac
    
    echo ""
    read -p "Appuyez sur Entrée pour continuer..."
    clear
done
