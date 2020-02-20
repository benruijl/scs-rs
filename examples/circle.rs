use std::ptr;

extern crate scs;

fn main() {
    // maximize k_x
    // f1 <= 50
    // |(k_x,k_y) + (2,2)| <= f1
    // expected solution: k_x=48, ky=-2
    let mut q = vec![3i32]; // 1 quadratic constraint with 3 equations
    let cone = scs::ScsCone {
        f: 0,
        l: 1, // 1 linear constraint
        q: &mut q[0] as *mut scs::scs_int,
        qsize: q.len() as i32,
        s: ptr::null_mut(),
        ssize: 0,
        ep: 0,
        ed: 0,
        p: ptr::null_mut(),
        psize: 0,
    };

    // defaults
    let mut settings = scs::ScsSettings {
        normalize: 1,
        scale: 1.0,
        rho_x: 1e-3,
        max_iters: 5000,
        eps: 1e-5,
        alpha: 1.5,
        cg_rate: 2.0,
        verbose: 0,
        warm_start: 1,
        acceleration_lookback: 10,
        write_data_filename: ptr::null(),
    };

    // compressed sparse column representation
    /*[[ 0.  0.  1.]
    [ 0.  0. -1.]
    [-1.  0.  0.]
    [ 0. -1.  0.]]
    */
    let mut x = [-1., -1., 1., -1.];
    let mut i = [2, 3, 0, 1];
    let mut p = [0, 1, 2, 4];
    let mut A = scs::ScsMatrix {
        x: &mut x[0] as *mut f64,
        i: &mut i[0] as *mut scs::scs_int,
        p: &mut p[0] as *mut scs::scs_int,
        m: 4,
        n: 3,
    };
    let mut b = vec![50., 0., 2., 2.];
    let mut c = vec![-1., 0., 0.];

    let data = scs::ScsData {
        m: 4, // rows
        n: 3, // cols
        A: &mut A as *mut scs::ScsMatrix,
        b: &mut b[0] as *mut f64,
        c: &mut c[0] as *mut f64,
        stgs: &mut settings as *mut scs::ScsSettings,
    };

    let mut info = scs::ScsInfo::default();

    let mut sol_x = vec![0.; 3];
    let mut sol_y = vec![0.; 4];
    let mut sol_s = vec![0.; 4];
    let mut sol = scs::ScsSolution {
        x: &mut sol_x[0] as *mut f64,
        y: &mut sol_y[0] as *mut f64,
        s: &mut sol_s[0] as *mut f64,
    };

    unsafe {
        let workspace = scs::scs_init(
            &data as *const scs::ScsData,
            &cone as *const scs::ScsCone,
            &mut info as *mut scs::ScsInfo,
        );

        let _state = scs::scs_solve(
            workspace,
            &data as *const scs::ScsData,
            &cone as *const scs::ScsCone,
            &mut sol as *mut scs::ScsSolution,
            &mut info as *mut scs::ScsInfo,
        );

        scs::scs_finish(workspace);
    }

    println!("Solution: {:?}", sol_x);
}
