import {Application, Graphics, Sprite,} from "https://cdn.jsdelivr.net/npm/pixi.js@8.0.0/dist/pixi.min.mjs";

let positionsX = [];
let positionsY = [];
let velocitiesX = [];
let velocitiesY = [];
let rectSprites = [];

function getRandomVelocity() {
    let v = Math.random() * 5;
    if (Math.random() < 0.5) v *= -1;
    return v;
}

let isRunning = false;
let app = null;

async function runPixiBenchmark(canvasWidth, canvasHeight, boxSize, boxNumber) {
    if (app) app.destroy(true); // Clean up previous app instance if it exists
    app = new Application();

    const canvasWrapEl = document.getElementById("window-wrap");
    const pixiCanvasEl = document.createElement("canvas");
    pixiCanvasEl.setAttribute("id", "pixi");
    canvasWrapEl.append(pixiCanvasEl);

    await app.init({
        view: pixiCanvasEl,
        width: canvasWidth,
        height: canvasHeight,
        background: "#000000",
        preference: "webgpu",
        sharedTicker: false,
        antialias: true,
    });

    const graphics = new Graphics();
    graphics.beginFill(0x66cc66, 0.8);
    graphics.drawRect(0, 0, boxSize, boxSize);
    graphics.endFill();
    const rectTexture = app.renderer.generateTexture(graphics);

    positionsX = [];
    positionsY = [];
    velocitiesX = [];
    velocitiesY = [];
    rectSprites = [];

    for (let i = 0; i < boxNumber; i++) {
        positionsX.push(Math.random() * canvasWidth);
        positionsY.push(Math.random() * canvasHeight);
        velocitiesX.push(getRandomVelocity());
        velocitiesY.push(getRandomVelocity());

        const sprite = new Sprite(rectTexture);
        sprite.x = positionsX[i];
        sprite.y = positionsY[i];
        app.stage.addChild(sprite);
        rectSprites.push(sprite);
    }

    let frameCount = 0;
    let lastMeasureTime = performance.now();
    const anim = () => {
        for (let i = 0; i < boxNumber; i++) {
            positionsX[i] += velocitiesX[i];
            positionsY[i] += velocitiesY[i];

            if (positionsX[i] < 0 || positionsX[i] > canvasWidth)
                velocitiesX[i] *= -1;
            if (positionsY[i] < 0 || positionsY[i] > canvasHeight)
                velocitiesY[i] *= -1;

            rectSprites[i].x = positionsX[i];
            rectSprites[i].y = positionsY[i];
        }

        frameCount++;
        const now = performance.now();
        if (now - lastMeasureTime > 1000) {
            const fps =
                Math.round((frameCount / ((now - lastMeasureTime) / 1000)) * 100) / 100;
            frameCount = 0;
            lastMeasureTime = now;
            const fpsEl = document.querySelector("#fps");
            fpsEl.textContent = fps;
        }
    };

    app.ticker.add(anim);
    app.ticker.start();
}

const startButtonEl = document.querySelector("#start-pixi-webgl");
startButtonEl.addEventListener("click", () => {
    const canvasWidth = parseInt(document.querySelector("#canvas-width").value);
    const canvasHeight = parseInt(document.querySelector("#canvas-height").value);
    const boxSize = parseInt(document.querySelector("#box-size").value);
    const boxNumber = parseInt(document.querySelector("#box-number").value);
    isRunning = true;
    runPixiBenchmark(canvasWidth, canvasHeight, boxSize, boxNumber);
});

const stopButtonEl = document.querySelector("#stop");
stopButtonEl.addEventListener("click", () => {
    isRunning = false;
    if (app) {
        app.destroy(true);
        app = null;
    }
    const pixiCanvasEl = document.querySelector("#window-wrap #pixi");
    if (pixiCanvasEl) pixiCanvasEl.remove();

    document.querySelector("#fps").textContent = "--.--";
});
