use std::convert;
use std::ops;
use std::fmt;
use std::cmp;


/// Convenience type, acts as an int, but can be converted to a bool.
///
/// Because the Todoist API specifies some fields as integers,
/// but their value is always boolean this type will serialize and deserialize as an `isize`, 
/// while converting to and from both an isize and bool.
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct IntBool(isize);


impl convert::From<bool> for IntBool {
    fn from(v : bool) -> IntBool {
        IntBool(match v {
            true => 1,
            false => 0,
        })
    }
}

impl convert::From<isize> for IntBool {
    fn from(v : isize) -> IntBool {
        IntBool(v)
    }
}

impl convert::Into<bool> for IntBool {
    fn into(self) -> bool {
        match self {
            IntBool(0) => false,
            IntBool(_) => true,
        }
    }
}


impl convert::Into<isize> for IntBool {
    fn into(self) -> isize {
        self.0
    }
}

impl fmt::Debug for IntBool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
            if self.0 == 0 {
                "false"
            } else {
                "true"
            }
        )
    }
}

impl cmp::PartialEq<bool> for IntBool {
    fn eq(&self, v : &bool) -> bool {
        *v as isize == self.0
    }
}


impl cmp::PartialEq<IntBool> for IntBool {
    fn eq(&self, v : &IntBool) -> bool {
        v.0 == self.0
    }
}

impl cmp::PartialEq<isize> for IntBool {
    fn eq(&self, v : &isize) -> bool {
        *v == self.0
    }
}

impl ops::Not for IntBool {
    type Output = bool;

    fn not(self) -> bool {
        if self.0 == 0 {
            true
        } else {
            false
        }
    }
}
