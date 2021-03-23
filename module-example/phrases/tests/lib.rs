#[cfg(test)]
mod tests {
    extern crate phrases;

    //#[ignore]
    #[test]
    #[should_panic]
    fn english_greeting_correct() {
        assert_eq!("Hello", phrases::greetings::english::hello());
        assert_eq!("Hello2", phrases::greetings::english::hello());
    }
}