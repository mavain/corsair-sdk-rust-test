#![allow(clippy::missing_safety_doc)]
#![allow(non_snake_case)]

mod cuesdk;

use self::cuesdk::*;
use std::os::raw::*;
use std::ffi::CString;
use std::slice;
use glua_sys::*;

#[link(name = "CUESDK.x64_2017")]
extern "C" {}

#[no_mangle]
pub unsafe extern "C" fn gmod13_open(L: *mut lua_State) -> c_int {
    print(L, "Corsair CUESDK for Garry's Mod Loaded".to_string());
    
    CorsairPerformProtocolHandshake();

    lua_newtable!(L);

    glua_register_to_table(L, -2, "GetDeviceCount", corsair_get_device_count);
    glua_register_to_table(L, -2, "GetLastError", corsair_get_last_error);
    glua_register_to_table(L, -2, "GetLEDIDFromKeyName", corsair_get_led_id_for_key_name);
    glua_register_to_table(L, -2, "SetLEDColor", corsair_set_led_color_single);
    glua_register_to_table(L, -2, "SetLEDColors", corsair_set_led_color_multiple);
    glua_register_to_table(L, -2, "SetLightingControl", corsair_set_lighting_control);
    glua_register_to_table(L, -2, "GetLEDPositionsByDeviceIndex", corsair_get_led_positions_by_device_index);

    glua_setglobal(L, "corsair");

    0

}

#[no_mangle]
pub extern "C" fn gmod13_close(_L: *mut lua_State) -> c_int {
    0
}

extern "C" fn corsair_get_device_count(L: *mut lua_State) -> c_int {
    unsafe {
        lua_pushinteger(L, CorsairGetDeviceCount() as _);
    }
    1
}

extern "C" fn corsair_set_lighting_control(L: *mut lua_State) -> c_int {
    unsafe {
        CorsairRequestControl(CorsairAccessMode_CAM_ExclusiveLightingControl);
    }
    0
}

extern "C" fn corsair_set_led_color_single(L: *mut lua_State) -> c_int {
    unsafe {
        luaL_checktype(L, 1, LUA_TNUMBER as _);
        luaL_checktype(L, 2, LUA_TTABLE as _);
        
        let keyID = lua_tonumber(L, 1) as i32;

        glua_getfield(L, 2, "r");
        luaL_checktype(L, -1, LUA_TNUMBER as _);
        let red = lua_tonumber(L, -1) as i32;
        lua_pop!(L, 1);

        glua_getfield(L, 2, "g");
        luaL_checktype(L, -1, LUA_TNUMBER as _);
        let green = lua_tonumber(L, -1) as i32;
        lua_pop!(L, 1);

        glua_getfield(L, 2, "b");
        luaL_checktype(L, -1, LUA_TNUMBER as _);
        let blue = lua_tonumber(L, -1) as i32;
        lua_pop!(L, 1);

        let mut color = CorsairLedColor {
            ledId: keyID,
            r: red,
            g: green,
            b: blue
        };

        CorsairSetLedsColors(1, &mut color);
    }
    0
}

extern "C" fn corsair_set_led_color_multiple(L: *mut lua_State) -> c_int {
    unsafe {
        luaL_checktype(L, 1, LUA_TTABLE as _);
        
        let table_size = lua_objlen(L, 1) as usize;

        let mut color_vector = Vec::<CorsairLedColor>::with_capacity(table_size);

        lua_pushnil(L);
        while lua_next(L, 1) != 0 {

            /*
            -2 is the key
            -1 is the value
            */
            
            glua_getfield(L, -1, "id");
            luaL_checktype(L, -1, LUA_TNUMBER as _);
            let id = lua_tonumber(L, -1) as i32;
            lua_pop!(L, 1);

            glua_getfield(L, -1, "color");
            luaL_checktype(L, -1, LUA_TTABLE as _);

            glua_getfield(L, -1, "r");
            luaL_checktype(L, -1, LUA_TNUMBER as _);
            let red = lua_tonumber(L, -1) as i32;
            lua_pop!(L, 1);

            glua_getfield(L, -1, "g");
            luaL_checktype(L, -1, LUA_TNUMBER as _);
            let green = lua_tonumber(L, -1) as i32;
            lua_pop!(L, 1);

            glua_getfield(L, -1, "b");
            luaL_checktype(L, -1, LUA_TNUMBER as _);
            let blue = lua_tonumber(L, -1) as i32;
            lua_pop!(L, 1);

            lua_pop!(L, 1);

            color_vector.push(CorsairLedColor {
                ledId: id,
                r: red,
                g: green,
                b: blue
            });

            lua_pop!(L, 1);
        }

        CorsairSetLedsColors(color_vector.len() as i32, color_vector.as_mut_ptr());
    }
    0
}

extern "C" fn corsair_get_led_positions_by_device_index(L: *mut lua_State) -> c_int {
    unsafe {
        luaL_checktype(L, 1, LUA_TNUMBER as _);
        let device_index = lua_tonumber(L, 1) as i32;

        let led_positions_struct = *CorsairGetLedPositionsByDeviceIndex(device_index);
        let led_positions = slice::from_raw_parts(led_positions_struct.pLedPosition, led_positions_struct.numberOfLed as _);

        lua_newtable!(L);
        for led_index in 0 .. led_positions_struct.numberOfLed {

            let led_position: CorsairLedPosition = led_positions[led_index as usize];
            let led_id = led_position.ledId;
            let led_x = led_position.left;
            let led_y = led_position.top;
            let led_width = led_position.width;
            let led_height = led_position.height;

            lua_pushinteger(L, led_id as _);
            lua_newtable!(L);

            glua_push_string(L, "x");
            lua_pushnumber(L, led_x);
            lua_settable(L, -3);

            glua_push_string(L, "y");
            lua_pushnumber(L, led_y);
            lua_settable(L, -3);

            glua_push_string(L, "width");
            lua_pushnumber(L, led_width);
            lua_settable(L, -3);

            glua_push_string(L, "height");
            lua_pushnumber(L, led_height);
            lua_settable(L, -3);

            lua_settable(L, -3);
        }
    }
    1
}

extern "C" fn corsair_get_led_id_for_key_name(L: *mut lua_State) -> c_int {
    unsafe {
        luaL_checktype(L, 1, LUA_TSTRING as _);
        let lua_str = lua_tostring!(L, 1);
        let led_id = CorsairGetLedIdForKeyName(slice::from_raw_parts(lua_str, 1)[0]);
        lua_pushinteger(L, led_id as _);
    }
    1
}

extern "C" fn corsair_get_last_error(L: *mut lua_State) -> c_int {
    unsafe {
        /*
        switch (error) {
        case CE_Success:
            return "CE_Success";
        case CE_ServerNotFound:
            return "CE_ServerNotFound";
        case CE_NoControl:
            return "CE_NoControl";
        case CE_ProtocolHandshakeMissing:
            return "CE_ProtocolHandshakeMissing";
        case CE_IncompatibleProtocol:
            return "CE_IncompatibleProtocol";
        case CE_InvalidArguments:
            return "CE_InvalidArguments";
        default:
            return "unknown error";
        }
        */
        let error_str = match CorsairGetLastError() {
            CorsairError_CE_Success => { "CE_Success" },
            CorsairError_CE_ServerNotFound => { "CE_ServerNotFound" },
            CorsairError_CE_NoControl => { "CE_NoControl" }
            CorsairError_CE_ProtocolHandshakeMissing => { "CE_ProtocolHandshakeMissing" }
            CorsairError_CE_IncompatibleProtocol => { "CE_IncompatibleProtocol" }
            CorsairError_CE_InvalidArguments => { "CE_InvalidArguments" }
            _ => "Unknown Error"
        };
        match CString::new(error_str) {
            Ok(error_cstring) => {
                lua_pushstring(L, error_cstring.as_ptr());
            }
            Err(e) => {
                println!("Failed to create CString! {}", e);
            }
        }
    }
    1
}

fn glua_setglobal(L: *mut lua_State, lua_name: &str) {
    match CString::new(lua_name) {
        Ok(cstring_name) => {
            unsafe {
                lua_setglobal!(L, cstring_name.as_ptr());
            }
        }
        Err(e) => {
            println!("Failed to create CString! {}", e);
        }
    }
}

fn glua_register_to_table(L: *mut lua_State, table_index: i32, lua_name: &str, func: unsafe extern "C" fn(*mut lua_State) -> c_int) {
    match CString::new(lua_name) {
        Ok(cstring_name) => {
            unsafe {
                lua_pushcfunction!(L, Some(func));
                lua_setfield(L, table_index, cstring_name.as_ptr());
            }
        }
        Err(e) => {
            println!("Failed to create CString! {}", e);
        }
    }
}

fn glua_getfield(L: *mut lua_State, table_index: i32, lua_name: &str) {
    match CString::new(lua_name) {
        Ok(cstring_name) => {
            unsafe {
                lua_getfield(L, table_index, cstring_name.as_ptr());
            }
        }
        Err(e) => {
            println!("Failed to create CString! {}", e);
        }
    }
}

fn glua_push_string(L: *mut lua_State, lua_name: &str) {
    match CString::new(lua_name) {
        Ok(cstring_name) => {
            unsafe {
                lua_pushstring(L, cstring_name.as_ptr());
            }
        }
        Err(e) => {
            println!("Failed to create CString! {}", e);
        }
    }
}

fn print(L: *mut lua_State, s: String) {
    let length = s.len();
    let c_string = CString::new(s).unwrap();
    let c_print = CString::new("print").unwrap();

    unsafe {
        lua_getglobal!(L, c_print.as_ptr());
        lua_pushlstring(L, c_string.as_ptr(), length as _);
        lua_call(L, 1, 1)
    }
}