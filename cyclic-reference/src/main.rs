#![allow(dead_code)]
use std::fmt;

enum EA {
    Empty,
    MyBox { val: Box<ObjA> },
}

enum EB {
    Empty,
    MyBox { val: Box<ObjB> },
}

struct ObjB {
    x: EA,
}

struct ObjA {
    x: EB,
}

impl fmt::Debug for EA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EA::Empty => write!(f, "EA::Empty"),
            EA::MyBox { .. } => write!(f, "EA::MyBox containing ObjA"),
        }
    }
}

impl fmt::Debug for EB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EB::Empty => write!(f, "EB::Empty"),
            EB::MyBox { .. } => write!(f, "EB::MyBox containing ObjB"),
        }
    }
}

impl fmt::Debug for ObjA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ObjA with {:?}", self.x)
    }
}

impl fmt::Debug for ObjB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ObjB with {:?}", self.x)
    }
}

impl ObjB {
    fn set_x(x_in: ObjA) -> ObjB {
        ObjB {
            x: EA::MyBox {
                val: Box::new(x_in),
            },
        }
    }
}

impl ObjA {
    fn set_x(x_in: ObjB) -> ObjA {
        ObjA {
            x: EB::MyBox {
                val: Box::new(x_in),
            },
        }
    }
}

fn main() {
    let _my_a = ObjA { x: EB::Empty };
    let my_b = ObjB { x: EA::Empty };

    let my_aa = ObjA::set_x(my_b);
    let _my_bb = ObjB::set_x(my_aa);

    println!("{:?}", _my_bb);
}
