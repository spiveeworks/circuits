struct And(JunctionMem);
struct Or(JunctionMem);



impl Deref for And
{
    Target = JunctionMem;
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
    Target = JunctionMem;
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
    fn infer_output(active: u8, total: u8) -> bool
      {active >= 1}
}
