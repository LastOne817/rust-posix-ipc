pub mod signals {
    extern crate errno;
    extern crate libc;
    use std::mem;
    use self::errno::errno;

    #[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
    pub enum Signal {
        None = 0,
        Hup,
        Int,
        Quit,
        Ill,
        Trap,
        Abrt,
        Bus,
        Fpe,
        Kill,
        Usr1,
        Segv,
        Usr2,
        Pipe,
        Alrm,
        Term,
        StkFlt,
        Chld,
        Cont,
        Stop,
        Tstp,
        Ttin,
        Ttou,
        Urg,
        XCpu,
        Xfsz,
        Vtalrm,
        Prof,
        Winch,
        Io,
        Pwr,
        Sys
    }

    impl Signal {
        pub fn raise(self) -> Result<(), usize> {
            match unsafe { raise(self as libc::c_int) } {
                0 => Result::Ok(()),
                _ => Result::Err(errno().0 as usize)
            }
        }

        pub fn kill(self, pid: libc::pid_t) -> Result<(), usize> {
            match unsafe { kill(pid, self as libc::c_int) } {
                0 => Result::Ok(()),
                _ => Result::Err(errno().0 as usize)
            }
        }

        pub unsafe fn handle(self, handler: Box<FnMut(Signal)>) -> Result<(), usize> {
            match { signal (self as libc::c_int, mem::transmute(glue::rust_signal_handler)) } {
                -1 => Result::Err(errno().0 as usize),
                _ => { glue::set_handler(self, handler); Result::Ok(()) }
            }
        }

        pub fn from_i32(n: i32) -> Option<Signal> {
            match n {
                0 => Some(Signal::None),
                1 => Some(Signal::Hup),
                2 => Some(Signal::Int),
                3 => Some(Signal::Quit),
                4 => Some(Signal::Ill),
                5 => Some(Signal::Trap),
                6 => Some(Signal::Abrt),
                7 => Some(Signal::Bus),
                8 => Some(Signal::Fpe),
                9 => Some(Signal::Kill),
                10 => Some(Signal::Usr1),
                11 => Some(Signal::Segv),
                12 => Some(Signal::Usr2),
                13 => Some(Signal::Pipe),
                14 => Some(Signal::Alrm),
                15 => Some(Signal::Term),
                16 => Some(Signal::StkFlt),
                17 => Some(Signal::Chld),
                18 => Some(Signal::Cont),
                19 => Some(Signal::Stop),
                20 => Some(Signal::Tstp),
                21 => Some(Signal::Ttin),
                22 => Some(Signal::Ttou),
                23 => Some(Signal::Urg),
                24 => Some(Signal::XCpu),
                25 => Some(Signal::Xfsz),
                26 => Some(Signal::Vtalrm),
                27 => Some(Signal::Prof),
                28 => Some(Signal::Winch),
                29 => Some(Signal::Io),
                30 => Some(Signal::Pwr),
                31 => Some(Signal::Sys),
                _ => None
            }
        }
    }

    mod glue {
        extern crate libc;
        use super::Signal;
        use std::mem;

        #[derive(Clone, Copy, Debug)]
        struct FnPtr {
            foo: usize,
            bar: usize
        }

        static mut handlers: [FnPtr; 18] = [
            FnPtr {foo: 0, bar: 0},
            FnPtr {foo: 0, bar: 0},
            FnPtr {foo: 0, bar: 0},
            FnPtr {foo: 0, bar: 0},
            FnPtr {foo: 0, bar: 0},
            FnPtr {foo: 0, bar: 0},
            FnPtr {foo: 0, bar: 0},
            FnPtr {foo: 0, bar: 0},
            FnPtr {foo: 0, bar: 0},
            FnPtr {foo: 0, bar: 0},
            FnPtr {foo: 0, bar: 0},
            FnPtr {foo: 0, bar: 0},
            FnPtr {foo: 0, bar: 0},
            FnPtr {foo: 0, bar: 0},
            FnPtr {foo: 0, bar: 0},
            FnPtr {foo: 0, bar: 0},
            FnPtr {foo: 0, bar: 0},
            FnPtr {foo: 0, bar: 0},
        ];

        pub unsafe fn set_handler (sig: Signal, f: Box<FnMut(Signal)>) {
            handlers[sig as usize] = mem::transmute(f);
        }

        fn null_handler(s: Signal) {}

        pub unsafe extern "C" fn rust_signal_handler(sig: libc::c_int) {
            let f: *mut FnMut(Signal) = mem::transmute(handlers[sig as usize]);
            let p: FnPtr = mem::transmute(f);
            if p.foo != 0 && p.bar != 0 {
                match Signal::from_i32(sig) {
                    Some(s) => (*f)(s),
                    None => panic!("Unknown signal: {}", sig)
                }
            }
        }
    }

    extern "C" {
        fn raise(sig: libc::c_int) -> libc::c_int;
        fn signal(sig: libc::c_int, handler: *const libc::c_void) -> libc::c_int;
        fn kill(pid: libc::pid_t, sig: libc::c_int) -> libc::c_int;
    }
}
