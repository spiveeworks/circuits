use std::ops::{Deref, DerefMut};
pub mod update
{
    pub struct Update 
    {
        pub rising: Box<[u8]>, 
        pub falling: Box<[u8]>, 
        pub thin: Box<[u8]>,
    }
    type Triple<T> = (T, T, T);
    fn singleton(input: u8) -> Triple<Box<[u8]>>
      {(Box::new([input]), Box::default(), Box::default())}
    
    impl Update
    {
        pub fn thick_singleton(state: bool) -> Self
        {
            let (updated, empty1, empty2) = singleton(0);
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
        pub fn thin_singleton() -> Self
        {
            let (thin, thick1, thick2) = singleton(0);
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


pub trait InferGate
{
    type State;
    fn from_input(Self::State, &[bool]) -> (Self, Box<[bool]>) where Self: Sized;
    fn into_state(Self) -> Self::State;
}

pub trait UpdateGate
{
    fn update(&mut self, update::Update) -> update::Update;
}

pub struct JunctionMem {active: u8, total: u8}

impl JunctionMem
{
    fn from_input(inputs: &[bool]) -> Self
    {
        JunctionMem
        {
            active: 
            {
                let add_bools = |active, &each|
                    if each
                      {active + 1}
                    else
                      {active};
                inputs
                    .iter()
                    .fold(0, add_bools)
            },
            total: 
                inputs.len() as u8,
        }
    }
}

pub trait JunctionGate: Deref<Target=JunctionMem> + DerefMut + From<JunctionMem>
{
    fn infer_output(active: u8, total: u8) -> bool;

    fn mem_output(mem: &JunctionMem) -> bool
      {Self::infer_output(mem.active, mem.total)}
    fn output(&self) -> bool
      {Self::mem_output(self)}
}

impl<G: JunctionGate> InferGate for G
{
    type State = ();
    fn from_input(_: (), inputs: &[bool]) -> (Self, Box<[bool]>)
    {
        let mem = JunctionMem::from_input(inputs);
        let output = G::mem_output(&mem);
        (Self::from(mem), Box::new([output]))
    }
    fn into_state(_: G) -> () {}
}
impl<G: JunctionGate> UpdateGate for G
{
    fn update(&mut self, update: update::Update) -> update::Update
    {
        let before = self.output();
        self.active += update.rising.len() as u8;
        self.active -= update.falling.len() as u8;
        let after = self.output();
        if before == after
          {update::Update::default()}
        else
          {update::Update::thick_singleton(after)}
    }
}


pub trait ExplicitUpdateGate
{
    fn update(&mut self, update::ExplicitUpdate) -> update::Update;
}

pub trait ThinGate
{
    fn result(&self, update::ExplicitUpdate) -> bool;
}

impl<G: ThinGate> ExplicitUpdateGate for G
{
    fn update(&mut self, update: update::ExplicitUpdate) -> update::Update
    {
        if G.result(update)
          {update::Update::thin_singleton()}
        else
         {update::<Update as Default>::default()}
    }
}
