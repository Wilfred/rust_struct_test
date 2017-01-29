#![allow(non_upper_case_globals)]

/// This module contains Rust definitions whose C equivalents live in
/// lisp.h.

extern crate libc;

use std::os::raw::c_char;
#[cfg(test)]
use std::cmp::max;
use std::mem;
use std::ops::Deref;
use std::fmt::{Debug, Formatter, Error};

/// Emacs values are represented as tagged pointers. A few bits are
/// used to represent the type, and the remaining bits are either used
/// to store the value directly (e.g. integers) or the address of a
/// more complex data type (e.g. a cons cell).
///
/// TODO: example representations
///
/// `EmacsInt` represents an integer big enough to hold our tagged
/// pointer representation.
///
/// In Emacs C, this is `EMACS_INT`.
///
/// `EmacsUint` represents the unsigned equivalent of `EmacsInt`.
/// In Emacs C, this is `EMACS_UINT`.
///
/// Their definition are determined in a way consistent with Emacs C.
/// Under casual systems, they're the type isize and usize respectively.

include!(concat!(env!("OUT_DIR"), "/definitions.rs"));
/// These are an example of the casual case.
#[cfg(dummy = "impossible")]
pub type EmacsInt = isize;
#[cfg(dummy = "impossible")]
pub type EmacsUint = usize;
#[cfg(dummy = "impossible")]
pub type EmacsDouble = f64;
#[cfg(dummy = "impossible")]
pub const EMACS_INT_MAX: EmacsInt = 0x7FFFFFFFFFFFFFFF_i64;
#[cfg(dummy = "impossible")]
pub const EMACS_INT_SIZE: EmacsInt = 8;
#[cfg(dummy = "impossible")]
pub const EMACS_FLOAT_SIZE: EmacsInt = 8;
#[cfg(dummy = "impossible")]
pub const GCTYPEBITS: EmacsInt = 3;
#[cfg(dummy = "impossible")]
pub const USE_LSB_TAG: bool = true;

#[repr(C)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct LispObject(EmacsInt);

pub fn wrong_type_argument(predicate: LispObject, value: LispObject) -> LispObject {
  panic!("wrong type argument");
}

pub const Qnil: LispObject = LispObject(0);

impl LispObject {
    #[inline]
    pub unsafe fn from_raw(i: EmacsInt) -> LispObject {
        LispObject(i)
    }

    #[inline]
    pub fn to_raw(self) -> EmacsInt {
        self.0
    }
}

// Number of bits in a Lisp_Object tag.
#[allow(dead_code)]
const VALBITS: EmacsInt = EMACS_INT_SIZE * 8 - GCTYPEBITS;

const INTTYPEBITS: EmacsInt = GCTYPEBITS - 1;

#[allow(dead_code)]
const FIXNUM_BITS: EmacsInt = VALBITS + 1;

const VAL_MAX: EmacsInt = EMACS_INT_MAX >> (GCTYPEBITS - 1);

const VALMASK: EmacsInt = [VAL_MAX, -(1 << GCTYPEBITS)][USE_LSB_TAG as usize];

const INTMASK: EmacsInt = (EMACS_INT_MAX >> (INTTYPEBITS - 1));

/// Bit pattern used in the least significant bits of a lisp object,
/// to denote its type.
#[repr(u8)]
#[derive(PartialEq, Eq)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub enum LispType {
    // Symbol.  XSYMBOL (object) points to a struct Lisp_Symbol.
    Lisp_Symbol = 0,

    // Miscellaneous.  XMISC (object) points to a union Lisp_Misc,
    // whose first member indicates the subtype.
    Lisp_Misc = 1,

    // Integer.  XINT (obj) is the integer value.
    Lisp_Int0 = 2,
    Lisp_Int1 = 3 + (USE_LSB_TAG as usize as u8) * 3, // 3 | 6

    // String.  XSTRING (object) points to a struct Lisp_String.
    // The length of the string, and its contents, are stored therein.
    Lisp_String = 4,

    // Vector of Lisp objects, or something resembling it.
    // XVECTOR (object) points to a struct Lisp_Vector, which contains
    // the size and contents.  The size field also contains the type
    // information, if it's not a real vector object.
    Lisp_Vectorlike = 5,

    // Cons.  XCONS (object) points to a struct Lisp_Cons.
    Lisp_Cons = 6 - (USE_LSB_TAG as usize as u8) * 3, // 6 | 3

    Lisp_Float = 7,
}

impl LispObject {
    #[allow(unused_unsafe)]
    pub fn get_type(self) -> LispType {
        let raw = self.to_raw() as EmacsUint;
        let res = (if USE_LSB_TAG {
            raw & (!VALMASK as EmacsUint)
        } else {
            raw >> VALBITS
        }) as u8;
        unsafe { mem::transmute(res) }
    }

    #[inline]
    pub fn get_untaggedptr(self) -> *mut libc::c_void {
        (self.to_raw() & VALMASK) as libc::intptr_t as *mut libc::c_void
    }
}

// Symbol support (LispType == Lisp_Symbol == 0)

impl LispObject {
    pub fn is_symbol(self) -> bool {
        self.get_type() == LispType::Lisp_Symbol
    }
}

// Misc support (LispType == Lisp_Misc == 1)

// This is the set of data types that share a common structure.
// The first member of the structure is a type code from this set.
// The enum values are arbitrary, but we'll use large numbers to make it
// more likely that we'll spot the error if a random word in memory is
// mistakenly interpreted as a Lisp_Misc.
#[repr(u16)]
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub enum LispMiscType {
    Free = 0x5eab,
    Marker,
    Overlay,
    SaveValue,
    Finalizer,
}

/// Flag bits in a character.  These also get used in termhooks.h.
/// Richard Stallman <rms@gnu.ai.mit.edu> thinks that MULE
/// (MUlti-Lingual Emacs) might need 22 bits for the character value
/// itself, so we probably shouldn't use any bits lower than 0x0400000.  */
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[repr(isize)]
pub enum CharBits {
    CHAR_ALT = 0x0400000,
    CHAR_SUPER = 0x0800000,
    CHAR_HYPER = 0x1000000,
    CHAR_SHIFT = 0x2000000,
    CHAR_CTL = 0x4000000,
    CHAR_META = 0x8000000,
    // TODO implement BitOr and other traits related to
    // bit operations.
    CHAR_MODIFIER_MASK = 0x0400000 | 0x0800000 | 0x1000000
        | 0x2000000 | 0x4000000 | 0x8000000,
    // Actually, the current Emacs uses 22 bits for the character value
    // itself.
    CHARACTERBITS = 22,
}

// Lisp_Misc is a union. Now we don't really care about its variants except the
// super type layout. LispMisc is an unsized type for this, and LispMiscAny is
// only the header and a padding, which is consistent with the c version.
// directly creating and moving or copying this struct is simply wrong!
// If needed, we can calculate all variants size and allocate properly.

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ExternalPtr<T>(*mut T);

impl<T> ExternalPtr<T> {
    pub fn new(p: *mut T) -> ExternalPtr<T> {
        ExternalPtr(p)
    }

    pub fn as_ptr(&self) -> *const T {
        self.0
    }
}

impl<T> Deref for ExternalPtr<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}

pub type LispMiscRef = ExternalPtr<LispMiscAny>;

// Supertype of all Misc types.
#[repr(C)]
pub struct LispMiscAny {
    pub ty: LispMiscType,
    // This is actually a GC marker bit plus 15 bits of padding, but
    // we don't care right now.
    padding: u16,
}

#[test]
fn test_lisp_misc_any_size() {
    // Should be 32 bits, which is 4 bytes.
    assert!(mem::size_of::<LispMiscAny>() == 4);
}

impl LispObject {
    pub fn is_misc(self) -> bool {
        self.get_type() == LispType::Lisp_Misc
    }

    pub unsafe fn to_misc_unchecked(self) -> LispMiscRef {
        LispMiscRef::new(mem::transmute(self.get_untaggedptr()))
    }

    pub unsafe fn get_misc_type_unchecked(self) -> LispMiscType {
        self.to_misc_unchecked().ty
    }
}

// Fixnum(Integer) support (LispType == Lisp_Int0 | Lisp_Int1 == 2 | 6(LSB) )

/// Fixnums are inline integers that fit directly into Lisp's tagged word.
/// There's two LispType variants to provide an extra bit.

// Largest and smallest numbers that can be represented as fixnums in
// Emacs lisp.
pub const MOST_POSITIVE_FIXNUM: EmacsInt = EMACS_INT_MAX >> INTTYPEBITS;
#[allow(dead_code)]
pub const MOST_NEGATIVE_FIXNUM: EmacsInt = (-1 - MOST_POSITIVE_FIXNUM);

/// Natnums(natural number) are the non-negative fixnums.
/// There were special branches in the original code for better performance.
/// However they are unified into the fixnum logic under LSB mode.
/// TODO: Recheck these logic in original C code.

impl LispObject {
    #[inline]
    pub unsafe fn from_fixnum_unchecked(n: EmacsInt) -> LispObject {
        let o = if USE_LSB_TAG {
            (n << INTTYPEBITS) as EmacsUint + LispType::Lisp_Int0 as EmacsUint
        } else {
            (n & INTMASK) as EmacsUint + ((LispType::Lisp_Int0 as EmacsUint) << VALBITS)
        };
        LispObject::from_raw(o as EmacsInt)
    }

    #[inline]
    pub fn to_fixnum(self) -> Option<EmacsInt> {
        if self.is_fixnum() {
            let raw = self.to_raw();
            if !USE_LSB_TAG {
                Some(raw & INTMASK)
            } else {
                Some(raw >> INTTYPEBITS)
            }
        } else {
            None
        }
    }

    #[inline]
    pub fn is_fixnum(self) -> bool {
        let ty = self.get_type();
        (ty as u8 & ((LispType::Lisp_Int0 as u8) | !(LispType::Lisp_Int1 as u8))) ==
        LispType::Lisp_Int0 as u8
    }

    /// TODO: Bignum support? (Current Emacs doesn't have it)
    #[inline]
    pub fn is_integer(self) -> bool {
        self.is_fixnum()
    }
}


// Float support (LispType == Lisp_Float == 7 )

/// Represents a floating point value in elisp, or GC bookkeeping for
/// floats.
///
/// # Porting from C
///
/// `Lisp_Float` in C uses a union between a `double` and a
/// pointer. We assume a double, as that's the common case, and
/// require callers to transmute to a `LispFloatChain` if they need
/// the pointer.
#[repr(C)]
pub struct LispFloat {
    data: [u8; EMACS_FLOAT_SIZE as usize],
}

#[repr(C)]
pub struct LispFloatChainRepr(*const LispFloat);

impl LispFloat {
    pub fn as_data(&self) -> &EmacsDouble {
        unsafe { &*(self.data.as_ptr() as *const EmacsDouble) }
    }
    pub fn as_chain(&self) -> &LispFloatChainRepr {
        unsafe { &*(self.data.as_ptr() as *const LispFloatChainRepr) }
    }
}

#[test]
fn test_lisp_float_size() {
    let double_size = mem::size_of::<EmacsDouble>();
    let ptr_size = mem::size_of::<*const LispFloat>();

    assert!(mem::size_of::<LispFloat>() == max(double_size, ptr_size));
}

pub type LispFloatRef = ExternalPtr<LispFloat>;

impl LispObject {
    #[inline]
    pub fn is_float(self) -> bool {
        self.get_type() == LispType::Lisp_Float
    }

    #[inline]
    pub unsafe fn to_float_unchecked(self) -> LispFloatRef {
        LispFloatRef::new(mem::transmute(self.get_untaggedptr()))
    }

    pub unsafe fn get_float_data_unchecked(self) -> EmacsDouble {
        *self.to_float_unchecked().as_data()
    }

    pub fn to_float(self) -> Option<EmacsDouble> {
        if self.is_float() {
            Some(unsafe { self.get_float_data_unchecked() })
        } else {
            None
        }
    }

    /// If the LispObject is a number (of any kind), get a floating point value for it
    pub fn extract_float(self) -> Option<EmacsDouble> {
        let d = self.to_float();
        d.or(self.to_fixnum().map(|i| i as EmacsDouble))
    }
}

// String support (LispType == 4)

/// Represents a string value in elisp
#[repr(C)]
pub struct LispString {
    pub size: libc::ptrdiff_t,
    pub size_byte: libc::ptrdiff_t,
    pub intervals: *mut libc::c_void, // @TODO implement
    pub data: *mut libc::c_char,
}

impl LispObject {
    #[inline]
    pub fn is_string(self) -> bool {
        self.get_type() == LispType::Lisp_String
    }
}

// Other functions

impl LispObject {
    #[inline]
    pub fn is_number(self) -> bool {
        self.is_integer() || self.is_float()
    }
}




const PSEUDOVECTOR_SIZE_BITS: libc::c_int = 12;
#[allow(dead_code)]
const PSEUDOVECTOR_SIZE_MASK: libc::c_int = (1 << PSEUDOVECTOR_SIZE_BITS) - 1;
const PSEUDOVECTOR_REST_BITS: libc::c_int = 12;
#[allow(dead_code)]
const PSEUDOVECTOR_REST_MASK: libc::c_int = (((1 << PSEUDOVECTOR_REST_BITS) - 1) <<
                                             PSEUDOVECTOR_SIZE_BITS);
pub const PSEUDOVECTOR_AREA_BITS: libc::c_int = PSEUDOVECTOR_SIZE_BITS + PSEUDOVECTOR_REST_BITS;
#[allow(dead_code)]
const PVEC_TYPE_MASK: libc::c_int = 0x3f << PSEUDOVECTOR_AREA_BITS;

#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub enum PvecType {
    // TODO: confirm these are the right numbers.
    PVEC_NORMAL_VECTOR = 0,
    PVEC_FREE = 1,
    PVEC_PROCESS = 2,
    PVEC_FRAME = 3,
    PVEC_WINDOW = 4,
    PVEC_BOOL_VECTOR = 5,
    PVEC_BUFFER = 6,
    PVEC_HASH_TABLE = 7,
    PVEC_TERMINAL = 8,
    PVEC_WINDOW_CONFIGURATION = 9,
    PVEC_SUBR = 10,
    PVEC_OTHER = 11,
    PVEC_XWIDGET = 12,
    PVEC_XWIDGET_VIEW = 13,

    PVEC_COMPILED = 14,
    PVEC_CHAR_TABLE = 15,
    PVEC_SUB_CHAR_TABLE = 16,
    PVEC_FONT = 17,
}

#[repr(C)]
pub struct VectorLikeHeader {
    pub size: libc::ptrdiff_t,
}

/// Represents an elisp function.
#[repr(C)]
pub struct LispSubr {
    pub header: VectorLikeHeader,
    /// The C or Rust function that we will call when the user invokes
    /// the elisp function.
    pub function: *const libc::c_void,
    /// The minimum number of arguments to the elisp function.
    pub min_args: libc::c_short,
    /// The maximum number of arguments to the elisp function.
    pub max_args: libc::c_short,
    /// The name of the function in elisp.
    pub symbol_name: *const c_char,
    /// The interactive specification. This may be a normal prompt
    /// string, such as `"bBuffer: "` or an elisp form as a string.
    /// If the function is not interactive, this should be a null
    /// pointer.
    pub intspec: *const c_char,
    /// The docstring of our function.
    pub doc: *const c_char,
}

/// Used to denote functions that have no limit on the maximum number
/// of arguments.
pub const MANY: i16 = -2;

impl Debug for LispObject {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let ty = self.get_type();
        let self_ptr = &self as *const _ as usize;
        if ty as u8 >= 8 {
            write!(f,
                   "#<INVALID-OBJECT @ {:#X}: VAL({:#X})>",
                   self_ptr,
                   self.to_raw())
                ?;
            return Ok(());
        }
        match ty {
            LispType::Lisp_Symbol => {
                write!(f, "#<SYMBOL @ {:#X}: VAL({:#X})>", self_ptr, self.to_raw())?;
            }
            LispType::Lisp_Cons => {
                write!(f, "#<CONS @ {:#X}: VAL({:#X})>", self_ptr, self.to_raw())?;
            }
            LispType::Lisp_Float => {
                write!(f, "#<FLOAT @ {:#X}: VAL({:#X})>", self_ptr, self.to_raw())?;
            }
            LispType::Lisp_Vectorlike => {
                write!(f,
                       "#<VECTOR-LIKE @ {:#X}: VAL({:#X})>",
                       self_ptr,
                       self.to_raw())
                    ?;
            }
            LispType::Lisp_Int0 |
            LispType::Lisp_Int1 => {
                write!(f, "#<INT @ {:#X}: VAL({:#X})>", self_ptr, self.to_raw())?;
            }
            LispType::Lisp_Misc => {
                write!(f, "#<MISC @ {:#X}: VAL({:#X})>", self_ptr, self.to_raw())?;
            }
            LispType::Lisp_String => {
                write!(f, "#<STRING @ {:#X}: VAL({:#X})>", self_ptr, self.to_raw())?;
            }
        }
        Ok(())
    }
}

/// # Porting Notes
///
/// This module contains some functions that is originally contained in Emacs C code
/// as macros and global functions, which does not conforms to Rust naming rules well
/// and lacks unsafe marks. However we'll keep them during the porting process to make
/// the porting easy, we should be able to remove once the relevant functionality is Rust-only.
mod deprecated {
    use super::*;
    use ::libc;
    use ::std;

    /// Return the type of a LispObject.
    #[allow(non_snake_case)]
    pub fn XTYPE(a: LispObject) -> LispType {
        a.get_type()
    }

    /// Convert a tagged pointer to a normal C pointer.
    ///
    /// See the docstring for `LispType` for more information on tagging.
    #[allow(non_snake_case)]
    pub fn XUNTAG(a: LispObject, _: LispType) -> *const libc::c_void {
        a.get_untaggedptr()
    }

}

pub use self::deprecated::*;
