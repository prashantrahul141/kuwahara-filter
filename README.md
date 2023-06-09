<h1 align="center">Kuwahara Filter</h1>
<p align="center">
The Kuwahara filter is a non-linear smoothing filter used in image processing for adaptive noise reduction. Most filters that are used for image smoothing are linear low-pass filters that effectively reduce noise but also blur out the edges. However the Kuwahara filter is able to apply smoothing on the image while preserving the edges. <a href="https://en.wikipedia.org/wiki/Kuwahara_filter">learn more. </a>
</p>

<h2>Usage</h2>

1. Clone the repository / or download the source code.

```sh
git clone https://github.com/prashantrahul141/kuwahara-filter
```

2. Build using cargo, or download binaries from the <a href="https://github.com/prashantrahul141/kuwahara-filter/releases">releases page</a>.

```sh
cargo build
```

3. Using

put your `"your_image.jpg"` in the build folder, same place the `kuwahara-filter` executable is.

```sh
kuwahara-filter.exe --filename "your_image.jpg" --kernel 12
```

kernel can be any interger above 3 and not divisble by 2. With testing I have found a value somewhere around 7 - 15 works the best.

<h1>Images</h1>
original image:

<img src="https://raw.githubusercontent.com/prashantrahul141/kuwahara-filter/main/meta/original.jpg" width="600"/>

<br>
with kernel = 9:<br>
<img src="https://raw.githubusercontent.com/prashantrahul141/kuwahara-filter/main/meta/9.jpg" width="600"/>

<br>
with kernel = 11:<br>
<img src="https://raw.githubusercontent.com/prashantrahul141/kuwahara-filter/main/meta/11.jpg" width="600"/>

<br>
with kernel = 15:<br>
<img src="https://raw.githubusercontent.com/prashantrahul141/kuwahara-filter/main/meta/15.jpg" width="600"/>

Photo by Flo Dahm: https://www.pexels.com/photo/couple-passing-on-street-541217/
