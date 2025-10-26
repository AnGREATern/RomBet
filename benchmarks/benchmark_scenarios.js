import http from 'k6/http';
import { check, group } from 'k6';
import { Trend, Counter, Rate } from 'k6/metrics';

// Custom metrics for detailed analysis
const responseTimeTrend = new Trend('response_time');
const roundResultsTime = new Trend('round_results_response_time');
const prepareRoundTime = new Trend('prepare_round_response_time');
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
        'round_results_response_time': ['p(99)<1000', 'p(95)<500', 'p(90)<300'],
        'prepare_round_response_time': ['p(99)<500', 'p(95)<250', 'p(90)<150'],
    },
};

const BASE_URL = __ENV.BASE_URL || 'http://localhost:8080';

export default function () {
    group('Round Results Test', function () {
        // First create a round to test against
        const prep_start = Date.now();
        const prep_resp = http.post(`${BASE_URL}/api/v1/round`);
        const prep_duration = Date.now() - prep_start;
        
        prepareRoundTime.add(prep_duration);

        const prep_success = check(prep_resp, {
            'prepare status is 200': (r) => r.status === 200
        });

        if (!prep_success) {
            errorRate.add(1);
            return;
        }

        // Now test the round/results endpoint
        const start = Date.now();
        const response = http.post(`${BASE_URL}/api/v1/round/results`);
        const duration = Date.now() - start;
        
        roundResultsTime.add(duration);
        responseTimeTrend.add(duration);

        const success = check(response, {
            'status is 200': (r) => r.status === 200,
            'has games_stat': (r) => {
                if (r.status !== 200) return false;
                try {
                    const body = JSON.parse(r.body);
                    return Array.isArray(body.games_stat);
                } catch (e) {
                    return false;
                }
            },
            'has profit': (r) => {
                if (r.status !== 200) return false;
                try {
                    const body = JSON.parse(r.body);
                    return typeof body.profit === 'number';
                } catch (e) {
                    return false;
                }
            }
        });

        requestsCounter.add(1);
        errorRate.add(!success);
    });
}

// Export handles for detailed metrics collection
export function handleSummary(data) {
    return {
        'stdout': JSON.stringify(data, null, 2),
        'results/raw/summary.json': JSON.stringify(data, null, 2),
    };
}
