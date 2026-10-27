//! T2accel.C: five source helpers on canonical contiguous matrices.
//! native/backends.c fixes the measured provider ABI; no timing core is linked.
use crate::tk_matrix::BlasMatrix;
use std::cell::Cell;

pub const NATIVE_DEFAULT: i32 = if cfg!(feature = "qr-default") { 2 } else { 1 };
thread_local! { static MODE: Cell<i32> = const { Cell::new(0) }; }
pub fn acceleration() -> i32 {
    MODE.get()
}
/// Explicit call scope for direct toolkit consumers and FitOptions. Restores
/// selection even on error/panic; does not acquire the clock process mutex.
pub fn with_acceleration<T>(mode: i32, operation: impl FnOnce() -> T) -> T {
    let _scope = select(mode);
    operation()
}
pub(crate) struct AccelerationScope(i32);
impl Drop for AccelerationScope {
    fn drop(&mut self) {
        MODE.set(self.0);
    }
}
pub(crate) fn select(mode: i32) -> AccelerationScope {
    AccelerationScope(MODE.replace(mode))
}
pub fn blas_enabled() -> bool {
    cfg!(feature = "blas") && acceleration() != 0
}
pub fn lapack_enabled() -> bool {
    cfg!(feature = "lapack") && acceleration() != 0
}
pub fn qr_enabled() -> bool {
    cfg!(feature = "lapack") && acceleration() == 2
}

#[cfg(feature = "blas")]
unsafe extern "C" {
    fn t2_dgemv(rows: i32, cols: i32, a: *const f64, x: *const f64, y: *mut f64);
    fn t2_dgemm(rows: i32, inner: i32, cols: i32, a: *const f64, b: *const f64, c: *mut f64);
}
#[cfg(feature = "lapack")]
unsafe extern "C" {
    fn t2_dpotrf(n: i32, a: *mut f64) -> i32;
    fn t2_dpotf2(n: i32, a: *mut f64) -> i32;
    fn t2_dtptri(n: i32, a: *mut f64, upper: i32) -> i32;
    fn t2_dgels(rows: i32, cols: i32, a: *mut f64, b: *mut f64, work: *mut f64, lwork: i32) -> i32;
    fn t2_dtrmm(n: i32, alpha: f64, a: *mut f64, b: *mut f64);
}
#[cfg(feature = "blas")]
fn dim(n: usize) -> Result<i32, String> {
    i32::try_from(n).map_err(|_| "provider LP64 dimension overflow".into())
}
#[cfg(feature = "lapack")]
fn square(m: &BlasMatrix) -> Result<i32, String> {
    if m.get_blas_rows() != m.get_blas_cols() {
        return Err("factorization requires a square matrix".into());
    }
    dim(m.get_blas_rows())
}

/// Source DPOTRF upper column-major = lower row-major. The other triangle stays.
pub fn accel_cholfac(m: &mut BlasMatrix) -> Result<i32, String> {
    #[cfg(feature = "lapack")]
    {
        let n = square(m)?;
        // SAFETY: contiguous n*n binary64 cells with stride n, unique COW borrow;
        // measured C shim uses four-byte integers and retains no pointers.
        Ok(unsafe { t2_dpotrf(n, m.cells_mut().as_mut_ptr()) })
    }
    #[cfg(not(feature = "lapack"))]
    {
        let _ = m;
        Err("LAPACK backend unavailable".into())
    }
}
/// Source unused-but-public DPOTF2/DTPTRI path: lower inverse, upper zeros.
pub fn accel_uinv(m: &mut BlasMatrix) -> Result<i32, String> {
    #[cfg(feature = "lapack")]
    {
        let n = square(m)?;
        // SAFETY: validated square extent, unique live storage, synchronous ABI.
        let info = unsafe { t2_dpotf2(n, m.cells_mut().as_mut_ptr()) };
        if info != 0 {
            return Ok(info);
        }
        let k = n as usize;
        let mut packed = Vec::with_capacity(k * (k + 1) / 2);
        for j in 0..k {
            for i in j..k {
                packed.push(m.cells()[j * k + i]);
            }
        }
        // SAFETY: packed lower column-major triangle has n*(n+1)/2 elements;
        // uniquely borrowed buffer, no retained pointer or alias to m.
        let info = unsafe { t2_dtptri(n, packed.as_mut_ptr(), 0) };
        if info != 0 {
            return Ok(info);
        }
        let out = m.cells_mut();
        let mut cursor = 0;
        for j in 0..k {
            for i in 0..j {
                out[i * k + j] = 0.0;
            }
            for i in j..k {
                out[i * k + j] = packed[cursor];
                cursor += 1;
            }
        }
        Ok(0)
    }
    #[cfg(not(feature = "lapack"))]
    {
        let _ = m;
        Err("LAPACK backend unavailable".into())
    }
}

/// Mutates A and the real RHS exactly as DGELS; absent parameter output still
/// computes chi-square. The optional covariance receives source rescaling.
pub fn accel_lsq_qr(
    a: &mut BlasMatrix,
    data: &mut [f64],
    parameters: Option<&mut [f64]>,
    covariance: Option<&mut BlasMatrix>,
    rescale: bool,
) -> Result<f64, String> {
    #[cfg(feature = "lapack")]
    {
        let rows = a.get_blas_rows();
        let cols = a.get_blas_cols();
        if rows < cols
            || data.len() != rows
            || parameters.as_ref().is_some_and(|v| v.len() != cols)
            || covariance
                .as_ref()
                .is_some_and(|v| v.get_blas_rows() != cols || v.get_blas_cols() != cols)
        {
            return Err("QR dimensions/output extents".into());
        }
        let nr = dim(rows)?;
        let nc = dim(cols)?;
        let mut query = 0.0;
        // SAFETY: row-major A is column-major cols*rows with lda=cols;
        // transposed solve has rows RHS cells. Distinct mutable borrows, live
        // one-element query for lwork=-1; the header-checked shim fixes ABI.
        let info = unsafe {
            t2_dgels(
                nr,
                nc,
                a.cells_mut().as_mut_ptr(),
                data.as_mut_ptr(),
                &mut query,
                -1,
            )
        };
        // fda6ad4 T2accel.C:135-139 casts an unchecked workspace query before
        // allocation. Check provider status/range before Rust allocation/cast.
        if info != 0
            || !query.is_finite()
            || query < 1.0
            || query > i32::MAX as f64
            || query.fract() != 0.0
        {
            return Err(format!("DGELS workspace query: INFO={info}, size={query}"));
        }
        let mut work = Vec::new();
        work.try_reserve_exact(query as usize)
            .map_err(|e| format!("DGELS workspace allocation: {e}"))?;
        work.resize(query as usize, 0.0);
        // SAFETY: same valid A/RHS extents, separate uniquely owned workspace
        // of the queried positive size. Provider writes only these buffers.
        let info = unsafe {
            t2_dgels(
                nr,
                nc,
                a.cells_mut().as_mut_ptr(),
                data.as_mut_ptr(),
                work.as_mut_ptr(),
                query as i32,
            )
        };
        if info != 0 {
            return Err(format!("DGELS failed: INFO={info}"));
        }
        if let Some(p) = parameters {
            p.copy_from_slice(&data[..cols]);
        }
        let mut chi = 0.0;
        for v in &data[cols..] {
            // fda6ad4 T2accel.C::accel_lsq_qr chi-square loop: selected GCC16
            // arm64 object uses fmadd (AI accel-assembly.txt). Preserve that
            // measured binary64 rounding; other targets need provider evidence.
            #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
            {
                chi = v.mul_add(*v, chi);
            }
            #[cfg(not(all(target_os = "macos", target_arch = "aarch64")))]
            {
                chi += v * v;
            }
        }
        if let Some(cov) = covariance {
            let mut packed = Vec::with_capacity(cols * (cols + 1) / 2);
            for j in 0..cols {
                for i in 0..=j {
                    packed.push(a.cells()[i * cols + j]);
                }
            }
            // SAFETY: packed upper triangle extent nc*(nc+1)/2, unique owner.
            let info = unsafe { t2_dtptri(nc, packed.as_mut_ptr(), 1) };
            if info != 0 {
                return Err(format!("DTPTRI failed: INFO={info}"));
            }
            let mut inv = BlasMatrix::malloc_uinv(cols).map_err(|e| e.to_string())?;
            cov.cells_mut().fill(0.0);
            let mut cursor = 0;
            for j in 0..cols {
                for i in 0..=j {
                    inv.cells_mut()[j * cols + i] = packed[cursor];
                    cov.cells_mut()[j * cols + i] = packed[cursor];
                    cursor += 1;
                }
            }
            let alpha = if rescale {
                chi / (rows - cols) as f64
            } else {
                1.0
            };
            // SAFETY: separate nc*nc column-major inverse/covariance buffers;
            // DTRMM mutates only covariance, no pointers survive this call.
            unsafe {
                t2_dtrmm(
                    nc,
                    alpha,
                    inv.cells_mut().as_mut_ptr(),
                    cov.cells_mut().as_mut_ptr(),
                );
            }
        }
        Ok(chi)
    }
    #[cfg(not(feature = "lapack"))]
    {
        let _ = (a, data, parameters, covariance, rescale);
        Err("LAPACK backend unavailable".into())
    }
}

pub fn accel_mult_matrix_vec(a: &BlasMatrix, b: &[f64]) -> Result<Vec<f64>, String> {
    #[cfg(feature = "blas")]
    {
        if b.len() != a.get_blas_cols() {
            return Err("DGEMV dimensions".into());
        }
        let nr = dim(a.get_blas_rows())?;
        let nc = dim(a.get_blas_cols())?;
        let mut out = vec![0.0; a.get_blas_rows()];
        // SAFETY: canonical rows*cols contiguous A, cols inputs, rows outputs;
        // immutable inputs may alias each other; output has a distinct owner.
        unsafe {
            t2_dgemv(nr, nc, a.cells().as_ptr(), b.as_ptr(), out.as_mut_ptr());
        }
        Ok(out)
    }
    #[cfg(not(feature = "blas"))]
    {
        let _ = (a, b);
        Err("BLAS backend unavailable".into())
    }
}
pub fn accel_mult_matrix(a: &BlasMatrix, b: &BlasMatrix) -> Result<BlasMatrix, String> {
    #[cfg(feature = "blas")]
    {
        if a.get_blas_cols() != b.get_blas_rows() {
            return Err("DGEMM dimensions".into());
        }
        let nr = dim(a.get_blas_rows())?;
        let nk = dim(a.get_blas_cols())?;
        let nc = dim(b.get_blas_cols())?;
        let mut out = BlasMatrix::malloc_blas(a.get_blas_rows(), b.get_blas_cols())
            .map_err(|e| e.to_string())?;
        // SAFETY: validated contiguous rectangular extents; reversed operands
        // implement the source row-major product. Output cannot alias inputs.
        unsafe {
            t2_dgemm(
                nr,
                nk,
                nc,
                a.cells().as_ptr(),
                b.cells().as_ptr(),
                out.cells_mut().as_mut_ptr(),
            );
        }
        Ok(out)
    }
    #[cfg(not(feature = "blas"))]
    {
        let _ = (a, b);
        Err("BLAS backend unavailable".into())
    }
}
