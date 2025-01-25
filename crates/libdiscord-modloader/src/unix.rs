use std::{
    ffi::{c_char, c_void},
    sync::LazyLock,
};

use retour::static_detour;

static MODLOADER_ASAR_PATH: LazyLock<String> =
    LazyLock::new(|| std::env::var("MODLOADER_ASAR_PATH").unwrap());

#[link(name = "dl")]
unsafe extern "C" {
    unsafe fn dlsym(handle: *const c_void, symbol: *const c_char) -> *const c_void;
}

unsafe extern "C" {
    #[link_name = "uv_fs_lstat"]
    unsafe fn original_uv_fs_lstat(
        loop_: *const c_void,
        req: *const c_void,
        path: *const c_char,
        buf: *mut c_void,
    ) -> i32;
}

type MainFn = extern "C" fn(i32, *const *const c_void, *const *const c_void) -> i32;

type LibcStartMainFn = fn(
    MainFn,
    i32,
    *const *const char,
    MainFn,
    extern "C" fn(),
    extern "C" fn(),
    *mut c_void,
) -> i32;

#[no_mangle]
unsafe extern "C" fn __libc_start_main(
    main: MainFn,
    argc: i32,
    argv: *const *const char,
    init: MainFn,
    fini: extern "C" fn(),
    rtld_fini: extern "C" fn(),
    stack_end: *mut c_void,
) -> i32 {
    #[allow(clippy::missing_transmute_annotations)]
    UvFsLstatDetour
        .initialize(
            std::mem::transmute::<UvFsLstat, _>(original_uv_fs_lstat),
            uv_fs_lstat,
        )
        .unwrap();

    UvFsLstatDetour.enable().unwrap();

    let orig_libc_start_main_addr: *const c_void = dlsym(
        libc::RTLD_NEXT,
        c"__libc_start_main".as_ptr() as *const c_char,
    );

    let orig_libc_start_main: LibcStartMainFn = std::mem::transmute(orig_libc_start_main_addr);

    orig_libc_start_main(main, argc, argv, init, fini, rtld_fini, stack_end)
}

type UvFsLstat = unsafe extern "C" fn(
    loop_: *const c_void,
    req: *const c_void,
    path: *const c_char,
    cb: *mut c_void,
) -> i32;

static_detour! {
    static UvFsLstatDetour: fn(*const c_void, *const c_void, *const c_char, *mut c_void) -> i32;
}

#[export_name = "uv_fs_lstat"]
unsafe extern "C" fn export_uv_vs_lstat(
    loop_: *const c_void,
    req: *const c_void,
    path: *const c_char,
    buf: *mut c_void,
) -> i32 {
    uv_fs_lstat(loop_, req, path, buf)
}

fn uv_fs_lstat(
    loop_: *const c_void,
    req: *const c_void,
    path: *const c_char,
    buf: *mut c_void,
) -> i32 {
    let path_str = unsafe { std::ffi::CStr::from_ptr(path).to_str().unwrap() };
    if path_str.contains("resources/_app.asar") {
        let redirect_to = path_str.replace("/_app.asar", "/app.asar");
        let redirect_to_c = std::ffi::CString::new(redirect_to.as_str()).unwrap();
        return UvFsLstatDetour.call(loop_, req, redirect_to_c.as_ptr(), buf);
    }

    UvFsLstatDetour.call(loop_, req, path, buf)
}

type XStat64 = unsafe extern "C" fn(i32, *const c_char, *mut libc::stat64) -> i64;

#[no_mangle]
unsafe extern "C" fn __xstat64(ver: i32, path: *const c_char, out: *mut libc::stat64) -> i64 {
    use std::sync::LazyLock;

    static ORIGINAL_XSTAT64: LazyLock<XStat64> = LazyLock::new(|| unsafe {
        std::mem::transmute(dlsym(libc::RTLD_NEXT, c"__xstat64".as_ptr()))
    });

    let path_str = std::ffi::CStr::from_ptr(path).to_str().unwrap();

    // If calling _app.asar, return the original app.asar
    if path_str.contains("resources/_app.asar") {
        let redirect_to = path_str.replace("/_app.asar", "/app.asar");
        let redirect_to_c = std::ffi::CString::new(redirect_to.as_str()).unwrap();
        return ORIGINAL_XSTAT64(ver, redirect_to_c.as_ptr(), out);
    }

    // If calling app.asar, return the custom app.asar
    if path_str.contains("resources/app.asar") {
        let asar_path_cstr = std::ffi::CString::new(MODLOADER_ASAR_PATH.as_str()).unwrap();
        return ORIGINAL_XSTAT64(ver, asar_path_cstr.as_ptr(), out);
    }

    ORIGINAL_XSTAT64(ver, path, out)
}

type Open64 = unsafe extern "C" fn(*const c_char, i32, i32) -> i32;

#[no_mangle]
unsafe extern "C" fn open64(path: *const c_char, flags: i32, mode: i32) -> i32 {
    use std::sync::LazyLock;

    static ORIGINAL_OPENAT64: LazyLock<Open64> = LazyLock::new(|| unsafe {
        std::mem::transmute(dlsym(libc::RTLD_NEXT, c"open64".as_ptr()))
    });

    let path_str = std::ffi::CStr::from_ptr(path).to_str().unwrap();

    // If calling _app.asar, return the original app.asar
    if path_str.contains("resources/_app.asar") {
        let redirect_to = path_str.replace("/_app.asar", "/app.asar");
        let redirect_to_c = std::ffi::CString::new(redirect_to.as_str()).unwrap();

        return ORIGINAL_OPENAT64(redirect_to_c.as_ptr(), flags, mode);
    }

    // If calling app.asar, return the custom app.asar
    if path_str.contains("resources/app.asar") {
        let redirect_to = std::ffi::CString::new(MODLOADER_ASAR_PATH.as_str()).unwrap();

        return ORIGINAL_OPENAT64(redirect_to.as_ptr(), flags, mode);
    }

    ORIGINAL_OPENAT64(path, flags, mode)
}
