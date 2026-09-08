import { browser } from "k6/browser";
import { check } from "k6";

export const options = {
    scenarios: {
        browser_profile: {
            executor: "constant-vus",
            vus: 5,
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

export default async function () {
    const context = await browser.newContext();
    const page = await context.newPage();

    try {
        await page.goto(`${BASE_URL}/u/seinaru`, {
            waitUntil: "domcontentloaded",
        });

        check(page, {
            "browser page loaded": () => page.url().includes("/u/seinaru"),
        });
    } finally {
        await page.close();
        await context.close();
    }
}