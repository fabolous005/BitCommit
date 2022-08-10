

use std::{
    net::TcpListener,
};


macro_rules! dispatch_to {
    ($val:expr => {$($func:ident),*}) => {
            match $val {
                $(
                    stringify!($func) => $func(),
                )*
                _ => {},
            }
    }
}


fn test_fn() {
    print!("Something")
}


fn test_fn2() {
    println!("Something else")
}

//fn dispatch(s: &str) {
//    // this is how you can use the macro
//    dispatch_to!(s => {test_fn, test_fn2});
//}


struct Router {
    uint: std::net::TcpListener
}

impl  Router {
    
    pub fn new() -> Self {
        let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

        Self {
            uint: listener
        }
    }

    pub fn route(route: String) {
        
    }

    pub fn call(function_string: &str) {
        let function = function_string;
        dispatch_to!( function_string => {test_fn, test_fn2} )
    }

}
