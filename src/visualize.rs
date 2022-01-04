use nalgebra::DVector;
use plotters::prelude::*;

pub fn show(file_name: &str, v: &DVector<i64>) {
    let w = 7;
    let h = 7;



    let root = SVGBackend::new(file_name, (1024, 768)).into_drawing_area();

    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption(file_name, ("sans-serif", 20))
        .margin(5)
        .top_x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0i32..(w as i32), (h as i32)..0i32)
        .unwrap();

    chart
        .configure_mesh()
        .x_labels(w)
        .y_labels(h)
        .x_label_offset(35)
        .y_label_offset(25)
        .disable_x_mesh()
        .disable_y_mesh()
        .label_style(("sans-serif", 20))
        .draw()
        .unwrap();

    let mut matrix = vec![vec![0; w]; h];

    assert_eq!(matrix.len(), h);
    assert_eq!(matrix[0].len(), w);

    for i in 0..7 {
        for j in 0..7 {
            matrix[i][j] = match v[i * 7 + j] {
                1 => 1,
                _ => 0,
            };
        }
    }

    chart
        .draw_series(
            matrix
                .iter()
                .zip(0..)
                .map(|(l, y)| l.iter().zip(0..).map(move |(v, x)| (x as i32, y as i32, v)))
                .flatten()
                .map(|(x, y, v)| {
                    Rectangle::new(
                        [(x, y), (x + 1, y + 1)],
                        match v {
                            1 => BLACK,
                            _ => WHITE,
                        }
                        .filled(),
                    )
                }),
        )
        .unwrap();
}
