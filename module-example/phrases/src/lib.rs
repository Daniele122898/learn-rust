pub mod greetings {
    pub mod english;

    pub mod french {
        //! This module contains French phrases
        //! # Examples
        //! ```
        //! let username = "John";
        //! println!("{}, {}!",
        //!     phrases::greetings::french::hello(),
        //!     username);
        //! ```


        /// Says hello in french: `bonjour`.
        pub fn hello() -> String {
            return "bonjour".to_string();
        }
    }
}