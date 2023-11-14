# Distortoise: A VST3 Distortion Plugin
**Distortoise** amplifies and clips sounds while adding interesting harmonic content with its eight unique algorithms. The plugin also features a **noise** parameter for some texture when clipping.

![screenshot](/assets/screenshot.PNG)

## Installing

Simply download the [latest release](https://github.com/CHRISO1503/distortoise/releases/tag/v1.0.0) and place it into your plugins folder. Rescan your plugins folder in your DAW if necessary.

## Algorithms (for nerds)

Distortoise supports the following algorithms for $x=ds$ where $s\in[-1,1]$ is the incoming sample and $d\in[1,10]$ is the drive parameter.
* **Softclip**:
```math
\textrm{softclip}(x)=\begin{cases}-1&\textrm{for}\ x\leq-1\\ \frac32(x-\frac13x^3)&\textrm{for}\ -1< x<1\\ 1&\textrm{for}\ x>1,\end{cases}
```
* **Hardclip**:
```math
\textrm{hardclip}(x)=\begin{cases}-1&\textrm{for}\ x\leq-1\\ x&\textrm{for}\ -1< x<1\\ 1&\textrm{for}\ x>1,\end{cases}
```
* **Radial**: 
```math
\textrm{radial}(x)=\begin{cases}0&\textrm{for}\ y=0\\ \frac y{|y|}\left(y-\left(\frac y{|y|}\right)^2\right)^\frac12&\textrm{for}\ y\neq0\end{cases}
```
where $y=\textrm{hardclip}(x)$,
* **Chomper**:
```math
\textrm{chomper}(x)=\textrm{hardclip}\left(\frac32x-\frac7{10}x^3\right),
```
* **Sine**:
```math
\textrm{sine}(x)=\sin\left(\frac{\pi x}2\right),
```
* **Stepper**:
```math
\textrm{stepper}(x)=\textrm{hardclip}\left(\frac12\left(x\cos\left(2\pi x\right)+x\right)\right),
```
* **Humpback**:
```math
\textrm{humpback}(x)=\textrm{hardclip}(0.14x^\frac12-1.15x^3+1.9x),
```
* **Absolute**:
```math
\textrm{absolute}(x)=\textrm{hardclip}(|x|).
```

## Building (for developers)

After installing [Rust](https://rustup.rs/), you can compile Distortoise as follows:

```shell
cargo xtask bundle testicular_distortion --release
```
This will also create a CLAP plugin, but this has never been tested.

