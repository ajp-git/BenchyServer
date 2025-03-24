# BenchyServer
Le "3DBenchy" des serveurs : un benchmark multi-langages pour tester la charge brute.

## v1 : Charge brute
- **But** : Comparer Rust, Node.js, et Symfony sous 1000 connexions simultanées.
- **Spec** : Voir [spec.md](spec.md).
- **Hypothèse** : Symfony moins performant (overhead), Rust optimal.

## Prérequis
- Docker

## Usage
1. `docker-compose up --build`
2. `k6 run tests/load_test.js` (port 8080 pour Rust, 8081 Node.js, 8082 Symfony)
3. `docker stats` pour CPU/mémoire

## Prochaines étapes
- Implémentations Rust, Node.js, Symfony.
- Résultats benchmarks.