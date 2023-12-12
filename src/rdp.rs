use num::traits::Pow;

pub fn rdp_downsample(input: &[(f64, f64)], epsilon: f64) -> Vec<(f64, f64)> {
    let start = &input[0];
    let end = &input[input.len() - 1];
    let mut down_sampled_points = Vec::<(f64, f64)>::new();
    let furthest_point = input
        .iter()
        .map(|x| distance_from_line((start, end), x))
        .enumerate()
        .fold((0, 0.0), |(idx_max, val_max), (idx, val)| {
            if val_max > val {
                (idx_max, val_max)
            } else {
                (idx, val)
            }
        });
    if furthest_point.1 > epsilon {
        down_sampled_points.append(&mut rdp_downsample(&input[0..furthest_point.0], epsilon));
        down_sampled_points.append(&mut rdp_downsample(&input[furthest_point.0..], epsilon));
        return down_sampled_points;
    } else {
        return vec![input[0], input.iter().last().unwrap().clone()];
    }
}

// fn linear_distance<U, V>(p1: &(U, V), p2: &(U, V)) -> f64
// where
//     U: Into<f64>,
//     V: Into<f64>,
// {
//     let distance: f64 =
//         f64::sqrt(((p2.0).into() - (p1.0).into()).pow(2) + ((p2.1).into() - (p1.1).into()).pow(2));
//     return distance;
// }

/// Compute delta y of a point from a line given by 2 points
fn distance_from_line(line: (&(f64, f64), &(f64, f64)), point: &(f64, f64)) -> f64 {
    let x0 = point.0;
    let x1 = line.0 .0;
    let x2 = line.1 .0;
    let y0 = point.1;
    let y1 = line.0 .1;
    let y2 = line.1 .1;

    let numerator = f64::abs((x2 - x1) * (y1 - y0) - (x1 - x0) * (y2 - y1));
    let denominator = f64::sqrt((x2 - x1).pow(2) + (y2 - y1).pow(2));
    return numerator / denominator;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rdp_simple() {
        let input = vec![(1.0, 1.0), (2.0, 2.1), (3.0, 3.0), (4.0, 4.0), (5.0, 5.0)];
        let correct_output = vec![(1.0, 1.0), (5.0, 5.0)];
        let result = rdp_downsample(&input, 5.0);
        assert_eq!(correct_output, result);
    }

    #[test]
    fn test_rdp_with_splits() {
        let input = vec![(1.0, 1.0), (2.0, 2.), (3.0, 3.0), (4.0, 10.0), (5.0, 11.0), (6.0, 12.0)];
        let correct_output = vec![(1.0, 1.0), (3.0, 3.0), (4.0, 10.0), (6.0, 12.0)];
        let result = rdp_downsample(&input, 0.1);
        assert_eq!(correct_output, result);
    }
}
