let positionsX = [];
let positionsY = [];
let velocitiesX = [];
let velocitiesY = [];
let boxSize = 20;
let boxNumber = 100;
let canvas = null;
let ctx = null;
let animationFrameId = null;
let isRunning = false;

function getRandomVelocity() {
    let v = Math.random() * 5;
    if (Math.random() < 0.5) v *= -1;
    return v;
}

function runCanvasBenchmark(canvasWidth, canvasHeight, size, number) {
    if (canvas) {
        cancelAnimationFrame(animationFrameId);
        canvas.remove();
    }

    boxSize = size;
    boxNumber = number;

    const canvasWrapEl = document.getElementById("window-wrap");
    canvas = document.createElement("canvas");
    canvas.width = canvasWidth;
    canvas.height = canvasHeight;
    canvas.id = "canvas-api";
    canvasWrapEl.appendChild(canvas);

    ctx = canvas.getContext("2d");

    positionsX = [];
    positionsY = [];
    velocitiesX = [];
    velocitiesY = [];

    for (let i = 0; i < boxNumber; i++) {
        positionsX.push(Math.random() * canvasWidth);
        positionsY.push(Math.random() * canvasHeight);
        velocitiesX.push(getRandomVelocity());
        velocitiesY.push(getRandomVelocity());
    }

    let frameCount = 0;
    let lastMeasureTime = performance.now();

    function anim() {
        ctx.fillStyle = "black";
        ctx.fillRect(0, 0, canvasWidth, canvasHeight);

        ctx.fillStyle = "rgba(102, 204, 102, 0.8)";

        for (let i = 0; i < boxNumber; i++) {
            positionsX[i] += velocitiesX[i];
            positionsY[i] += velocitiesY[i];

            if (positionsX[i] < 0 || positionsX[i] + boxSize > canvasWidth)
                velocitiesX[i] *= -1;
            if (positionsY[i] < 0 || positionsY[i] + boxSize > canvasHeight)
                velocitiesY[i] *= -1;

            ctx.fillRect(positionsX[i], positionsY[i], boxSize, boxSize);
        }

        frameCount++;
        const now = performance.now();
        if (now - lastMeasureTime > 1000) {
            const fps =
                Math.round((frameCount / ((now - lastMeasureTime) / 1000)) * 100) / 100;
            frameCount = 0;
            lastMeasureTime = now;
            document.querySelector("#fps").textContent = fps;
        }

        if (isRunning) animationFrameId = requestAnimationFrame(anim);
    }

    isRunning = true;
    anim();
}

const startButtonEl = document.querySelector("#start-canvas-api");
startButtonEl.addEventListener("click", () => {
    console.log("start?")
    const canvasWidth = parseInt(document.querySelector("#canvas-width").value);
    const canvasHeight = parseInt(document.querySelector("#canvas-height").value);
    const size = parseInt(document.querySelector("#box-size").value);
    const number = parseInt(document.querySelector("#box-number").value);
    runCanvasBenchmark(canvasWidth, canvasHeight, size, number);
});

const stopButtonEl = document.querySelector("#stop");
stopButtonEl.addEventListener("click", () => {
    isRunning = false;
    if (animationFrameId) cancelAnimationFrame(animationFrameId);
    if (canvas) {
        canvas.remove();
        canvas = null;
    }
    document.querySelector("#fps").textContent = "--.--";
});
