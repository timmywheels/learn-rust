# Mandelbrot

A simple project, implementing a Mandelbrot set, to demonstrate Rust's concurrency model.

## Resources:
Julia & Mandelbrot Sets Explained: https://acko.net/blog/how-to-fold-a-julia-fractal/

## Benchmarks
All commands were ran with a prefix of `time`, e.g. `time target/release/mandelbrot mandelbrot.png 4000x3000 -1.20,0.35 -1,0.20`

#### 4000 x 3000
- 1 Core: `target/release/mandelbrot mandelbrot.png 4000x3000 -1.20,0.35 -1,0.20  4.97s user 0.02s system 96% cpu 5.178 total`
- 10 Cores: `target/release/mandelbrot mandelbrot.png 4000x3000 -1.20,0.35 -1,0.20  3.90s user 0.02s system 351% cpu 1.113 total`

#### 8000 x 6000
- 1 Core: `target/release/mandelbrot mandelbrot.png 8000x6000 -1.20,0.35 -1,0.20  19.90s user 0.19s system 99% cpu 20.113 total`
- 10 Cores: `target/release/mandelbrot mandelbrot.png 8000x6000 -1.20,0.35 -1,0.20  14.80s user 0.07s system 405% cpu 3.665 total`

#### 16000 x 12000
- 1 Core: `target/release/mandelbrot mandelbrot.png 16000x12000 -1.20,0.35 -1,0.20  78.81s user 0.43s system 99% cpu 1:19.29 total`
- 10 Cores: `target/release/mandelbrot mandelbrot.png 16000x12000 -1.20,0.35 -1,0.20  58.45s user 0.26s system 402% cpu 14.585 total`