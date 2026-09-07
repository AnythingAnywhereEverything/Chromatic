import { browser } from "k6/browser";
import { check } from "k6";

export const options = {
    scenarios: {
        browser_homepage: {
            executor: "constant-vus",
            vus: 2,
            duration: "30s",
            options: {
                browser: {
                    type: "chromium",
                },
            },
        },
    },
};

const BASE_URL = __ENV.BASE_URL || "http://localhost";
const TOKEN = __ENV.TOKEN;

export default async function () {
    const context = await browser.newContext();

    try {
        await context.addInitScript(
            `localStorage.setItem("token", ${JSON.stringify(TOKEN)});`,
        );

        const page = await context.newPage();

        while (true) {
            await page.goto(`${BASE_URL}/`, {
                waitUntil: "domcontentloaded",
            });

            check(page, {
                "browser page loaded": () => page.url().includes("/"),
            });
        }
    } finally {
        await context.close();
    }
}