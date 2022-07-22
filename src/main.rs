use mlua::{lua_State, Lua, Value};

extern "C" {
    fn luaopen_yue(state: *mut lua_State) -> std::os::raw::c_int;
}

fn main() {
    let lua = Lua::new();
    let yue = unsafe { lua.create_c_function(luaopen_yue).unwrap() };
    lua.load_from_function::<_, Value>("yue", yue).unwrap();
    lua.load(
        r#"
        local yue = require("yue")
        local codes, err, globals = yue.to_lua([[
f = ->
  print "hello world"
f!
        ]],{
          implicit_return_root = true,
          reserve_line_number = true,
          lint_global = true
        })
        if err then
          error(err)
        end
        load(codes)()
    "#,
    )
    .exec()
    .unwrap();
}
