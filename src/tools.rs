use libm::atan2;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::env::temp_dir;
use std::vec;

/// generate one random direction -> (phi, theta).
pub fn random_direction(seed: u64) -> (f64, f64) {
    let mut random = StdRng::seed_from_u64(seed);
    let a: f64 = random.gen();
    let b: f64 = random.gen();
    let theta: f64 = (2.0 * a - 1.0).acos();
    let phi: f64 = 2.0 * std::f64::consts::PI * b;
    return (phi.to_degrees(), theta.to_degrees());
}

/// Convert from (phi, theta) angles to cartesian Vec<f64>.
pub fn spherical_to_cartesian(r: f64, phi: f64, theta: f64) -> Vec<f64> {
    let phi: f64 = phi.to_radians();
    let theta: f64 = theta.to_radians();
    let x: f64 = r * theta.sin() * phi.cos();
    let y: f64 = r * theta.sin() * phi.sin();
    let z: f64 = r * theta.cos();
    return vec![x, y, z];
}

/// Convert from cartesian Vec<f64> to (phi, theta) angles
pub fn cartesian_to_spherical(vector: Vec<f64>) -> (f64, f64, f64) {
    let r: f64 = vector3_norm(vector.clone());
    let phi: f64 = atan2(vector[1], vector[0]);
    let theta: f64 = (vector[3] / r).acos();
    return (r, phi.to_degrees(), theta.to_degrees());
}

/// length of vector
pub fn vector3_norm(vector: Vec<f64>) -> f64 {
    let mut norm: f64 = 0.0;
    for i in 0..vector.len() {
        norm += vector[i] * vector[i];
    }
    norm.sqrt()
}

pub fn elementwise_subtraction(vec_a: &Vec<f64>, vec_b: &Vec<f64>) -> Vec<f64> {
    vec_a.into_iter().zip(vec_b).map(|(a, b)| a - b).collect()
}

pub fn elementwise_addition(vec_a: &Vec<f64>, vec_b: &Vec<f64>) -> Vec<f64> {
    vec_a.into_iter().zip(vec_b).map(|(a, b)| a + b).collect()
}

/// Generate N random directions in a Vec of tuples of (phi, theta) angles
pub fn N_random_directions(n: usize, seed: u64) -> Vec<(f64, f64)> {
    let mut random = StdRng::seed_from_u64(seed);
    let mut directions: Vec<(f64, f64)> = Vec::new();
    for _i in 0..n {
        let a: f64 = random.gen();
        let b: f64 = random.gen();
        let theta: f64 = (2.0 * a - 1.0).acos();
        let phi: f64 = 2.0 * std::f64::consts::PI * b;
        directions.push((phi.to_degrees(), theta.to_degrees()))
    }
    return directions;
}

/// Generate PKA positions based on a list of energies and box vectors,
/// and suggested minimum distance between pkas
pub fn generate_pka_positions(vec_pka: &Vec<f64>, vec_x: &Vec<f64>, vec_y: &Vec<f64>, vec_z: &Vec<f64>, r:f64, seed: u64) -> Vec<Vec<f64>> {
    let mut positions: Vec<Vec<f64>> = Vec::new();
    let mut random = StdRng::seed_from_u64(seed);
    for _ in 0..vec_pka.len(){
        if positions.len() == 0{
            // generate random first position in cell
            let mut position = vec![random.gen_range(vec_x[0]..vec_x[1]),
                                              random.gen_range(vec_y[0]..vec_y[1]),
                                              random.gen_range(vec_z[0]..vec_z[1])];
            positions.push(position);
        }
        else{
            // generate new position trying to maximize distance from previous points
            let mut best_pos = Vec::new();
            let mut best_dist = 0.0;
            let mut good_found = false;
            for _i in 0..1000 {
                let mut current_position = vec![random.gen_range(vec_x[0]..vec_x[1]),
                                                          random.gen_range(vec_y[0]..vec_y[1]),
                                                          random.gen_range(vec_z[0]..vec_z[1])];
                let mut shortest = 99999999.0;
                for p in &positions {
                    let mut dist = distance_between_periodic(&p,&current_position, vec_x, vec_y, vec_z);
                    if dist < shortest{
                        shortest = dist;
                    }
                }

                if shortest > r{
                    good_found = true;
                    best_pos = current_position;
                    best_dist = shortest;
                    break;
                } else if shortest >= best_dist {
                    best_dist = shortest;
                    best_pos = current_position;
                }

            }
            positions.push(best_pos);
        }
    }  
    return positions;
}

/// distance between two points a and b.
pub fn distance_between(vec_a: &Vec<f64>, vec_b: &Vec<f64>) -> f64 {
    let mut dist: f64 = 0.0;
    for i in 0..vec_a.len() {
        dist += (vec_b[i] - vec_a[i]).powi(2);
    }
    dist.sqrt()
}

/// distance between two points a and b in a periodic cubic cell.
pub fn distance_between_periodic(vec_a: &Vec<f64>, vec_b: &Vec<f64>, vec_x: &Vec<f64>, vec_y: &Vec<f64>, vec_z: &Vec<f64> ) -> f64 {
    let mut dist: f64 = 0.0;
    let mut lengths = vec![vec_x[1]-vec_x[0],vec_y[1]-vec_y[0],vec_z[1]-vec_z[0]];
    for i in 0..vec_a.len() {
        let mut tmp_dist = vec_b[i] - vec_a[i];
        if tmp_dist < -lengths[i] * 0.5 {
            tmp_dist = tmp_dist + lengths[i]
        }
        if tmp_dist >= lengths[i] * 0.5 {
            tmp_dist = tmp_dist - lengths[i]
        }
        dist += (tmp_dist).powi(2);
    }
    dist.sqrt()
}
