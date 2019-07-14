![fill mode](docs/images/rusty.svg)

# rust-image-worker

Dynamically crop, resize and cache images, all on the CDN

## Why

The scenario:

- You need to display images optimized for the layout of your website.
- Perhaps these images get uploaded by your users, or they come from an external image provider.
- You don't want to send images larger than what is neccesary, because you are considerate of users who may be accessing your site from slower connections such as mobiles.
- Maybe you don't know what the size of all the source images are, but you still want them to fit neatly into your design.
- Or, maybe you want to allow your users to change how their profile images are cropped, to move and scale within fixed dimensions, without having to upload again.

I've used and built such services in the past. I thought it would be a straightforward yet useful thing to build with [Rust](https://www.rust-lang.org) and [WASM](https://webassembly.org)) on [Cloudflare workers](https://www.cloudflare.com/en-gb/products/cloudflare-workers/).

Rust has great native crates that do not need to pull in shared libraries or call out to other processes.

With workers we are able to cache the source and resulting images directly on the CDN.

### Caveat

This software is currently used only for demonstration purposes and you should be aware of the [limitations](#limitations) before using it for real.

## How to use

### Deploying the worker

1. Follow instructions to install 🤠[wrangler](https://github.com/cloudflare/wrangler).
2. Add your [account id and zone id](https://workers.cloudflare.com/docs/quickstart/api-keys/) to the [`wrangler.toml`](wrangler.toml)
3. Run `$ wrangler publish`

You should see something like:

```
🥳 Successfully published your script.
🥳 Successfully made your script available at image-worker...workers.dev
✨ Success! Your worker was successfully published. ✨
```

### Calling the worker

You will be able to call the worker at the domain provided,e.g. [http://image-worker...workers.dev](http://factorymethod.uk/image).

The URL path should be formatted as an image filename with a file extension signifying the target image format. Supported output formats are PNG (`.png`) and JPEG (`.jpg` or `.jpeg`).

The query parameters should include a combination of:

- **origin**: the full _URL_ to the source image (required)
- **mode**: one of _fill_, _fit_ and _limit_ (required, see [modes](#modes) for examples)
- **width**, **height**: the desired dimensions (both required when mode is _fill_ or _limit_, either one or both for _fit_)
- **dx**, **dy**: the relative position when the image is cropped, numbers between _-1.0_ (left/top) and _1.0_ (right/bottom) (default: _0.0_, center)
- **scale**: a positive rational number to scale the source image by (default: _1.0_)
- **bg**: a color in [hex triplet](https://en.wikipedia.org/wiki/Web_colors#Hex_triplet) format (default: transparent)

## Modes

### Fill mode

The source image is cropped in order to ensure that the full _width_ and _height_ is filled. The source image can be positioned using relative center offset _dx_ and _dy_.

![fill mode](docs/images/fill.png)

Examples:

| URL                                                                                                                                                                                                                               | Image                                                                                                                              |
| --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| [https://.../image.png?<br/>mode=fill&<br/>width=180&<br/>height=200&<br/>origin=https://.../test_pattern.png](https://factorymethod.uk/image.png?mode=fill&width=180&height=200&origin=http://factorymethod.uk/test_pattern.png) | ![fill example](https://factorymethod.uk/image.png?mode=fill&width=180&height=200&origin=http://factorymethod.uk/test_pattern.png) |
| [https://.../image.jpg?<br/>mode=fill&<br/>width=180&<br/>height=200&<br/>origin=https://.../Apollo_17.jpeg](https://factorymethod.uk/image.png?mode=fill&width=180&height=200&origin=http://factorymethod.uk/Apollo_17.jpeg)     | ![fill example](https://factorymethod.uk/image.jpeg?mode=fill&width=180&height=200&origin=http://factorymethod.uk/Apollo_17.jpeg)  |

### Fit mode

The output image is exactly sized according to the given width and height, with no cropping of the source image. The source image can be positioned using relative center offset _dx_ and _dy_.

![fit mode](docs/images/fit.png)

Examples:

| URL                                                                                                                                                                                                                                                                                                                      | Image                                                                                                                                                |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------- |
| [https://.../image.png?<br/>mode=fit&<br/>width=180&<br/>height=200&<br/>bg=abc&<br/>origin=https://.../test_pattern.png](https://factorymethod.uk/image.png?mode=fit&width=180&height=200&bg=abc&origin=http://factorymethod.uk/test_pattern.png)                                                                       | ![fit example](https://factorymethod.uk/image.png?mode=fit&width=180&height=200&bg=abc&origin=http://factorymethod.uk/test_pattern.png)              |
| [https://.../image.jpg?<br/>mode=fit&<br/>width=180&<br/>height=200&<br/>bg=abc&<br/>origin=https://.../Apollo_17.jpeg](https://factorymethod.uk/image.png?mode=fit&width=180&height=200&bg=abc&origin=http://factorymethod.uk/Apollo_17.jpeg)                                                                           | ![fit example](https://factorymethod.uk/image.jpeg?mode=fit&width=180&height=200&bg=abc&origin=http://factorymethod.uk/Apollo_17.jpeg)               |
| Scaled up and cropped to bottom-left<br/>[https://.../image.jpg?<br/>mode=fit&<br/>width=180&<br/>height=200&<br/>scale=1.5&<br/>dx=-1&dy=1&<br/>origin=https://.../Apollo_17.jpeg](https://factorymethod.uk/image.png?mode=fit&width=180&height=200&dx=-1&dy=1&scale=1.5&origin=http://factorymethod.uk/Apollo_17.jpeg) | ![fit example](https://factorymethod.uk/image.jpeg?mode=fit&width=180&height=200&dx=-1&dy=1&scale=1.5&origin=http://factorymethod.uk/Apollo_17.jpeg) |

### Limit mode

The source image is scaled to fit within the given _width_ and _height_.

![limit mode](docs/images/limit.png)

Examples:

| URL                                                                                                                                                                                                                                                                                                                          | Image                                                                                                                                                    |
| ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [https://.../image.png?<br/>mode=limit&<br/>width=180&<br/>height=200&<br/>origin=https://.../test_pattern.png](https://factorymethod.uk/image.png?mode=limit&width=180&height=200&&origin=http://factorymethod.uk/test_pattern.png)                                                                                         | ![limit example](https://factorymethod.uk/image.png?mode=limit&width=180&height=200&origin=http://factorymethod.uk/test_pattern.png)                     |
| [https://.../image.jpg?<br/>mode=limit&<br/>width=180&<br/>height=200&<br/>origin=https://.../Apollo_17.jpeg](https://factorymethod.uk/image.png?mode=limit&width=180&height=200&&origin=http://factorymethod.uk/Apollo_17.jpeg)                                                                                             | ![limit example](https://factorymethod.uk/image.jpeg?mode=limit&width=180&height=200&origin=http://factorymethod.uk/Apollo_17.jpeg)                      |
| Scaled up and cropped to bottom-left<br/>[https://.../image.jpg?<br/>mode=limit&<br/>width=180&<br/>height=200&<br/>scale=1.5&<br/>dx=-1&dy=1&<br/>origin=https://.../Apollo_17.jpeg](https://factorymethod.uk/image.png?mode=limit&width=180&height=200&dx=-1&dy=1&scale=1.5&origin=http://factorymethod.uk/Apollo_17.jpeg) | ![limit example](https://factorymethod.uk/image.jpeg?mode=limit&width=180&height=200&dx=-1&dy=1&scale=1.5&origin=http://factorymethod.uk/Apollo_17.jpeg) |

## Limitations

- Cloudflare workers are [limited](https://developers.cloudflare.com/workers/writing-workers/resource-limits/) in the amount of CPU time they are allowed to take per request (between 5ms for free and 50ms for business/enterprise accounts). This means that large images (> 1000 pixels in width or height), sometimes run out of processing time.

## Development

To run pure Rust tests:

```
$ cargo test
```

And for a headless browser smoke test using Chrome:

```
$ wasm-pack test --headless --chrome
```

## License

Apache 2.0
