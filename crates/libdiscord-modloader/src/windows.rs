use std::{ffi::CString, str::FromStr, sync::LazyLock};

use detours_sys::{
    DetourAttach, DetourCreateProcessWithDllW, DetourIsHelperProcess, DetourRestoreAfterWith,
    DetourTransactionAbort, DetourTransactionBegin, DetourTransactionCommit, DetourUpdateThread,
    PVOID,
};

use widestring::U16CString;
use winapi::{
    shared::{
        minwindef::{BOOL, DWORD, HINSTANCE, LPVOID},
        ntdef::{HANDLE, LPCWSTR, LPWSTR},
    },
    um::{
        fileapi::{CreateFileW, GetFileAttributesW},
        minwinbase::LPSECURITY_ATTRIBUTES,
        processthreadsapi::{
            CreateProcessW, GetCurrentThread, ResumeThread, LPPROCESS_INFORMATION, LPSTARTUPINFOW,
        },
        winnt::DLL_PROCESS_ATTACH,
        winuser::MessageBoxA,
    },
};

static MODLOADER_ASAR_PATH: LazyLock<String> =
    LazyLock::new(|| std::env::var("MODLOADER_ASAR_PATH").unwrap());

static MODLOADER_DLL_PATH: LazyLock<String> =
    LazyLock::new(|| std::env::var("MODLOADER_DLL_PATH").unwrap());

static mut ORIGINAL_GET_FILE_ATTRIBUTES_W: PVOID = GetFileAttributesW as _;
static mut ORIGINAL_CREATE_FILE_W: PVOID = CreateFileW as _;
static mut ORIGINAL_CREATE_PROCESS_W: PVOID = CreateProcessW as _;

macro_rules! error_hooking_msg {
    ($msg:expr) => {
        MessageBoxA(
            std::ptr::null_mut(),
            $msg.as_ptr() as *const i8,
            "Error Hooking".as_ptr() as *const i8,
            0,
        );
    };
}

#[no_mangle]
unsafe extern "stdcall" fn DllMain(
    _hinst_dll: HINSTANCE,
    fwd_reason: DWORD,
    _lpv_reserved: LPVOID,
) -> i32 {
    if DetourIsHelperProcess() == 1 {
        return 1;
    }

    if fwd_reason != DLL_PROCESS_ATTACH {
        return 1;
    }

    DetourRestoreAfterWith();

    DetourTransactionBegin();
    DetourUpdateThread(GetCurrentThread());

    let result = DetourAttach(
        &raw mut ORIGINAL_GET_FILE_ATTRIBUTES_W,
        get_file_attributes_w as _,
    );

    if result != 0 {
        error_hooking_msg!("Failed to hook GetFileAttributesW. Please report this issue.");
        DetourTransactionAbort();
        return 1;
    }

    let result = DetourAttach(&raw mut ORIGINAL_CREATE_FILE_W, create_file_w as _);

    if result != 0 {
        error_hooking_msg!("Failed to hook CreateFileW. Please report this issue.");
        DetourTransactionAbort();
        return 1;
    }

    let result = DetourAttach(&raw mut ORIGINAL_CREATE_PROCESS_W, create_process_w as _);

    if result != 0 {
        error_hooking_msg!("Failed to hook CreateProcessW. Please report this issue on GitHub.");
        DetourTransactionAbort();
        return 1;
    }

    DetourTransactionCommit();

    1
}

unsafe extern "C" fn get_file_attributes_w(lp_file_name: LPCWSTR) -> DWORD {
    let file_name = U16CString::from_ptr_str(lp_file_name).to_string().unwrap();

    let get_file_attributes_w: extern "C" fn(LPCWSTR) -> DWORD =
        std::mem::transmute(ORIGINAL_GET_FILE_ATTRIBUTES_W);

    if file_name.contains("resources\\_app.asar") {
        let redirect_to = file_name.replace("\\_app.asar", "\\app.asar");
        let redirect_to_c = std::ffi::CString::new(redirect_to.as_str()).unwrap();
        let redirect_to = U16CString::from_str(redirect_to_c.to_str().unwrap()).unwrap();

        return get_file_attributes_w(redirect_to.as_ptr());
    }

    if file_name.contains("resources\\app.asar") {
        let asar_path_cstr = std::ffi::CString::new(MODLOADER_ASAR_PATH.as_str()).unwrap();
        let asar_path = U16CString::from_str(asar_path_cstr.to_str().unwrap()).unwrap();

        return get_file_attributes_w(asar_path.as_ptr());
    }

    get_file_attributes_w(lp_file_name)
}

unsafe extern "C" fn create_file_w(
    lp_file_name: LPCWSTR,
    dw_desired_access: DWORD,
    dw_share_mode: DWORD,
    lp_security_attributes: LPSECURITY_ATTRIBUTES,
    dw_creation_disposition: DWORD,
    dw_flags_and_attributes: DWORD,
    h_template_file: HANDLE,
) -> HANDLE {
    let file_name = U16CString::from_ptr_str(lp_file_name).to_string().unwrap();

    let create_file_w: extern "C" fn(
        lp_file_name: LPCWSTR,
        dw_desired_access: DWORD,
        dw_share_mode: DWORD,
        lp_security_attributes: LPSECURITY_ATTRIBUTES,
        dw_creation_disposition: DWORD,
        dw_flags_and_attributes: DWORD,
        h_template_file: HANDLE,
    ) -> HANDLE = std::mem::transmute(ORIGINAL_CREATE_FILE_W);

    if file_name.contains("resources\\_app.asar") {
        let redirect_to = file_name.replace("\\_app.asar", "\\app.asar");
        let redirect_to_c = std::ffi::CString::new(redirect_to.as_str()).unwrap();
        let redirect_to = U16CString::from_str(redirect_to_c.to_str().unwrap()).unwrap();

        return create_file_w(
            redirect_to.as_ptr(),
            dw_desired_access,
            dw_share_mode,
            lp_security_attributes,
            dw_creation_disposition,
            dw_flags_and_attributes,
            h_template_file,
        );
    }

    if file_name.contains("resources\\app.asar") {
        let asar_path_cstr = std::ffi::CString::new(MODLOADER_ASAR_PATH.as_str()).unwrap();
        let asar_path = U16CString::from_str(asar_path_cstr.to_str().unwrap()).unwrap();

        return create_file_w(
            asar_path.as_ptr(),
            dw_desired_access,
            dw_share_mode,
            lp_security_attributes,
            dw_creation_disposition,
            dw_flags_and_attributes,
            h_template_file,
        );
    }

    create_file_w(
        lp_file_name,
        dw_desired_access,
        dw_share_mode,
        lp_security_attributes,
        dw_creation_disposition,
        dw_flags_and_attributes,
        h_template_file,
    )
}

type FnCreateProcessW = unsafe extern "C" fn(
    lp_application_name: LPCWSTR,
    lp_command_line: LPWSTR,
    lp_process_attributes: LPSECURITY_ATTRIBUTES,
    lp_thread_attributes: LPSECURITY_ATTRIBUTES,
    b_inherit_handles: BOOL,
    dw_creation_flags: DWORD,
    lp_environment: LPVOID,
    lp_current_directory: LPCWSTR,
    lp_startup_info: LPSTARTUPINFOW,
    lp_process_information: LPPROCESS_INFORMATION,
) -> BOOL;

unsafe extern "C" fn create_process_w(
    lp_application_name: LPCWSTR,
    lp_command_line: LPWSTR,
    lp_process_attributes: LPSECURITY_ATTRIBUTES,
    lp_thread_attributes: LPSECURITY_ATTRIBUTES,
    b_inherit_handles: BOOL,
    dw_creation_flags: DWORD,
    lp_environment: LPVOID,
    lp_current_directory: LPCWSTR,
    lp_startup_info: LPSTARTUPINFOW,
    lp_process_information: LPPROCESS_INFORMATION,
) -> BOOL {
    let create_process_w: FnCreateProcessW = std::mem::transmute(ORIGINAL_CREATE_PROCESS_W);

    let command_line = U16CString::from_ptr_str(lp_command_line)
        .to_string()
        .unwrap();

    if !command_line.contains("--type=renderer") {
        return create_process_w(
            lp_application_name,
            lp_command_line,
            lp_process_attributes,
            lp_thread_attributes,
            b_inherit_handles,
            dw_creation_flags,
            lp_environment,
            lp_current_directory,
            lp_startup_info,
            lp_process_information,
        );
    }

    let dll_path = CString::from_str(&MODLOADER_DLL_PATH).unwrap();

    #[allow(
        clippy::missing_transmute_annotations,
        reason = "Excessive boilerplate"
    )]
    let success = DetourCreateProcessWithDllW(
        lp_application_name,
        lp_command_line,
        lp_process_attributes as _,
        lp_thread_attributes as _,
        b_inherit_handles,
        dw_creation_flags,
        lp_environment as _,
        lp_current_directory,
        lp_startup_info as _,
        lp_process_information as _,
        dll_path.as_ptr(),
        Some(std::mem::transmute(ORIGINAL_CREATE_PROCESS_W)),
    );

    if success != 1 {
        eprintln!("[Discord Modloader] Failed to create process");
        return success;
    }

    ResumeThread((*lp_process_information).hThread as _);

    success
}
