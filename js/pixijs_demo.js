import {Application, Graphics, Sprite,} from "https://cdn.jsdelivr.net/npm/pixi.js@8.0.0/dist/pixi.min.mjs";

const WIDTH = 800;
const HEIGHT = 600;
const BOX_SIZE = 4;
const NUM_RECTS = 50000;

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

async function runPixiBenchmark() {
    const app = new Application();
    await app.init({
        view: document.getElementById("pixi-canvas"),
        width: WIDTH,
        height: HEIGHT,
        background: "#000000",
        preference: "webgpu",
    });

    // Create a rectangle texture
    const graphics = new Graphics();
    graphics.beginFill(0x66cc66, 0.8);
    graphics.drawRect(0, 0, BOX_SIZE, BOX_SIZE);
    graphics.endFill();
    const rectTexture = app.renderer.generateTexture(graphics);

    // Create container
    const container = new Sprite();
    app.stage.addChild(container);

    // Initialize positions, velocities and sprites
    for (let i = 0; i < NUM_RECTS; i++) {
        positionsX.push(Math.random() * WIDTH);
        positionsY.push(Math.random() * HEIGHT);
        velocitiesX.push(getRandomVelocity());
        velocitiesY.push(getRandomVelocity());

        const sprite = new Sprite(rectTexture);
        sprite.x = positionsX[i];
        sprite.y = positionsY[i];
        container.addChild(sprite);
        rectSprites.push(sprite);
    }

    let frameCount = 0;
    let lastMeasureTime = performance.now();
    const results = new Map();

    app.ticker.add(() => {
        for (let i = 0; i < NUM_RECTS; i++) {
            positionsX[i] += velocitiesX[i];
            positionsY[i] += velocitiesY[i];

            if (positionsX[i] < 0 || positionsX[i] + BOX_SIZE > WIDTH) {
                velocitiesX[i] *= -1;
            }
            if (positionsY[i] < 0 || positionsY[i] + BOX_SIZE > HEIGHT) {
                velocitiesY[i] *= -1;
            }

            rectSprites[i].x = positionsX[i];
            rectSprites[i].y = positionsY[i];
        }

        frameCount++;
        const now = performance.now();
        if (now - lastMeasureTime > 3000) {
            const elapsedSecs = (now - lastMeasureTime) / 1000.0;
            const fps = frameCount / elapsedSecs;
            results.set(NUM_RECTS, fps);
            frameCount = 0;
            lastMeasureTime = now;
            console.log("fps: ", fps);
        }
    });
}

document
    .getElementById("start-pixi")
    .addEventListener("click", runPixiBenchmark);
