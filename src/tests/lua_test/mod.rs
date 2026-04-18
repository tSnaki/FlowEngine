use mlua::Lua;

pub fn lun() {
    // Create Lua
    let lua = Lua::new();

    // Get reference to global variables
    let globals = lua.globals();

    globals.set("string_var", "hello").unwrap();
    globals.set("int_var", 42).unwrap();

    assert_eq!(globals.get::<String>("string_var").unwrap(), "hello");
    assert_eq!(globals.get::<i64>("int_var").unwrap(), 42);

    // Loading lua code
    lua.load(
        r#"
        global = 'foo'..'bar'
        "#
    )
    .set_name("example code")
    .exec().unwrap();
    assert_eq!(globals.get::<String>("global").unwrap(), "foobar");

    assert_eq!(lua.load("1 + 1").eval::<i32>().unwrap(), 2);
    assert_eq!(lua.load("false == false").eval::<bool>().unwrap(), true);
    assert_eq!(lua.load("return 1 + 2").eval::<i32>().unwrap(), 3);
}