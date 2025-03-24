import http from 'k6/http';
import { sleep } from 'k6';

export const options = {
    vus: 1000,         // 1000 utilisateurs virtuels
    iterations: 10000, // 10 requêtes par VU
    duration: '30s',   // Limite à 30s
};

export default function () {
    const payload = JSON.stringify({
        client_id: `uuid-${Math.random().toString(36).substring(2)}`,
        request: 'Hello',
        timestamp: new Date().toISOString(),
    });

    const params = {
        headers: {
            'Content-Type': 'application/json',
        },
    };

    http.post('http://localhost:8082', payload, params);
    sleep(0.1);
}