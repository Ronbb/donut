use std::{cell::RefCell, collections::HashMap, sync::Arc};

use mlua::Lua;
use tokio::sync::RwLock;

use crate::{
    base::Next,
    cursor::Cursor,
    error::Error,
    state::{State, Variant},
};

pub struct Script {
    cursor: Arc<RwLock<Cursor>>,
    lua: Lua,
}

impl Script {
    pub fn new(cursor: Arc<RwLock<Cursor>>) -> Script {
        let script = Script {
            cursor,
            lua: Lua::new(),
        };

        script
    }

    pub fn execute(&self, script: &String) -> Result<State, Error> {
        let lua = &self.lua;
        let mut state = State::new();

        lua.scope(|scope| {
            lua.globals().set(
                "set_state",
                scope.create_function_mut(|_, (key, value): (String, Variant)| {
                    state.set(key, value);
                    Ok(())
                })?,
            )?;

            lua.load(script).exec()?;

            Ok(())
        })?;

        Ok(state)
    }

    pub async fn execute_for_next(&self, script: &String) -> Result<Next, Error> {
        let lua = &self.lua;
        let cursor = &self.cursor.read().await;
        let procedure = cursor.procedure().upgrade().unwrap();

        let next = RefCell::new(Next::Null);

        lua.scope(|scope| {
            let globals = lua.globals();

            globals.set(
                "set_continue",
                scope.create_function_mut(|_, ()| {
                    next.replace(Next::Continue);
                    Ok(())
                })?,
            )?;

            globals.set(
                "set_one",
                scope.create_function_mut(|_, name: String| {
                    next.replace(Next::One(
                        procedure
                            .find(&name)
                            .map_err(|_| mlua::Error::external("not found"))?,
                    ));
                    Ok(())
                })?,
            )?;

            globals.set(
                "set_complete",
                scope.create_function_mut(|_, ()| {
                    next.replace(Next::Complete);
                    Ok(())
                })?,
            )?;

            globals.set(
                "set_bubble",
                scope.create_function_mut(|_, ()| {
                    next.replace(Next::Bubble);
                    Ok(())
                })?,
            )?;

            lua.load(script).exec()?;

            Ok(())
        })?;

        Ok(next.into_inner())
    }
}

impl mlua::FromLua<'_> for Variant {
    fn from_lua(value: mlua::Value<'_>, lua: &'_ mlua::Lua) -> mlua::Result<Self> {
        match value {
            mlua::Value::Nil => Ok(Variant::Null),
            mlua::Value::String(s) => Ok(Variant::String(s.to_str()?.to_string())),
            mlua::Value::Integer(i) => Ok(Variant::Integer(i)),
            mlua::Value::Number(n) => Ok(Variant::Float(n)),
            mlua::Value::Boolean(b) => Ok(Variant::Boolean(b)),
            mlua::Value::Table(t) => {
                let mut array: Vec<Variant> = vec![];
                let mut object: HashMap<String, Variant> = HashMap::new();
                t.for_each::<mlua::Value, mlua::Value>(|key, value| {
                    if key.is_string() {
                        if let Some(key) = key.as_string() {
                            object
                                .insert(key.to_str()?.to_string(), Variant::from_lua(value, lua)?);
                        }
                        return Ok(());
                    }

                    if key.is_integer() {
                        array.push(Variant::from_lua(value, lua)?);
                        return Ok(());
                    }

                    Ok(())
                })?;

                if object.is_empty() {
                    Ok(Variant::Array(array))
                } else {
                    Ok(Variant::Object(object))
                }
            }
            mlua::Value::LightUserData(_) => Ok(Variant::Null),
            mlua::Value::Function(_) => Ok(Variant::Null),
            mlua::Value::Thread(_) => Ok(Variant::Null),
            mlua::Value::UserData(_) => Ok(Variant::Null),
            mlua::Value::Error(_) => Ok(Variant::Null),
        }
    }
}
