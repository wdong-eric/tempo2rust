//! TEMPO2 TKsvd.C: TKbidiagonal, TKsingularValueDecomposition_lsq,
//! TKbacksubstitution_svd and TKpythag; T2toolkit.C::TKsign_d/TKretMax_d boundaries.
// Copyright (C) 2006-2009 George Hobbs, Russell Edwards and TEMPO2 contributors.
// Port under GPL-3.0-or-later. Keep source index/operation order, including f64 helpers.
#![allow(clippy::needless_range_loop, clippy::assign_op_pattern)]
use crate::PRECISION_BITS;
use rug::Float;

pub(crate) fn ld(value: f64) -> Float {
    Float::with_val(PRECISION_BITS, value)
}

// TKsign_d(double,double): narrowing is intentional, and -0 compares >= 0.
fn tk_sign_d(a: Float, b: &Float) -> Float {
    ld(if b.to_f64() >= 0.0 {
        a.to_f64().abs()
    } else {
        -a.to_f64().abs()
    })
}

fn tk_pythag(a: &Float, b: &Float) -> Float {
    let absa = a.clone().abs();
    let absb = b.clone().abs();
    if absa > absb {
        let ratio = absb / &absa;
        absa * (ld(1.0) + ratio.square()).sqrt()
    } else if absb == 0 {
        ld(0.0)
    } else {
        let ratio = absa / &absb;
        absb * (ld(1.0) + ratio.square()).sqrt()
    }
}

pub(crate) struct Svd {
    pub u: Vec<Vec<Float>>,
    pub v: Vec<Vec<Float>>,
    pub w: Vec<Float>,
}

fn tk_bidiagonal(
    a: &mut [Vec<Float>],
    v: &mut [Vec<Float>],
    w: &mut [Float],
    rv1: &mut [Float],
) -> Float {
    let ndata = a.len();
    let nfit = w.len();
    let mut g = ld(0.0);
    let mut scale = ld(0.0);
    let mut an = ld(0.0);
    for i in 0..nfit {
        let l = i + 1;
        rv1[i] = scale.clone() * &g;
        g = ld(0.0);
        let mut s = ld(0.0);
        scale = ld(0.0);
        for k in i..ndata {
            scale += a[k][i].clone().abs();
        }
        if scale != 0 {
            for k in i..ndata {
                a[k][i] /= &scale;
                s += a[k][i].clone() * &a[k][i];
            }
            let mut f = a[i][i].clone();
            g = -tk_sign_d(s.clone().sqrt(), &f);
            let h = f.clone() * &g - &s;
            a[i][i] = f.clone() - &g;
            for j in l..nfit {
                s = ld(0.0);
                for k in i..ndata {
                    s += a[k][i].clone() * &a[k][j];
                }
                f = s.clone() / &h;
                for k in i..ndata {
                    let product = f.clone() * &a[k][i];
                    a[k][j] += product;
                }
            }
            for k in i..ndata {
                a[k][i] *= &scale;
            }
        }
        w[i] = scale.clone() * &g;
        g = ld(0.0);
        s = ld(0.0);
        scale = ld(0.0);
        for k in l..nfit {
            scale += a[i][k].clone().abs();
        }
        if scale != 0 {
            for k in l..nfit {
                a[i][k] /= &scale;
                s += a[i][k].clone() * &a[i][k];
            }
            let f = a[i][l].clone();
            g = -tk_sign_d(s.clone().sqrt(), &f);
            let h = f.clone() * &g - &s;
            a[i][l] = f - &g;
            for k in l..nfit {
                rv1[k] = a[i][k].clone() / &h;
            }
            for j in l..ndata {
                s = ld(0.0);
                for k in l..nfit {
                    s += a[j][k].clone() * &a[i][k];
                }
                for k in l..nfit {
                    a[j][k] += s.clone() * &rv1[k];
                }
            }
            for k in l..nfit {
                a[i][k] = scale.clone() * &a[i][k];
            }
        }
        // TKretMax_d takes/returns double even though an and the sum are longdouble.
        let b = (w[i].clone().abs() + rv1[i].clone().abs()).to_f64();
        an = ld(if an.to_f64() > b { an.to_f64() } else { b });
    }
    let mut l = nfit;
    for i in (0..nfit).rev() {
        if i < nfit - 1 {
            if g != 0 {
                for j in l..nfit {
                    v[j][i] = (a[i][j].clone() / &a[i][l]) / &g;
                }
                for j in l..nfit {
                    let mut s = ld(0.0);
                    for k in l..nfit {
                        s += a[i][k].clone() * &v[k][j];
                    }
                    for k in l..nfit {
                        let product = s.clone() * &v[k][i];
                        v[k][j] += product;
                    }
                }
            }
            for j in l..nfit {
                v[i][j] = ld(0.0);
                v[j][i] = ld(0.0);
            }
        }
        v[i][i] = ld(1.0);
        g = rv1[i].clone();
        l = i;
    }
    for i in (0..nfit).rev() {
        l = i + 1;
        g = w[i].clone();
        for j in l..nfit {
            a[i][j] = ld(0.0);
        }
        if g != 0 {
            g = ld(1.0) / g;
            for j in l..nfit {
                let mut s = ld(0.0);
                for k in l..ndata {
                    s += a[k][i].clone() * &a[k][j];
                }
                let f = (s / &a[i][i]) * &g;
                for k in i..ndata {
                    let product = f.clone() * &a[k][i];
                    a[k][j] += product;
                }
            }
            for j in i..ndata {
                a[j][i] *= &g;
            }
        } else {
            for j in i..ndata {
                a[j][i] = ld(0.0);
            }
        }
        a[i][i] += 1;
    }
    an
}

pub(crate) fn tk_singular_value_decomposition(mut a: Vec<Vec<Float>>) -> Result<Svd, String> {
    let n = a.len();
    let nf = a[0].len();
    let mut v = vec![vec![ld(0.0); nf]; nf];
    let mut w = vec![ld(0.0); nf];
    let mut rv1 = vec![ld(0.0); nf];
    let an = tk_bidiagonal(&mut a, &mut v, &mut w, &mut rv1);
    for k in (0..nf).rev() {
        for its in 1..=40 {
            let mut pos1 = false;
            let mut l = k;
            loop {
                if rv1[l].clone().abs() + &an == an {
                    pos1 = true;
                    break;
                }
                // TKsvd.C next reads w[l-1]. Nonfinite arithmetic can reach l=0;
                // the C source then accesses invalid memory. Do not invent an output.
                if l == 0 {
                    return Err(
                        "TKsvd source boundary: invalid w[-1] access after nonfinite arithmetic"
                            .into(),
                    );
                }
                if w[l - 1].clone().abs() + &an == an {
                    break;
                }
                l -= 1;
            }
            if !pos1 {
                let nm = l - 1;
                let mut c = ld(0.0);
                let mut s = ld(1.0);
                for i in l..=k {
                    let f = s.clone() * &rv1[i];
                    rv1[i] = c.clone() * &rv1[i];
                    if f.clone().abs() + &an == an {
                        break;
                    }
                    let g = w[i].clone();
                    let mut h = tk_pythag(&f, &g);
                    w[i] = h.clone();
                    h = ld(1.0) / h;
                    c = g * &h;
                    s = -(f * &h);
                    for j in 0..n {
                        let y = a[j][nm].clone();
                        let z = a[j][i].clone();
                        a[j][nm] = y.clone() * &c + z.clone() * &s;
                        a[j][i] = -(y * &s) + z * &c;
                    }
                }
            }
            let mut z = w[k].clone();
            if l == k {
                if z < 0 {
                    w[k] = -z;
                    for j in 0..nf {
                        v[j][k] = -v[j][k].clone();
                    }
                }
                break;
            }
            // Source exits at 30, despite max_its=40. Rust returns contextual failure.
            if its == 30 {
                return Err("TKsvd: no convergence after 30 iterations (source exits)".into());
            }
            let mut x = w[l].clone();
            let nm = k - 1;
            let mut y = w[nm].clone();
            let mut g = rv1[nm].clone();
            let mut h = rv1[k].clone();
            let mut f = ((y.clone() - &z) * (y.clone() + &z) + (g.clone() - &h) * (g.clone() + &h))
                / (ld(2.0) * &h * &y);
            g = tk_pythag(&f, &ld(1.0));
            let sign = tk_sign_d(g.clone(), &f);
            f = ((x.clone() - &z) * (x.clone() + &z) + h.clone() * ((y.clone() / (f + sign)) - &h))
                / &x;
            let mut c = ld(1.0);
            let mut s = ld(1.0);
            for j in l..=nm {
                let i = j + 1;
                g = rv1[i].clone();
                y = w[i].clone();
                h = s.clone() * &g;
                g = c.clone() * g;
                z = tk_pythag(&f, &h);
                rv1[j] = z.clone();
                c = f.clone() / &z;
                s = h.clone() / &z;
                f = x.clone() * &c + g.clone() * &s;
                g = -(x.clone() * &s) + g * &c;
                h = y.clone() * &s;
                y *= &c;
                for jj in 0..nf {
                    x = v[jj][j].clone();
                    z = v[jj][i].clone();
                    v[jj][j] = x.clone() * &c + z.clone() * &s;
                    v[jj][i] = -(x.clone() * &s) + z.clone() * &c;
                }
                z = tk_pythag(&f, &h);
                w[j] = z.clone();
                if z != 0 {
                    z = ld(1.0) / z;
                    c = f.clone() * &z;
                    s = h.clone() * &z;
                }
                f = c.clone() * &g + s.clone() * &y;
                x = -(s.clone() * &g) + c.clone() * &y;
                for jj in 0..n {
                    y = a[jj][j].clone();
                    z = a[jj][i].clone();
                    a[jj][j] = y.clone() * &c + z.clone() * &s;
                    a[jj][i] = -(y.clone() * &s) + z.clone() * &c;
                }
            }
            rv1[l] = ld(0.0);
            rv1[k] = f;
            w[k] = x;
        }
    }
    Ok(Svd { u: a, v, w })
}

pub(crate) fn tk_backsubstitution_svd(svd: &Svd, b: &[f64]) -> Vec<Float> {
    backsubstitution(svd, &b.iter().map(|v| ld(*v)).collect::<Vec<_>>())
}

fn backsubstitution(svd: &Svd, b: &[Float]) -> Vec<Float> {
    let nf = svd.w.len();
    let mut utb = vec![ld(0.0); nf];
    for i in 0..nf {
        if svd.w[i] != 0 {
            for (j, value) in b.iter().enumerate() {
                utb[i] += svd.u[j][i].clone() * value;
            }
            utb[i] /= &svd.w[i];
        }
    }
    (0..nf)
        .map(|i| {
            let mut x = ld(0.0);
            for j in 0..nf {
                x += svd.v[i][j].clone() * &utb[j];
            }
            x
        })
        .collect()
}

/// Owned TKsingularValueDecomposition_lsq output, in raw source mode order.
/// Set a singular value to zero to omit that mode during back substitution;
/// this low-level operation does not apply the TKfit cutoff automatically.
#[derive(Clone, Debug, serde::Serialize)]
pub struct Decomposition {
    pub u: Vec<Vec<crate::FitNumber>>,
    pub v: Vec<Vec<crate::FitNumber>>,
    pub singular_values: Vec<crate::FitNumber>,
}

impl Decomposition {
    /// TKbacksubstitution_svd with an extended-precision right-hand side.
    pub fn back_substitute(
        &self,
        data: &[crate::FitNumber],
    ) -> Result<Vec<crate::FitNumber>, String> {
        let nf = self.singular_values.len();
        if nf == 0
            || self.u.len() != data.len()
            || self.v.len() != nf
            || self.u.iter().chain(&self.v).any(|row| row.len() != nf)
        {
            return Err("inconsistent SVD back-substitution dimensions".into());
        }
        let raw = Svd {
            u: self
                .u
                .iter()
                .map(|row| row.iter().map(|v| v.0.clone()).collect())
                .collect(),
            v: self
                .v
                .iter()
                .map(|row| row.iter().map(|v| v.0.clone()).collect())
                .collect(),
            w: self.singular_values.iter().map(|v| v.0.clone()).collect(),
        };
        Ok(
            backsubstitution(&raw, &data.iter().map(|v| v.0.clone()).collect::<Vec<_>>())
                .into_iter()
                .map(crate::FitNumber)
                .collect(),
        )
    }
}

fn input_matrix(matrix: Vec<Vec<crate::FitNumber>>) -> Result<Vec<Vec<Float>>, String> {
    let nf = matrix.first().ok_or("empty SVD matrix")?.len();
    if nf == 0 || matrix.len() < nf || matrix.iter().any(|row| row.len() != nf) {
        return Err("SVD requires a rectangular matrix with rows >= columns > 0".into());
    }
    Ok(matrix
        .into_iter()
        .map(|row| row.into_iter().map(|v| v.0).collect())
        .collect())
}

/// TKsingularValueDecomposition_lsq on canonical Rug128 values. Source sign,
/// iteration order and numerical boundaries are shared with all fitting routes.
pub fn decompose(matrix: Vec<Vec<crate::FitNumber>>) -> Result<Decomposition, String> {
    let raw = tk_singular_value_decomposition(input_matrix(matrix)?)?;
    Ok(Decomposition {
        u: raw
            .u
            .into_iter()
            .map(|row| row.into_iter().map(crate::FitNumber).collect())
            .collect(),
        v: raw
            .v
            .into_iter()
            .map(|row| row.into_iter().map(crate::FitNumber).collect())
            .collect(),
        singular_values: raw.w.into_iter().map(crate::FitNumber).collect(),
    })
}

/// TKbidiagonal output before diagonalization. `u` is the source's overwritten
/// input matrix; its separate `u` pointer argument is unused in the pinned body.
#[derive(Clone, Debug, serde::Serialize)]
pub struct Bidiagonal {
    pub u: Vec<Vec<crate::FitNumber>>,
    pub v: Vec<Vec<crate::FitNumber>>,
    pub diagonal: Vec<crate::FitNumber>,
    pub superdiagonal: Vec<crate::FitNumber>,
    pub norm: crate::FitNumber,
}

pub fn bidiagonal(matrix: Vec<Vec<crate::FitNumber>>) -> Result<Bidiagonal, String> {
    let mut u = input_matrix(matrix)?;
    let nf = u[0].len();
    let mut v = vec![vec![ld(0.0); nf]; nf];
    let mut w = vec![ld(0.0); nf];
    let mut rv1 = vec![ld(0.0); nf];
    let norm = tk_bidiagonal(&mut u, &mut v, &mut w, &mut rv1);
    Ok(Bidiagonal {
        u: u.into_iter()
            .map(|row| row.into_iter().map(crate::FitNumber).collect())
            .collect(),
        v: v.into_iter()
            .map(|row| row.into_iter().map(crate::FitNumber).collect())
            .collect(),
        diagonal: w.into_iter().map(crate::FitNumber).collect(),
        superdiagonal: rv1.into_iter().map(crate::FitNumber).collect(),
        norm: crate::FitNumber(norm),
    })
}

/// TKpythag, retaining the source's overflow-resistant expression order.
pub fn pythag(a: &crate::FitNumber, b: &crate::FitNumber) -> crate::FitNumber {
    crate::FitNumber(tk_pythag(&a.0, &b.0))
}
