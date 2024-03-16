use std::env;
use std::path::Path;


pub fn get_ndk_path() -> String {
    // android-ndk-r26c/toolchains/llvm/prebuilt/linux-x86_64/
    let ndk: String = String::from(env::var("ANDROID_NDK_ROOT").expect("env var ANDROID_NDK_ROOT must be set."));

    let new_path = Path::new(&ndk).join("toolchains").join("llvm").join("prebuilt").join("linux-x86_64").join("bin").join("clang");
    if new_path.exists() {
        return String::from(new_path.to_str().unwrap());
    }
    let new_path = Path::new(&ndk).join("bin").join("clang");
    if new_path.exists() {
        return String::from(new_path.to_str().unwrap());
    }
    panic!("Provide a valid android ndk path in env variable ANDROID_NDK_ROOT. Could not find {}", new_path.to_str().unwrap());
}

pub fn main() {
    let ndk: String = get_ndk_path();
    let arch: String = env::var("ANDROID_ARCH").unwrap_or(String::from("aarch64"));
    let tgt_sdk_version: String = env::var("ANDROID_SDK_VERSION").unwrap_or(String::from("34"));
    println!("cargo:rustc-env=CC_aarch64-linux-android={ndk}/bin/{arch}-linux-android{tgt_sdk_version}-clang", ndk=ndk, arch=arch, tgt_sdk_version=tgt_sdk_version);

    println!("cargo:rustc-env=CXX_aarch64-linux-android={ndk}/bin/{arch}-linux-android{tgt_sdk_version}-clang++", ndk=ndk, arch=arch, tgt_sdk_version=tgt_sdk_version);

    
}
