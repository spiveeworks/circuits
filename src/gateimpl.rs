use std::ops::{Deref, DerefMut};

use gate::{JunctionMem, JunctionGate};



pub struct And(JunctionMem);
pub struct Or(JunctionMem);



impl Deref for And
{
    type Target = JunctionMem;
    fn deref(&self) -> &JunctionMem
      {&self.0}
}

impl DerefMut for And
{
    fn deref_mut(&mut self) -> &mut JunctionMem
      {&mut self.0}
}

impl From<JunctionMem> for And
{
    fn from(mem: JunctionMem) -> Self
      {And(mem)}
}


impl Deref for Or
{
    type Target = JunctionMem;
    fn deref(&self) -> &JunctionMem
      {&self.0}
}

impl DerefMut for Or
{
    fn deref_mut(&mut self) -> &mut JunctionMem
      {&mut self.0}
}

impl From<JunctionMem> for Or
{
    fn from(mem: JunctionMem) -> Self
      {Or(mem)}
}



impl JunctionGate for And
{
    fn infer_output(active: u8, total: u8) -> bool
      {active >= total}
}

impl JunctionGate for Or
{
    fn infer_output(active: u8,_total: u8) -> bool
      {active >= 1}
}




impl ThinGate for DetectRE
{
    fn result(&mut self, update: update::ExplicitUpdate) -> bool
      {update.thick[0] == (false, true)}
}




impl ThinGate for DetectFE
{
    fn result(&mut self, update: update::ExplicitUpdate) -> bool
      {update.thick[0] == (true, false)}
}




impl ThinGate for DetectE
{
    fn result(&mut self, update: update::ExplicitUpdate) -> bool
      {update.thick[0].0 != update.thick[0].1}
}


imple ThinGate for ThinDisjunct
{
    fn result(&mut self, update: update::ExplicitUpdate) -> bool
      {update.thin.any(|x| x)
