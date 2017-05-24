
trait InferGate
{
    type State;
    fn from_input(State, &[bool]) -> Self;
    fn into_state(Self) -> State;
}

trait UpdateGate
{
    fn update(&mut self, Update) -> Update;
}

struct Junction {inputs: u8, active: u8}

impl InferGate for Junction
{
    type State = ();
    fn from_input(_, inputs: &[bool]) -> Self
    {
        Junction
        {
            inpute: 
                inputs.size(),
            active: 
                inputs
                    .iter()
                    .fold(|active, each|
                if each
                  {active + 1}
                else
                  {active},
        }
    }
    fn into_state(_) -> () {}
}

struct And(Junction);

impl InferGate for And
{
    State = <Junction as InferGate>::State;
    fn from_input(state: State, inputs: &[bool]) -> Self
      {And(InferGate::from_input(state, inputs))}
    fn into_state(gate: And) -> State
      {gate.0.into_state()}
}
