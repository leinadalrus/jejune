import {test, expect, type Page} from "@playwright/test";

const attrItems = [
    "HeaderMeta",
    "FooterMeta",
    "ContentBody",
    "EditorLayout"
];

test.describe("HeaderMeta", () => {
    test.beforeEach(async ({page}) => {
        await page.goto("https://britomart.is/");
    })

    test("https://www.Britomart.is/index/", async ({page}) => {
        await page.goto("https://britomart.is/");
        await expect(page).toHaveTitle(/Britomartis/);

        const helloWelcome = page.getbyRole("link", {name: "Hello, and Welcome!"});
        await expect(helloWelcome).toHaveAttribute("href", "/app/index");
        
        await helloWelcome.click();
        await expect(page).toHaveURL(/.*index/);
    });
});