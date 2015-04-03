extern crate rand;

use sys::pixels as ll;

#[derive(PartialEq)] #[allow(raw_pointer_derive, missing_copy_implementations)]
pub struct Palette {
    raw: *const ll::SDL_Palette
}

impl_raw_accessors!((Palette, *const ll::SDL_Palette));

#[derive(PartialEq, Clone, Copy)]
pub enum Color {
    RGB(u8, u8, u8),
    RGBA(u8, u8, u8, u8)
}

impl Color {
    pub fn to_u32(&self, format: &PixelFormat) -> u32 {
        match self {
            &Color::RGB(r, g, b) => {
                unsafe { ll::SDL_MapRGB(format.raw, r, g, b) }
            }
            &Color::RGBA(r, g, b, a) => {
                unsafe { ll::SDL_MapRGBA(format.raw, r, g, b, a) }
            }
        }
    }

    pub fn from_u32(format: &PixelFormat, pixel: u32) -> Color {
        let r: u8 = 0;
        let g: u8 = 0;
        let b: u8 = 0;
        let a: u8 = 0;

        unsafe {
            ll::SDL_GetRGBA(pixel, format.raw, &r, &g, &b, &a)
        };
        Color::RGBA(r, g, b, a)
    }

    pub fn get_rgb(&self) -> (u8, u8, u8) {
        match self {
            &Color::RGB(r, g, b) => (r, g, b),
            &Color::RGBA(r, g, b, _) => (r, g, b)
        }
    }
}

impl rand::Rand for Color {
    fn rand<R: rand::Rng>(rng: &mut R) -> Color {
        if rng.gen() { Color::RGBA(rng.gen(), rng.gen(), rng.gen(), rng.gen()) }
        else { Color::RGB(rng.gen(), rng.gen(), rng.gen()) }
    }
}

#[derive(PartialEq)] #[allow(raw_pointer_derive, missing_copy_implementations)]
pub struct PixelFormat {
    raw: *const ll::SDL_PixelFormat
}

impl_raw_accessors!((PixelFormat, *const ll::SDL_PixelFormat));
impl_raw_constructor!((PixelFormat, PixelFormat (raw: *const ll::SDL_PixelFormat)));

#[derive(Copy, Clone, PartialEq, Debug, FromPrimitive)]
pub enum PixelFormatEnum {
    Unknown = ll::SDL_PIXELFORMAT_UNKNOWN as isize,
    Index1LSB = ll::SDL_PIXELFORMAT_INDEX1LSB as isize,
    Index1MSB = ll::SDL_PIXELFORMAT_INDEX1MSB as isize,
    Index4LSB = ll::SDL_PIXELFORMAT_INDEX4LSB as isize,
    Index4MSB = ll::SDL_PIXELFORMAT_INDEX4MSB as isize,
    Index8 = ll::SDL_PIXELFORMAT_INDEX8 as isize,
    RGB332 = ll::SDL_PIXELFORMAT_RGB332 as isize,
    RGB444 = ll::SDL_PIXELFORMAT_RGB444 as isize,
    RGB555 = ll::SDL_PIXELFORMAT_RGB555 as isize,
    BGR555 = ll::SDL_PIXELFORMAT_BGR555 as isize,
    ARGB4444 = ll::SDL_PIXELFORMAT_ARGB4444 as isize,
    RGBA4444 = ll::SDL_PIXELFORMAT_RGBA4444 as isize,
    ABGR4444 = ll::SDL_PIXELFORMAT_ABGR4444 as isize,
    BGRA4444 = ll::SDL_PIXELFORMAT_BGRA4444 as isize,
    ARGB1555 = ll::SDL_PIXELFORMAT_ARGB1555 as isize,
    RGBA5551 = ll::SDL_PIXELFORMAT_RGBA5551 as isize,
    ABGR1555 = ll::SDL_PIXELFORMAT_ABGR1555 as isize,
    BGRA5551 = ll::SDL_PIXELFORMAT_BGRA5551 as isize,
    RGB565 = ll::SDL_PIXELFORMAT_RGB565 as isize,
    BGR565 = ll::SDL_PIXELFORMAT_BGR565 as isize,
    RGB24 = ll::SDL_PIXELFORMAT_RGB24 as isize,
    BGR24 = ll::SDL_PIXELFORMAT_BGR24 as isize,
    RGB888 = ll::SDL_PIXELFORMAT_RGB888 as isize,
    RGBX8888 = ll::SDL_PIXELFORMAT_RGBX8888 as isize,
    BGR888 = ll::SDL_PIXELFORMAT_BGR888 as isize,
    BGRX8888 = ll::SDL_PIXELFORMAT_BGRX8888 as isize,
    ARGB8888 = ll::SDL_PIXELFORMAT_ARGB8888 as isize,
    RGBA8888 = ll::SDL_PIXELFORMAT_RGBA8888 as isize,
    ABGR8888 = ll::SDL_PIXELFORMAT_ABGR8888 as isize,
    BGRA8888 = ll::SDL_PIXELFORMAT_BGRA8888 as isize,
    ARGB2101010 = ll::SDL_PIXELFORMAT_ARGB2101010 as isize,
    YV12 = ll::SDL_PIXELFORMAT_YV12 as isize,
    IYUV = ll::SDL_PIXELFORMAT_IYUV as isize,
    YUY2 = ll::SDL_PIXELFORMAT_YUY2 as isize,
    UYVY = ll::SDL_PIXELFORMAT_UYVY as isize,
    YVYU = ll::SDL_PIXELFORMAT_YVYU as isize
}

impl PixelFormatEnum {
    /// Calculates the total byte size of an image buffer, given its pitch
    /// and height.
    pub fn byte_size_from_pitch_and_height(&self, pitch: usize, height: usize) -> usize {
        match *self {
            PixelFormatEnum::YV12 | PixelFormatEnum::IYUV => {
                // YUV is 4:2:0.
                // `pitch` is the width of the Y component, and
                // `height` is the height of the Y component.
                // U and V have half the width and height of Y.
                pitch * height + 2 * (pitch / 2 * height / 2)
            },
            _ => pitch * height
        }
    }

    pub fn byte_size_of_pixels(&self, num_of_pixels: usize) -> usize {
        match *self {
            PixelFormatEnum::RGB332
                => num_of_pixels * 1,
            PixelFormatEnum::RGB444 | PixelFormatEnum::RGB555 |
            PixelFormatEnum::BGR555 | PixelFormatEnum::ARGB4444 |
            PixelFormatEnum::RGBA4444 | PixelFormatEnum::ABGR4444 |
            PixelFormatEnum::BGRA4444 | PixelFormatEnum::ARGB1555 |
            PixelFormatEnum::RGBA5551 | PixelFormatEnum::ABGR1555 |
            PixelFormatEnum::BGRA5551 | PixelFormatEnum::RGB565 |
            PixelFormatEnum::BGR565
                => num_of_pixels * 2,
            PixelFormatEnum::RGB24 | PixelFormatEnum::BGR24
                => num_of_pixels * 3,
            PixelFormatEnum::RGB888 | PixelFormatEnum::RGBX8888 |
            PixelFormatEnum::BGR888 | PixelFormatEnum::BGRX8888 |
            PixelFormatEnum::ARGB8888 | PixelFormatEnum::RGBA8888 |
            PixelFormatEnum::ABGR8888 | PixelFormatEnum::BGRA8888 |
            PixelFormatEnum::ARGB2101010
                => num_of_pixels * 4,
            // YUV formats
            // FIXME: rounding error here?
            PixelFormatEnum::YV12 | PixelFormatEnum::IYUV
                => num_of_pixels / 2 * 3,
            PixelFormatEnum::YUY2 | PixelFormatEnum::UYVY |
            PixelFormatEnum::YVYU
                => num_of_pixels * 2,
            // Unsupported formats
            PixelFormatEnum::Index8
                => num_of_pixels * 1,
            PixelFormatEnum::Unknown | PixelFormatEnum::Index1LSB |
            PixelFormatEnum::Index1MSB | PixelFormatEnum::Index4LSB |
            PixelFormatEnum::Index4MSB
                => panic!("not supported format: {:?}", *self),
        }
    }

    pub fn byte_size_per_pixel(&self) -> usize {
        match *self {
            PixelFormatEnum::RGB332
                => 1,
            PixelFormatEnum::RGB444 | PixelFormatEnum::RGB555 |
            PixelFormatEnum::BGR555 | PixelFormatEnum::ARGB4444 |
            PixelFormatEnum::RGBA4444 | PixelFormatEnum::ABGR4444 |
            PixelFormatEnum::BGRA4444 | PixelFormatEnum::ARGB1555 |
            PixelFormatEnum::RGBA5551 | PixelFormatEnum::ABGR1555 |
            PixelFormatEnum::BGRA5551 | PixelFormatEnum::RGB565 |
            PixelFormatEnum::BGR565
                => 2,
            PixelFormatEnum::RGB24 | PixelFormatEnum::BGR24
                => 3,
            PixelFormatEnum::RGB888 | PixelFormatEnum::RGBX8888 |
            PixelFormatEnum::BGR888 | PixelFormatEnum::BGRX8888 |
            PixelFormatEnum::ARGB8888 | PixelFormatEnum::RGBA8888 |
            PixelFormatEnum::ABGR8888 | PixelFormatEnum::BGRA8888 |
            PixelFormatEnum::ARGB2101010
                => 4,
            // YUV formats
            PixelFormatEnum::YV12 | PixelFormatEnum::IYUV
                => 2,
            PixelFormatEnum::YUY2 | PixelFormatEnum::UYVY |
            PixelFormatEnum::YVYU
                => 2,
            // Unsupported formats
            PixelFormatEnum::Index8
                => 1,
            PixelFormatEnum::Unknown | PixelFormatEnum::Index1LSB |
            PixelFormatEnum::Index1MSB | PixelFormatEnum::Index4LSB |
            PixelFormatEnum::Index4MSB
                => panic!("not supported format: {:?}", *self),
        }
    }
}
