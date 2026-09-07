import http from 'k6/http';
import { check, sleep } from 'k6';

export const options = {
  scenarios: {
    homepage_1000users: {
      executor: 'ramping-vus',
      stages: [
        { duration: '30s', target: 100 },
        { duration: '30s', target: 50 },
      ],
    },
  },
  thresholds: {
    http_req_duration: ['p(95)<500'],
    http_req_failed: ['rate<0.01'],
  },
};

const BASE_URL = __ENV.BASE_URL || 'http://localhost';

export default function () {
  const res = http.get(`${BASE_URL}/u/seinaru`);
  check(res, {
    'health returns 200': (r) => r.status === 200,
    'transaction is less than 500ms': (r) => r.timings.duration < 500,
  });    
  sleep(1);
}
