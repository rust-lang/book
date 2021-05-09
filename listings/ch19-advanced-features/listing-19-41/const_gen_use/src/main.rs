mod traffic_light{
    /// type representing the traffic light
    pub struct Light<const RED: bool, const AMBER: bool, const GREEN: bool> {
        // private field ensures we can't construct
        // our own Light struct outside this module
        _private : (),
    }

    // public type aliases for the traffic light
    pub type RedLight = Light<true, false, false>;
    pub type RedAmberLight = Light<true, true, false>;
    pub type AmberLight = Light<false, true, false>;
    pub type GreenLight = Light<false, false, true>;

    // private constants for each state
    const REDV : (bool,bool,bool) = (true,false,false);
    const REDAMBERV : (bool,bool,bool) = (true,true,false);
    const AMBERV : (bool,bool,bool) = (false,true,false);
    const GREENV : (bool,bool,bool) = (false,false,true);


    impl<const RED: bool, const AMBER: bool, const GREEN: bool> Light<RED, AMBER, GREEN> {

        /// function that prints out the traffic light state
        pub fn print(&self) {
            match (RED, AMBER, GREEN) {
                REDV => println!("I am Red"),
                REDAMBERV => println!("I am RedAmber"),
                AMBERV => println!("I am Amber"),
                GREENV => println!("I am Green"),
                _ => unreachable!(),
            }
        }
    }

    /// public function that creates a red traffic light
    pub fn new() -> RedLight {
        RedLight{
            _private : ()
        }
    }

    impl RedLight {
        /// transition function turning a Red light to a Red and Amber light
        pub fn transition(self) -> RedAmberLight{
            RedAmberLight{
                _private : ()
            }
        }
    }

    impl RedAmberLight {
        /// transition function turning a Red and Amber light to a Green light
        pub fn transition(self) -> GreenLight{
            GreenLight{
                _private : ()
            }
        }
    }

    impl AmberLight {
        /// transition function turning an Amber light to an Red light
        pub fn transition(self) -> RedLight{
            RedLight{
                _private : ()
            }
        }
    }


    impl GreenLight {
        /// transition function turning a Green light to an Amber light
        pub fn transition(self) -> AmberLight{
            AmberLight{
                _private : ()
            }
        }
    }
}

use crate::traffic_light::*;

fn main() {
    let rlight : RedLight = new();
    rlight.print();
    let glight : GreenLight = rlight.transition().transition();
    glight.print();
    let alight : AmberLight = glight.transition();
    alight.print();
    let ralight : RedAmberLight = alight.transition().transition();
    ralight.print();
}