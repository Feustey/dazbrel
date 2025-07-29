# 🧪 Guide de Test - Dazno Umbrel

## Vue d'ensemble

Ce projet dispose d'une suite de tests complète qui simule les interactions avec l'API api.dazno.de et valide le comportement de l'intégration Umbrel locale.

## 🚀 Lancement rapide

```bash
# Lancer tous les tests
./run_tests.sh

# Lancer une catégorie spécifique
./run_tests.sh unit          # Tests unitaires
./run_tests.sh integration   # Tests d'intégration  
./run_tests.sh performance   # Tests de performance
./run_tests.sh mock         # Tests avec mock API
```

## 📊 Types de tests

### 1. Tests unitaires (`src/api/mcp_client.rs`)

Tests des fonctions individuelles du client MCP avec mocks complets.

**Couverts :**
- ✅ `get_recommendations()` - Récupération des recommandations
- ✅ `submit_action_result()` - Soumission des résultats d'actions
- ✅ `health_check()` - Vérification de santé de l'API
- ✅ `submit_node_metrics()` - Envoi des métriques du nœud  
- ✅ `get_performance_analysis()` - Analyse de performance
- ✅ Gestion des erreurs réseau
- ✅ Authentification avec clé API
- ✅ Requêtes concurrentes
- ✅ Sérialisation/désérialisation JSON

**Exemple :**
```rust
#[tokio::test]
async fn test_get_recommendations_success() {
    let mock_server = MockServer::start().await;
    let client = MCPClient::new(mock_server.uri(), None);
    let recommendations = client.get_recommendations(node_pubkey).await;
    assert!(recommendations.is_ok());
}
```

### 2. Tests d'intégration (`tests/integration_tests.rs`)

Tests de bout en bout simulant des workflows complets.

**Scénarios testés :**
- ✅ Workflow complet MCP (collecte → envoi → recommandations → actions)
- ✅ Intégration Lightning locale avec fallback mock
- ✅ Conversion des données Lightning vers format MCP
- ✅ Gestion de la résilience et récupération d'erreurs
- ✅ Validation des données et cas limites
- ✅ Opérations concurrentes multiples

**Exemple :**
```rust
#[tokio::test]
async fn test_full_integration_flow() {
    // 1. Récupérer données Lightning
    let node_info = lightning_client.get_local_node_info().await.unwrap();
    
    // 2. Convertir vers format MCP
    let node_metrics = NodeMetrics { /* ... */ };
    
    // 3. Soumettre à MCP
    let _result = mcp_client.submit_node_metrics(node_metrics).await;
}
```

### 3. Tests de performance (`tests/performance_tests.rs`)

Validation des performances et temps de réponse.

**Métriques testées :**
- ✅ Temps de réponse par endpoint (< 300ms pour recommandations)
- ✅ Performance sous charge concurrente (100 requêtes simultanées)
- ✅ Gestion des gros payloads (1000+ canaux)
- ✅ Récupération après erreurs
- ✅ Utilisation mémoire sous charge
- ✅ Gestion des timeouts
- ✅ Simulation de rate limiting
- ✅ Efficacité de compression des données

**Exemple :**
```rust
#[tokio::test] 
async fn test_concurrent_request_performance() {
    let start = Instant::now();
    let results = futures_util::future::join_all(tasks).await;
    let duration = start.elapsed();
    assert!(duration < Duration::from_secs(5));
}
```

### 4. Mock API Server (`tests/mock_api_server.rs`)

Serveur mock simulant api.dazno.de avec réponses réalistes.

**Endpoints simulés :**
- ✅ `GET /api/v1/health` - Status de santé
- ✅ `GET /api/v1/recommendations/{pubkey}` - Recommandations ML
- ✅ `GET /api/v1/analysis/{pubkey}/performance` - Analyse de performance
- ✅ `POST /api/v1/metrics` - Soumission de métriques
- ✅ `POST /api/v1/actions/result` - Résultats d'actions
- ✅ Gestion d'erreurs (429, 503, 400, 404)
- ✅ Fonctionnalités premium avec authentification

**Exemple de réponse réaliste :**
```json
{
  "performance_metrics": {
    "current_roi_percentage": 15.8,
    "routing_success_rate": 94.2,
    "avg_response_time_ms": 145
  },
  "competitive_analysis": {
    "vs_amboss_advantage": 15.3,
    "dazno_ml_accuracy": 94.7,
    "amboss_accuracy": 87.2
  }
}
```

## 🔧 Configuration des tests

### Variables d'environnement

```bash
# Niveau de logs pour les tests
export RUST_LOG=debug

# Activer les tests avec la vraie API (optionnel)
export ENABLE_REAL_API_TESTS=true

# Clé API pour les tests premium (optionnel)
export DAZNO_TEST_API_KEY=your_api_key_here

# Nombre de threads pour les tests
export TEST_THREADS=4
```

### Structure des tests

```
tests/
├── integration_tests.rs     # Tests d'intégration bout-en-bout
├── performance_tests.rs     # Tests de performance et charge
├── mock_api_server.rs       # Serveur mock api.dazno.de
└── test_config.rs          # Configuration et utilitaires de test
```

## 📋 Exécution détaillée

### Tests unitaires uniquement
```bash
cargo test --lib api::mcp_client::tests
```

### Tests d'intégration avec logs détaillés
```bash
RUST_LOG=debug cargo test --test integration_tests
```

### Tests de performance
```bash
cargo test --test performance_tests -- --test-threads=1
```

### Tests avec la vraie API (si disponible)
```bash
ENABLE_REAL_API_TESTS=true cargo test test_real_api
```

## 🎯 Cas de test spécifiques

### Simulation des interactions api.dazno.de

**1. Récupération de recommandations**
```rust
// Test avec 3 types de recommandations : AdjustFees, RebalanceChannel, OpenChannel
let recommendations = client.get_recommendations(node_pubkey).await?;
assert_eq!(recommendations.len(), 3);
assert!(recommendations[0].expected_roi_impact > 0.0);
```

**2. Analyse de performance vs Amboss**
```rust
let analysis = client.get_performance_analysis(node_pubkey, 30).await?;
assert_eq!(analysis["competitive_analysis"]["vs_amboss_advantage"], 15.3);
assert_eq!(analysis["performance_metrics"]["dazno_ml_accuracy"], 94.7);
```

**3. Soumission de métriques de nœud Lightning**
```rust
let metrics = NodeMetrics {
    pubkey: node_info.pubkey,
    channels: lightning_channels,
    routing_fees_earned: total_fees,
    // ...
};
let result = client.submit_node_metrics(metrics).await?;
```

## 📊 Métriques de performance attendues

| Endpoint | Temps max | Charge max | Succès min |
|----------|-----------|------------|------------|
| Health check | 100ms | 100 req/s | 99.9% |
| Recommendations | 300ms | 50 req/s | 99.5% |
| Performance analysis | 600ms | 20 req/s | 99.0% |
| Metrics submission | 2s | 10 req/s | 98.0% |

## 🚨 Gestion des erreurs testées

### Erreurs réseau
- ✅ Timeout de connexion
- ✅ Serveur indisponible  
- ✅ Réponses malformées
- ✅ Interruptions réseau

### Erreurs API
- ✅ 429 Rate Limiting
- ✅ 503 Service Unavailable
- ✅ 400 Bad Request
- ✅ 404 Not Found
- ✅ 401 Unauthorized

### Données invalides
- ✅ Node pubkey invalide
- ✅ Métriques incomplètes
- ✅ JSON malformé
- ✅ Valeurs extrêmes

## 🔍 Debug et diagnostic

### Activer les logs détaillés
```bash
RUST_LOG=trace ./run_tests.sh
```

### Logs spécifiques par module
```bash
RUST_LOG=dazno_umbrel::api::mcp_client=debug cargo test
```

### Analyser les failures
```bash
# Relancer seulement les tests qui ont échoué
cargo test --test integration_tests -- --nocapture

# Afficher tous les outputs
cargo test -- --show-output
```

## 📈 Couverture de code

### Générer un rapport de couverture
```bash
# Installer cargo-tarpaulin
cargo install cargo-tarpaulin

# Générer le rapport
cargo tarpaulin --out Html --output-dir target/coverage
```

### Objectifs de couverture
- ✅ Client MCP : > 95%
- ✅ Client Lightning : > 90%
- ✅ Handlers web : > 85%
- ✅ Utilitaires : > 90%

## 🎮 Tests interactifs

### Mock API Server standalone
```bash
# Démarrer le serveur mock pour tests manuels
cargo test mock_api_server::start_server -- --nocapture
# Puis tester avec: curl http://localhost:port/api/v1/health
```

### Test avec vrais certificats Umbrel
```bash
# Copier les vrais certificats
./setup-umbrel.sh

# Tester la connexion LND réelle
RUST_LOG=debug cargo test test_local_lightning_integration
```

## ✅ Checklist de validation

Avant chaque release, vérifier que tous ces tests passent :

- [ ] **Tests unitaires** : Toutes les fonctions MCP
- [ ] **Tests d'intégration** : Workflow complet
- [ ] **Tests de performance** : Temps de réponse corrects
- [ ] **Tests mock API** : Simulation réaliste
- [ ] **Tests de résilience** : Gestion d'erreurs
- [ ] **Tests de charge** : 100+ requêtes concurrentes
- [ ] **Validation des données** : Sérialisation/désérialisation
- [ ] **Tests de régression** : Pas de fonctionnalité cassée

## 🚀 Intégration CI/CD

### GitHub Actions (exemple)
```yaml
- name: Run test suite
  run: |
    ./run_tests.sh
    
- name: Upload coverage
  uses: codecov/codecov-action@v1
  with:
    file: ./target/coverage/cobertura.xml
```

---

**Tous les tests simulent des interactions réalistes avec api.dazno.de et garantissent la robustesse de l'intégration Umbrel !** 🎯