<h1 align="center">Rust-Ray-Tracing</h1>

<p align="center">
  <img src="https://user-images.githubusercontent.com/20236175/134610290-fc3b06d8-5222-4f2f-88b2-f44e6cfcc365.png" />
</p>
  
## Introduction
A repo to learn how Ray tracing algorithms works and how to implement them. 
Choosed Rust now for more resources control and speed. Note that I have the same algorithms implemented in python on this [repo](https://github.com/peterhttps/Python-Ray-Tracing)

`Feel free to contribute`

## Overview
- [x] Output an image
- [x] Casting rays into the scene
- [x] Adding sphere and ray create a ray traced image
- [x] Shading with surface normals
- [x] Listing hitable objects
- [x] Creating objects with full reflection
- [x] Antialiasing
- [ ] Improving data structure
- [ ] Adding difuse materials
- [ ] Lambertian Reflection
- [ ] Metalic objects

#### Performance 
- [ ] Improve main for loops
- [ ] Clean code logic
- [ ] Add paralel computation

#### Utils
- [ ] Add progress bar indicator
- [ ] Split files in a smart way

## How to run

If you have cargo installed

```bash
# Build application
$ cargo build

# Run application
$ cargo run
```

Main logic comes of book [Ray Tracing in one Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) and playlist [Programação Dinâmica - Computação Gráfica](https://www.youtube.com/playlist?list=PL5TJqBvpXQv5zNlgvgH2HGuhZHpnkT3oo)
