use nalgebra::{DMatrix, DVector, Vector3};

fn main() {
    let mut buffer = String::with_capacity(1024);

    let mut beacons = Vec::new();
    let mut means = Vec::new(); // of production

    while let Ok(s) = std::io::stdin().read_line(&mut buffer) {
        if s == 0 {
            break;
        }

        // read the first line
        buffer.clear();
        let mut positions = Vec::new();
        let mut mean = Vector3::zeros();
        while let Ok(_size) = std::io::stdin().read_line(&mut buffer) {
            let line: &str = buffer.as_str().trim_end();
            if line.len() == 0 {
                break;
            }

            let mut p = line.split(',');
            let x: f32 = p.next().expect("next").parse().expect("parse");
            let y: f32 = p.next().expect("next").parse().expect("parse");
            let z: f32 = p.next().expect("next").parse().expect("parse");

            positions.push(x);
            positions.push(y);
            positions.push(z);

            mean += Vector3::new(x, y, z);

            buffer.clear();
        }

        mean /= (positions.len() / 3) as f32;
        means.push(mean);

        let positions = DMatrix::from_vec(3, positions.len() / 3, positions);
        beacons.push(positions);
    }

    for (i, posa) in beacons.iter().enumerate().take(1) {
        let shift_a = shift_mean_to_origin(&posa, means[i]);

        for (j, posb) in beacons.iter().enumerate().filter(|(j, _)| i != *j).take(1) {
            // translate both a and b, so that their mean is 0,0,0
            let shift_b = shift_mean_to_origin(&posb, means[j]);

            let r = kabsch(&shift_a, &shift_b);
            println!("{}\n------------------------------", r);

            let foo = &r * posb;

            println!("{}\npogpogpogpog", foo.transpose());
        }
    }
}

fn shift_mean_to_origin(mat: &DMatrix<f32>, mean: Vector3<f32>) -> DMatrix<f32> {
    let mut shifted = mat.clone();
    for mut col in shifted.column_iter_mut() {
        let res = &col - mean;
        col.copy_from(&res);
    }
    shifted
}

/// find the rotation matrix between two sets of points via the Kabsch algo
///
/// assumes that the centroids of a and b are 0!
fn kabsch(a: &DMatrix<f32>, b: &DMatrix<f32>) -> DMatrix<f32> {
    // covariance matrix
    let b = a * b.transpose();

    let svd = b.svd(true, true);

    let u = svd.u.unwrap();
    let v_t = svd.v_t.unwrap();

    let m = DMatrix::from_diagonal(&DVector::from_row_slice(&[
        1.0,
        1.0,
        u.determinant() * v_t.determinant(), // mirror compensation
    ]));

    let r = v_t.transpose() * m * u.transpose();
    r
}
