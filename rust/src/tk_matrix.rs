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
