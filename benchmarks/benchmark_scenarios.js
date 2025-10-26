import http from 'k6/http';
import { check } from 'k6';
import { Trend, Counter, Rate } from 'k6/metrics';

const responseTimeTrend = new Trend('response_time');
const errorRate = new Rate('errors');
const requestsCounter = new Counter('total_requests');

export const options = {
    scenarios: {
        // Сценарий 1: Поиск точки деградации
        degradation_point: {
            executor: 'ramping-arrival-rate',
            startRate: 1,
            timeUnit: '1s',
            preAllocatedVUs: 10,
            maxVUs: 1000,
            stages: [
                { target: 10, duration: '30s' },   // 10 RPS
                { target: 50, duration: '30s' },   // 50 RPS  
                { target: 100, duration: '30s' },  // 100 RPS
                { target: 200, duration: '30s' },  // 200 RPS
                { target: 500, duration: '30s' },  // 500 RPS
                { target: 1000, duration: '30s' }, // 1000 RPS
                { target: 0, duration: '30s' },    // Recovery
            ],
        },

        // Сценарий 2: Максимальная нагрузка
        max_load: {
            executor: 'constant-arrival-rate',
            rate: 500, // RPS - будет корректироваться по результатам первого теста
            timeUnit: '1s',
            duration: '5m',
            preAllocatedVUs: 100,
            maxVUs: 1000,
        },

        // Сценарий 3: Восстановление после перегруза
        recovery: {
            executor: 'ramping-arrival-rate',
            startRate: 1000,
            timeUnit: '1s',
            preAllocatedVUs: 50,
            maxVUs: 200,
            stages: [
                { target: 1000, duration: '1m' },  // Перегруз
                { target: 100, duration: '2m' },   // Восстановление
                { target: 10, duration: '1m' },    // Стабилизация
            ],
        }
    },
    thresholds: {
        http_req_duration: ['p(95)<500'], // 95% запросов < 500ms
        errors: ['rate<0.01'],            // < 1% ошибок
    },
};

const BASE_URL = 'http://localhost:3080';

export default function () {
    const prep_resp = http.post(`${BASE_URL}/v1/round`);

    check(prep_resp, {
        'prepare status is 200': (r) => r.status === 200
    });

    const response = http.post(`${BASE_URL}/v1/round/results`);

    const success = check(response, {
        'status is 200': (r) => r.status === 200
    });

    responseTimeTrend.add(response.timings.duration);
    requestsCounter.add(1);
    errorRate.add(!success);
}
