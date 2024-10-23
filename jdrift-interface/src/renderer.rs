use ocl::{Kernel, Device, ProQue, builders::BufferBuilder, prm::Uchar3};
use thiserror::Error;

pub type Pixel = Uchar3;

#[derive(Debug, Clone)]
pub struct Renderer {
    size: [usize; 2],
    fill_kernel: ProQue,
    frame: Vec<Pixel>
}

#[derive(Debug, Error, PartialEq)]
#[error("Could not calculate size due to overflow")]
pub struct Overflow;

impl Renderer {
    const FILL_KERNEL_NAME: &str = "fill";
    const FILL_KERNEL_SOURCE: &str = include_str!("renderer/fill.cl");

    pub fn new(device: Device, size: [usize; 2]) -> Result<Self, Overflow> {
        let fill_kernel = ProQue::builder()
            .src(Self::FILL_KERNEL_SOURCE)
            .dims(size[0] * size[1] * 3)
            .device(device)
            .build()
            // The builder is used correctly
            .unwrap();

        Ok(Self {
            size,
            fill_kernel,
            frame: vec![Uchar3::zero(); Self::get_pixel_count(size)?]
        })
    }

    fn get_pixel_count(size: [usize; 2]) -> Result<usize, Overflow> {
        Ok(size[0].checked_mul(size[1]).ok_or(Overflow)?)
    }

    pub fn get_frame(&self) -> &[Pixel] {
        &self.frame
    }

    pub fn resize(&mut self, size: [usize; 2]) -> Result<(), Overflow> {
        self.frame = vec![Uchar3::zero(); Self::get_pixel_count(size)?];
        self.fill_kernel.set_dims(self.frame.len());
        Ok(())
    }

    /// # OpenCL Kernel Signature
    /// ```cl
    /// __kernel void fill(__global float* buffer, __global ulong2* size)
    /// ```
    pub fn fill(&mut self) {
        unsafe {
            let buffer = self.fill_kernel.create_buffer::<Pixel>().unwrap();
            let size = BufferBuilder::<'_, u64>::new()
                .len(self.frame.len())
                .context(self.fill_kernel.context())
                .build()
                .unwrap();
            let kernel = self.fill_kernel.kernel_builder(Self::FILL_KERNEL_NAME)
                .arg(&buffer)
                .arg(size)
                .build()
                .unwrap();

            kernel.enq();
            buffer.read(&mut self.frame).enq();
        }
    }
}