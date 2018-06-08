extern crate sidequest;
extern crate gif;
extern crate failure;

use failure::Error;
use gif::{SetParameter, Encoder, Frame, Repeat};
use sidequest::core::*;
use std::f64::consts::{FRAC_PI_4, PI};
use std::fs::File;

fn main() -> Result<(), Error> {
    const ELEMS: usize = 3;
    let size = 512;
    let mut img = ImgVec::new(vec![[0; ELEMS]; size * size], size, size);

    let scene = &[
        Sphere::new(Point3::new(0., -2., 0.), 3.),
        Sphere::new(Point3::new(0., 3., 0.), 1.5),
        Sphere::new(Point3::new(4., 0., 0.), 1.),
        Sphere::new(Point3::new(-4., 0., 0.), 1.),
        Sphere::new(Point3::new(0., 0., 4.), 1.),
        Sphere::new(Point3::new(0., 0., -4.), 1.),
    ];

    let mut gif = Encoder::new(File::create("demo.gif")?, size as u16, size as u16, &[])?;
    gif.set(Repeat::Infinite)?;

    let frames = 60;
    for i in 0..frames {
        {
            let imgref = RasterLayer::new(img.as_mut());
            let angle = 2. * PI * (i as f64 / frames as f64);
            let cam = PerspectiveCamera::new(
                Isometry3::new_observer_frame(
                    &Point3::new(angle.sin() * 12., 8., angle.cos() * 12.),
                    &Point3::new(0., 0., 0.),
                    &Vector3::new(0., 1., 0.),
                ),
                FRAC_PI_4,
                0.1,
                100.,
            );

            fn splat(x: f64) -> u8 {
                let x = x / 2. + 0.5;
                let x = x.min(1.).max(0.);
                (x * 255.) as u8
            }

            render_spheres(scene, &cam, imgref, |b| match b {
                Some(i) => [splat(i.norm[0]), splat(i.norm[1]), splat(i.norm[2])],
                None => [128; 3],
            });
        }

        use std::slice::from_raw_parts;
        let buf: *const [u8; ELEMS] = img.buf.as_ptr();

        let mut frame = Frame::from_rgb(
            img.width() as u16,
            img.height() as u16,
            unsafe { from_raw_parts(buf as *mut u8, img.buf.len() * ELEMS) },
        );
        frame.delay = 100 / 30;

        gif.write_frame(&frame)?;
    }

    Ok(())
}