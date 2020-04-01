use ndarray::*;
use eom::traits::*;

#[derive(Clone,Copy,Debug)]
pub struct SirModel {
    pub lambda: f64,
    pub gamma: f64,
}

impl ModelSpec for SirModel {
    type Scalar = f64;
    type Dim = Ix1;

    fn model_size(&self) -> usize {
        3
    }
}

impl Explicit for SirModel {
    fn rhs<'a, S>(&mut self, v: &'a mut ArrayBase<S, Ix1>) -> &'a mut ArrayBase<S, Ix1>
    where
        S: DataMut<Elem=f64>,
    {
        let s = v[0];
        let i = v[1];
        let _r = v[2];
        v[0] = -self.lambda*s*i;
        v[1] = self.lambda*s*i - self.gamma*i;
        v[2] = self.gamma*i;
        v
    }
}
