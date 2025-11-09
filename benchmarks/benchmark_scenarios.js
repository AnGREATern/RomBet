import http from 'k6/http';
import { check, group } from 'k6';
import { Trend, Counter, Rate } from 'k6/metrics';

const simulationTime = new Trend('simulation_response_time');
// const teamsTime = new Trend('teams_response_time');
const errorRate = new Rate('errors');
const requestsCounter = new Counter('total_requests');

export const options = {
    scenarios: {
        degradation_point: {
            executor: 'ramping-arrival-rate',
            startRate: 1,
            timeUnit: '1s',
            preAllocatedVUs: 10,
            maxVUs: 800,
            stages: [
                { target: 10, duration: '15s' },
                { target: 100, duration: '1m' },
                { target: 200, duration: '1m' },
                { target: 500, duration: '1m' },
                { target: 800, duration: '1m' },
                { target: 0, duration: '1m' },
            ],
        },

        // max_load: {
        //     executor: 'constant-arrival-rate',
        //     rate: 520,
        //     timeUnit: '1s',
        //     duration: '3m',
        //     preAllocatedVUs: 100,
        //     maxVUs: 800,
        // },

        // recovery: {
        //     executor: 'ramping-arrival-rate',
        //     startRate: 800,
        //     timeUnit: '1s',
        //     preAllocatedVUs: 50,
        //     maxVUs: 800,
        //     stages: [
        //         { target: 800, duration: '1m' },
        //         { target: 100, duration: '2m' },
        //     ],
        // }
    },
};

const BASE_URL = __ENV.BASE_URL || 'http://localhost:8080';

export default function () {
    group('Simulation starting test', function () {
        const start = Date.now();
        const response = http.post(`${BASE_URL}/api/v1/simulation`);
        const duration = Date.now() - start;
        
        simulationTime.add(duration);

        const success = check(response, {
            'status is 200': (r) => r.status === 200,
            'has id': (r) => {
                if (r.status !== 200) return false;
                try {
                    const body = JSON.parse(r.body);
                    return typeof body.id === 'string' && body.id.length > 0;
                } catch (e) {
                    return false;
                }
            },
            'has balance': (r) => {
                if (r.status !== 200) return false;
                try {
                    const body = JSON.parse(r.body);
                    return typeof body.balance === 'number';
                } catch (e) {
                    return false;
                }
            }
        });

        requestsCounter.add(1);
        errorRate.add(!success);
    });

    // group('Get all teams test', function () {
    //     const start = Date.now();
    //     const resp = http.get(`${BASE_URL}/api/v1/teams`);
    //     const duration = Date.now() - start;
        
    //     teamsTime.add(duration);

    //     const success = check(resp, {
    //         'prepare status is 200': (r) => r.status === 200
    //     });
        
    //     requestsCounter.add(1);
    //     errorRate.add(!success);
    // });
}

export function handleSummary(data) {
    return {
        'stdout': JSON.stringify(data, null, 2),
        '/results/raw/summary.json': JSON.stringify(data, null, 2),
    };
}