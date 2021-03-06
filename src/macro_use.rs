#![macro_use]
use crate::value::*;

macro_rules! fail {
    ($expr:expr) => (
        return Err(::std::convert::From::from($expr));
    )
}

macro_rules! ensure {
    ($expr:expr, $err_result:expr) => (
        if !($expr) { fail!($err_result) }
    )
}

macro_rules! unwrap_or {
    ($expr:expr, $or:expr) => (
        match $expr {
            Some(x) => x,
            None => { $or }
        }
    )
}

macro_rules! try_read {
    ($expr:expr, $val:expr) => (
        {
            if $expr? != $val {
                fail!((ErrorKind::NoLeftSpaceError, "must left space to read "));
            }
        }
    )
}

macro_rules! decode_array {
    ($expr:expr, $path:path, $match_path:path) => (
        {
            let mut value : Vec<Value> = vec![];
            loop {
                let sub_value = $expr?;
                match sub_value {
                    Value::Null => {
                        break;
                    }
                    $match_path(_) => {}
                    _ => fail!((ErrorKind::TypeNotMatchError, "must match type")),
                }
                value.push(sub_value);
            }
            Ok($path(value))
        }
    )
}


macro_rules! check_vailed {
    ($value:ident, $expr:expr) => (
        {
            let t = get_value_type($value);
            ensure!($expr == t, (ErrorKind::TypeNotMatchError, "must match type"));
        }
    )
}