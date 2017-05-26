#[cfg(test)]
mod tests {
    mod gate;
    mod gateimpl;

    #[test]
    fn test_gate() 
    {
        let (and, or) = 
        {
            let inputs = [false, false];
            let (and, and_out) = <And as InferGate>::from_input((), &inputs);
            let ( or,  or_out) = < Or as InferGate>::from_input((), &inputs);
            assert!(!and_out);
            assert!( !or_out);
            (and, or)
        };
        {
            let input1 = update::Update::thick_singleton(true);
            let input2 = update::Update::thick_singleton(true);
            let and_up = and.update(input1);
            let or_up = or.update(input2);
            assert!(and_up.rising.size() == 0);
            assert!(and_up.falling.size() == 0);
            assert!(and_up.thin.size() == 0);
            assert!(or_up.rising.size() == 1);
            assert!(or_up.rising[0] == 0);
            assert!(or_up.falling.size() == 0);
            assert!(or_up.thin.size() == 0);
        };
        let () = and.into_state;
        let () = or.into_state;
    }
}
