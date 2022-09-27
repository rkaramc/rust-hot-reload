#[derive(Default, Debug)]
pub struct State {
    pub counter: usize,
}

#[no_mangle]
pub fn step(state: &mut State) {
    state.counter += 1;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(4, 2 + 2);
    }
}
