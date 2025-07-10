# 🛡️ PortHunter

**Scanner de ports avancé avec interface graphique native en Rust**

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
[![GitHub stars](https://img.shields.io/github/stars/wh0amix/PortHunter?style=for-the-badge)](https://github.com/wh0amix/PortHunter/stargazers)
[![GitHub issues](https://img.shields.io/github/issues/wh0amix/PortHunter?style=for-the-badge)](https://github.com/wh0amix/PortHunter/issues)

<div align="center">

![PortHunter Demo](https://via.placeholder.com/800x400/1a1a2e/eee?text=PortHunter+GUI+Demo)

*Interface graphique moderne avec scan en temps réel*

</div>

---

## 🌟 Aperçu

PortHunter est un scanner de ports moderne écrit en Rust, offrant une **interface graphique native** et une **version CLI** dans le même binaire. Conçu pour être à la fois performant et facile à utiliser, il intègre des fonctionnalités avancées de sécurité et d'analyse.

### 🎯 Pourquoi PortHunter ?

- **🚀 Performance** : Écrit en Rust pour une vitesse maximale
- **🎨 Interface moderne** : GUI native avec egui, responsive et intuitive
- **🔄 Double mode** : Interface graphique ET ligne de commande
- **🛡️ Sécurité** : Conseils et recommandations intégrés
- **📊 Temps réel** : Scan asynchrone avec statistiques live
- **📖 Éducatif** : Guide des ports détaillé pour l'apprentissage

---

## ✨ Fonctionnalités

### 🖥️ **Interface Graphique**
- **Scanner en temps réel** avec progression visuelle
- **Affichage détaillé** des ports ouverts trouvés
- **Guide des ports** intégré avec descriptions et conseils sécurité
- **Export JSON** des résultats
- **Interface responsive** qui s'adapte à toutes les tailles d'écran
- **Contrôles intuitifs** (start/stop/reset/nouveau scan)

### 💻 **Version CLI**
- **Interface texte** fidèle à l'original
- **Scan parallèle** avec rayon pour de meilleures performances
- **Résolution DNS** automatique
- **Affichage coloré** et formaté des résultats

### 🔍 **Fonctionnalités de scan**
- **Détection de bannières** pour identifier les services
- **Plages de ports personnalisables** (ex: 1-1000, 80,443,22)
- **Timeout configurable** pour optimiser la vitesse/précision
- **Résolution DNS** avec affichage du hostname
- **Tri automatique** des résultats par port

### 🛡️ **Sécurité & Analyse**
- **Base de données de ports** avec 20+ services courants
- **Codes couleur** selon le niveau de risque :
  - 🟢 **Sécurisé** (HTTPS, SSH, IMAPS...)
  - 🟡 **Attention** (HTTP, FTP, Telnet...)
  - 🔴 **Dangereux** (RDP exposé, Redis sans auth...)
- **Recommandations** pour chaque port trouvé
- **Informations d'usage** typique des services

---

## 🚀 Installation

### Prérequis

```bash
# Installer Rust (si pas déjà fait)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### Installation rapide

```bash
# Cloner le projet
git clone https://github.com/wh0amix/PortHunter.git
cd PortHunter

# Compiler et lancer
cargo run
```

### Installation système

```bash
# Compiler en mode release
cargo build --release

# Installer dans le système (optionnel)
cargo install --path .

# Ou utiliser le Makefile
make install
```

---

## 📱 Utilisation

### Interface Graphique (par défaut)

```bash
# Lancer l'interface graphique
cargo run

# Ou avec le Makefile
make run
```

**Configuration du scan :**
1. **Cible** : Entrez une IP (`192.168.1.1`) ou un domaine (`scanme.nmap.org`)
2. **Plage de ports** : Définissez début et fin (ex: `1` à `1000`)
3. **Timeout** : Ajustez selon vos besoins (défaut: `1000ms`)
4. Cliquez **▶️ Démarrer le scan**

### Interface CLI

```bash
# Lancer la version CLI
cargo run -- --cli

# Ou avec variable d'environnement
PORTHUNTER_MODE=cli cargo run

# Ou avec le Makefile
make cli
```

### Exemples pratiques

```bash
# Scan rapide d'un serveur web
# Cible: 93.184.216.34 (example.com)
# Ports: 80-443

# Scan complet d'un réseau local
# Cible: 192.168.1.1
# Ports: 1-65535 (attention: très long !)

# Scan ciblé services courants
# Cible: target.domain.com
# Ports: 21,22,25,53,80,110,143,443,993,995
```

---

## 🛠️ Développement

### Commandes Make

```bash
make help          # Afficher toutes les commandes
make build         # Compiler en mode debug
make release       # Compiler optimisé
make test          # Exécuter les tests
make clean         # Nettoyer les builds
make dev           # Développement avec rechargement auto
make check         # Vérifier le code (clippy + fmt)
```

### Structure du projet

```
src/
├── main.rs        # Application principale + GUI
└── cli.rs         # Module CLI original

Cargo.toml         # Dépendances
.gitignore         # Exclusions Git
README.md          # Documentation
LICENSE            # Licence MIT
Makefile           # Commandes de build
```

### Technologies utilisées

| Crate | Usage | Version |
|-------|-------|---------|
| **egui** | Interface graphique native | `0.24` |
| **eframe** | Framework d'application | `0.24` |
| **rayon** | Parallélisation | `1.7` |
| **dns-lookup** | Résolution DNS | `2.0` |
| **figlet-rs** | Bannière ASCII | `0.1` |
| **serde** | Sérialisation JSON | `1.0` |

---

## 📊 Captures d'écran

### Interface Graphique

<div align="center">

**Scanner Principal**
![Scanner](https://via.placeholder.com/600x400/2d1b69/eee?text=Scanner+Interface)

**Guide des Ports**
![Guide](https://via.placeholder.com/600x400/1a1a2e/eee?text=Port+Guide)

**Résultats Détaillés**
![Results](https://via.placeholder.com/600x400/0f3460/eee?text=Detailed+Results)

</div>

### Interface CLI

```
 ____            _   _   _             _            
|  _ \ ___  _ __| |_| | | |_   _ _ __ | |_ ___ _ __ 
| |_) / _ \| '__| __| |_| | | | | '_ \| __/ _ \ '__|
|  __/ (_) | |  | |_|  _  | |_| | | | | ||  __/ |   
|_|   \___/|_|   \__|_| |_|\__,_|_| |_|\__\___|_|   

==============================
🔧 Menu Principal
1️⃣  Scanner les ports
2️⃣  Afficher le guide des ports
3️⃣  Ouvrir l'interface graphique
0️⃣  Quitter
==============================
```

---

## 🔒 Sécurité & Éthique

### ⚠️ Avertissement Important

**PortHunter est destiné uniquement à :**
- ✅ **Tests de sécurité autorisés** sur vos propres systèmes
- ✅ **Formation et éducation** en cybersécurité
- ✅ **Audit de sécurité** avec autorisation écrite
- ✅ **Recherche académique** dans un cadre légal

### 🚫 Utilisation Interdite

- ❌ **Scanner sans autorisation** des systèmes tiers
- ❌ **Reconaissance malveillante** pour des attaques
- ❌ **Violation de la vie privée** ou des CGU
- ❌ **Toute activité illégale** selon votre juridiction

### 🛡️ Recommandations

1. **Obtenez toujours une autorisation** avant de scanner
2. **Respectez les limites de taux** pour éviter la surcharge
3. **Documentez vos tests** pour prouver le caractère légitime
4. **Informez les propriétaires** des vulnérabilités trouvées

---

## 🤝 Contribution

Les contributions sont les bienvenues ! Voici comment participer :

### 1. Fork & Clone
```bash
git clone https://github.com/wh0amix/PortHunter.git
cd PortHunter
```

### 2. Créer une branche
```bash
git checkout -b feature/nouvelle-fonctionnalite
```

### 3. Développer
```bash
# Installer les outils de dev
make dev-setup

# Développement avec rechargement auto
make dev

# Vérifier le code
make check
```

### 4. Tester
```bash
make test
```

### 5. Soumettre
```bash
git commit -am "✨ Ajouter nouvelle fonctionnalité"
git push origin feature/nouvelle-fonctionnalite
```

### 🎯 Idées de contributions

- 🌐 **Support IPv6**
- 🔄 **Scan UDP**
- 📡 **Détection d'OS**
- 🎨 **Thèmes personnalisables**
- 🌍 **Internationalisation**
- 📈 **Graphiques de performance**
- 🔌 **Support de plugins**

---

## 📈 Roadmap

### Version 1.1 (Q2 2024)
- [ ] Support IPv6 complet
- [ ] Scan UDP avancé
- [ ] Export en multiple formats (CSV, XML)
- [ ] Thèmes sombres/clairs

### Version 1.2 (Q3 2024)
- [ ] Détection d'OS (fingerprinting)
- [ ] Scripts NSE (Nmap Script Engine)
- [ ] Interface web optionnelle
- [ ] API REST

### Version 2.0 (Q4 2024)
- [ ] Scanner de vulnérabilités
- [ ] Rapports automatisés
- [ ] Base de données des scans
- [ ] Mode collaboratif

---

## 📊 Statistiques

<div align="center">

![GitHub Stats](https://github-readme-stats.vercel.app/api?username=wh0amix&repo=PortHunter&show_icons=true&theme=radical)

</div>


## 📜 Licence

Ce projet est sous **licence MIT**. Voir le fichier [LICENSE](LICENSE) pour plus de détails.

```
MIT License - Copyright (c) 2024 wh0amix

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.
```


<div align="center">

**⭐ Si ce projet vous plaît, n'hésitez pas à lui donner une étoile !**

[![GitHub stars](https://img.shields.io/github/stars/wh0amix/PortHunter?style=social)](https://github.com/wh0amix/PortHunter/stargazers)

---

*Développé avec ❤️ en Rust par [wh0amix] et [Zayko1]*

</div>
