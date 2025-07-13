document.querySelector("#benchmark").addEventListener("click", () => {
  const startBtnEls = document.querySelectorAll("#start-wrap button");
  measureAll(startBtnEls);
});

async function measureAll(startBtnEls) {
  const results = {};
  for (const startBtn of startBtnEls) {
    const id = startBtn.id;
    console.log("Starting measurement for", id);
    startBtn.click();

    // Wait for the canvas to appear
    const canvas = await waitForElementAsync("#window-wrap canvas");
    // Lcate your fps display and stop button
    const fpsEl = document.querySelector("#fps");
    const stopBtn = document.querySelector("#stop");

    // Wait for 1 sec to make the renderer stable
    await sleep(1000);

    // Sample FPS until we have 10 readings
    console.log("Start");
    const samples = await sampleFpsUntilCount(canvas, stopBtn, fpsEl, 10);
    results[id] = samples;
    console.log(`Finished ${id}:`, samples);
  }
  console.log("ALL results:", JSON.stringify(results));
  return results;
}

function sampleFpsUntilCount(canvasEl, stopBtnEl, fpsEl, sampleCount = 10) {
  return new Promise((resolve) => {
    const samples = [];
    const interval = setInterval(() => {
      const fps = parseFloat(fpsEl.textContent);
      if (!isNaN(fps)) samples.push(fps);
      if (samples.length >= sampleCount) {
        clearInterval(interval);
        stopBtnEl.click();
        resolve(samples);
      }
    }, 1000);
  });
}

function waitForElementAsync(selector) {
  return new Promise((resolve) => {
    waitForElement(selector, (el) => resolve(el));
  });
}

function waitForElement(selector, callback) {
  const observer = new MutationObserver((mutations, observer) => {
    const element = document.querySelector(selector);
    if (element) {
      observer.disconnect();
      callback(element);
    }
  });

  observer.observe(document.body, {
    childList: true,
    subtree: true,
  });
}

async function sleep(ms) {
  return new Promise((resolve) => {
    setTimeout(() => {
      resolve();
    }, ms);
  });
}
