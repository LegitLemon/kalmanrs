use nalgebra::{vector, Matrix, Matrix3, RowVector3, Vector1, Vector3};
use plotters::prelude::*;

pub fn generate_sim_results_plot(
    simdata: &Vec<Vector3<f32>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("./test.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let length = simdata.len();
    let mut chart = ChartBuilder::on(&root)
        .caption("simulation results", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f32..length as f32, 0f32..10000f32)?;

    chart.configure_mesh().draw()?;

    let mut first_dimension = Vec::new();
    for (idx, i) in simdata.iter().enumerate() {
        let vec = i.x;
        first_dimension.push((idx as f32, vec))
    }

    let mut second_dimension = Vec::new();
    for (idx, i) in simdata.iter().enumerate() {
        let vec = i.y;
        second_dimension.push((idx as f32, vec))
    }

    let mut third_dimension = Vec::new();
    for (idx, i) in simdata.iter().enumerate() {
        let vec = i.z;
        third_dimension.push((idx as f32, vec))
    }

    chart
        .draw_series(LineSeries::new(first_dimension, &RED))?
        .label("first dimension")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .draw_series(LineSeries::new(second_dimension, &BLUE))?
        .label("second dimension")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart
        .draw_series(LineSeries::new(third_dimension, &GREEN))?
        .label("third dimension")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}


pub fn heavyside_step(K: usize, step_time: usize, step_value: f32) -> Vec<Vector1<f32>> {
    let mut step: Vec<Vector1<f32>> = Vec::new();
    for i in 0..K {
        if i >= step_time {
            let val = vector![step_value];
            step.push(val);
        } else {
            let val = vector![0.0];
            step.push(val);
        }
    }
    step
}

// discrete time linear state space
pub struct DTLSS<T> {
    pub A: Matrix3<T>,
    pub B: Vector3<T>,
    pub C: RowVector3<T>,
    pub D: Vector1<T>,
}

impl<T> DTLSS<T> {
    pub fn new(A: Matrix3<T>, B: Vector3<T>, C: RowVector3<T>, D: Vector1<T>) -> Self {
        Self {A, B, C, D}
    }

}


impl DTLSS<f32> {
    
    pub fn lsim
    (
        &self,
        K: usize,
        U: Vec<Vector1<f32>>,
    ) -> (Vec<Vector3<f32>>, Vec<Vector1<f32>>) {
        let mut states: Vec<Vector3<f32>> = Vec::new();
        let mut outputs: Vec<Vector1<f32>> = Vec::new();
        states.push(vector![2.0, 1.0, 3.0]);
        outputs.push(&self.C * vector![2.0, 1.0, 3.0]);
        for k in 1..K {
            states.push(&self.A * states[k - 1] + &self.B * U[k - 1]);
            outputs.push(&self.C * &states[k]);
        }
        (states, outputs)
    }

    pub fn step(&self, K: usize) -> (Vec<Vector3<f32>>, Vec<Vector1<f32>>){
        let step_time = 0;
        let step_value = 1.0;
        let input = heavyside_step(K, step_time, step_value);
        self.lsim(K, input)
    }
}
