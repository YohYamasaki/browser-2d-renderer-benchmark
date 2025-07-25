# Browser 2D Rendering Engine Comparison

https://yohyamasaki.github.io/browser-2d-renderer-benchmark/

You need WebGPU to run the demo for pixi(WebGPU) and vello. See https://github.com/gpuweb/gpuweb/wiki/Implementation-Status#chromium-chrome-edge-etc

- Comparison of 2D rendering engines that run on browsers
- Comparing the performance of rendering engines is difficult, so this is only a benchmark from a certain perspective
- Compare FPS by rendering particles bouncing off walls on Canvas
- The implementation is straightforward, so the results of optimization for each library are unknown
- The following box plots were verified using the following two types

```
800px x 600px canvas size、50,000 particles (4px)
MX x86_64 6.14.10-2-liquorix-amd64
AMD Ryzen 7 5800H with Radeon Graphics
Firefox Nightly 142.0a1 
```

![Ryzen 7 5800H](images/ryzen7.png)

```
4096px x 2160px canvas size、150,000 particles (4px)
macos Sequoia 15.5
M1 macbook air 
Chromium 129.0.6668.100
```

![M1](images/m1.png)

## Run locally

1. Install trunk

```
cargo install --locked trunk
```

2. Start dev server

```
trunk serve --release
```
