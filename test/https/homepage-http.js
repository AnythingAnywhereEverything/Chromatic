import http from "k6/http";
import { check, sleep } from "k6";

export const options = {
    scenarios: {
        http_homepage: {
            executor: "constant-vus",
            vus: 100,
            duration: "30s",
        },
    },
    thresholds: {
        http_req_duration: ["p(95)<500"],
        http_req_failed: ["rate<0.01"],
    },
};

const BASE_URL = __ENV.BASE_URL || "http://localhost";
const TOKEN = __ENV.TOKEN;

export default function () {
    const res = http.get(`${BASE_URL}`, {
        headers: {
            token: TOKEN,
        },
    });

    check(res, {
        "HTTP returns 200": (r) => {
            if (r.status !== 200 && Math.random() < 0.01) {
                console.log(`status=${r.status} body=${r.body.slice(0, 200)}`);
            }
            return r.status === 200;
        },

        "HTTP under 500ms": (r) => r.timings.duration < 500,
    });

    // sleep(1)
}
