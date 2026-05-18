# AionOS – OS ultra‑léger pour agents IA

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)]()

**AionOS** est un système d’exploitation conçu **uniquement pour exécuter des agents IA autonomes** (LLM, agents décisionnels, RAG) sur matériel contraint (Raspberry Pi, vieux PC, IoT) ou même dans le navigateur via WebAssembly.

Il ne cherche pas à remplacer Windows/Linux pour le grand public, mais à offrir une **alternative ultra‑légère** (noyau < 10 Mo, mémoire totale ~12 Mo hors modèles) avec :
- Un **gestionnaire d’agents** natif (remplace systemd)
- Un **runtime LLM intégré** (`llama.cpp`)
- Une **API REST** pour piloter les agents
- Une **version WASM** jouable dans un navigateur

## Pourquoi AionOS ?

| Problème des OS classiques | Solution AionOS |
|----------------------------|----------------|
| Surcharge mémoire (Windows : 2 Go) | Noyau minimal + busybox + libc musl |
| Ordonnanceur non dédié aux agents | Priorité aux processus IA via cgroups |
| Latences imprévisibles | Kernel patches `PREEMPT_RT` optionnels |
| Pas d’isolation fine | Conteneurs légers (cgroups/namespaces) |

## Statut du projet

🚧 **Phase 1 – MVP** en cours : noyau fonctionnel, agent manager Rust.
