#[allow(non_camel_case_types)]
pub type scs_int = ::std::os::raw::c_int;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScsMatrix {
    pub x: *mut f64, // column compressed format
    pub i: *mut scs_int,   // row index
    pub p: *mut scs_int,   // column pointer
    pub m: scs_int,        // rows
    pub n: scs_int,        // cols
}

// TODO: what to do here?
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AaWork {
    _unused: [u8; 0],
}

extern "C" {
    pub fn aa_init(dim: scs_int, aa_mem: scs_int, type1: scs_int) -> *mut AaWork;
    pub fn aa_apply(f: *mut f64, x: *const f64, a: *mut AaWork) -> scs_int;
    pub fn aa_finish(a: *mut AaWork);
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScsLinSysWork {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScsAccelWork {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScsConeWork {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[allow(non_snake_case)]
pub struct ScsData {
    pub m: scs_int,
    pub n: scs_int,
    pub A: *mut ScsMatrix,
    pub b: *mut f64,
    pub c: *mut f64,
    pub stgs: *mut ScsSettings,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScsSettings {
    pub normalize: scs_int,
    pub scale: f64,
    pub rho_x: f64,
    pub max_iters: scs_int,
    pub eps: f64,
    pub alpha: f64,
    pub cg_rate: f64,
    pub verbose: scs_int,
    pub warm_start: scs_int,
    pub acceleration_lookback: scs_int,
    pub write_data_filename: *const ::std::os::raw::c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScsCone {
    pub f: scs_int,
    pub l: scs_int,
    pub q: *mut scs_int,
    pub qsize: scs_int,
    pub s: *mut scs_int,
    pub ssize: scs_int,
    pub ep: scs_int,
    pub ed: scs_int,
    pub p: *mut f64,
    pub psize: scs_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScsSolution {
    pub x: *mut f64,
    pub y: *mut f64,
    pub s: *mut f64,
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct ScsInfo {
    pub iter: scs_int,
    pub status: [::std::os::raw::c_char; 32usize],
    pub status_val: scs_int,
    pub pobj: f64,
    pub dobj: f64,
    pub res_pri: f64,
    pub res_dual: f64,
    pub res_infeas: f64,
    pub res_unbdd: f64,
    pub rel_gap: f64,
    pub setup_time: f64,
    pub solve_time: f64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[allow(non_snake_case)]
pub struct ScsScaling {
    pub D: *mut f64,
    pub E: *mut f64,
    pub mean_norm_row_a: f64,
    pub mean_norm_col_a: f64,
}

extern "C" {
    ///This initializes the ScsWork struct containing the workspace that scs will
    /// use, and performs the necessary preprocessing (e.g. matrix factorization).
    ///All inputs `d`, `k`, and `info` must be memory allocated before calling.
    pub fn scs_init(d: *const ScsData, k: *const ScsCone, info: *mut ScsInfo) -> *mut ScsWork;

    ///This solves the problem as defined by ScsData `d` and ScsCone `k` using the
    ///workspace in `w`. The solution is returned in `sol` and information about
    ///the solve is returned in `info` (outputs must have memory allocated before
    ///calling).  None of the inputs can be NULL. You can call `scs_solve` many
    ///times for one call to `scs_init`, so long as the matrix `A` does not change
    ///(vectors `b` and `c` can change).
    pub fn scs_solve(
        w: *mut ScsWork,
        d: *const ScsData,
        k: *const ScsCone,
        sol: *mut ScsSolution,
        info: *mut ScsInfo,
    ) -> scs_int;

    ///Called after all solves completed to free allocated memory and other
    ///cleanup.
    pub fn scs_finish(w: *mut ScsWork);

    ///Convenience method that simply calls all the above routines in order, for
    ///cases where the workspace does not need to be reused. All inputs must have
    ///memory allocated before this call.
    pub fn scs(
        d: *const ScsData,
        k: *const ScsCone,
        sol: *mut ScsSolution,
        info: *mut ScsInfo,
    ) -> scs_int;

    /// Get the SCS version.
    pub fn scs_version() -> *const ::std::os::raw::c_char;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[allow(non_snake_case)]
pub struct ScsWork {
    pub u: *mut f64,
    pub v: *mut f64,
    pub u_t: *mut f64,
    pub u_prev: *mut f64,
    pub v_prev: *mut f64,
    pub h: *mut f64,
    pub g: *mut f64,
    pub pr: *mut f64,
    pub dr: *mut f64,
    pub g_th: f64,
    pub sc_b: f64,
    pub sc_c: f64,
    pub nm_b: f64,
    pub nm_c: f64,
    pub b: *mut f64,
    pub c: *mut f64,
    pub m: scs_int,
    pub n: scs_int,
    pub A: *mut ScsMatrix,
    pub p: *mut ScsLinSysWork,
    pub stgs: *mut ScsSettings,
    pub scal: *mut ScsScaling,
    pub cone_work: *mut ScsConeWork,
    pub accel: *mut AaWork,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScsResiduals {
    pub last_iter: scs_int,
    pub res_dual: f64,
    pub res_pri: f64,
    pub res_infeas: f64,
    pub res_unbdd: f64,
    pub rel_gap: f64,
    pub ct_x_by_tau: f64,
    pub bt_y_by_tau: f64,
    pub tau: f64,
    pub kap: f64,
}
