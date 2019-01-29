#[cfg(feature = "fresh")]
extern crate bindgen;

#[cfg(feature = "fresh")]
use std::env;
#[cfg(feature = "fresh")]
use std::fs;
#[cfg(feature = "fresh")]
use std::path::{PathBuf};

#[cfg(all(
    not(feature = "fresh"),
    not(any(
        feature = "cuda_6_5",
        feature = "cuda_7_0",
        feature = "cuda_7_5",
        feature = "cuda_8_0",
        feature = "cuda_9_0",
        feature = "cuda_9_1",
        feature = "cuda_9_2",
        feature = "cuda_10_0",
    ))
))]
fn main() {
  compile_error!("a cuda version feature must be enabled");
}

#[cfg(all(
    not(feature = "fresh"),
    any(
        feature = "cuda_6_5",
        feature = "cuda_7_0",
        feature = "cuda_7_5",
        feature = "cuda_8_0",
        feature = "cuda_9_0",
        feature = "cuda_9_1",
        feature = "cuda_9_2",
        feature = "cuda_10_0",
    )
))]
fn main() {
  println!("cargo:rustc-link-lib=cuda");
}

#[cfg(feature = "fresh")]
fn main() {
  println!("cargo:rustc-link-lib=cuda");

  let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
  let cuda_dir = PathBuf::from(
      env::var("CUDA_HOME")
        .or_else(|_| env::var("CUDA_PATH"))
        .unwrap_or_else(|_| "/usr/local/cuda".to_owned())
  );
  let cuda_include_dir = cuda_dir.join("include");

  #[cfg(feature = "cuda_6_5")]
  let a_cuda_version_feature_must_be_enabled = "v6_5";
  #[cfg(feature = "cuda_7_0")]
  let a_cuda_version_feature_must_be_enabled = "v7_0";
  #[cfg(feature = "cuda_7_5")]
  let a_cuda_version_feature_must_be_enabled = "v7_5";
  #[cfg(feature = "cuda_8_0")]
  let a_cuda_version_feature_must_be_enabled = "v8_0";
  #[cfg(feature = "cuda_9_0")]
  let a_cuda_version_feature_must_be_enabled = "v9_0";
  #[cfg(feature = "cuda_9_1")]
  let a_cuda_version_feature_must_be_enabled = "v9_1";
  #[cfg(feature = "cuda_9_2")]
  let a_cuda_version_feature_must_be_enabled = "v9_2";
  #[cfg(feature = "cuda_10_0")]
  let a_cuda_version_feature_must_be_enabled = "v10_0";
  let v = a_cuda_version_feature_must_be_enabled;

  let gensrc_dir = manifest_dir.join("gensrc").join("ffi").join(v);
  println!("cargo:rerun-if-changed={}", gensrc_dir.display());
  fs::create_dir_all(&gensrc_dir).ok();

  println!("cargo:rerun-if-changed={}", gensrc_dir.join("_cuda").display());
  fs::remove_file(gensrc_dir.join("_cuda.rs")).ok();
  bindgen::Builder::default()
    .clang_arg(format!("-I{}", cuda_include_dir.as_os_str().to_str().unwrap()))
    .header("wrapped_cuda.h")
    .whitelist_recursively(false)
    .whitelist_function("cuGetErrorName")
    .whitelist_function("cuGetErrorString")
    .whitelist_function("cuInit")
    .whitelist_function("cuDriverGetVersion")
    .whitelist_function("cuDeviceGet")
    .whitelist_function("cuDeviceGetAttribute")
    .whitelist_function("cuDeviceGetCount")
    .whitelist_function("cuDeviceGetLuid")
    .whitelist_function("cuDeviceGetName")
    .whitelist_function("cuDeviceGetUuid")
    .whitelist_function("cuDeviceTotalMem")
    .whitelist_function("cuDevicePrimaryCtxGetState")
    .whitelist_function("cuDevicePrimaryCtxRelease")
    .whitelist_function("cuDevicePrimaryCtxReset")
    .whitelist_function("cuDevicePrimaryCtxRetain")
    .whitelist_function("cuDevicePrimaryCtxSetFlags")
    .whitelist_function("cuCtxCreate")
    .whitelist_function("cuCtxDestroy")
    .whitelist_function("cuCtxGetApiVersion")
    //.whitelist_function("cuCtxGetCacheConfig")
    .whitelist_function("cuCtxGetCurrent")
    .whitelist_function("cuCtxGetDevice")
    .whitelist_function("cuCtxGetFlags")
    //.whitelist_function("cuCtxGetLimit")
    //.whitelist_function("cuCtxGetSharedMemConfig")
    .whitelist_function("cuCtxGetStreamPriorityRange")
    .whitelist_function("cuCtxPopCurrent")
    .whitelist_function("cuCtxPushCurrent")
    //.whitelist_function("cuCtxSetCacheConfig")
    .whitelist_function("cuCtxSetCurrent")
    //.whitelist_function("cuCtxSetLimit")
    //.whitelist_function("cuCtxSetSharedMemConfig")
    .whitelist_function("cuCtxSynchronize")
    //.whitelist_function("cuLinkAddData")
    //.whitelist_function("cuLinkAddFile")
    //.whitelist_function("cuLinkComplete")
    //.whitelist_function("cuLinkCreate")
    //.whitelist_function("cuLinkDestroy")
    .whitelist_function("cuModuleGetFunction")
    .whitelist_function("cuModuleGetGlobal")
    //.whitelist_function("cuModuleGetSurfRef")
    //.whitelist_function("cuModuleGetTexRef")
    .whitelist_function("cuModuleLoad")
    .whitelist_function("cuModuleLoadData")
    .whitelist_function("cuModuleLoadDataEx")
    .whitelist_function("cuModuleLoadFatBinary")
    .whitelist_function("cuModuleUnload")
    .whitelist_function("cuMemsetD16")
    .whitelist_function("cuMemsetD16Async")
    .whitelist_function("cuMemsetD32")
    .whitelist_function("cuMemsetD32Async")
    .whitelist_function("cuMemsetD8")
    .whitelist_function("cuMemsetD8Async")
    //.whitelist_function("cuMemAdvise")
    .whitelist_function("cuMemPrefetchAsync")
    //.whitelist_function("cuMemRangeGetAttribute")
    //.whitelist_function("cuMemRangeGetAttributes")
    //.whitelist_function("cuPointerGetAttribute")
    //.whitelist_function("cuPointerGetAttributes")
    //.whitelist_function("cuPointerSetAttribute")
    .whitelist_function("cuStreamGetCtx")
    //.whitelist_function("cuStreamBatchMemOp")
    //.whitelist_function("cuStreamWaitValue32")
    //.whitelist_function("cuStreamWaitValue64")
    //.whitelist_function("cuStreamWriteValue32")
    //.whitelist_function("cuStreamWriteValue64")
    //.whitelist_function("cuFuncGetAttribute")
    //.whitelist_function("cuFuncSetAttribute")
    //.whitelist_function("cuFuncSetCacheConfig")
    //.whitelist_function("cuFuncSetSharedMemConfig")
    .whitelist_function("cuLaunchCooperativeKernel")
    .whitelist_function("cuLaunchCooperativeKernelMultiDevice")
    .whitelist_function("cuLaunchHostFunc")
    .whitelist_function("cuLaunchKernel")
    .generate_comments(false)
    .rustfmt_bindings(true)
    .generate()
    .expect("bindgen failed to generate driver bindings")
    .write_to_file(gensrc_dir.join("_cuda.rs"))
    .expect("bindgen failed to write driver bindings");
}
