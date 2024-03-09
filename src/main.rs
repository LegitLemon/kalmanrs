use nalgebra::{matrix, Matrix3, Vector3, RowVector3, Vector1, vector};
use kalmanrs::{DTLSS, heavyside_step,  generate_sim_results_plot};

fn main() {
    let A: Matrix3<f32> = matrix![1.0,0.0,1.0;1.0,1.0,1.0;0.0,2.0,1.0];
    let B: Vector3<f32> = vector![1.0, 0.0, 0.0];
    let C: RowVector3<f32> = vector![1.0, 0.0, 0.0].transpose();
    let D: Vector1<f32> = Vector1::zeros();
    let K = 10;

    let step_time = 0;
    let step_value = 1.0;

    let input = heavyside_step(K, step_time, step_value);

    let ss = DTLSS::new(A,B, C, D);
    let states = ss.lsim(K, input);
    generate_sim_results_plot(&states).unwrap();
}
