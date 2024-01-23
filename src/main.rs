#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, non_snake_case))]
use crate::pka::PKA;
mod pka;
mod tools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
mod commandline;
use clap::Parser;

//use clap::Parser;

fn main() -> std::io::Result<()> {
    // Element mass mapping
    let mut masses = HashMap::from([
        ("H", 1.008),
        ("He", 4.002602),
        ("Be", 9.0121831),
        ("Al", 26.9815385),
        ("Ti", 47.867),
        ("V", 50.9415),
        ("Cr", 51.9961),
        ("Mn", 54.938044),
        ("Fe", 55.845),
        ("Co", 58.933194),
        ("Ni", 58.6934),
        ("Cu", 63.546),
        ("Zn", 65.38),
        ("Ga", 69.723),
        ("Ge", 72.630),
        ("Nb", 92.90637),
        ("Mo", 95.95),
        ("Ta", 180.94788),
        ("W", 183.84),
    ]);

    let args: commandline::Arguments = commandline::Arguments::parse();

    // Element Id mapping
    let mut id_to_element: HashMap<String, String> = HashMap::new();
    for (i, e) in args.map.iter().enumerate() {
        let type_id = (i + 1).to_string();
        id_to_element.insert(type_id, e.to_string());
    }
    // remapping masses if given
    if args.mass.len() != 0 {
        if args.mass.len() != args.map.len() {
            eprintln!("Error: --map has to have same number of arguments as --map");
            std::process::exit(1)
        } else {
            for (i, m) in args.mass.iter().enumerate() {
                let type_id = (i + 1).to_string();
                masses.insert(id_to_element[&type_id].as_str(), m.clone());
            }
        }
    }

    // Definitions
    let file = File::open(args.filename)?;
    let reader = BufReader::new(file);
    let mut center_pos: Vec<f64> = vec![0.0, 0.0, 0.0];
    let mut box_check_done: bool = false;
    let mut reading_atoms: bool = false;
    let mut PKA_positions: Vec<Vec<f64>> = Vec::new();
    let mut directions = tools::N_random_directions(args.n, args.seed);
    let mut total = 1;

    // Start reading data file
    for line in reader.lines() {
        // start reading atoms
        if box_check_done {
            if reading_atoms {
                let temp = line?;
                //println!("{}",temp);
                if temp.contains("Velocities") {
                    eprintln!("Error: Not all PKA found, check rPKA and tolerance!");
                    std::process::exit(1)
                }
                //empty line check
                if temp.len() < 3 {
                    continue;
                } else {
                    let collected: Vec<&str> = temp.split(" ").collect();
                    let id = collected[0];
                    let element_type = collected[1];
                    let elem = id_to_element[element_type].as_str();
                    let position: Vec<f64> = vec![
                        collected[2].parse::<f64>().unwrap(),
                        collected[3].parse::<f64>().unwrap(),
                        collected[4].parse::<f64>().unwrap(),
                    ];
                    // Optimization: if position too far from center skip checking agains positions.
                    if tools::distance_between(&center_pos, &position)
                        > args.rPKA + 2.0 * args.tolerance
                    {
                        continue;
                    }
                    // Check if atom position is suitable for a PKA.
                    let mut index_to_remove: Vec<usize> = Vec::new();
                    for i in 0..PKA_positions.len() {
                        if tools::distance_between(&position, &PKA_positions[i]) <= args.tolerance {
                            let dir = tools::spherical_to_cartesian(
                                args.rPKA,
                                directions[i].0,
                                directions[i].1,
                            );
                            let pka = pka::PKA::new(
                                elem,
                                id.parse::<i64>().unwrap(),
                                masses[elem],
                                args.energy,
                                &dir,
                            );
                            if args.verbose {
                                println!("--- RUN {} ---", total);
                                println!("PKA element: {}", elem);
                                println!("PKA energy: {}", args.energy);
                                println!("PKA mass: {}", masses[elem]);
                                println!("PKA lammps id: {}", id);
                                println!("center position: {:?}", center_pos);
                                println!("PKA position: {:?}", position);
                                println!("PKA direction: {:?}", dir);
                                println!("PKA velocity: [{} {} {}]", pka.vx, pka.vy, pka.vz);
                            }
                            if args.lammps {
                                pka.lammpsformatting();
                            } else {
                                println!("{}", pka);
                            }
                            index_to_remove.push(i);
                            total += 1;
                        }
                    }
                    for index in index_to_remove {
                        PKA_positions.remove(index);
                        directions.remove(index);
                        if PKA_positions.len() == 0 {
                            std::process::exit(0)
                        }
                    }
                }
            } else {
                // Detect that Atom information is coming.
                if line.unwrap().contains("Atoms #") {
                    reading_atoms = true;
                }
            }
        } else {
            // Read file untill all box information is defined
            let temp = line?;
            let collected: Vec<&str> = temp.split(" ").collect();
            if collected.len() < 4 {
                continue;
            }
            let pattern = collected[2];
            match pattern {
                "xlo" => {
                    center_pos[0] = (collected[1].parse::<f64>().unwrap()
                        - collected[0].parse::<f64>().unwrap())
                        / 2.0
                }
                "ylo" => {
                    center_pos[1] = (collected[1].parse::<f64>().unwrap()
                        - collected[0].parse::<f64>().unwrap())
                        / 2.0
                }
                "zlo" => {
                    center_pos[2] = (collected[1].parse::<f64>().unwrap()
                        - collected[0].parse::<f64>().unwrap())
                        / 2.0;
                    // Last center_pos coordinate defined above then generation of PKA_positions.
                    for i in 0..directions.len() {
                        let delta_vector: Vec<f64> = tools::spherical_to_cartesian(
                            args.rPKA,
                            directions[i].0,
                            directions[i].1,
                        );
                        let PKApos = tools::elementwise_subtraction(&center_pos, &delta_vector);
                        PKA_positions.push(PKApos)
                    }
                    box_check_done = true;
                }
                _ => (),
            }
        }
    }
    Ok(())
}
