use std::ops::{Deref, DerefMut};
mod update
{
    pub struct Update 
    {
        rising: Box<[u8]>, 
        falling: Box<[u8]>, 
        thin: Box<[u8]>,
    }
    
    fn singleton(input: u8) -> [Box<u8>;3]
      {[Box::new([input]), Box::default(), Box::default()]}
    
    impl Update
    {
        fn thick_singleton(state: bool) -> Self
        {
            let [updated, empty1, empty2] = singleton(0);
            if state
            {
                Update 
                {
                    rising: updated, 
                    falling: empty1, 
                    thin: empty2,
                }
            }
            else
            {
                Update 
                {
                    rising: empty1,
                    falling: updated,
                    thin: empty2,
                }
            }
        }
        fn thin_singleton() -> Self
        {
            let [thin, thick1, thick2] = singleton(0);
            Update
            {
                rising: thick1,
                falling: thick2,
                thin: thin,
            }
        }
    }
    impl Default for Update
    {
        fn default() -> Self
        {
            Update
            {
                rising: Box::default(),
                falling: Box::default(),
                thin: Box::default(),
            }
        }
    }
}


trait InferGate
{
    type State;
    fn from_input(Self::State, &[bool]) -> (Self, Box<bool>);
    fn into_state(Self) -> Self::State;
}

trait UpdateGate
{
    fn update(&mut self, update::Update) -> update::Update;
}

struct JunctionMem {active: u8, total: u8}

impl JunctionMem
{
    fn from_input(inputs: &[bool]) -> Self
    {
        JunctionMem
        {
            active: 
            {
                let add_bools = |active, each|
                    if each
                      {active + 1}
                    else
                      {active};
                inputs
                    .iter()
                    .fold(add_bools)
            },
            total: 
                inputs.size(),
        }
    }
}

trait JunctionGate: Deref<Target=JunctionMem> + DerefMut + From<JunctionMem>
{
    fn infer_output(active: u8, total: u8) -> bool;

    fn mem_output(mem: &JunctionMem) -> bool
      {Self::infer_output(mem.active, mem.total)}
    fn get_output(&self) -> bool
      {Self::mem_output(self)}
}

impl<G: JunctionGate> InferGate for G
{
    type State = ();
    fn from_input(state: (), inputs: &[bool]) -> (Self, Box<[bool]>)
    {
        let mem = JunctionMem::from_input(inputs);
        let output = G::mem_output(mem);
        (Self::from(mem), Box::new([output]))
    }
    fn into_state(gate: G) -> () {}
}
impl<G: JunctionGate> UpdateGate for G
{
    fn update(gate: &mut G, update: update::Update) -> update::Update
    {
        let before = gate.output();
        gate.active += update.rising.size();
        gate.active -= update.falling.size();
        let after = gate.output();
        if before == after
          {update::Update::default()}
        else
          {update::Update::thick_singleton(after)}
    }
}
