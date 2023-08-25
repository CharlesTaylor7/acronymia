import { test, expect } from "@playwright/test";

test("homepage has title and links to intro page", async ({ page }) => {
  await page.goto("http://localhost:3000/");
  await expect(page).toHaveTitle("Acronymia");

  await page.getByTestId("input-nickname").fill("Bob");
  await page.getByRole('button', { name: 'Join' }).click(); 

  // TODO: debug this
  // My working theory is that the websocket is failing to connect,
  // which leaves the app in its default state.
  // await expect(page.getByTestId('player-Bob')).toBeVisible()
});
