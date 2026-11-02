//! Standalone AN observer, linked to an immutable crate build and observe.c.
//! Never installed or linked into the measured production CLI.
use std::{ffi::c_void, hint::black_box, time::Instant};
use tempo2_rust::{pulsar::MAX_FIT, pulsar_lifecycle::PulsarContext, tk_matrix::BlasMatrix};

unsafe extern "C" {
    fn an_snapshot(
        phase: *const u8,
        pointers: *const *const c_void,
        bytes: *const usize,
        bases: *const *const c_void,
        count: usize,
        elapsed: f64,
    );
}

fn snapshot(phase: &'static [u8], matrices: &[&BlasMatrix], elapsed: f64) {
    let pointers: Vec<_> = matrices.iter().map(|m| m.cells().as_ptr().cast()).collect();
    let bytes: Vec<_> = matrices
        .iter()
        .map(|m| std::mem::size_of_val(m.cells()))
        .collect();
    // SAFETY: NUL-terminated fixed labels and live arrays/owners outlive the call.
    // The C observer only queries ranges; default System owns these allocations.
    unsafe {
        an_snapshot(
            phase.as_ptr(),
            pointers.as_ptr(),
            bytes.as_ptr(),
            pointers.as_ptr(),
            matrices.len(),
            elapsed,
        )
    };
}

fn released(phase: &'static [u8], addresses: &[usize], elapsed: f64) {
    let pointers: Vec<_> = addresses.iter().map(|&p| p as *const c_void).collect();
    let bytes = vec![MAX_FIT * MAX_FIT * 8; addresses.len()];
    let bases = vec![std::ptr::null(); addresses.len()];
    // SAFETY: Only the numerical ranges reach mincore/VM queries. Null bases
    // prohibit allocator introspection, and C never dereferences freed memory.
    unsafe {
        an_snapshot(
            phase.as_ptr(),
            pointers.as_ptr(),
            bytes.as_ptr(),
            bases.as_ptr(),
            addresses.len(),
            elapsed,
        )
    };
}

fn touch(matrix: &mut BlasMatrix, control: &str) {
    match control {
        "read" => {
            // Full sequential read control; separate from untouched observations.
            let bits = matrix
                .cells()
                .iter()
                .fold(0, |v, x| v | black_box(x.to_bits()));
            assert_eq!(bits, 0);
        }
        "zero" => assert!(matrix.cells().iter().all(|x| black_box(x.to_bits()) == 0)),
        "write" => {
            matrix.cells_mut().fill(1.0);
            black_box(matrix.cells());
        }
        "untouched" => (),
        _ => panic!("unknown control"),
    }
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    #[cfg(fail_allocator)]
    if args[1] == "failure" {
        failure::check();
        return;
    }
    let count: usize = args[2].parse().unwrap();
    assert!((1..=2).contains(&count));
    let control = args[3].as_str();
    snapshot(b"warmup\0", &[], 0.0);
    snapshot(b"start\0", &[], 0.0);
    if args[1] == "matrix" {
        let mut matrices = Vec::new();
        for _ in 0..count {
            let t = Instant::now();
            matrices.push(BlasMatrix::malloc_blas(MAX_FIT, MAX_FIT).unwrap());
            let elapsed = t.elapsed().as_secs_f64();
            snapshot(
                b"allocated\0",
                &matrices.iter().collect::<Vec<_>>(),
                elapsed,
            );
        }
        if control == "cow" {
            let t = Instant::now();
            let mut clone = matrices[0].clone();
            snapshot(
                b"cloned\0",
                &[&matrices[0], &clone],
                t.elapsed().as_secs_f64(),
            );
            assert_eq!(matrices[0].cells().as_ptr(), clone.cells().as_ptr());
            let t = Instant::now();
            clone.cells_mut()[MAX_FIT + 1] = 7.0;
            let elapsed = t.elapsed().as_secs_f64();
            snapshot(b"cow_write\0", &[&matrices[0], &clone], elapsed);
            assert_eq!(matrices[0].cells()[MAX_FIT + 1].to_bits(), 0);
            assert_eq!(clone.cells()[MAX_FIT + 1], 7.0);
            drop(clone);
            snapshot(b"clone_dropped\0", &[&matrices[0]], 0.0);
        } else if control != "untouched" {
            let t = Instant::now();
            for m in &mut matrices {
                touch(m, control);
            }
            snapshot(
                b"touched\0",
                &matrices.iter().collect::<Vec<_>>(),
                t.elapsed().as_secs_f64(),
            );
        }
        let addresses: Vec<_> = matrices
            .iter()
            .map(|m| m.cells().as_ptr() as usize)
            .collect();
        let t = Instant::now();
        drop(matrices);
        released(b"dropped\0", &addresses, t.elapsed().as_secs_f64());
    } else {
        assert_eq!(args[1], "context");
        let mut c = PulsarContext::unwritten(count);
        snapshot(b"unwritten_context\0", &[], 0.0);
        let mut owners = Vec::new();
        for p in 0..count {
            let t = Instant::now();
            drop(c.initialise_one(p, 10000, 1, 1).unwrap());
            let elapsed = t.elapsed().as_secs_f64();
            owners.push(c.pulsars()[p].covar.unwrap().unwrap());
            snapshot(
                b"initialised\0",
                &owners
                    .iter()
                    .map(|&o| c.matrix(o).unwrap())
                    .collect::<Vec<_>>(),
                elapsed,
            );
        }
        if control == "reinit" {
            let t = Instant::now();
            let retired = c.initialise_one(0, 10000, 1, 1).unwrap();
            let elapsed = t.elapsed().as_secs_f64();
            owners.push(c.pulsars()[0].covar.unwrap().unwrap());
            snapshot(
                b"reinitialised_old_owner_retained\0",
                &owners
                    .iter()
                    .map(|&o| c.matrix(o).unwrap())
                    .collect::<Vec<_>>(),
                elapsed,
            );
            drop(retired);
        } else if control == "cow" {
            let t = Instant::now();
            let mut clone = c.clone();
            let owner = clone.pulsars()[0].covar.unwrap().unwrap();
            snapshot(
                b"context_cloned\0",
                &[c.matrix(owners[0]).unwrap(), clone.matrix(owner).unwrap()],
                t.elapsed().as_secs_f64(),
            );
            let t = Instant::now();
            clone.matrix_mut(owner).unwrap().cells_mut()[1] = 7.0;
            snapshot(
                b"context_cow_write\0",
                &[c.matrix(owners[0]).unwrap(), clone.matrix(owner).unwrap()],
                t.elapsed().as_secs_f64(),
            );
            assert_eq!(c.matrix(owners[0]).unwrap().cells()[1].to_bits(), 0);
            drop(clone);
        } else if control != "untouched" {
            let t = Instant::now();
            for &o in &owners {
                touch(c.matrix_mut(o).unwrap(), control);
            }
            snapshot(
                b"touched\0",
                &owners
                    .iter()
                    .map(|&o| c.matrix(o).unwrap())
                    .collect::<Vec<_>>(),
                t.elapsed().as_secs_f64(),
            );
        }
        let addresses: Vec<_> = owners
            .iter()
            .map(|&o| c.matrix(o).unwrap().cells().as_ptr() as usize)
            .collect();
        for p in 0..count {
            c.destroy_one(p).unwrap();
        }
        let retained: Vec<_> = owners.iter().filter_map(|&o| c.matrix(o).ok()).collect();
        snapshot(b"destroyed_current_owners\0", &retained, 0.0);
        let t = Instant::now();
        drop(c);
        released(b"context_dropped\0", &addresses, t.elapsed().as_secs_f64());
    }
}

#[cfg(fail_allocator)]
mod failure {
    use super::*;
    use std::{
        alloc::{GlobalAlloc, Layout, System},
        sync::atomic::{AtomicBool, Ordering},
    };
    static FAIL: AtomicBool = AtomicBool::new(false);
    struct Allocator;
    #[global_allocator]
    static ALLOCATOR: Allocator = Allocator;
    // SAFETY: This isolated control delegates each non-rejected request and
    // matching deallocation to System, preserving layout. Only a 64 KiB payload
    // is rejected while armed; diagnostics and context setup remain allocatable.
    unsafe impl GlobalAlloc for Allocator {
        unsafe fn alloc(&self, l: Layout) -> *mut u8 {
            if l.size() == 65536 && FAIL.load(Ordering::SeqCst) {
                std::ptr::null_mut()
            } else {
                unsafe { System.alloc(l) }
            }
        }
        unsafe fn alloc_zeroed(&self, l: Layout) -> *mut u8 {
            if l.size() == 65536 && FAIL.load(Ordering::SeqCst) {
                std::ptr::null_mut()
            } else {
                unsafe { System.alloc_zeroed(l) }
            }
        }
        unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
            unsafe { System.dealloc(p, l) }
        }
        unsafe fn realloc(&self, p: *mut u8, l: Layout, n: usize) -> *mut u8 {
            unsafe { System.realloc(p, l, n) }
        }
    }
    pub fn check() {
        let mut context = PulsarContext::unwritten(0);
        FAIL.store(true, Ordering::SeqCst);
        let error = context.malloc_blas(64, 128).unwrap_err();
        FAIL.store(false, Ordering::SeqCst);
        assert_eq!(error.operation, "malloc_blas");
        assert_eq!(error.field, "cells");
        assert_eq!(error.kind, "allocation_failure");
        assert_eq!(context.malloc_blas(1, 1).unwrap_err(), error);
        let mut fresh = PulsarContext::unwritten(0);
        let owner = fresh.malloc_blas(64, 128).unwrap();
        assert!(
            fresh
                .matrix(owner)
                .unwrap()
                .cells()
                .iter()
                .all(|v| v.to_bits() == 0)
        );
        println!("allocation failure, terminal context, fresh recovery: pass");
    }
}
