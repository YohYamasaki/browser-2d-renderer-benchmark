let isRunning = false;

class BouncingRect {
  constructor(canvasWidth, canvasHeight, boxSize) {
    this.canvasWidth = canvasWidth;
    this.canvasHeight = canvasHeight;
    this.box_x = (canvasWidth - boxSize) * Math.random();
    this.box_y = (canvasHeight - boxSize) * Math.random();
    this.box_size = boxSize;
    this.velocity_x = getRandomVelocity();
    this.velocity_y = getRandomVelocity();
  }

  update() {
    if (this.box_x <= 0 || this.box_x + this.box_size >= this.canvasWidth) {
      this.velocity_x *= -1;
    }
    if (this.box_y <= 0 || this.box_y + this.box_size >= this.canvasHeight) {
      this.velocity_y *= -1;
    }
    this.box_x += this.velocity_x;
    this.box_y += this.velocity_y;
  }

  draw(CanvasKit, canvas, paint) {
    const rect = CanvasKit.LTRBRect(
      this.box_x,
      this.box_y,
      this.box_x + this.box_size,
      this.box_y + this.box_size
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

function run_canvaskit_benchmark(
  canvasWidth,
  canvasHeight,
  boxSize,
  boxNumber
) {
  CanvasKitInit({
    locateFile: (file) => "https://unpkg.com/canvaskit-wasm@0.19.0/bin/" + file,
  }).then((CanvasKit) => {
    const canvasWrapEl = document.getElementById("window-wrap");
    const canvasEl = document.createElement("canvas");
    canvasEl.setAttribute("id", "canvaskit");
    canvasEl.width = canvasWidth;
    canvasEl.height = canvasHeight;
    canvasWrapEl.append(canvasEl);

    const surface = CanvasKit.MakeCanvasSurface("canvaskit");
    const paint = new CanvasKit.Paint();
    paint.setColor(CanvasKit.Color4f(0.4, 0.8, 0.4, 0.8));
    paint.setStyle(CanvasKit.PaintStyle.Fill);
    paint.setAntiAlias(true);

    const rects = [];
    for (let i = 0; i < boxNumber; i++) {
      rects.push(new BouncingRect(canvasWidth, canvasHeight, boxSize));
    }

    const startTime = performance.now();
    let lastMeasureTime = startTime;
    let frameCount = 0;

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
      if (now - lastMeasureTime > 1000) {
        const elapsedSecs = (now - lastMeasureTime) / 1000.0;
        const fps = Math.round((frameCount / elapsedSecs) * 100) / 100;
        frameCount = 0;
        lastMeasureTime = now;

        const fpsEl = document.querySelector("#fps");
        fpsEl.textContent = fps;
      }
      if (isRunning) {
        console.log("redraw");
        surface.requestAnimationFrame(drawFrame);
      }
    }
    isRunning = true;
    surface.requestAnimationFrame(drawFrame);
  });
}

const startButtonEl = document.querySelector("#start-canvaskit");
startButtonEl.addEventListener("click", () => {
  const canvasWidth = parseInt(document.querySelector("#canvas-width").value);
  const canvasHeight = parseInt(document.querySelector("#canvas-height").value);
  const boxSize = parseInt(document.querySelector("#box-size").value);
  const boxNumber = parseInt(document.querySelector("#box-number").value);
  run_canvaskit_benchmark(canvasWidth, canvasHeight, boxSize, boxNumber);
});

const stopButtonEl = document.querySelector("#stop");
stopButtonEl.addEventListener("click", () => {
  const canvaskitEl = document.querySelector("#canvaskit");
  if (canvaskitEl) {
    isRunning = false;
    console.log("isRunning", isRunning);
    canvaskitEl.remove();
  }
});
