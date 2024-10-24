use ocl::{Kernel, Device, ProQue, builders::BufferBuilder, prm::Uchar3, Buffer};
use ocl::prm::Ulong2;
use thiserror::Error;

pub type Pixel = Uchar3;

#[derive(Debug, Clone)]
pub struct Renderer {
    size: [u64; 2],
    fill_kernel: ProQue,
    frame: Vec<Pixel>,
    buffer: Buffer<Uchar3>,
    time: f32
}

#[derive(Debug, Error, PartialEq)]
#[error("Could not calculate size due to overflow")]
pub struct Overflow;

impl Renderer {
    const FILL_KERNEL_NAME: &str = "fill";
    const FILL_KERNEL_SOURCE: &str = include_str!("renderer/fill.c");

    pub fn new(device: Device, size: [u64; 2]) -> Result<Self, Overflow> {
        let fill_kernel = ProQue::builder()
            .src(Self::FILL_KERNEL_SOURCE)
            .dims((size[0] * size[1] * 3) as usize)
            .device(device)
            .build()
            // The builder is used correctly
            .unwrap();

        Ok(Self {
            size,
            buffer: fill_kernel.create_buffer().unwrap(), // todo: err
            fill_kernel,
            frame: vec![Uchar3::zero(); Self::get_pixel_count(size)?],
            time: 0.0
        })
    }

    fn get_pixel_count(size: [u64; 2]) -> Result<usize, Overflow> {
        Ok(size[0].checked_mul(size[1]).ok_or(Overflow)? as usize)
    }

    pub fn get_frame(&self) -> &[Pixel] {
        &self.frame
    }

    pub fn resize(&mut self, size: [u64; 2]) -> Result<(), Overflow> {
        self.size = size;
        self.frame = vec![Uchar3::zero(); Self::get_pixel_count(size)?];
        self.fill_kernel.set_dims(self.frame.len());
        self.buffer = self.fill_kernel.create_buffer().unwrap(); // todo: error handle
        Ok(())
    }

    /// # OpenCL Kernel Signature
    /// ```cl
    /// __kernel void fill(__global uchar3* buffer, __global ulong2* size)
    /// ```
    pub fn fill(&mut self) {
        unsafe {
            let kernel = self.fill_kernel.kernel_builder(Self::FILL_KERNEL_NAME)
                .arg(&self.buffer)
                .arg(Ulong2::new(self.size[0], self.size[1]))
                .build()
                .unwrap();
            
            kernel.enq(); // todo: error handling
            self.buffer.read(&mut self.frame).enq();
        }
    }
}