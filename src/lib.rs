pub mod gate;
pub mod gateimpl;

#[cfg(test)]
mod tests {
    use super::gateimpl::{And, Or};
    use super::gate::update;
    use super::gate;
    #[test]
    fn test_gate() 
    {
        let (mut and, mut or) = 
        {
            let inputs = [false, false];
            let (and, and_out) = <And as gate::InferGate>::from_input((), &inputs);
            let ( or,  or_out) = < Or as gate::InferGate>::from_input((), &inputs);
            assert!(!and_out[0]);
            assert!( !or_out[0]);
            (and, or)
        };
        {
            use gate::UpdateGate;

            let input1 = update::Update::thick_singleton(true);
            let input2 = update::Update::thick_singleton(true);
            let and_up = and.update(input1);
            let or_up = or.update(input2);
            assert!(and_up.rising.len() == 0);
            assert!(and_up.falling.len() == 0);
            assert!(and_up.thin.len() == 0);
            assert!(or_up.rising.len() == 1);
            assert!(or_up.rising[0] == 0);
            assert!(or_up.falling.len() == 0);
            assert!(or_up.thin.len() == 0);
        };
        let () = <And as gate::InferGate>::into_state(and);
        let () = <Or as gate::InferGate>::into_state(or);
    }

    #[test]
    fn test_thin_singleton()
    {
        let single = update::Update::thin_singleton();
        assert!(single.rising.len() == 0);
        assert!(single.falling.len() == 0);
        assert!(single.thin.len() == 1);
        assert!(single.thin[0] == 0);
    }
}
