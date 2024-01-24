# findPKA

Command line tool for generating random spherically distributed Primary Knock-on Atoms (PKA) for very large lammps data files. 


## Building
The tool can be built by running the command: 

```cargo build```

### Dependencies

rand = "0.8.4"

libm = "0.2.8"

clap = { version = "4.4.2", features = ["derive"] }

## Usage
In order to use the tool you need to specify in the command line (at least) the following:

* filename/path of lammps data file
* rPKA = distance of PKA atom from center of box (Å)
* energy = energy of PKA atom (eV)
* element mapping ```--map``` in the same order as the types given in the data file. For example if the data file has 3 types ```--map Fe Ni Cr``` assumes that Fe is type 1, Ni type id 2 and Cr type 3. **order matters!** 

```
Usage: findPKA [OPTIONS] --r-pka <R_PKA> --energy <ENERGY> --map <MAP>... <FILENAME>

Arguments:
  <FILENAME>  input lammps data file

Options:
  -n <N>                       number of PKAs [default: 1]
  -t, --tolerance <TOLERANCE>  tolerance to find PKA (Å). [default: 5]
      --seed <SEED>            seed random number generator. [default: 1234]
  -r, --r-pka <R_PKA>          PKA distance from center (Å).
  -e, --energy <ENERGY>        PKA energy (eV).
  -m, --map <MAP>...           Map elements ('Al,W...') in same order as type in data file
      --mass <MASS>...         Give masses in same order as in element mapping (if not given mass infered)
  -v, --verbose                verbose output
  -l, --lammps                 print lammps ready string
  -h, --help                   Print help

```

```--mass``` (optional) can be used to define masses of the types given in ```--map``` (has to be the same amount of arguments!). Note that some atomic masses are hard coded into the program (main.rs) as well, the command ```--mass``` overrides these.

## examples

### verbose run
```
./findPKA -n 2 -r 30 -e 1000 --map Al Cr Cu Fe Ni --verbose --lammps relax_boxes/data.relaxed-80
```
verbose output given in the following form:
```
--- RUN 1 ---
PKA element: Al
PKA energy: 1000
PKA mass: 26.9815385
PKA lammps id: 293750
center position: [146.15028711923364, 146.15028711923364, 146.15028711923364]
PKA position: [136.77317620669012, 129.55078942364506, 164.10260345960427]
PKA direction: [9.378151835360041, 16.97015772923964, -22.892444491527353]
PKA velocity: [264.3676479987336 478.3843089563687 -645.3320241998733]
-var PKA 293750 -var vx 264.3676479987336 -var vy 478.3843089563687 -var vz -645.3320241998733
--- RUN 2 ---
PKA element: Fe
PKA energy: 1000
PKA mass: 55.845
PKA lammps id: 1279222
center position: [146.15028711923364, 146.15028711923364, 146.15028711923364]
PKA position: [166.04959424484156, 129.56079624110717, 142.42241780869387]
PKA direction: [-21.228921987007535, 20.03436060943738, 6.925118500110463]
PKA velocity: [-415.9682621123177 392.5615332864986 135.69363103674334]
-var PKA 1279222 -var vx -415.9682621123177 -var vy 392.5615332864986 -var vz 135.69363103674334
```

## todo

* Infer masses from data file
