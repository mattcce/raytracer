# raytracer

A personal pet project.

## Description

This project is intended to be a relatively simple way to declaratively create
3D worlds and generate images (and potentially generate animations) of said
worlds.

The core foundation of the ray tracer was made with the following reference:
- J. Buck, _The Ray Tracer Challenge,_ 1st ed., The Pragmatic Programmers, Flower Mound, TX, 2019.

## Project Status

Far from done. As of writing, the project is very barebones. The remaining core
features to implement include (roughly in order of importance):
- Implementing IO functionality with a custom data format and parser as well
as the full OBJ specification, along with implementing other raster formats
for outputting.
- Implementing more realistic objects, including, but is not limited to,
volumetric effects, perspective cameras, and area/volumetric lights.
- Refactor/restructure program to better organise types in a more intuitive
manner.
- Public API for using this as a library.

These are features of low importance, but should eventually make it in:
- Supersampling
- Texture maps
- Post-processing effects
- Animating
- Ray tracing Optimisations (e.g. pre-tracing, multiprocessing)
- Math Optimisations (e.g matrix multiplication)

## Public API

Currently, no public API is available/has been implemented.
