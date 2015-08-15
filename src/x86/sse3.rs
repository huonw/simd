use sixty_four::*;
use super::super::*;

#[allow(dead_code)]
extern "platform-intrinsic" {
    fn x86_mm_addsub_pd(x: f64x2, y: f64x2) -> f64x2;
    fn x86_mm_addsub_ps(x: f32x4, y: f32x4) -> f32x4;
    fn x86_mm_hadd_pd(x: f64x2, y: f64x2) -> f64x2;
    fn x86_mm_hadd_ps(x: f32x4, y: f32x4) -> f32x4;
    fn x86_mm_hsub_pd(x: f64x2, y: f64x2) -> f64x2;
    fn x86_mm_hsub_ps(x: f32x4, y: f32x4) -> f32x4;
}
