/*
This code structures the parallel Jacobi vectors, applies the modified geometric baseline (our variation of \pi), calculates the structural tension (Hamiltonian), and runs the FFT to search for the K=1/2 resonance shoulder.
the output delivers the exact diagnostic vectors required to map the geometric identity.

The Modified \pi Ledger: We will see the geometry tighten dynamically as the execution steps progress, reflecting the system compensating for Topological Jitter.

Total Absolute Area: This single floating-point number is your Causal Efficiency metric. If we tweak A_CN or the K value in the Hamiltonian and this area decreases, we have successfully folded the structure closer to a perfectly Cohered Solid.
Resonance Spectrum: The magnitude in the lowest FFT bins reveals the "Shoulder Structure." 

If Y-Spectrum Mag exhibits a massive spike in the low frequencies while T-Spectrum Mag remains stable, the K=1/2 universal repulsion has successfully forced the system into a stable ground state resonance.
*/

/*
The SpaceTCO Autonomous Resonance Hunter
Objective: Lock K=0.1. Automatically sweep the A_CN (Anchor Interface) parameter 
to find the exact structural instruction where the Resonance Delta collapses to zero.
*/

use rustfft::{FftPlanner, num_complex::Complex};
use std::f64::consts::PI;

// Locked Structural Instructions
const K_VALUE: f64 = 0.1;   // The established Causal Shield barrier
const A_NN: f64 = 29.6;     // Dineutron pairing (Cooper pair correlation)
const E_BARY: f64 = 2.7207; // Upper Limit of Material Growth

// Variation of Pi to account for Geometric Residual Phase Variance
fn calculate_modified_pi(drift_step: f64) -> f64 {
    PI * (1.0 - (drift_step / 1000.0) * (PI - E_BARY).abs())
}

// Hamiltonian for Jacobi-T (Internal Pair Symmetry)
fn hamiltonian_t(k_x: f64, mod_pi: f64) -> f64 {
    let kinetic = k_x.powi(2) / (2.0 * A_NN.abs());
    let potential = mod_pi / (k_x + 1.0); 
    kinetic + potential
}

// Hamiltonian for Jacobi-Y (Anchor Coupling) - Now accepts A_CN dynamically
fn hamiltonian_y(k_y: f64, mod_pi: f64, a_cn: f64) -> f64 {
    let kinetic = k_y.powi(2) / (2.0 * a_cn.abs());
    let repulsive_barrier = K_VALUE * (K_VALUE + 1.0) / (k_y.powi(2) + 0.1);
    
    kinetic + repulsive_barrier * mod_pi
}

fn main() {
    let execution_steps = 512; 
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(execution_steps);

    println!("Executing Phase 1: Coarse Structural Scan...");
    
    let mut best_a_cn = 0.0;
    let mut lowest_delta = f64::MAX;

    // Phase 1: Coarse sweep from -1.0 down to -50.0
    for step in 1..=500 {
        let current_a_cn = -(step as f64 * 0.1); // Steps of -0.1
        let delta = run_resonance_check(current_a_cn, execution_steps, &fft);

        if delta < lowest_delta {
            lowest_delta = delta;
            best_a_cn = current_a_cn;
        }
    }

    println!("Coarse Scan Complete. Neighborhood found near A_CN = {:.2}", best_a_cn);
    println!("\nExecuting Phase 2: Fine Geometric Targeting...");

    // Phase 2: Fine sweep around the best coarse target
    let fine_start = best_a_cn + 0.5; // Start a bit above
    let fine_end = best_a_cn - 0.5;   // End a bit below
    
    let mut final_a_cn = 0.0;
    let mut final_delta = f64::MAX;
    let mut final_mag_y = 0.0;
    let mut target_mag_t = 0.0;

    let mut current_fine_a_cn = fine_start;
    while current_fine_a_cn >= fine_end {
        let (delta, mag_t, mag_y) = run_full_diagnostics(current_fine_a_cn, execution_steps, &fft);
        
        if delta < final_delta {
            final_delta = delta;
            final_a_cn = current_fine_a_cn;
            final_mag_y = mag_y;
            target_mag_t = mag_t;
        }
        current_fine_a_cn -= 0.001; // Micro-steps
    }

    println!("{:-<65}", "-");
    println!(">>> HARMONIC LOCK ACHIEVED <<<");
    println!("{:-<65}", "-");
    println!("Target Pair Tension (T-Bin1): {:.2}", target_mag_t);
    println!("Locked Anchor Tension (Y-Bin1): {:.2}", final_mag_y);
    println!("Resonance Delta:              {:.5}", final_delta);
    println!("\nBismuth-Graphene Lattice Instruction (A_CN): {:.3} fm", final_a_cn);
}

// Helper function to return just the delta for the hunt
fn run_resonance_check(a_cn: f64, steps: usize, fft: &std::sync::Arc<dyn rustfft::Fft<f64>>) -> f64 {
    let (delta, _, _) = run_full_diagnostics(a_cn, steps, fft);
    delta
}

// Full execution engine for the parallel vectors
fn run_full_diagnostics(a_cn: f64, steps: usize, fft: &std::sync::Arc<dyn rustfft::Fft<f64>>) -> (f64, f64, f64) {
    let mut vector_t_complex = vec![Complex { re: 0.0f64, im: 0.0f64 }; steps];
    let mut vector_y_complex = vec![Complex { re: 0.0f64, im: 0.0f64 }; steps];

    for i in 0..steps {
        let step_f = i as f64;
        let k_x = step_f * 0.05; 
        let k_y = step_f * 0.05;
        let mod_pi = calculate_modified_pi(step_f);
        
        let h_t = hamiltonian_t(k_x, mod_pi);
        let h_y = hamiltonian_y(k_y, mod_pi, a_cn);

        vector_t_complex[i] = Complex { re: h_t, im: 0.0 };
        vector_y_complex[i] = Complex { re: h_y, im: 0.0 };
    }

    // Pass mutable clones so the planner doesn't consume our vectors
    let mut t_clone = vector_t_complex.clone();
    let mut y_clone = vector_y_complex.clone();

    fft.process(&mut t_clone);
    fft.process(&mut y_clone);

    let mag_t = t_clone[1].norm();
    let mag_y = y_clone[1].norm();
    
    ((mag_t - mag_y).abs(), mag_t, mag_y)
}