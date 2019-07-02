#[derive(PartialEq, Debug)]
pub enum TransformMode {
    Fill { width: u32, height: u32 },
    Fit { width: u32, height: u32 },
    FitWidth(u32),
    FitHeight(u32),
    Limit { width: u32, height: u32 },
}

#[derive(PartialEq, Debug)]
pub struct Coords {
    pub x: f32,
    pub y: f32,
}

#[derive(PartialEq, Debug)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

#[derive(PartialEq, Debug)]
pub struct Offset {
    pub dx: f32,
    pub dy: f32,
}

#[derive(PartialEq, Debug)]
pub struct Dimensions {
    pub size: Size,
    pub origin: Coords,
}

#[derive(PartialEq, Debug)]
pub struct PixelCoords {
    pub x: i64,
    pub y: i64,
}

#[derive(PartialEq, Debug)]
pub struct PixelSize {
    pub width: u32,
    pub height: u32,
}

#[derive(PartialEq, Debug)]
pub struct PixelDimensions {
    pub canvas: PixelSize,
    pub size: PixelSize,
    pub origin: PixelCoords,
}

pub struct Transform {
    input_size: Size,
    canvas_size: Size,
    mode: TransformMode,
    pub relative_center_offset: Offset,
    pub scale: f32,
}

impl Transform {
    pub fn new(input_pixel_size: &PixelSize, mode: TransformMode) -> Self {
        let input_size = Size {
            width: input_pixel_size.width as f32,
            height: input_pixel_size.height as f32,
        };

        let input_ratio = input_size.height / input_size.width;

        let canvas_size = match mode {
            TransformMode::Fill { width, height } | TransformMode::Fit { width, height } => Size {
                width: width as f32,
                height: height as f32,
            },
            TransformMode::FitWidth(width) => Size {
                width: width as f32,
                height: (width as f32) * input_ratio,
            },
            TransformMode::FitHeight(height) => Size {
                width: (height as f32) / input_ratio,
                height: height as f32,
            },
            TransformMode::Limit { width, height } => Size {
                width: ((height as f32) / input_ratio).min(width as f32),
                height: ((width as f32) * input_ratio).min(height as f32),
            },
        };

        Transform {
            input_size: input_size,
            canvas_size: canvas_size,
            mode: mode,
            relative_center_offset: Offset { dx: 0.0, dy: 0.0 },
            scale: 1.0,
        }
    }

    fn get_output_size(&self) -> Size {
        let input_size = &self.input_size;
        let input_ratio = input_size.height / input_size.width;

        let canvas_size = &self.canvas_size;
        let canvas_ratio = canvas_size.height / canvas_size.width;

        let mut output_size = Size {
            width: 0.0,
            height: 0.0,
        };

        match &self.mode {
            TransformMode::Fill {
                width: _,
                height: _,
            } => {
                if canvas_ratio > input_ratio {
                    output_size.height = canvas_size.height
                } else {
                    output_size.width = canvas_size.width
                }
            }
            _ => {
                if input_ratio < 1.0 && input_ratio < canvas_ratio {
                    output_size.width = canvas_size.width
                } else {
                    output_size.height = canvas_size.height
                }
            }
        }

        if output_size.height > 0.0 {
            let ratio = output_size.height / input_size.height;

            output_size.width = ratio * input_size.width;
        } else {
            let ratio = output_size.width / input_size.width;

            output_size.height = ratio * input_size.height;
        }

        output_size.width = output_size.width * self.scale;
        output_size.height = output_size.height * self.scale;

        output_size
    }

    fn get_output_origin(&self, output_size: &Size) -> Coords {
        let center = Coords {
            x: output_size.width / 2.0,
            y: output_size.height / 2.0,
        };

        let canvas_center = Coords {
            x: self.canvas_size.width / 2.0,
            y: self.canvas_size.height / 2.0,
        };

        Coords {
            x: canvas_center.x - center.x
                + (canvas_center.x - center.x) * self.relative_center_offset.dx,
            y: canvas_center.y - center.y
                + (canvas_center.y - center.y) * self.relative_center_offset.dy,
        }
    }

    fn get_output_dimensions(&self) -> Dimensions {
        let output_size = self.get_output_size();

        Dimensions {
            origin: self.get_output_origin(&output_size),
            size: output_size,
        }
    }

    pub fn get_output_pixel_dimensions(&self) -> PixelDimensions {
        let output_dimensions = self.get_output_dimensions();

        PixelDimensions {
            canvas: PixelSize {
                width: self.canvas_size.width.round() as u32,
                height: self.canvas_size.height.round() as u32,
            },
            size: PixelSize {
                width: output_dimensions.size.width.round() as u32,
                height: output_dimensions.size.height.round() as u32,
            },
            origin: PixelCoords {
                x: output_dimensions.origin.x.round() as i64,
                y: output_dimensions.origin.y.round() as i64,
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::{
        Coords, Dimensions, PixelCoords, PixelDimensions, PixelSize, Size, Transform, TransformMode,
    };

    #[test]
    fn fixed_ratios() {
        let transform = Transform::new(
            &PixelSize {
                width: 100,
                height: 100,
            },
            TransformMode::Fill {
                width: 50,
                height: 50,
            },
        );

        let output_dimensions = transform.get_output_dimensions();

        assert_eq!(
            output_dimensions,
            Dimensions {
                origin: Coords { x: 0.0, y: 0.0 },
                size: Size {
                    width: 50.0,
                    height: 50.0
                }
            }
        );
    }

    #[test]
    fn portrait_input_and_portrait_canvas() {
        let transform = Transform::new(
            &PixelSize {
                width: 200,
                height: 300,
            },
            TransformMode::Fill {
                width: 20,
                height: 25,
            },
        );

        let output_dimensions = transform.get_output_dimensions();

        assert_eq!(
            output_dimensions,
            Dimensions {
                origin: Coords { x: 0.0, y: -2.5 },
                size: Size {
                    width: 20.0,
                    height: 30.0
                }
            }
        );
    }

    #[test]
    fn portrait_input_and_longer_portrait_canvas() {
        let transform = Transform::new(
            &PixelSize {
                width: 200,
                height: 300,
            },
            TransformMode::Fill {
                width: 20,
                height: 40,
            },
        );

        let output_dimensions = transform.get_output_dimensions();

        assert_eq!(
            output_dimensions,
            Dimensions {
                origin: Coords {
                    x: -3.333334,
                    y: 0.0
                },
                size: Size {
                    width: 26.666668,
                    height: 40.0
                }
            }
        );
    }

    #[test]
    fn positions_output_with_relative_center() {
        let mut transform = Transform::new(
            &PixelSize {
                width: 200,
                height: 300,
            },
            TransformMode::Fill {
                width: 20,
                height: 25,
            },
        );

        transform.relative_center_offset.dy = -1.0; // Top
        transform.relative_center_offset.dx = 0.0; // Center

        let mut output_dimensions = transform.get_output_dimensions();

        assert_eq!(
            output_dimensions,
            Dimensions {
                origin: Coords { x: 0.0, y: 0.0 },
                size: Size {
                    width: 20.0,
                    height: 30.0
                }
            }
        );

        transform.relative_center_offset.dy = 0.0; // Center
        transform.relative_center_offset.dx = 1.0; // Right

        output_dimensions = transform.get_output_dimensions();
        assert_eq!(
            output_dimensions,
            Dimensions {
                origin: Coords { x: 0.0, y: -2.5 },
                size: Size {
                    width: 20.0,
                    height: 30.0
                }
            }
        );
    }

    #[test]
    fn fits_landscape_image() {
        let transform = Transform::new(
            &PixelSize {
                width: 300,
                height: 200,
            },
            TransformMode::Fit {
                width: 20,
                height: 30,
            },
        );

        let output_dimensions = transform.get_output_dimensions();

        assert_eq!(
            output_dimensions,
            Dimensions {
                origin: Coords {
                    x: 0.0,
                    y: 8.333333
                },
                size: Size {
                    width: 20.0,
                    height: 13.333334
                }
            }
        );
    }

    #[test]
    fn fits_to_width_only() {
        let transform = Transform::new(
            &PixelSize {
                width: 300,
                height: 200,
            },
            TransformMode::FitWidth(20),
        );

        let output_dimensions = transform.get_output_pixel_dimensions();

        assert_eq!(
            output_dimensions,
            PixelDimensions {
                canvas: PixelSize {
                    width: 20,
                    height: 13
                },
                origin: PixelCoords { x: 0, y: 0 },
                size: PixelSize {
                    width: 20,
                    height: 13
                }
            }
        );
    }

    #[test]
    fn fits_to_height_only() {
        let transform = Transform::new(
            &PixelSize {
                width: 300,
                height: 200,
            },
            TransformMode::FitHeight(20),
        );

        let output_dimensions = transform.get_output_pixel_dimensions();

        assert_eq!(
            output_dimensions,
            PixelDimensions {
                canvas: PixelSize {
                    width: 30,
                    height: 20
                },
                origin: PixelCoords { x: 0, y: 0 },
                size: PixelSize {
                    width: 30,
                    height: 20
                }
            }
        );
    }

    #[test]
    fn limit_to_scaled_square() {
        let mut transform = Transform::new(
            &PixelSize {
                width: 200,
                height: 300,
            },
            TransformMode::Limit {
                width: 60,
                height: 60,
            },
        );

        transform.scale = 1.2;

        let output_dimensions = transform.get_output_dimensions();

        assert_eq!(
            output_dimensions,
            Dimensions {
                origin: Coords { x: -4.0, y: -6.0 },
                size: Size {
                    width: 48.0,
                    height: 72.0
                }
            }
        );
    }
}
