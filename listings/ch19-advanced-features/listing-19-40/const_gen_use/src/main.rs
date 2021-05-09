mod traffic_light{

    /// type encoding the red state
    pub struct Red;

    /// type encoding the red and amber state
    pub struct RedAmber;

    /// type encoding the green state
    pub struct Green;

    /// type encoding the amber state
    pub struct Amber;

    /// trait satisfied by all possible states of traffic light
    pub trait LightState {
        /// prints the current state
        fn print(&self);
    }

    impl LightState for Red {
        /// prints the current state
        fn print(&self) {
            println!("I am Red");
        }
    }

    impl LightState for RedAmber {
        /// prints the current state
        fn print(&self) {
            println!("I am Red and Amber");
        }
    }

    impl LightState for Amber {
        /// prints the current state
        fn print(&self) {
            println!("I am Amber");
        }
    }

    impl LightState for Green {
            /// prints the current state
        fn print(&self) {
            println!("I am Green");
        }
    }

    /// type representing the traffic light
    pub struct Light<State> where State : LightState {
        state : State
    }

    /// creates a new light that is red
    pub fn new() -> Light<Red> {
        Light::<Red>{
            state : Red
        }
    }

    impl<State> Light<State> where State : LightState
    {
        /// prints the current state
        pub fn print(&self) {
            self.state.print();
        }
    }

    impl Light<Red> {
        /// transition function turning Light<Red> to Light<RedAmber>
        pub fn transition(self) -> Light<RedAmber> {
            Light::<RedAmber> {
                state : RedAmber
            }
        }
    }

    impl Light<RedAmber> {
        /// transition function turning Light<RedAmber> to Light<Green>
        pub fn transition(self) -> Light<Green> {
            Light::<Green> {
                state : Green
            }
        }
    }

    impl Light<Green> {
        /// transition function turning Light<Green> to Light<Amber>
        pub fn transition(self) -> Light<Amber> {
            Light::<Amber> {
                state : Amber
            }
        }
    }

    impl Light<Amber> {
        /// transition function turning Light<Amber> to Light<Red>
        pub fn transition(self) -> Light<Red> {
            Light::<Red> {
                state : Red
            }
        }
    }

    pub type RedLight = Light<Red>;
    pub type RedAmberLight = Light<RedAmber>;
    pub type GreenLight = Light<Green>;
    pub type AmberLight = Light<Amber>;
}

use crate::traffic_light::{RedLight,RedAmberLight,GreenLight, AmberLight};

fn main() {
    let rlight : RedLight = traffic_light::new();
    rlight.print();
    let glight : GreenLight = rlight.transition().transition();
    glight.print();
    let alight : AmberLight = glight.transition();
    alight.print();
    let ralight : RedAmberLight = alight.transition().transition();
    ralight.print();
}