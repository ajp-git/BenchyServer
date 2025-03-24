import http from 'k6/http';
import { sleep } from 'k6';

export const options = {
    stages: [
        { duration: '10s', target: 500 },  // Monte à 500 VUs en 10s
        { duration: '10s', target: 1000 }, // Monte à 1000 VUs
        { duration: '10s', target: 1000 }, // Maintient 1000 VUs
        { duration: '10s', target: 0 },    // Redescend à 0
    ],
    thresholds: {
        http_req_failed: ['rate<0.01'],   // <1% d’échecs
        http_req_duration: ['p(95)<200'], // 95% des requêtes <200ms
    },
};

export default function () {
    const payload = JSON.stringify({
        client_id: `uuid-${Math.random().toString(36).substring(2)}`,
        request: 'Hello',
        timestamp: new Date().toISOString(),
    });

    const params = {
        headers: { 'Content-Type': 'application/json' },
    };

    const res = http.post('http://localhost:8082', payload, params);
    sleep(0.1);
}