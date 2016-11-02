extern crate libc;

pub mod error;
mod redis;
pub mod throttle;

use libc::c_int;
use redis::raw::*;

const MODULE_NAME: &'static str = "redis-throttle";
const MODULE_VERSION: c_int = 1;

struct ThrottleCommand {
}

impl ThrottleCommand {
    fn name() -> &'static str {
        "throttle"
    }

    fn str_flags() -> &'static str {
        "readonly"
    }
}

impl redis::Command for ThrottleCommand {
    fn run(&self, r: redis::Redis, args: &[&str]) -> redis::CommandResult {

        println!("arguments = {:?}", args);
        Ok(true)
    }
}

#[allow(non_snake_case)]
#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn Throttle_RedisCommand(ctx: *mut RedisModuleCtx,
                                        argv: *mut *mut RedisModuleString,
                                        argc: c_int)
                                        -> Status {
    redis::harness_command(&ThrottleCommand {}, ctx, argv, argc)
}

#[allow(non_snake_case)]
#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn RedisModule_OnLoad(ctx: *mut RedisModuleCtx,
                                     argv: *mut *mut RedisModuleString,
                                     argc: c_int)
                                     -> Status {
    unsafe {
        if Export_RedisModule_Init(ctx,
                                   format!("{}\0", MODULE_NAME).as_ptr(),
                                   MODULE_VERSION,
                                   REDISMODULE_APIVER_1) == Status::Err {
            return Status::Err;
        }

        if RedisModule_CreateCommand(ctx,
                                     format!("{}\0", ThrottleCommand::name()).as_ptr(),
                                     Some(Throttle_RedisCommand),
                                     format!("{}\0", ThrottleCommand::str_flags()).as_ptr(),
                                     0,
                                     0,
                                     0) == Status::Err {
            return Status::Err;
        }

    }

    return Status::Ok;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
