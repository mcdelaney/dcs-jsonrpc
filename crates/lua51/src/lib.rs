#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ptr;

use libc::{c_char, c_int};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub const LUA_OK: u32 = 0;

pub unsafe extern "C" fn lua_tostring(L: *mut lua_State, idx: c_int) -> *const c_char {
    lua_tolstring(L, idx, ptr::null_mut())
}

pub unsafe extern "C" fn lua_pop(L: *mut lua_State, n: c_int) {
    lua_settop(L, -(n) - 1);
}

pub unsafe extern "C" fn lua_isboolean(L: *mut lua_State, idx: c_int) -> bool {
    lua_type(L, idx) == LUA_TBOOLEAN as i32
}

pub unsafe extern "C" fn lua_isfunction(L: *mut lua_State, idx: c_int) -> bool {
    lua_type(L, idx) == LUA_TFUNCTION as i32
}

pub unsafe extern "C" fn lua_isnil(L: *mut lua_State, idx: c_int) -> bool {
    lua_type(L, idx) == LUA_TNIL as i32
}

pub unsafe extern "C" fn lua_isnone(L: *mut lua_State, idx: c_int) -> bool {
    lua_type(L, idx) == LUA_TNONE as i32
}

pub unsafe extern "C" fn lua_isnoneornil(L: *mut lua_State, idx: c_int) -> bool {
    lua_type(L, idx) <= 0
}

//pub unsafe extern "C" fn lua_isnumber(L: *mut lua_State, idx: c_int) -> bool {
//    lua_type(L, idx) == LUA_TBOOLEAN as i32
//}
//pub unsafe extern "C" fn lua_isstring(L: *mut lua_State, idx: c_int) -> bool {
//    lua_type(L, idx) == LUA_TBOOLEAN as i32
//}

pub unsafe extern "C" fn lua_istable(L: *mut lua_State, idx: c_int) -> bool {
    lua_type(L, idx) == LUA_TTABLE as i32
}

pub unsafe extern "C" fn lua_isthread(L: *mut lua_State, idx: c_int) -> bool {
    lua_type(L, idx) == LUA_TTHREAD as i32
}
