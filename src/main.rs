use nalgebra::{matrix, vector, Matrix3, RowVector3, Vector3, Vector1};
use plotters::prelude::*;

fn generate_sim_results_plot(simdata: &Vec<Vector3<f32>>) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("./test.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let length = simdata.len();
    let mut chart = ChartBuilder::on(&root)
        .caption("simulation results", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f32..8f32, 0f32..length as f32)?;

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
        .draw_series(LineSeries::new(
            first_dimension,
            &RED,
        ))?
        .label("first dimension")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    

    chart
        .draw_series(LineSeries::new(
            second_dimension,
            &BLUE,
        ))?
        .label("second dimension")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    
    chart
        .draw_series(LineSeries::new(
            third_dimension,
            &GREEN,
        ))?
        .label("third dimension")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}

fn lsim(A: &Matrix3<f32>, B: &Vector3<f32>, C: &RowVector3<f32>,  D:&Vector1<f32>, K: usize) -> Vec<Vector3<f32>>{
    let output: Vec<Vector3<f32>> = Vec::new();
    let mut states: Vec<Vector3<f32>> = Vec::new();
    states.push(vector![2.0,1.0,3.0]);
    for k in 1..K {
        states.push(A*states[k-1]);
    }
    states
}

fn main() {
    println!("Hello, world!");
    let A: Matrix3<f32> = matrix![3.0,0.0,0.0;0.0,3.0,0.0;0.0,0.0,3.0];
    let B: Vector3<f32> = vector![1.0, 0.0, 0.0];
    let C: RowVector3<f32> = vector![1.0, 0.0, 0.0].transpose();
    let D: Vector1<f32> = Vector1::zeros();
    let states = lsim(&A, &B, &C, &D, 10);
    generate_sim_results_plot(&states);
    println!("{:?}", states);
}
