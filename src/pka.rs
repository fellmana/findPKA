pub struct PKA {
    pub element: String,
    pub id: i64,
    pub mass: f64,
    pub energy: f64,
    pub direction: Vec<f64>,
    pub vx: f64,
    pub vy: f64,
    pub vz: f64,
}

/// Generate velocity components based on energy, mass and direction vector.
pub fn ekin_to_velocity(energy: f64, mass: f64, direction: &Vec<f64>) -> (f64, f64, f64) {
    let norm: f64 = direction.iter().map(|i| (*i) as f64).sum();
    let norm_dir: Vec<f64> = vec![
        direction[0] / norm,
        direction[1] / norm,
        direction[2] / norm,
    ];
    let vel_abs = (2.0 * energy / mass).sqrt();
    let eVu_to_Aps: f64 = 98.22695; // (eV/u)**0.5 to Å/ps
    return (
        eVu_to_Aps * norm_dir[0] * vel_abs,
        eVu_to_Aps * norm_dir[1] * vel_abs,
        eVu_to_Aps * norm_dir[2] * vel_abs,
    );
}

impl PKA {
    pub fn new(e: &str, id: i64, mass: f64, energy: f64, dir: &Vec<f64>) -> PKA {
        let (vx, vy, vz) = ekin_to_velocity(energy, mass, dir);
        PKA {
            element: e.to_string(),
            id: id,
            mass: mass,
            energy: energy,
            direction: dir.to_vec(),
            vx: vx,
            vy: vy,
            vz: vz,
        }
    }
    /// lammps ready string for cascade script
    pub fn lammpsformatting(&self) {
        println!(
            "-var PKA {} -var vx {} -var vy {} -var vz {}",
            self.id, self.vx, self.vy, self.vz
        );
    }
}

impl std::fmt::Display for PKA {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {} {} {} {}",
            self.element, self.id, self.energy, self.mass, self.vx, self.vy, self.vz
        )
    }
}
