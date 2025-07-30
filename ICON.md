# 🎨 Dazno App Icon & Branding

## Icône principale

### Fichiers disponibles

- `icon.png` (512x512) - Icône principale pour Umbrel
- `static/images/icon.svg` - Version vectorielle source
- `static/images/icon.png` - Copie de l'icône dans les assets

### Design

L'icône Dazno utilise un design moderne basé sur le logo officiel du site dazno.de :

- **Forme principale** : Hexagone double couche
- **Couleur extérieure** : Bleu (#2563eb) 
- **Couleur intérieure** : Jaune/Or (#fbbf24)
- **Symbole central** : Éclair stylisé pour représenter Lightning Network
- **Effet** : Ombre portée subtile pour la profondeur

### Usage dans Umbrel

L'icône est configurée dans `umbrel-app.yml` et sera automatiquement utilisée par Umbrel pour :

- Affichage dans l'App Store
- Icône de l'application installée
- Interface de gestion des apps

## Images de galerie

### Fichiers créés

- `gallery/1.jpg` - Dashboard principal 
- `gallery/2.jpg` - Interface supérieure avancée
- `gallery/3.jpg` - Comparaison vs Amboss

### Contenu des images

**Image 1 - Dashboard principal (600x400)**
- Aperçu des performances du nœud
- Recommandations ML en temps réel  
- Analytics de performance
- ROI et métriques clés

**Image 2 - Interface supérieure (600x400)**
- Dashboard avancé avec alertes
- Système d'automatisation
- Graphiques de performance
- Métriques détaillées

**Image 3 - Comparaison Amboss (600x400)**
- Avantages Dazno vs Amboss
- Métriques de performance comparative
- Sécurité et intégration locale
- Proposition de valeur claire

## Génération automatique

Pour régénérer les images de galerie :

```bash
./gallery/create_gallery.sh
```

## Branding cohérent

### Palette de couleurs

- **Bleu principal** : #2563eb (Lightning blue)
- **Jaune/Or** : #fbbf24 (Bitcoin gold)
- **Vert succès** : #10b981 (Profit green)  
- **Orange warning** : #f59e0b (Alert orange)
- **Rouge danger** : #ef4444 (Error red)
- **Gris foncé** : #1e293b (Background dark)

### Typographie

- **Titre principal** : 24px, blanc, gras
- **Sous-titres** : 16px, blanc, normal
- **Texte descriptif** : 14px, blanc, normal
- **Métriques** : 12px, blanc, normal

## Intégration Umbrel

L'icône et les images sont automatiquement intégrées dans le package Umbrel via :

1. **umbrel-app.yml** - Métadonnées et référence galerie
2. **docker-compose.yml** - Configuration du service  
3. **Dockerfile** - Construction de l'image avec assets

## Conformité Umbrel App Store

✅ **Icône 512x512 PNG** - Format requis  
✅ **Images galerie 600x400** - Dimensions recommandées
✅ **Branding cohérent** - Design professionnel
✅ **Lisibilité** - Texte clair et contrasté
✅ **Représentativité** - Montre les vraies fonctionnalités

L'app est prête pour soumission à l'Umbrel App Store ! 🚀