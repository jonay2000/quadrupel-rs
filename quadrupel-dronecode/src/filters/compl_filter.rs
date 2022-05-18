use crate::library::fixed_point::FI32;

pub struct ComplFilter {
    b: FI32,
    phi : FI32,
    c1 : FI32,
    c2 : FI32
}

impl ComplFilter {
    pub fn new(c1:FI32, c2:FI32) -> Self{
        ComplFilter {
            b: FI32::from_num(0),
            phi: FI32::from_num(0),
            c1,
            c2
        }
    }

    pub fn filter(&mut self, sp:FI32, sphi:FI32, dt:FI32) -> (FI32,FI32) {
    let p = sp-self.b;
    self.phi = self.phi + p*dt;
    let e = self.phi - sphi;
    self.phi = self.phi - e/self.c1;
    self.b = self.b+(e/dt)/self.c2;
    (p, self.phi)
    }

}