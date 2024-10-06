use cudarc::driver::{CudaDevice, CudaSlice, DriverError};
use kvikioxide::CuFile;

fn main() -> Result<(), DriverError> {
    let mut cufile = CuFile::new();
    cufile.close();

    let dev = CudaDevice::new(0)?;

    // unsafe initialization of unset memory
    let a: CudaSlice<f32> = unsafe { dev.alloc::<f32>(10) }?;

    // this will have memory initialized as 0
    let b: CudaSlice<f64> = dev.alloc_zeros::<f64>(10)?;

    // initialize with a rust vec
    let c: CudaSlice<usize> = dev.htod_copy(vec![0; 10])?;

    // or finially, initialize with a slice. this is synchronous though.
    let d: CudaSlice<u32> = dev.htod_sync_copy(&[1, 2, 3])?;

    Ok(())
}
