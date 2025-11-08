import http from 'k6/http';
import { check, group } from 'k6';
import { Trend, Rate, Counter } from 'k6/metrics';

const simulationDuration = new Trend('simulation_response_time', true);
const errorRate = new Rate('http_errors');
const requestCounter = new Counter('http_requests_total');
const customResponseTime = new Trend('http_req_duration_custom', true);
const dataReceived = new Trend('data_received_bytes', true);

export const options = {
  scenarios: {
    degradation_point: {
      executor: 'ramping-arrival-rate',
      startRate: 1,
      timeUnit: '1s',
      preAllocatedVUs: 10,
      maxVUs: 1000,
      stages: [
        { target: 10, duration: '30s' },
        { target: 50, duration: '30s' },
        { target: 100, duration: '30s' },
        { target: 200, duration: '30s' },
        { target: 500, duration: '30s' },
        { target: 1000, duration: '30s' },
        { target: 0, duration: '30s' },
      ],
    },
    max_load: {
      executor: 'constant-arrival-rate',
      rate: 500,
      timeUnit: '1s',
      duration: '5m',
      preAllocatedVUs: 100,
      maxVUs: 1000,
    },
    recovery: {
      executor: 'ramping-arrival-rate',
      startRate: 1000,
      timeUnit: '1s',
      preAllocatedVUs: 50,
      maxVUs: 200,
      stages: [
        { target: 1000, duration: '1m' },
        { target: 100, duration: '2m' },
        { target: 10, duration: '1m' },
      ],
    },
  },
  thresholds: {
    // Используем кастомную метрику для более точного контроля
    'http_req_duration_custom{metric:"http_req_duration_custom"}': ['p(95)<500', 'p(99)<1000'],
    'simulation_response_time{metric:"simulation_response_time"}': ['p(95)<500', 'p(99)<1000'],
    'http_errors': ['rate<0.01'],
  },
  ext: {
    loadimpact: {
      distribution: 'amazon:us:ashburn',
      apm: [],
    },
  },
};

const BASE_URL = __ENV.BASE_URL || 'http://localhost:8080';

export default function () {
  group('Simulation Initialization Test', function () {
    const start = new Date();

    const res = http.post(`${BASE_URL}/api/v1/simulation`, null, {
      tags: { name: 'simulation_init' },
    });

    const end = new Date();
    const durationMs = end - start;
    const durationSec = durationMs / 1000;

    customResponseTime.add(durationSec);           // кастомная гистограмма
    simulationDuration.add(durationSec);           // время симуляции
    dataReceived.add(res.body.length);             // объём данных
    requestCounter.add(1);                         // счётчик

    const success = check(res, {
      'status is 200': (r) => r.status === 200,
      'has valid id': (r) => {
        try {
          const body = JSON.parse(r.body);
          return typeof body.id === 'string' && body.id.length > 0;
        } catch (e) {
          return false;
        }
      },
      'has balance': (r) => {
        try {
          const body = JSON.parse(r.body);
          return typeof body.balance === 'number';
        } catch (e) {
          return false;
        }
      },
    });

    errorRate.add(!success);
  });
}

export function handleSummary(data) {
  return {
    'stdout': JSON.stringify(data, null, 2),
    '/results/raw/summary.json': JSON.stringify(data, null, 2),
  };
}