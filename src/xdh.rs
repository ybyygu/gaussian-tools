// [[file:../xo-tools.note::006191b3][006191b3]]
//! Reference: https://github.com/igor-1982/xDH4Gau
// 006191b3 ends here

// [[file:../xo-tools.note::0c085add][0c085add]]
#[allow(non_camel_case_types)]

use super::*;
// 0c085add ends here

// [[file:../xo-tools.note::101dbb9a][101dbb9a]]
mod output;
mod input;
// 101dbb9a ends here

// [[file:../xo-tools.note::8e5ac845][8e5ac845]]
const N: usize = 7;
type Component = [f64; N];

/// The xDH family of DH functionals
#[derive(Clone, Copy, Debug)]
pub enum Functional {
    XYG3,
    XYG5,
    XYG6,
    XYG7,
    XYGJ_OS,
    revXYG3,
}

pub struct xDH {
    energy_no_xc: f64,
    component: Component,
}
// 8e5ac845 ends here

// [[file:../xo-tools.note::75f764fb][75f764fb]]
impl Functional {
    fn parameters(&self) -> Component {
        match self {
            Self::XYG3 => [0.8033, -0.0140, 0.2107, 0.0000, 0.6789, 0.3211, 0.3211],
            Self::XYG5 => [0.9150, 0.0612, 0.0238, 0.0000, 0.4957, 0.4548, 0.2764],
            Self::XYG6 => [0.9105, 0.1576, -0.0681, 0.1800, 0.2244, 0.4695, 0.2426],
            Self::XYG7 => [0.8971, 0.2055, -0.1408, 0.4056, 0.1159, 0.4052, 0.2589],
            Self::XYGJ_OS => [0.7731, 0.2269, 0.0000, 0.2309, 0.2754, 0.4364, 0.0000],
            Self::revXYG3 => [0.9196, -0.0222, 0.1026, 0.0000, 0.6059, 0.3941, 0.3941],
        }
    }
}
// 75f764fb ends here

// [[file:../xo-tools.note::eb6caf8d][eb6caf8d]]
impl xDH {
    /// Return final xDH functional energy from energy components
    pub fn energy(&self, functional: Functional) -> f64 {
        let energy: f64 = self
            .component
            .into_iter()
            .zip(functional.parameters())
            .map(|(energy, param)| energy * param)
            .sum();
        energy + self.energy_no_xc
    }
}
// eb6caf8d ends here

// [[file:../xo-tools.note::595d4056][595d4056]]
#[test]
fn test_xdh_energy() -> Result<()> {
    let f: &Path = "tests/files/Job_o2.log".as_ref();

    let xdh = xDH::collect_from_gaussian(f)?;
    assert_eq!(-150.25844295353738, xdh.energy(Functional::XYG3));
    assert_eq!(-150.05851758259632, xdh.energy(Functional::XYG5));
    assert_eq!(-150.005907822815, xdh.energy(Functional::XYG6));
    assert_eq!(-149.5397701816522, xdh.energy(Functional::XYG7));
    assert_eq!(-149.96275308354984, xdh.energy(Functional::XYGJ_OS));
    assert_eq!(-150.25238782233433, xdh.energy(Functional::revXYG3));

    Ok(())
}
// 595d4056 ends here
