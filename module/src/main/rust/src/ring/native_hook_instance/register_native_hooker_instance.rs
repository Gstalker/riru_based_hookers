use jni::JNIEnv;
use super::{
    register_dexfile,
    dlopen,
    libgstalker_dlopen_test,
};

pub fn register(env: &mut JNIEnv, path: String) -> bool {
    let result = true;
    info!("Register hooks");
    register_dexfile::register(env,&path);
    dlopen::register(env,&path);
    libgstalker_dlopen_test::register(env,&path);
    info!("log hook finished!");
    result
}