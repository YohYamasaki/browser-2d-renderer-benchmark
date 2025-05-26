// canvaskit_demo.js

const WIDTH = 800;
const HEIGHT = 600;
const BOX_SIZE = 4;

class BouncingRect {
  constructor(canvasWidth, canvasHeight) {
    this.box_x = canvasWidth / 2;
    this.box_y = canvasHeight / 2;
    this.velocity_x = getRandomVelocity();
    this.velocity_y = getRandomVelocity();
  }

  update() {
    if (this.box_x <= 0 || this.box_x + BOX_SIZE >= WIDTH) {
      this.velocity_x *= -1;
    }
    if (this.box_y <= 0 || this.box_y + BOX_SIZE >= HEIGHT) {
      this.velocity_y *= -1;
    }
    this.box_x += this.velocity_x;
    this.box_y += this.velocity_y;
  }

  draw(CanvasKit, canvas, paint) {
    const rect = CanvasKit.LTRBRect(
      this.box_x,
      this.box_y,
      this.box_x + BOX_SIZE,
      this.box_y + BOX_SIZE
    );
    canvas.drawRect(rect, paint);
  }
}

function getRandomVelocity() {
  let v = Math.random() * 5;
  if (Math.random() < 0.5) v *= -1;
  return v;
}

function receiveResults(results, selector) {
  window.receiveResults(results, selector);
}

function run_canvaskit_benchmark() {
  CanvasKitInit({
    locateFile: (file) => "https://unpkg.com/canvaskit-wasm@0.19.0/bin/" + file,
  }).then((CanvasKit) => {
    const canvas = document.getElementById("canvaskit");
    canvas.width = WIDTH;
    canvas.height = HEIGHT;

    const surface = CanvasKit.MakeCanvasSurface("canvaskit");
    const paint = new CanvasKit.Paint();
    paint.setColor(CanvasKit.Color4f(0.4, 0.8, 0.4, 0.8));
    paint.setStyle(CanvasKit.PaintStyle.Fill);
    paint.setAntiAlias(true);

    const rects = [];
    for (let i = 0; i < 50000; i++) {
      rects.push(new BouncingRect(WIDTH, HEIGHT));
    }

    const startTime = performance.now();
    let lastMeasureTime = startTime;
    let frameCount = 0;
    const results = new Map();

    function drawFrame() {
      const canvasObj = surface.getCanvas();
      canvasObj.clear(CanvasKit.BLACK);

      rects.forEach((rect) => {
        rect.update();
        rect.draw(CanvasKit, canvasObj, paint);
      });

      surface.flush();
      frameCount++;

      const now = performance.now();
      if (now - lastMeasureTime > 3000) {
        const elapsedSecs = (now - lastMeasureTime) / 1000.0;
        const fps = frameCount / elapsedSecs;
        results.set(rects.length, fps);
        frameCount = 0;
        lastMeasureTime = now;
        console.log("fps: ", fps);

        // for (let i = 0; i < 10000; i++) {
        //   rects.push(new BouncingRect(WIDTH, HEIGHT));
        // }
      }

      // if (now - startTime > 10000) {
      //     receiveResults(results, '#canvaskit-chart');
      //     return;
      // }

      surface.requestAnimationFrame(drawFrame);
    }

    surface.requestAnimationFrame(drawFrame);
  });
}

const startButtonEl = document.querySelector("#start-canvaskit");
startButtonEl.addEventListener("click", () => run_canvaskit_benchmark());
