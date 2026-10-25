//! TKmatrix.C malloc_blas/accessors/free and square wrappers.
use crate::pulsar_lifecycle::{LifecycleBoundary, fail};
use std::sync::Arc;

/// Source hidden size_t header and row pointers become typed side metadata.
/// The complete dense allocation remains contiguous, with exactly cols stride.
#[derive(Clone, Debug)]
pub struct BlasMatrix {
    rows: usize,
    cols: usize,
    cells: Arc<Vec<f64>>,
}
impl BlasMatrix {
    pub fn malloc_blas(rows: usize, cols: usize) -> Result<Self, LifecycleBoundary> {
        if rows == 0 || cols == 0 {
            return Err(fail(
                "malloc_blas",
                "dimensions",
                "zero_dimension_assertion",
            ));
        }
        let len = rows
            .checked_mul(cols)
            .filter(|n| {
                n.checked_add(2)
                    .and_then(|n| n.checked_mul(8))
                    .is_some_and(|b| b <= isize::MAX as usize)
            })
            .ok_or_else(|| fail("malloc_blas", "dimensions", "size_overflow"))?;
        rows.checked_mul(std::mem::size_of::<usize>())
            .filter(|n| *n <= isize::MAX as usize)
            .ok_or_else(|| fail("malloc_blas", "row_table", "size_overflow"))?;
        let mut cells = Vec::new();
        cells
            .try_reserve_exact(len)
            .map_err(|_| fail("malloc_blas", "cells", "allocation_failure"))?;
        cells.resize(len, 0.0); // Every source calloc cell is positive zero.
        Ok(Self {
            rows,
            cols,
            cells: Arc::new(cells),
        })
    }
    pub fn malloc_uinv(n: usize) -> Result<Self, LifecycleBoundary> {
        Self::malloc_blas(n, n)
    }
    pub fn get_blas_rows(&self) -> usize {
        self.rows
    }
    pub fn get_blas_cols(&self) -> usize {
        self.cols
    }
    pub fn cells(&self) -> &[f64] {
        &self.cells
    }
    /// COW separates independent context clones; aliases inside one context
    /// resolve to this same owner, so they continue to see every write.
    pub fn cells_mut(&mut self) -> &mut [f64] {
        Arc::make_mut(&mut self.cells).as_mut_slice()
    }
    pub fn row(&self, row: usize) -> Result<&[f64], LifecycleBoundary> {
        if row >= self.rows {
            return Err(fail("matrix_read", "row", "capacity"));
        }
        Ok(&self.cells[row * self.cols..(row + 1) * self.cols])
    }
}

/// TKforwardSubVec: binary64 row-order forward substitution.
pub fn forward_sub_vec(l: &[Vec<f64>], b: &[f64]) -> Result<Vec<f64>, String> {
    let n = b.len();
    if l.len() != n || l.iter().any(|r| r.len() != n) {
        return Err("forward substitution dimensions".into());
    }
    let mut x = vec![0.0; n];
    for i in 0..n {
        let mut s = 0.0;
        for j in 0..i {
            s += l[i][j] * x[j];
        }
        x[i] = (b[i] - s) / l[i][i];
    }
    Ok(x)
}
/// TKforwardSub: columns outside rows, preserving the source summation order.
pub fn forward_sub(l: &[Vec<f64>], m: &[Vec<f64>]) -> Result<Vec<Vec<f64>>, String> {
    let n = m.len();
    let np = m.first().ok_or("empty forward substitution")?.len();
    if l.len() != n || l.iter().any(|r| r.len() != n) || m.iter().any(|r| r.len() != np) {
        return Err("forward substitution dimensions".into());
    }
    let mut x = vec![vec![0.0; np]; n];
    for p in 0..np {
        for i in 0..n {
            let mut s = 0.0;
            for j in 0..i {
                s += l[i][j] * x[j][p];
            }
            x[i][p] = (m[i][p] - s) / l[i][i];
        }
    }
    Ok(x)
}
/// TKmultMatrix: k/i/j accumulation order (nonaccelerated source branch).
pub fn mult_matrix(a: &[Vec<f64>], b: &[Vec<f64>]) -> Result<Vec<Vec<f64>>, String> {
    let n = b.len();
    let p = b.first().ok_or("empty matrix product")?.len();
    if a.iter().any(|r| r.len() != n) || b.iter().any(|r| r.len() != p) {
        return Err("matrix product dimensions".into());
    }
    let mut out = vec![vec![0.0; p]; a.len()];
    for k in 0..n {
        for i in 0..a.len() {
            for j in 0..p {
                out[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    Ok(out)
}
