
use libm::*;

pub trait RsMath {
    fn sqrt(&self) -> Self;

    fn cbrt(&self) -> Self;

    fn abs(&self) -> Self;

    fn floor(&self) -> Self;

    fn round(&self) -> Self;

    fn powf(&self, y: Self) -> Self;

    fn powi(&self, y: i32) -> Self;

    fn log10(&self) -> Self;

    fn sin(&self) -> Self;

    fn cos(&self) -> Self;

    fn tan(&self) -> Self;

    fn asin(&self) -> Self;

    fn acos(&self) -> Self;

    fn atan(&self) -> Self;

    fn atan2(&self, x: Self) -> Self;

}

impl RsMath for f64 {
    fn sqrt(&self) -> Self {
        sqrt(*self)
    }

    fn cbrt(&self) -> Self {
        cbrt(*self)
    }

    fn abs(&self) -> Self {
        fabs(*self)
    }

    fn floor(&self) -> Self {
        floor(*self)
    }

    fn round(&self) -> Self {
        round(*self)
    }

    fn powf(&self, y: Self) -> Self {
        pow(*self, y)
    }

    fn powi(&self, y: i32) -> Self {
        pow(*self, y as f64)
    }

    fn log10(&self) -> Self {
        log10(*self)
    }

    fn sin(&self) -> f64 {
        sin(*self)
    }

    fn cos(&self) -> Self {
        cos(*self)
    }

    fn tan(&self) -> Self {
        tan(*self)
    }

    fn asin(&self) -> Self {
        asin(*self)
    }

    fn acos(&self) -> Self {
        acos(*self)
    }

    fn atan(&self) -> Self {
        atan(*self)
    }

    fn atan2(&self, x: Self) -> Self {
        atan2(*self, x)
    }
}
