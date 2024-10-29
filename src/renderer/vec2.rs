use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ndc(f32);

impl Ndc {
    pub fn new(number: f32) -> Result<Self, ()> {
        if -1.0 >= number && number >= 1.0 { Err(()) }
        else { Ok(Self(number)) }
    }

    pub fn new_clamped(number: f32) -> Self {
        Self(number).clamp()
    }

    pub fn clamp(self) -> Self {
        if self.0 > 1.0 { Self(1.0) }
        else if self.0 < -1.0 { Self(-1.0) }
        else { self }
    }

    pub fn get(&self) -> f32 {
        self.0
    }
    
    pub fn square(self) -> f32 {
        self.0.sqrt()
    }
}

macro_rules! implement_ndc_operator {
    ($operator: tt, $trait: ty, $method: ident) => {
        impl $trait for Ndc {
            type Output = Self;

            fn $method(self, rhs: Self) -> Self::Output {
                Self::new_clamped(self.0 $operator rhs.0)
            }
        }
    };
}

implement_ndc_operator!(+, Add, add);
implement_ndc_operator!(-, Sub, sub);
implement_ndc_operator!(*, Mul, mul);
implement_ndc_operator!(/, Div, div);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2([Ndc; 2]);

impl Vec2 {
    pub fn new(first: Ndc, second: Ndc) -> Self {
        Self([ first, second ])
    }

    pub fn distance(self, other: Self) -> f32 {
        let first = other.0[0] - self.0[0];
        let second = other.0[1] - self.0[1];
        (first + second).square()
    }
    
    pub fn x(self) -> Ndc {
        self.0[0]
    }
    
    pub fn y(self) -> Ndc {
        self.0[1]
    }
    
    pub fn slope(self, other: Self) -> f32 {
        let first = other.y().get() - self.y().get();
        let second = other.x().get() - self.x().get();
        first / second
    }
}

macro_rules! implement_vec2_operator {
    ($operator: tt, $trait: ty, $method: ident) => {
        impl $trait for Vec2 {
            type Output = Self;

            fn $method(self, rhs: Self) -> Self::Output {
                Self([ self.0[0] $operator rhs.0[0], self.0[1] $operator rhs.0[1] ])
            }
        }
    }
}

implement_vec2_operator!(+, Add, add);
implement_vec2_operator!(-, Sub, sub);
implement_vec2_operator!(*, Mul, mul);
implement_vec2_operator!(/, Div, div);

#[macro_export]
macro_rules! vec2 {
     ($first: expr, $second: expr) => {
         Vec2::new(Ndc::new_clamped($first), Ndc::new_clamped($second))
     }
}