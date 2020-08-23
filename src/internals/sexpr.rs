use num_enum::{TryFromPrimitive, TryFromPrimitiveError};
use std::convert::TryFrom;
use std::fmt;
use std::io::Write;
//see also https://www.brodieg.com/2019/02/18/an-unofficial-reference-for-internal-inspect/
// also consider altrep of NA and sorted
//The first five bits of the sxpinfo header specify one of up to 32 SEXPTYPEs.
#[derive(Debug, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum SexpType {
    Nil = 0,
    Sym = 1,
    List = 2,
    Clo = 3,
    Env = 4,
    Prom = 5,
    Lang = 6,
    Special = 7,
    Builtin = 8,
    Char = 9,
    Lgl = 10,
    Int = 13,
    Real = 14,
    Cplx = 15,
    Str = 16,
    Dot = 17,
    Any = 18,
    Vec = 19,
    Expr = 20,
    Bcode = 21,
    Extptr = 22,
    Weakref = 23,
    Raw = 24,
    S4 = 25,
    New = 30,  /* Fresh Node Creaed In New Page */
    Free = 31, /* Node Released By Gc */
    Fun = 99,  /* Closure Or Builtin */
}
//https://colinfay.me/r-internals/r-internal-structures.html
#[repr(C)]
pub struct SexpHeaderInfo(u64);

impl SexpHeaderInfo {
    pub fn r#type(&self) -> Result<SexpType, TryFromPrimitiveError<SexpType>> {
        SexpType::try_from((self.0 & 31) as u32)
    }
    pub fn scalar(&self) -> bool {
        (self.0 & 32) != 0
    }
    pub fn obj(&self) -> bool {
        (self.0 & 64) != 0
    }
    pub fn alt(&self) -> bool {
        (self.0 & 128) != 0
    }
    // https://cran.r-project.org/doc/manuals/r-release/R-ints.html#Rest-of-header
    pub fn gp(&self) -> u16 {
        (self.0 >> 16) as u16
    }
    pub fn mark(&self) -> bool {
        (self.0 & (1 << 23)) != 0
    }
    pub fn debug(&self) -> bool {
        (self.0 & (1 << 24)) != 0
    }
    pub fn trace(&self) -> bool {
        (self.0 & (1 << 25)) != 0
    }
    pub fn spare(&self) -> bool {
        (self.0 & (1 << 26)) != 0
    }
    pub fn gcgen(&self) -> bool {
        (self.0 & (1 << 27)) != 0
    }
    pub fn gccls(&self) -> u8 {
        //three bits
        ((self.0 >> 28) & 7) as u8
    }
}

impl fmt::Debug for SexpHeaderInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.r#type() {
            Ok(t) => f
                .debug_struct("Header_info")
                .field("type", &t)
                .field("scalar", &self.scalar())
                .field("obj", &self.obj())
                .field("alt", &self.alt())
                .field("gp", &self.gp())
                .field("mark", &self.mark())
                .field("debug", &self.debug())
                .field("trace", &self.trace())
                .field("spare", &self.spare())
                .field("gcgen", &self.gcgen())
                .field("gccls", &self.gccls())
                .finish(),
            Err(_) => f
                .debug_struct("Header_info")
                .field("error", &self.0)
                .finish(),
        }
    }
}

pub mod sxp {
    //NOTE: this type is followed by an array of data.
    // should R change the repreenation, look at the slice functions!
    // we calculate the adress of the slice by offesecting the self pointer by one.
    // this might not be correct if R ever changes its represenation and/or alignment rules though this is unlikely to happen.
    #[repr(C)]
    #[derive(Clone, Copy, Debug)]
    pub struct Vec {
        pub length: usize,
        pub true_length: usize,
    }

    impl Vec {
        pub unsafe fn get_slice<T>(&self) -> &[T] {
            let ptr_to_array = {
                let self_ptr: *const Self = self;
                //NOTE! id
                self_ptr.offset(1)
            };
            std::slice::from_raw_parts(ptr_to_array as *const T, self.length)
        }
        pub unsafe fn get_mut_slice<T>(&mut self) -> &mut [T] {
            let ptr_to_array = {
                let self_ptr: *mut Self = self;
                self_ptr.offset(1)
            };
            std::slice::from_raw_parts_mut(
                //is this in _any_ way sane?
                std::mem::transmute::<*mut Self, *mut T>(ptr_to_array),
                self.length,
            )
        }
    }

    #[repr(C)]
    #[derive(Clone, Copy, Debug)]
    pub struct Prim {
        offset: i32,
    }

    #[repr(C)]
    #[derive(Clone, Copy, Debug)]
    pub struct Sym {
        pub pname: *mut super::Sexpr,
        pub value: *mut super::Sexpr,
        pub internal: *mut super::Sexpr,
    }
    #[repr(C)]
    #[derive(Clone, Copy, Debug)]
    pub struct List {
        pub car: *mut super::Sexpr,
        pub cdr: *mut super::Sexpr,
        pub tag: *mut super::Sexpr,
    }
    #[repr(C)]
    #[derive(Clone, Copy, Debug)]
    pub struct Env {
        //enters infinite loop?
        pub frame: *mut super::Sexpr,
        pub enclos: *mut super::Sexpr,
        pub hashtab: *mut super::Sexpr,
    }
    #[repr(C)]
    #[derive(Clone, Copy, Debug)]
    pub struct Clos {
        pub formals: *mut super::Sexpr,
        pub body: *mut super::Sexpr,
        pub env: *mut super::Sexpr,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct Prom {
        pub value: *mut super::Sexpr,
        pub expr: *mut super::Sexpr,
        pub env: *mut super::Sexpr,
    }
    impl std::fmt::Debug for Prom {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if self.env.is_null() {
                f.debug_struct("Prom")
                    .field("===VALUE===", &self.value)
                    .field("expr", &self.expr)
                    .field("env", &self.env)
                    .finish()
            } else {
                f.debug_struct("Prom")
                    .field("value", &self.value)
                    .field("expr", &self.expr)
                    .field("env", &self.env)
                    .finish()
            }
        }
    }
    #[repr(C)]
    #[derive(Clone, Copy, Debug)]
    pub struct Unknown {
        _make_reprc_able: u8,
    }
}
#[repr(C)]
pub union Sxp {
    pub vec: sxp::Vec,
    pub prim: sxp::Prim,
    pub sym: sxp::Sym,
    pub list: sxp::List,
    pub env: sxp::Env,
    pub clos: sxp::Clos,
    pub prom: sxp::Prom,
    unknown: sxp::Unknown,
}

#[repr(C)]
pub struct Sexpr {
    pub sxpinfo: SexpHeaderInfo,
    attrib: *mut Sexpr,
    gengc_next_node: *mut Sexpr,
    gengc_prev_node: *mut Sexpr,
    pub sxp: Sxp,
    // _filing : [u64;3]
}

impl fmt::Debug for Sexpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.sxpinfo.r#type() {
            Ok(SexpType::Nil) => f.write_str("Nil"),
            Ok(sexp_type) => f
                .debug_struct("Sexpr")
                .field("sxpinfo", &self.sxpinfo)
                .field("data", {
                    if self.sxpinfo.alt() {
                        unsafe { &self.sxp.unknown }
                    } else {
                        match sexp_type {
                            SexpType::List | SexpType::Lang => unsafe { &self.sxp.list },
                            SexpType::Nil => &"Nil",
                            SexpType::Sym => unsafe { &self.sxp.sym },
                            SexpType::Clo => unsafe { &self.sxp.clos },
                            SexpType::Env => unsafe { &self.sxp.env },
                            SexpType::Prom => unsafe { &self.sxp.prom },
                            SexpType::Builtin | SexpType::Special => unsafe { &self.sxp.prim },
                            SexpType::Dot => &"Dot",
                            SexpType::Any => &"Any",
                            SexpType::Char => unsafe {
                                print!(
                                    "<{:?}>",
                                    String::from_utf8(self.sxp.vec.get_slice::<u8>().to_vec())
                                        .unwrap(),
                                );
                                &self.sxp.vec
                            },
                            SexpType::Str => unsafe {
                                print!("<{:?}>", self.sxp.vec.get_slice::<&Self>().to_vec());
                                &self.sxp.vec
                            },
                            SexpType::Int => unsafe {
                                print!("<{:?}>", self.sxp.vec.get_slice::<i32>().to_vec());
                                &self.sxp.vec
                            },
                            SexpType::Real => unsafe {
                                print!(
                                    "<{:?}>",
                                    self.sxp
                                        .vec
                                        .get_slice::<f64>()
                                        .iter()
                                        .map(|x| {
                                            format!("{:b}", std::mem::transmute::<&f64, &u64>(x))
                                        })
                                        .collect::<Vec<_>>()
                                );
                                &self.sxp.vec
                            },
                            SexpType::Lgl => unsafe {
                                print!("<{:?}>", self.sxp.vec.get_slice::<i32>().to_vec());
                                &self.sxp.vec
                            },
                            SexpType::Cplx => unsafe {
                                print!("<{:?}>", self.sxp.vec.get_slice::<(f64, f64)>().to_vec());
                                &self.sxp.vec
                            },
                            SexpType::Vec | SexpType::Expr | SexpType::Raw => unsafe {
                                &self.sxp.vec
                            },
                            SexpType::Bcode => &"Bcode",
                            SexpType::Extptr => &"Extptr",
                            SexpType::Weakref => &"Weakref",
                            SexpType::S4 => &"S4",
                            SexpType::New => &"New",
                            SexpType::Free => &"Free",
                            SexpType::Fun => &"Fun",
                        }
                    }
                })
                .field("attrib", &unsafe { self.attrib })
                // .field("next", &unsafe { self.gengc_next_node.as_ref() })
                .finish(),
            Err(_) => f
                .debug_struct("Sexpr")
                .field("sxpinfo", &self.sxpinfo)
                .field("attrib", &unsafe { self.attrib })
                // .field("next", &self.gengc_next_node)
                .finish(),
        }
    }
}

impl Sexpr {
    pub fn is_null(&self) -> bool {
        match self.sxpinfo.r#type() {
            Ok(SexpType::Nil) => true,
            _ => false,
        }
    }
}
