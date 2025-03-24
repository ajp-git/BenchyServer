# BenchyServer v1
Le "3DBenchy" des serveurs : un benchmark multi-langages sous charge brute.

## Spécification
## Spécification
- **Endpoint** : POST /
- **Requête** :
  ```json
  {
    "client_id": "string",    // ex. : "uuid-1234"
    "request": "string",      // ex. : "Hello"
    "timestamp": "ISO string" // ex. : "2025-03-24T10:00:00Z"
  }

- **Réponses** :
  ```json
  {
  "success": true,
  "server_time": "ISO string" // ex. : "2025-03-24T10:00:01Z"
  }
- **Note** : Les timestamps sont décoratifs, métriques via k6.

## Test
- 1000 connexions, 10 requêtes par client via k6.
- Métriques : Latence (k6), CPU/Mémoire (Docker), Throughput, Erreurs.

## Lancer
- `docker-compose up` pour tous les serveurs.
- `k6 run tests/load_test.js`pour tester
