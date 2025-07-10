.PHONY: build run test clean install gui cli release help

# Variables
CARGO_FLAGS := --release
TARGET_DIR := target/release
BINARY_NAME := porthunter

# Couleurs pour l'affichage
RED := \033[0;31m
GREEN := \033[0;32m
YELLOW := \033[0;33m
BLUE := \033[0;34m
PURPLE := \033[0;35m
CYAN := \033[0;36m
WHITE := \033[0;37m
RESET := \033[0m

# Aide par défaut
help:
	@echo "$(PURPLE)🛡️  PortHunter - Commandes disponibles$(RESET)"
	@echo ""
	@echo "$(CYAN)🏗️  Build:$(RESET)"
	@echo "  $(GREEN)make build$(RESET)     - Compiler l'application"
	@echo "  $(GREEN)make release$(RESET)   - Compiler en mode release optimisé"
	@echo ""
	@echo "$(CYAN)🚀 Exécution:$(RESET)"
	@echo "  $(GREEN)make run$(RESET)       - Lancer l'interface graphique"
	@echo "  $(GREEN)make gui$(RESET)       - Lancer l'interface graphique (alias)"
	@echo "  $(GREEN)make cli$(RESET)       - Lancer l'interface en ligne de commande"
	@echo ""
	@echo "$(CYAN)🧪 Tests:$(RESET)"
	@echo "  $(GREEN)make test$(RESET)      - Exécuter les tests"
	@echo "  $(GREEN)make check$(RESET)     - Vérifier le code (clippy + fmt)"
	@echo ""
	@echo "$(CYAN)🧹 Maintenance:$(RESET)"
	@echo "  $(GREEN)make clean$(RESET)     - Nettoyer les fichiers de build"
	@echo "  $(GREEN)make install$(RESET)   - Installer l'application système"
	@echo "  $(GREEN)make dev-setup$(RESET) - Installer les outils de développement"
	@echo ""
	@echo "$(CYAN)📦 Docker:$(RESET)"
	@echo "  $(GREEN)make docker-build$(RESET) - Construire l'image Docker"
	@echo "  $(GREEN)make docker-run$(RESET)   - Exécuter avec Docker"

# Build l'application en mode debug
build:
	@echo "$(BLUE)🏗️  Compilation en mode debug...$(RESET)"
	cargo build

# Build l'application en mode release
release:
	@echo "$(BLUE)🏗️  Compilation en mode release...$(RESET)"
	cargo build $(CARGO_FLAGS)
	@echo "$(GREEN)✅ Build terminé: $(TARGET_DIR)/$(BINARY_NAME)$(RESET)"

# Exécuter en mode GUI (par défaut)
run: build
	@echo "$(PURPLE)🚀 Lancement de PortHunter (GUI)...$(RESET)"
	cargo run

# Alias pour l'interface graphique
gui: run

# Exécuter en mode CLI
cli: build
	@echo "$(PURPLE)🚀 Lancement de PortHunter (CLI)...$(RESET)"
	cargo run -- --cli

# Exécuter les tests
test:
	@echo "$(YELLOW)🧪 Exécution des tests...$(RESET)"
	cargo test

# Vérifier le code
check:
	@echo "$(YELLOW)🔍 Vérification du code...$(RESET)"
	cargo check
	cargo clippy -- -D warnings
	cargo fmt --check

# Formater le code
fmt:
	@echo "$(CYAN)📝 Formatage du code...$(RESET)"
	cargo fmt

# Nettoyer les fichiers de build
clean:
	@echo "$(RED)🧹 Nettoyage des fichiers de build...$(RESET)"
	cargo clean

# Installer l'application dans le système
install: release
	@echo "$(GREEN)📦 Installation de PortHunter...$(RESET)"
	cargo install --path .

# Installer les dépendances de développement
dev-setup:
	@echo "$(CYAN)🔧 Installation des outils de développement...$(RESET)"
	rustup component add clippy rustfmt
	cargo install cargo-watch
	cargo install cargo-edit
	@echo "$(GREEN)✅ Outils installés$(RESET)"

# Développement avec rechargement automatique
dev:
	@echo "$(PURPLE)🔄 Mode développement avec rechargement automatique...$(RESET)"
	cargo watch -x 'run'

# Développement CLI avec rechargement automatique
dev-cli:
	@echo "$(PURPLE)🔄 Mode développement CLI avec rechargement automatique...$(RESET)"
	cargo watch -x 'run -- --cli'

# Mise à jour des dépendances
update:
	@echo "$(CYAN)📦 Mise à jour des dépendances...$(RESET)"
	cargo update

# Documentation
doc:
	@echo "$(BLUE)📚 Génération de la documentation...$(RESET)"
	cargo doc --open

# Build Docker
docker-build:
	@echo "$(BLUE)🐳 Construction de l'image Docker...$(RESET)"
	docker build -t porthunter:latest .

# Run Docker
docker-run:
	@echo "$(PURPLE)🐳 Exécution avec Docker...$(RESET)"
	docker run -it --rm porthunter:latest

# Benchmark
bench:
	@echo "$(YELLOW)⚡ Exécution des benchmarks...$(RESET)"
	cargo bench

# Profiling
profile: release
	@echo "$(YELLOW)📊 Profiling de l'application...$(RESET)"
	perf record --call-graph=dwarf $(TARGET_DIR)/$(BINARY_NAME) --cli
	perf report

# Vérification de sécurité
audit:
	@echo "$(RED)🔐 Audit de sécurité des dépendances...$(RESET)"
	cargo audit

# Génération du changelog
changelog:
	@echo "$(CYAN)📝 Génération du changelog...$(RESET)"
	git log --oneline --decorate --graph > CHANGELOG.md

# Package pour distribution
package: release
	@echo "$(GREEN)📦 Création du package de distribution...$(RESET)"
	mkdir -p dist
	cp $(TARGET_DIR)/$(BINARY_NAME) dist/
	cp README.md LICENSE dist/
	tar -czf dist/porthunter-$(shell git describe --tags --always).tar.gz -C dist .
	@echo "$(GREEN)✅ Package créé dans dist/$(RESET)"

# Nettoyage complet
clean-all: clean
	@echo "$(RED)🧹 Nettoyage complet...$(RESET)"
	rm -rf dist/
	cargo clean

# Informations sur le build
info:
	@echo "$(CYAN)ℹ️  Informations de build:$(RESET)"
	@echo "Rust version: $(shell rustc --version)"
	@echo "Cargo version: $(shell cargo --version)"
	@echo "Target directory: $(TARGET_DIR)"
	@echo "Binary name: $(BINARY_NAME)"
	@echo "Features: $(CARGO_FLAGS)"