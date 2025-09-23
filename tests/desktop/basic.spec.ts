/* eslint-env mocha */
/* global browser $ */
import { before, describe, expect, it } from '@wdio/globals'

describe('Tauri desktop application', () => {
  before(async () => {
    await browser.waitUntil(
      async () => (await browser.getWindowHandles()).length > 0,
      {
        timeout: 20000,
        interval: 250,
        timeoutMsg: 'No Tauri webview window became available',
      }
    )

    const handles = await browser.getWindowHandles()
    await browser.switchToWindow(handles[handles.length - 1])

    await browser.waitUntil(
      async () =>
        (await browser.execute(() => document.readyState)) === 'complete',
      {
        timeout: 20000,
        interval: 250,
        timeoutMsg: 'Webview did not finish loading',
      }
    )

    await browser.waitUntil(
      async () =>
        (await browser.execute(() => document.querySelector('h1')))?.textContent
          ?.length,
      {
        timeout: 20000,
        interval: 250,
        timeoutMsg: 'Application UI did not render a heading in time',
      }
    )
  })

  it('renders the welcome heading', async () => {
    const heading = await $(
      "//h1[contains(normalize-space(), 'Welcome aboard')]"
    )
    await heading.waitForDisplayed({ timeout: 20000 })
    await expect(heading).toBeDisplayed()
  })

  it('shows starter guidance cards', async () => {
    const stackCard = await $(
      "//h2[contains(normalize-space(), 'Ready-to-use stack')]"
    )
    await stackCard.waitForDisplayed({ timeout: 20000 })

    const nextStepsCard = await $(
      "//h2[contains(normalize-space(), 'Where to go next')]"
    )
    await nextStepsCard.waitForDisplayed({ timeout: 20000 })

    await expect(stackCard).toBeDisplayed()
    await expect(nextStepsCard).toBeDisplayed()
  })
})
