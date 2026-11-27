#![windows_subsystem = "windows"]
#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code,
    static_mut_refs
)]

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::ptr::{null, null_mut};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

// ============================================================================
// 0. REAL-TIME DIAGNOSTIC LOGGER (touchpad_log.txt)
// ============================================================================
static mut LOG_TO_FILE: bool = false;

pub fn log_line(msg: &str) {
    let ts_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() % 1_000_000)
        .unwrap_or(0);
    let formatted = format!("[{:06}ms] {}\n", ts_ms, msg);
    let _ = std::io::stdout().write_all(formatted.as_bytes());
    if unsafe { LOG_TO_FILE } {
        if let Ok(mut f) = OpenOptions::new()
            .create(true)
            .append(true)
            .open("touchpad_log.txt")
        {
            let _ = f.write_all(formatted.as_bytes());
            let _ = f.flush();
        }
    }
}

// ============================================================================
// 1. WIN32 & HID FFI TYPES AND CONSTANTS
// ============================================================================
type HWND = *mut std::ffi::c_void;
type HANDLE = *mut std::ffi::c_void;
type HMENU = *mut std::ffi::c_void;
type HICON = *mut std::ffi::c_void;
type HRAWINPUT = *mut std::ffi::c_void;
type LRESULT = isize;
type WPARAM = usize;
type LPARAM = isize;
type UINT = u32;
type BOOL = i32;

const WM_DESTROY: UINT = 0x0002;
const WM_PAINT: UINT = 0x000F;
const WM_CLOSE: UINT = 0x0010;
const WM_INPUT: UINT = 0x00FF;
const WM_KEYDOWN: UINT = 0x0100;
const WM_COMMAND: UINT = 0x0111;
const WM_TIMER: UINT = 0x0113;
const WM_MOUSEMOVE: UINT = 0x0200;
const WM_LBUTTONDOWN: UINT = 0x0201;
const WM_LBUTTONUP: UINT = 0x0202;
const WM_LBUTTONDBLCLK: UINT = 0x0203;
const WM_RBUTTONDOWN: UINT = 0x0204;
const WM_RBUTTONUP: UINT = 0x0205;
const WM_MBUTTONDOWN: UINT = 0x0207;
const WM_MOUSEWHEEL: UINT = 0x020A;
const WM_MOUSEHWHEEL: UINT = 0x020E;
const WM_HOTKEY: UINT = 0x0312;
const WM_TRAYICON: UINT = 0x8001;

const RID_INPUT: UINT = 0x10000003;
const RIDI_PREPARSEDDATA: UINT = 0x20000005;
const RIDEV_INPUTSINK: u32 = 0x00000100;
const RIDEV_DEVNOTIFY: u32 = 0x00002000;
const RIM_TYPEMOUSE: u32 = 0;
const RIM_TYPEHID: u32 = 2;

const WS_POPUP: u32 = 0x80000000;
const WS_EX_TOPMOST: u32 = 0x00000008;
const WS_EX_TOOLWINDOW: u32 = 0x00000080;
const SW_SHOW: i32 = 5;
const SW_HIDE: i32 = 0;
const TRANSPARENT: i32 = 1;
const DT_LEFT: UINT = 0x00000000;
const DT_CENTER: UINT = 0x00000001;
const DT_VCENTER: UINT = 0x00000004;
const DT_SINGLELINE: UINT = 0x00000020;
const PS_SOLID: i32 = 0;

const MOD_CONTROL: UINT = 0x0002;
const VK_SPACE: UINT = 0x20;
const VK_ESCAPE: UINT = 0x1B;
const VK_T: UINT = 0x54;
const WH_MOUSE_LL: i32 = 14;

const NIM_ADD: u32 = 0x00000000;
const NIM_DELETE: u32 = 0x00000002;
const NIF_MESSAGE: u32 = 0x00000001;
const NIF_ICON: u32 = 0x00000002;
const NIF_TIP: u32 = 0x00000004;
const NIF_INFO: u32 = 0x00000010;
const NIIF_INFO: u32 = 0x00000001;
const ERROR_ALREADY_EXISTS: u32 = 183;
const MF_STRING: UINT = 0x00000000;
const MF_SEPARATOR: UINT = 0x00000800;
const TPM_BOTTOMALIGN: UINT = 0x0020;
const TPM_LEFTALIGN: UINT = 0x0000;
const TPM_RIGHTBUTTON: UINT = 0x0002;

const HIDP_STATUS_SUCCESS: i32 = 0x00110000;
const HidP_Input: i32 = 0;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct POINT {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct SIZE {
    pub cx: i32,
    pub cy: i32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MSLLHOOKSTRUCT {
    pub pt: POINT,
    pub mouseData: u32,
    pub flags: u32,
    pub time: u32,
    pub dwExtraInfo: usize,
}

#[repr(C)]
pub struct PAINTSTRUCT {
    pub hdc: HANDLE,
    pub fErase: BOOL,
    pub rcPaint: RECT,
    pub fRestore: BOOL,
    pub fIncUpdate: BOOL,
    pub rgbReserved: [u8; 32],
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct RAWINPUTDEVICE {
    pub usUsagePage: u16,
    pub usUsage: u16,
    pub dwFlags: u32,
    pub hwndTarget: HWND,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct RAWINPUTHEADER {
    pub dwType: u32,
    pub dwSize: u32,
    pub hDevice: HANDLE,
    pub wParam: WPARAM,
}

#[repr(C)]
pub struct WNDCLASSEXW {
    pub cbSize: UINT,
    pub style: UINT,
    pub lpfnWndProc: unsafe extern "system" fn(HWND, UINT, WPARAM, LPARAM) -> LRESULT,
    pub cbClsExtra: i32,
    pub cbWndExtra: i32,
    pub hInstance: HANDLE,
    pub hIcon: HANDLE,
    pub hCursor: HANDLE,
    pub hbrBackground: HANDLE,
    pub lpszMenuName: *const u16,
    pub lpszClassName: *const u16,
    pub hIconSm: HANDLE,
}

#[repr(C)]
pub struct MSG {
    pub hwnd: HWND,
    pub message: UINT,
    pub wParam: WPARAM,
    pub lParam: LPARAM,
    pub time: u32,
    pub pt_x: i32,
    pub pt_y: i32,
    pub lPrivate: u32,
}

#[repr(C)]
pub struct NOTIFYICONDATAW {
    pub cbSize: u32,
    pub hWnd: HWND,
    pub uID: u32,
    pub uFlags: u32,
    pub uCallbackMessage: UINT,
    pub hIcon: HICON,
    pub szTip: [u16; 128],
    pub dwState: u32,
    pub dwStateMask: u32,
    pub szInfo: [u16; 256],
    pub uTimeoutOrVersion: u32,
    pub szInfoTitle: [u16; 64],
    pub dwInfoFlags: u32,
    pub guidItem: [u8; 16],
    pub hBalloonIcon: HICON,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct HIDP_CAPS {
    pub Usage: u16,
    pub UsagePage: u16,
    pub InputReportByteLength: u16,
    pub OutputReportByteLength: u16,
    pub FeatureReportByteLength: u16,
    pub Reserved: [u16; 17],
    pub NumberLinkCollectionNodes: u16,
    pub NumberInputButtonCaps: u16,
    pub NumberInputValueCaps: u16,
    pub NumberInputDataIndices: u16,
    pub NumberOutputButtonCaps: u16,
    pub NumberOutputValueCaps: u16,
    pub NumberOutputDataIndices: u16,
    pub NumberFeatureButtonCaps: u16,
    pub NumberFeatureValueCaps: u16,
    pub NumberFeatureDataIndices: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HIDP_VALUE_CAPS {
    pub UsagePage: u16,
    pub ReportID: u8,
    pub IsAlias: u8,
    pub BitField: u16,
    pub LinkCollection: u16,
    pub LinkUsage: u16,
    pub LinkUsagePage: u16,
    pub IsRange: u8,
    pub IsStringRange: u8,
    pub IsDesignatorRange: u8,
    pub IsAbsolute: u8,
    pub HasNull: u8,
    pub Reserved: u8,
    pub BitSize: u16,
    pub ReportCount: u16,
    pub Reserved2: [u16; 5],
    pub UnitsExp: u32,
    pub Units: u32,
    pub LogicalMin: i32,
    pub LogicalMax: i32,
    pub PhysicalMin: i32,
    pub PhysicalMax: i32,
    pub UsageMinOrUsage: u16,
    pub UnionTail: [u16; 7],
}

#[link(name = "user32")]
unsafe extern "system" {
    fn RegisterClassExW(lpWndClass: *const WNDCLASSEXW) -> u16;
    fn CreateWindowExW(
        dwExStyle: u32,
        lpClassName: *const u16,
        lpWindowName: *const u16,
        dwStyle: u32,
        X: i32,
        Y: i32,
        nWidth: i32,
        nHeight: i32,
        hWndParent: HWND,
        hMenu: HANDLE,
        hInstance: HANDLE,
        lpParam: *mut std::ffi::c_void,
    ) -> HWND;
    fn DefWindowProcW(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    fn RegisterRawInputDevices(
        pRawInputDevices: *const RAWINPUTDEVICE,
        uiNumDevices: UINT,
        cbSize: UINT,
    ) -> BOOL;
    fn GetRawInputData(
        hRawInput: HRAWINPUT,
        uiCommand: UINT,
        pData: *mut std::ffi::c_void,
        pcbSize: *mut UINT,
        cbSizeHeader: UINT,
    ) -> UINT;
    fn GetRawInputDeviceInfoW(
        hDevice: HANDLE,
        uiCommand: UINT,
        pData: *mut std::ffi::c_void,
        pcbSize: *mut UINT,
    ) -> UINT;
    fn GetMessageW(lpMsg: *mut MSG, hWnd: HWND, wMsgFilterMin: UINT, wMsgFilterMax: UINT) -> BOOL;
    fn TranslateMessage(lpMsg: *const MSG) -> BOOL;
    fn DispatchMessageW(lpMsg: *const MSG) -> LRESULT;
    fn PostQuitMessage(nExitCode: i32);
    fn GetModuleHandleW(lpModuleName: *const u16) -> HANDLE;
    fn BeginPaint(hWnd: HWND, lpPaint: *mut PAINTSTRUCT) -> HANDLE;
    fn EndPaint(hWnd: HWND, lpPaint: *const PAINTSTRUCT) -> BOOL;
    fn InvalidateRect(hWnd: HWND, lpRect: *const RECT, bErase: BOOL) -> BOOL;
    fn ShowWindow(hWnd: HWND, nCmdShow: i32) -> BOOL;
    fn SetForegroundWindow(hWnd: HWND) -> BOOL;
    fn GetSystemMetrics(nIndex: i32) -> i32;
    fn DrawTextW(hdc: HANDLE, lpchText: *const u16, cchText: i32, lprc: *mut RECT, format: UINT) -> i32;
    fn SetTimer(hWnd: HWND, nIDEvent: usize, uElapse: UINT, lpTimerFunc: *mut std::ffi::c_void) -> usize;
    fn KillTimer(hWnd: HWND, uIDEvent: usize) -> BOOL;
    fn GetCursorPos(lpPoint: *mut POINT) -> BOOL;
    fn SetCursorPos(X: i32, Y: i32) -> BOOL;
    fn RegisterHotKey(hWnd: HWND, id: i32, fsModifiers: UINT, vk: UINT) -> BOOL;
    fn UnregisterHotKey(hWnd: HWND, id: i32) -> BOOL;
    fn GetAsyncKeyState(vKey: i32) -> i16;
    fn SetWindowsHookExW(
        idHook: i32,
        lpfn: unsafe extern "system" fn(i32, WPARAM, LPARAM) -> LRESULT,
        hmod: HANDLE,
        dwThreadId: u32,
    ) -> HANDLE;
    fn UnhookWindowsHookEx(hhk: HANDLE) -> BOOL;
    fn CallNextHookEx(hhk: HANDLE, nCode: i32, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    fn LoadIconW(hInstance: HANDLE, lpIconName: *const u16) -> HICON;
    fn LoadCursorW(hInstance: HANDLE, lpCursorName: *const u16) -> HANDLE;
    fn CreatePopupMenu() -> HMENU;
    fn AppendMenuW(hMenu: HMENU, uFlags: UINT, uIDNewItem: usize, lpNewItem: *const u16) -> BOOL;
    fn TrackPopupMenu(
        hMenu: HMENU,
        uFlags: UINT,
        x: i32,
        y: i32,
        nReserved: i32,
        hWnd: HWND,
        prcRect: *const RECT,
    ) -> BOOL;
    fn DestroyMenu(hMenu: HMENU) -> BOOL;
    fn FindWindowW(lpClassName: *const u16, lpWindowName: *const u16) -> HWND;
    fn PostMessageW(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> BOOL;
}

#[link(name = "kernel32")]
unsafe extern "system" {
    fn CreateMutexW(
        lpMutexAttributes: *mut std::ffi::c_void,
        bInitialOwner: BOOL,
        lpName: *const u16,
    ) -> HANDLE;
    fn GetLastError() -> u32;
}

#[link(name = "gdi32")]
unsafe extern "system" {
    fn CreateSolidBrush(color: u32) -> HANDLE;
    fn CreatePen(iStyle: i32, cWidth: i32, color: u32) -> HANDLE;
    fn SelectObject(hdc: HANDLE, h: HANDLE) -> HANDLE;
    fn DeleteObject(ho: HANDLE) -> BOOL;
    fn RoundRect(hdc: HANDLE, left: i32, top: i32, right: i32, bottom: i32, width: i32, height: i32) -> BOOL;
    fn Ellipse(hdc: HANDLE, left: i32, top: i32, right: i32, bottom: i32) -> BOOL;
    fn MoveToEx(hdc: HANDLE, x: i32, y: i32, lppt: *mut std::ffi::c_void) -> BOOL;
    fn LineTo(hdc: HANDLE, x: i32, y: i32) -> BOOL;
    fn SetBkMode(hdc: HANDLE, mode: i32) -> i32;
    fn SetTextColor(hdc: HANDLE, color: u32) -> u32;
    fn CreateFontW(
        cHeight: i32, cWidth: i32, cEscapement: i32, cOrientation: i32,
        cWeight: i32, bItalic: u32, bUnderline: u32, bStrikeOut: u32,
        iCharSet: u32, iOutPrecision: u32, iClipPrecision: u32,
        iQuality: u32, iPitchAndFamily: u32, pszFaceName: *const u16,
    ) -> HANDLE;
    fn CreateCompatibleDC(hdc: HANDLE) -> HANDLE;
    fn CreateCompatibleBitmap(hdc: HANDLE, cx: i32, cy: i32) -> HANDLE;
    fn BitBlt(hdcDest: HANDLE, x: i32, y: i32, cx: i32, cy: i32, hdcSrc: HANDLE, x1: i32, y1: i32, rop: u32) -> BOOL;
    fn DeleteDC(hdc: HANDLE) -> BOOL;
    fn GetTextExtentPoint32W(hdc: HANDLE, lpString: *const u16, c: i32, psizl: *mut SIZE) -> BOOL;
}

#[link(name = "shell32")]
unsafe extern "system" {
    fn ShellExecuteW(
        hwnd: HWND,
        lpOperation: *const u16,
        lpFile: *const u16,
        lpParameters: *const u16,
        lpDirectory: *const u16,
        nShowCmd: i32,
    ) -> HANDLE;
    fn Shell_NotifyIconW(dwMessage: u32, lpData: *const NOTIFYICONDATAW) -> BOOL;
}

#[link(name = "hid")]
unsafe extern "system" {
    fn HidP_GetCaps(PreparsedData: *const u8, Capabilities: *mut HIDP_CAPS) -> i32;
    fn HidP_GetValueCaps(
        ReportType: i32,
        ValueCaps: *mut HIDP_VALUE_CAPS,
        ValueCapsLength: *mut u16,
        PreparsedData: *const u8,
    ) -> i32;
    fn HidP_GetUsageValue(
        ReportType: i32,
        UsagePage: u16,
        LinkCollection: u16,
        Usage: u16,
        UsageValue: *mut u32,
        PreparsedData: *const u8,
        Report: *const u8,
        ReportLength: u32,
    ) -> i32;
    fn HidP_GetUsages(
        ReportType: i32,
        UsagePage: u16,
        LinkCollection: u16,
        UsageList: *mut u16,
        UsageLength: *mut u32,
        PreparsedData: *const u8,
        Report: *const u8,
        ReportLength: u32,
    ) -> i32;
}

#[link(name = "ole32")]
unsafe extern "system" {
    fn CoInitializeEx(pvReserved: *mut std::ffi::c_void, dwCoInit: u32) -> i32;
    fn CoUninitialize();
}

fn rgb(r: u8, g: u8, b: u8) -> u32 {
    (r as u32) | ((g as u32) << 8) | ((b as u32) << 16)
}

fn to_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

fn spawn_shell_open(app: String, param: String) {
    std::thread::spawn(move || unsafe {
        let hr = CoInitializeEx(null_mut(), 0x06);
        let op = to_wide("open");
        let file = to_wide(&app);
        let args = to_wide(&param);
        ShellExecuteW(
            null_mut(),
            op.as_ptr(),
            file.as_ptr(),
            if param.is_empty() { null() } else { args.as_ptr() },
            null(),
            SW_SHOW,
        );
        if hr >= 0 {
            CoUninitialize();
        }
    });
}

// ============================================================================
// 2. PRECISION TOUCHPAD (0x000D:0x0005) RAW HID PARSER
// ============================================================================
pub struct PtpDeviceCache {
    pub h_device: HANDLE,
    pub preparsed: Vec<u8>,
    pub collections: Vec<u16>,
    pub x_min: i32,
    pub x_max: i32,
    pub y_min: i32,
    pub y_max: i32,
}

impl PtpDeviceCache {
    pub fn from_device(h_device: HANDLE) -> Option<Self> {
        unsafe {
            let mut pp_size: UINT = 0;
            GetRawInputDeviceInfoW(h_device, RIDI_PREPARSEDDATA, null_mut(), &mut pp_size);
            if pp_size == 0 || pp_size > 65536 {
                log_line(&format!("⚠️ [PTP] RIDI_PREPARSEDDATA boyutu geçersiz: {}", pp_size));
                return None;
            }
            let mut preparsed = vec![0u8; pp_size as usize];
            let res = GetRawInputDeviceInfoW(
                h_device,
                RIDI_PREPARSEDDATA,
                preparsed.as_mut_ptr() as *mut _,
                &mut pp_size,
            );
            if res == u32::MAX {
                log_line("⚠️ [PTP] GetRawInputDeviceInfoW başarısız oldu!");
                return None;
            }

            let mut caps = HIDP_CAPS::default();
            if HidP_GetCaps(preparsed.as_ptr(), &mut caps) != HIDP_STATUS_SUCCESS {
                log_line("⚠️ [PTP] HidP_GetCaps başarısız oldu!");
                return None;
            }

            let mut num_vc = caps.NumberInputValueCaps;
            if num_vc == 0 {
                return None;
            }
            let mut vcaps: Vec<HIDP_VALUE_CAPS> = vec![std::mem::zeroed(); num_vc as usize];
            if HidP_GetValueCaps(HidP_Input, vcaps.as_mut_ptr(), &mut num_vc, preparsed.as_ptr())
                != HIDP_STATUS_SUCCESS
            {
                return None;
            }

            let mut collections = Vec::new();
            let mut x_min = 0;
            let mut x_max = 4095;
            let mut y_min = 0;
            let mut y_max = 4095;

            for vc in &vcaps[..num_vc as usize] {
                if vc.UsagePage == 0x01 && vc.UsageMinOrUsage == 0x30 {
                    if vc.LinkCollection != 0 && !collections.contains(&vc.LinkCollection) {
                        collections.push(vc.LinkCollection);
                    }
                    if vc.LogicalMax > vc.LogicalMin {
                        x_min = vc.LogicalMin;
                        x_max = vc.LogicalMax;
                    }
                } else if vc.UsagePage == 0x01 && vc.UsageMinOrUsage == 0x31 {
                    if vc.LogicalMax > vc.LogicalMin {
                        y_min = vc.LogicalMin;
                        y_max = vc.LogicalMax;
                    }
                }
            }

            collections.sort_unstable();
            if collections.is_empty() {
                collections.push(1);
                collections.push(2);
            }

            log_line(&format!(
                "📡 [PTP DONANIM BAĞLANDI] LinkCollections: {:?}, X: [{}..{}], Y: [{}..{}]",
                collections, x_min, x_max, y_min, y_max
            ));

            Some(Self {
                h_device,
                preparsed,
                collections,
                x_min,
                x_max,
                y_min,
                y_max,
            })
        }
    }

    pub fn parse_report(&self, report: &[u8]) -> Vec<(f32, f32)> {
        let mut contacts = Vec::with_capacity(5);
        let pp_ptr = self.preparsed.as_ptr();
        let rep_ptr = report.as_ptr();
        let rep_len = report.len() as u32;
        let x_span = (self.x_max - self.x_min).max(1) as f32;
        let y_span = (self.y_max - self.y_min).max(1) as f32;

        unsafe {
            let mut hw_contact_count: u32 = 0;
            let _ = HidP_GetUsageValue(
                HidP_Input,
                0x0D,
                0,
                0x54,
                &mut hw_contact_count,
                pp_ptr,
                rep_ptr,
                rep_len,
            );

            for &lc in &self.collections {
                let mut usages = [0u16; 16];
                let mut u_len: u32 = 16;
                let st = HidP_GetUsages(
                    HidP_Input,
                    0x0D,
                    lc,
                    usages.as_mut_ptr(),
                    &mut u_len,
                    pp_ptr,
                    rep_ptr,
                    rep_len,
                );

                let mut tip_down = false;
                if st == HIDP_STATUS_SUCCESS {
                    for &u in &usages[..(u_len as usize).min(16)] {
                        if u == 0x42 {
                            tip_down = true;
                            break;
                        }
                    }
                }

                if tip_down {
                    let mut x_val: u32 = 0;
                    let mut y_val: u32 = 0;
                    let sx = HidP_GetUsageValue(
                        HidP_Input, 0x01, lc, 0x30, &mut x_val, pp_ptr, rep_ptr, rep_len,
                    );
                    let sy = HidP_GetUsageValue(
                        HidP_Input, 0x01, lc, 0x31, &mut y_val, pp_ptr, rep_ptr, rep_len,
                    );

                    if sx == HIDP_STATUS_SUCCESS && sy == HIDP_STATUS_SUCCESS {
                        let nx = ((x_val as i32 - self.x_min) as f32 / x_span) * 480.0;
                        let ny = ((y_val as i32 - self.y_min) as f32 / y_span) * 320.0;
                        contacts.push((nx, ny));
                    }
                }
            }

            if contacts.is_empty() && hw_contact_count > 0 {
                for &lc in self.collections.iter().take(hw_contact_count as usize) {
                    let mut x_val: u32 = 0;
                    let mut y_val: u32 = 0;
                    let sx = HidP_GetUsageValue(
                        HidP_Input, 0x01, lc, 0x30, &mut x_val, pp_ptr, rep_ptr, rep_len,
                    );
                    let sy = HidP_GetUsageValue(
                        HidP_Input, 0x01, lc, 0x31, &mut y_val, pp_ptr, rep_ptr, rep_len,
                    );
                    if sx == HIDP_STATUS_SUCCESS && sy == HIDP_STATUS_SUCCESS && (x_val > 0 || y_val > 0) {
                        let nx = ((x_val as i32 - self.x_min) as f32 / x_span) * 480.0;
                        let ny = ((y_val as i32 - self.y_min) as f32 / y_span) * 320.0;
                        contacts.push((nx, ny));
                    }
                }
            }
        }
        contacts
    }
}

// ============================================================================
// 3. $1 UNISTROKE + GEOMETRIC CLASSIFIER + CONFIG.JSON
// ============================================================================
const NUM_RESAMPLE_POINTS: usize = 64;

#[derive(Clone)]
pub struct GestureRule {
    pub letter: char,
    pub name: String,
    pub app: String,
    pub param: String,
    pub custom_points: Vec<(f32, f32)>,
}

pub struct BuiltinTemplate {
    pub letter: char,
    pub points: Vec<(f32, f32)>,
}

pub fn path_length(pts: &[(f32, f32)]) -> f32 {
    let mut d = 0.0;
    for i in 1..pts.len() {
        d += (pts[i].0 - pts[i - 1].0).hypot(pts[i].1 - pts[i - 1].1);
    }
    d
}

pub fn normalize_stroke(raw_pts: &[(f32, f32)]) -> Vec<(f32, f32)> {
    if raw_pts.len() < 2 {
        return vec![(0.0, 0.0); NUM_RESAMPLE_POINTS];
    }

    let total_len = path_length(raw_pts);
    let interval = (total_len / (NUM_RESAMPLE_POINTS as f32 - 1.0)).max(1e-4);
    let mut pts = Vec::with_capacity(raw_pts.len() + NUM_RESAMPLE_POINTS);
    pts.extend_from_slice(raw_pts);

    let mut resampled = Vec::with_capacity(NUM_RESAMPLE_POINTS);
    resampled.push(pts[0]);

    let mut acc_d = 0.0;
    let mut i = 1;
    while i < pts.len() {
        let p1 = pts[i - 1];
        let p2 = pts[i];
        let d = (p2.0 - p1.0).hypot(p2.1 - p1.1);
        if (acc_d + d) >= interval {
            let t = (interval - acc_d) / d.max(1e-5);
            let qx = p1.0 + t * (p2.0 - p1.0);
            let qy = p1.1 + t * (p2.1 - p1.1);
            resampled.push((qx, qy));
            pts.insert(i, (qx, qy));
            acc_d = 0.0;
        } else {
            acc_d += d;
        }
        i += 1;
    }

    while resampled.len() < NUM_RESAMPLE_POINTS {
        resampled.push(*pts.last().unwrap());
    }
    resampled.truncate(NUM_RESAMPLE_POINTS);

    let cx = resampled.iter().map(|p| p.0).sum::<f32>() / (NUM_RESAMPLE_POINTS as f32);
    let cy = resampled.iter().map(|p| p.1).sum::<f32>() / (NUM_RESAMPLE_POINTS as f32);
    for p in &mut resampled {
        p.0 -= cx;
        p.1 -= cy;
    }

    let mut min_x = f32::MAX;
    let mut max_x = f32::MIN;
    let mut min_y = f32::MAX;
    let mut max_y = f32::MIN;
    for p in &resampled {
        if p.0 < min_x { min_x = p.0; }
        if p.0 > max_x { max_x = p.0; }
        if p.1 < min_y { min_y = p.1; }
        if p.1 > max_y { max_y = p.1; }
    }

    let scale = (max_x - min_x).max(max_y - min_y).max(1e-4);
    for p in &mut resampled {
        p.0 /= scale;
        p.1 /= scale;
    }

    resampled
}

pub fn template_similarity(candidate: &[(f32, f32)], template: &[(f32, f32)]) -> f32 {
    if candidate.len() != NUM_RESAMPLE_POINTS || template.len() != NUM_RESAMPLE_POINTS {
        return 0.0;
    }

    let mut best_dist = f32::MAX;
    for deg in (-24..=24).step_by(3) {
        let rad = (deg as f32).to_radians();
        let cos_a = rad.cos();
        let sin_a = rad.sin();
        let mut sum = 0.0;
        for i in 0..NUM_RESAMPLE_POINTS {
            let rx = candidate[i].0 * cos_a - candidate[i].1 * sin_a;
            let ry = candidate[i].0 * sin_a + candidate[i].1 * cos_a;
            sum += (rx - template[i].0).hypot(ry - template[i].1);
        }
        let avg = sum / (NUM_RESAMPLE_POINTS as f32);
        if avg < best_dist {
            best_dist = avg;
        }
    }

    ((1.0 - (best_dist / 0.48)).clamp(0.0, 1.0)) * 100.0
}

pub fn build_builtin_templates() -> Vec<BuiltinTemplate> {
    let mut list = Vec::new();

    // 1. C Harfi Şablonları
    let mut c1 = Vec::new();
    for i in 0..40 {
        let t = i as f32 / 39.0;
        let a = (-50.0f32 - t * 260.0).to_radians();
        c1.push((a.cos() * 85.0, a.sin() * 100.0));
    }
    list.push(BuiltinTemplate { letter: 'C', points: normalize_stroke(&c1) });

    let c2 = vec![
        (70.0, -80.0), (20.0, -100.0), (-45.0, -85.0), (-85.0, -40.0),
        (-95.0, 0.0), (-85.0, 45.0), (-45.0, 85.0), (20.0, 100.0), (75.0, 80.0),
    ];
    list.push(BuiltinTemplate { letter: 'C', points: normalize_stroke(&c2) });

    // 2. V Harfi Şablonları
    let v1 = vec![(-80.0, -100.0), (-40.0, 0.0), (0.0, 100.0), (40.0, 0.0), (80.0, -100.0)];
    let v2 = vec![(-55.0, -100.0), (0.0, 100.0), (55.0, -100.0)];
    let v3 = vec![(-90.0, -50.0), (-10.0, 100.0), (85.0, -100.0)];
    list.push(BuiltinTemplate { letter: 'V', points: normalize_stroke(&v1) });
    list.push(BuiltinTemplate { letter: 'V', points: normalize_stroke(&v2) });
    list.push(BuiltinTemplate { letter: 'V', points: normalize_stroke(&v3) });

    // 3. M Harfi Şablonları
    let m1 = vec![
        (-100.0, 90.0), (-70.0, -90.0), (0.0, 30.0), (70.0, -90.0), (100.0, 90.0),
    ];
    let m2 = vec![
        (-100.0, 90.0), (-90.0, -90.0), (0.0, 80.0), (90.0, -90.0), (100.0, 90.0),
    ];
    list.push(BuiltinTemplate { letter: 'M', points: normalize_stroke(&m1) });
    list.push(BuiltinTemplate { letter: 'M', points: normalize_stroke(&m2) });

    // 4. S Harfi Şablonu
    let s1 = vec![
        (65.0, -85.0), (10.0, -100.0), (-55.0, -75.0), (-60.0, -25.0),
        (0.0, 0.0), (60.0, 25.0), (55.0, 75.0), (-10.0, 100.0), (-65.0, 85.0),
    ];
    list.push(BuiltinTemplate { letter: 'S', points: normalize_stroke(&s1) });

    // 5. W Harfi Şablonu
    let w1 = vec![
        (-100.0, -90.0), (-55.0, 90.0), (0.0, -20.0), (55.0, 90.0), (100.0, -90.0),
    ];
    list.push(BuiltinTemplate { letter: 'W', points: normalize_stroke(&w1) });

    list
}

/// Saf Geometrik Özellik Analizörü (El yazısı sapmalarına karşı C, V, M, S, W harflerini kesin ayırır)
pub fn compute_geometric_scores(pts: &[(f32, f32)]) -> Vec<(char, f32)> {
    let mut scores = vec![('C', 0.0f32), ('V', 0.0), ('M', 0.0), ('S', 0.0), ('W', 0.0)];
    let n = pts.len();
    if n < 6 {
        return scores;
    }

    // Başlangıç ve bitişteki olası parmak koyma/kaldırma çengellerini yumuşatmak için ilk/son %8 ortalaması
    let k = (n / 12).clamp(1, 4);
    let start_x = pts[..k].iter().map(|p| p.0).sum::<f32>() / (k as f32);
    let start_y = pts[..k].iter().map(|p| p.1).sum::<f32>() / (k as f32);
    let end_x = pts[n - k..].iter().map(|p| p.0).sum::<f32>() / (k as f32);
    let end_y = pts[n - k..].iter().map(|p| p.1).sum::<f32>() / (k as f32);

    let mut min_x = f32::MAX;
    let mut max_x = f32::MIN;
    let mut min_y = f32::MAX;
    let mut max_y = f32::MIN;
    let mut min_x_idx = 0usize;
    let mut max_y_idx = 0usize;
    let mut min_y_idx = 0usize;

    for (i, &(x, y)) in pts.iter().enumerate() {
        if x < min_x {
            min_x = x;
            min_x_idx = i;
        }
        if x > max_x {
            max_x = x;
        }
        if y < min_y {
            min_y = y;
            min_y_idx = i;
        }
        if y > max_y {
            max_y = y;
            max_y_idx = i;
        }
    }

    let width = (max_x - min_x).max(1.0);
    let height = (max_y - min_y).max(1.0);

    // 1. 'V' Harfi Geometrisi:
    // En alt nokta (max_y) çizimin ortalarında (%12..%88), başlangıç ve bitiş noktaları max_y'den belirgin şekilde yukarıda!
    let v_pos_ratio = max_y_idx as f32 / (n as f32);
    let left_drop = (max_y - start_y) / height;
    let right_rise = (max_y - end_y) / height;
    let top_open = (start_x - min_x) / width;
    let bot_open = (end_x - min_x) / width;
    let looks_like_c_bow = min_x_idx > n / 5 && min_x_idx < (4 * n) / 5 && top_open > 0.35 && bot_open > 0.35;

    if v_pos_ratio >= 0.12
        && v_pos_ratio <= 0.88
        && left_drop >= 0.26
        && right_rise >= 0.26
        && !looks_like_c_bow
    {
        let mut v_sc = 80.0 + ((left_drop + right_rise) * 10.0).min(16.0);
        if end_x > start_x {
            v_sc += 4.0;
        }
        scores[1].1 = v_sc.min(98.0);
    }

    // 2. 'C' Harfi Geometrisi:
    // En sol nokta (min_x) çizimin ortalarında (%10..%90), hem başlangıç hem bitiş min_x'in sağında!
    let c_pos_ratio = min_x_idx as f32 / (n as f32);
    let vert_span = (end_y - start_y).abs() / height;
    if c_pos_ratio >= 0.10
        && c_pos_ratio <= 0.90
        && top_open >= 0.16
        && bot_open >= 0.16
        && vert_span >= 0.15
    {
        let mut c_sc = 80.0 + ((top_open + bot_open) * 10.0).min(16.0);
        // C harfinde en alt nokta (max_y) ikinci yarıdadır (%50..%100), V'de ise tam ortadadır
        if max_y_idx >= n / 2 {
            c_sc += 6.0;
        }
        scores[0].1 = c_sc.min(98.0);
    }

    // 3. 'M' Harfi Geometrisi:
    // İki tepe (küçük Y) ve ortada bir çukur (büyük Y)
    if n >= 12 && end_x > start_x + width * 0.25 {
        let q1_min_y = pts[..n / 3].iter().map(|p| p.1).fold(f32::MAX, f32::min);
        let mid_max_y = pts[n / 3..(2 * n) / 3].iter().map(|p| p.1).fold(f32::MIN, f32::max);
        let q3_min_y = pts[(2 * n) / 3..].iter().map(|p| p.1).fold(f32::MAX, f32::min);

        if (mid_max_y - q1_min_y) > height * 0.22
            && (mid_max_y - q3_min_y) > height * 0.22
            && (start_y - q1_min_y) > height * 0.20
            && (end_y - q3_min_y) > height * 0.20
        {
            scores[2].1 = 90.0;
        }
    }

    // 4. 'W' Harfi Geometrisi:
    // İki çukur (büyük Y) ve ortada bir tepe (küçük Y)
    if n >= 12 && end_x > start_x + width * 0.25 && min_y_idx > n / 4 && min_y_idx < (3 * n) / 4 {
        let q1_max_y = pts[..n / 2].iter().map(|p| p.1).fold(f32::MIN, f32::max);
        let q2_max_y = pts[n / 2..].iter().map(|p| p.1).fold(f32::MIN, f32::max);
        if (q1_max_y - min_y) > height * 0.25
            && (q2_max_y - min_y) > height * 0.25
            && (q1_max_y - start_y) > height * 0.25
            && (q2_max_y - end_y) > height * 0.25
        {
            scores[4].1 = 90.0;
        }
    }

    // 5. 'S' Harfi Geometrisi:
    // Yukarıdan aşağıya iner, ilk yarıda sola kıvrılır, ikinci yarıda sağa kıvrılıp sola döner
    if (end_y - start_y) > height * 0.35 {
        let first_half_min_x = pts[..n / 2].iter().map(|p| p.0).fold(f32::MAX, f32::min);
        let second_half_max_x = pts[n / 2..].iter().map(|p| p.0).fold(f32::MIN, f32::max);
        if (start_x - first_half_min_x) > width * 0.20
            && (second_half_max_x - end_x) > width * 0.20
            && (second_half_max_x - first_half_min_x) > width * 0.50
        {
            scores[3].1 = 87.0;
        }
    }

    scores
}

// --- config.json Okuyucu & Yazıcı ---
pub fn load_config(path: &str) -> (u128, Vec<GestureRule>) {
    let default_rules = vec![
        GestureRule {
            letter: 'C',
            name: "Hesap Makinesi".to_string(),
            app: "calc.exe".to_string(),
            param: "".to_string(),
            custom_points: Vec::new(),
        },
        GestureRule {
            letter: 'V',
            name: "Dosya Gezgini".to_string(),
            app: "explorer.exe".to_string(),
            param: "".to_string(),
            custom_points: Vec::new(),
        },
        GestureRule {
            letter: 'M',
            name: "Çizim Tuvali".to_string(),
            app: "mspaint.exe".to_string(),
            param: "".to_string(),
            custom_points: Vec::new(),
        },
        GestureRule {
            letter: 'S',
            name: "Ayarlar".to_string(),
            app: "ms-settings:".to_string(),
            param: "".to_string(),
            custom_points: Vec::new(),
        },
        GestureRule {
            letter: 'W',
            name: "Not Defteri".to_string(),
            app: "notepad.exe".to_string(),
            param: "".to_string(),
            custom_points: Vec::new(),
        },
    ];

    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return (4000, default_rules),
    };

    let timeout_ms = extract_json_u128(&content, "idle_timeout_ms").unwrap_or(4000);

    let mut rules = Vec::new();
    for block in content.split('{').skip(2) {
        let obj = block.split('}').next().unwrap_or("");
        let letter = extract_json_str(obj, "letter")
            .and_then(|s| s.chars().next())
            .map(|c| c.to_ascii_uppercase());
        let name = extract_json_str(obj, "name");
        let app = extract_json_str(obj, "app");
        let param = extract_json_str(obj, "param").unwrap_or_default();
        let custom_points = extract_json_points(obj, "custom_points");

        if let (Some(l), Some(n), Some(a)) = (letter, name, app) {
            rules.push(GestureRule {
                letter: l,
                name: n,
                app: a,
                param,
                custom_points,
            });
        }
    }

    (timeout_ms, if rules.is_empty() { default_rules } else { rules })
}

fn extract_json_u128(src: &str, key: &str) -> Option<u128> {
    let pat = format!("\"{}\"", key);
    let idx = src.find(&pat)?;
    let rest = &src[idx + pat.len()..];
    let colon = rest.find(':')?;
    let after = &rest[colon + 1..];
    let digits: String = after
        .chars()
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| c.is_ascii_digit())
        .collect();
    digits.parse::<u128>().ok()
}

fn extract_json_str(src: &str, key: &str) -> Option<String> {
    let pat = format!("\"{}\"", key);
    let idx = src.find(&pat)?;
    let after_colon = src[idx + pat.len()..].find(':')? + idx + pat.len() + 1;
    let first_quote = src[after_colon..].find('"')? + after_colon + 1;
    let end_quote = src[first_quote..].find('"')? + first_quote;
    Some(src[first_quote..end_quote].to_string())
}

fn extract_json_points(src: &str, key: &str) -> Vec<(f32, f32)> {
    let mut pts = Vec::new();
    let pat = format!("\"{}\"", key);
    if let Some(idx) = src.find(&pat) {
        let rest = &src[idx + pat.len()..];
        if let (Some(b_start), Some(b_end)) = (rest.find('['), rest.find(']')) {
            if b_end > b_start {
                let arr_str = &rest[b_start + 1..b_end];
                let nums: Vec<f32> = arr_str
                    .split(|c: char| c == ',' || c.is_whitespace())
                    .filter(|s| !s.is_empty())
                    .filter_map(|s| s.parse::<f32>().ok())
                    .collect();
                for pair in nums.chunks_exact(2) {
                    pts.push((pair[0], pair[1]));
                }
            }
        }
    }
    pts
}

pub fn save_rules_to_config(path: &str, idle_timeout_ms: u128, rules: &[GestureRule]) {
    let mut out = format!("{{\n  \"idle_timeout_ms\": {},\n  \"gestures\": [\n", idle_timeout_ms);
    for (i, r) in rules.iter().enumerate() {
        out.push_str("    {\n");
        out.push_str(&format!("      \"letter\": \"{}\",\n", r.letter));
        out.push_str(&format!("      \"name\": \"{}\",\n", r.name));
        out.push_str(&format!("      \"app\": \"{}\",\n", r.app));
        out.push_str(&format!("      \"param\": \"{}\",\n", r.param));
        out.push_str("      \"custom_points\": [");
        for (pi, p) in r.custom_points.iter().enumerate() {
            if pi > 0 {
                out.push_str(", ");
            }
            out.push_str(&format!("{:.4}, {:.4}", p.0, p.1));
        }
        out.push_str("]\n    }");
        if i + 1 < rules.len() {
            out.push(',');
        }
        out.push('\n');
    }
    out.push_str("  ]\n}\n");
    let _ = fs::write(path, out);
}

// ============================================================================
// 4. APPLICATION STATE & ENGINE
// ============================================================================
pub struct AppState {
    pub hud_hwnd: HWND,
    pub is_hud_visible: bool,
    pub is_paused: bool,
    pub is_teaching_mode: bool,
    pub teaching_letter: char,
    pub pos_x: i32,
    pub pos_y: i32,
    pub win_w: i32,
    pub win_h: i32,
    pub saved_cursor_pos: POINT,

    // Zamanlayıcı ve Otomatik Kapanma (Test Modu: 10sn)
    pub idle_timeout_ms: u128,
    pub last_activity_time: Instant,

    // Çizim Verileri (Sınırsız Sanal Koordinatlar + Dinamik Auto-Fit)
    pub virtual_points: Vec<(f32, f32)>,
    pub prev_contact_count: usize,
    pub last_centroid: Option<(f32, f32)>,
    pub last_ptp_packet_time: Option<Instant>,
    pub active_finger_count: usize,

    // PTP 2-Parmak "Tık-Tık" Durum Makinesi
    pub ptp_cache: Option<PtpDeviceCache>,
    pub tap_session_start: Option<Instant>,
    pub tap_max_fingers: usize,
    pub tap_start_centroid: Option<(f32, f32)>,
    pub tap_max_move: f32,
    pub last_valid_2f_tap: Option<Instant>,
    pub last_close_time: Option<Instant>,

    // Fare ile Çizim Yedeği
    pub mouse_left_down: bool,
    pub last_mouse_pt: Option<POINT>,

    // Arayüz & Kurallar
    pub status_text: String,
    pub rules: Vec<GestureRule>,
    pub builtins: Vec<BuiltinTemplate>,
}

static mut GLOBAL_APP: Option<AppState> = None;

impl AppState {
    pub fn new(pos_x: i32, pos_y: i32, win_w: i32, win_h: i32) -> Self {
        let (idle_timeout_ms, rules) = load_config("config.json");
        if !std::path::Path::new("config.json").exists() {
            save_rules_to_config("config.json", idle_timeout_ms, &rules);
        }
        let builtins = build_builtin_templates();

        Self {
            hud_hwnd: null_mut(),
            is_hud_visible: false,
            is_paused: false,
            is_teaching_mode: false,
            teaching_letter: 'C',
            pos_x,
            pos_y,
            win_w,
            win_h,
            saved_cursor_pos: POINT::default(),
            idle_timeout_ms,
            last_activity_time: Instant::now(),
            virtual_points: Vec::with_capacity(512),
            prev_contact_count: 0,
            last_centroid: None,
            last_ptp_packet_time: None,
            active_finger_count: 0,
            ptp_cache: None,
            tap_session_start: None,
            tap_max_fingers: 0,
            tap_start_centroid: None,
            tap_max_move: 0.0,
            last_valid_2f_tap: None,
            last_close_time: None,
            mouse_left_down: false,
            last_mouse_pt: None,
            status_text: "✨ Touchpad'de 1 veya 2 parmağınızı kaydırarak harf çizin...".to_string(),
            rules,
            builtins,
        }
    }

    pub fn show_hud(&mut self, reason: &str) {
        if self.hud_hwnd.is_null() || self.is_hud_visible {
            return;
        }

        if let Some(close_t) = self.last_close_time {
            if close_t.elapsed().as_millis() < 400 {
                return;
            }
        }

        log_line(&format!("🟢 [HUD AÇILDI] Tetikleyen: {}", reason));

        self.virtual_points.clear();
        self.prev_contact_count = 0;
        self.last_centroid = None;
        self.mouse_left_down = false;
        self.last_mouse_pt = None;
        self.active_finger_count = 0;
        self.is_teaching_mode = false;
        self.last_activity_time = Instant::now();
        self.status_text = "✍️ Touchpad'de 1 veya 2 parmakla 'C', 'V', 'M' çizin (Bıraktığınız an tanınır)...".to_string();
        self.is_hud_visible = true;

        unsafe {
            GetCursorPos(&mut self.saved_cursor_pos);
            ShowWindow(self.hud_hwnd, SW_SHOW);
            SetForegroundWindow(self.hud_hwnd);

            // HUD açıkken ESC (ID=2) ve T (ID=3) tuşlarını global olarak yakala (Odak başka pencerede olsa bile %100 çalışır!)
            RegisterHotKey(self.hud_hwnd, 2, 0, VK_ESCAPE);
            RegisterHotKey(self.hud_hwnd, 3, 0, VK_T);

            // 100ms Watchdog Timer (10sn geri sayım ve ESC kontrolü için)
            SetTimer(self.hud_hwnd, 1005, 100, null_mut());

            InvalidateRect(self.hud_hwnd, null(), 0);
        }
    }

    pub fn hide_hud(&mut self, reason: &str) {
        if self.hud_hwnd.is_null() || !self.is_hud_visible {
            return;
        }

        log_line(&format!("🔴 [HUD KAPANDI] Sebep: {}", reason));

        self.is_hud_visible = false;
        self.is_teaching_mode = false;
        self.virtual_points.clear();
        self.prev_contact_count = 0;
        self.last_centroid = None;
        self.mouse_left_down = false;
        self.last_mouse_pt = None;
        self.active_finger_count = 0;
        self.last_close_time = Some(Instant::now());

        unsafe {
            UnregisterHotKey(self.hud_hwnd, 2);
            UnregisterHotKey(self.hud_hwnd, 3);
            KillTimer(self.hud_hwnd, 1002);
            KillTimer(self.hud_hwnd, 1003);
            KillTimer(self.hud_hwnd, 1005);
            ShowWindow(self.hud_hwnd, SW_HIDE);
            SetCursorPos(self.saved_cursor_pos.x, self.saved_cursor_pos.y);
        }
    }

    pub fn toggle_teaching_mode(&mut self) {
        self.is_teaching_mode = !self.is_teaching_mode;
        self.virtual_points.clear();
        self.prev_contact_count = 0;
        self.last_centroid = None;
        self.last_activity_time = Instant::now();

        if self.is_teaching_mode {
            log_line(&format!("🎯 [ÖĞRETME MODU AÇILDI] Hedef Harf: '{}'", self.teaching_letter));
            self.status_text = format!(
                "🎯 ÖĞRETME MODU: Klavyeden harf seçin (Şu an: '{}') ve touchpad'de şekli çizin!",
                self.teaching_letter
            );
        } else {
            log_line("🎯 [ÖĞRETME MODU KAPATILDI]");
            self.status_text = "✍️ Touchpad'de 1 veya 2 parmakla 'C', 'V', 'M' çizin...".to_string();
        }
        unsafe {
            InvalidateRect(self.hud_hwnd, null(), 0);
        }
    }

    /// Precision Touchpad (0x000D:0x0005) Ham Parmak Verisi İşleyici
    pub fn process_ptp_contacts(&mut self, contacts: &[(f32, f32)]) {
        let now = Instant::now();
        self.last_ptp_packet_time = Some(now);
        self.active_finger_count = contacts.len();

        if !self.is_hud_visible {
            if self.is_paused {
                return;
            }
            if !contacts.is_empty() {
                let cx = contacts.iter().map(|p| p.0).sum::<f32>() / (contacts.len() as f32);
                let cy = contacts.iter().map(|p| p.1).sum::<f32>() / (contacts.len() as f32);

                if self.tap_session_start.is_none() {
                    self.tap_session_start = Some(now);
                    self.tap_max_fingers = contacts.len();
                    self.tap_start_centroid = Some((cx, cy));
                    self.tap_max_move = 0.0;
                } else {
                    if contacts.len() > self.tap_max_fingers {
                        self.tap_max_fingers = contacts.len();
                        self.tap_start_centroid = Some((cx, cy));
                    } else if let Some((sx, sy)) = self.tap_start_centroid {
                        let dist = (cx - sx).hypot(cy - sy);
                        if dist > self.tap_max_move {
                            self.tap_max_move = dist;
                        }
                    }
                }
            } else if let Some(start_t) = self.tap_session_start.take() {
                let dur_ms = now.duration_since(start_t).as_millis();
                if self.tap_max_fingers >= 2 && dur_ms <= 340 && self.tap_max_move < 42.0 {
                    log_line(&format!(
                        "👆 [PTP 2-PARMAK DOKUNUŞ] Süre: {}ms, Kayma: {:.1}",
                        dur_ms, self.tap_max_move
                    ));
                    if let Some(prev_tap) = self.last_valid_2f_tap {
                        let gap_ms = now.duration_since(prev_tap).as_millis();
                        if gap_ms >= 20 && gap_ms <= 550 {
                            self.last_valid_2f_tap = None;
                            self.show_hud("PTP 2-Parmak Çift Dokunuş (Tık-Tık)");
                            return;
                        }
                    }
                    self.last_valid_2f_tap = Some(now);
                }
                self.tap_max_fingers = 0;
                self.tap_start_centroid = None;
            }
            return;
        }

        // --- HUD AÇIKKEN: 1 veya 2 Parmak Delta-Centroid Çizim Motoru ---
        if !contacts.is_empty() {
            self.last_activity_time = now;

            let cx = contacts.iter().map(|p| p.0).sum::<f32>() / (contacts.len() as f32);
            let cy = contacts.iter().map(|p| p.1).sum::<f32>() / (contacts.len() as f32);

            if contacts.len() != self.prev_contact_count {
                log_line(&format!(
                    "🖐️ [PARMAK SAYISI DEĞİŞTİ] {} -> {} parmak (Merkez: {:.1}, {:.1})",
                    self.prev_contact_count,
                    contacts.len(),
                    cx,
                    cy
                ));
                self.prev_contact_count = contacts.len();
                self.last_centroid = Some((cx, cy));
                if self.virtual_points.is_empty() {
                    self.virtual_points.push((0.0, 0.0));
                }
            } else if let Some((prev_cx, prev_cy)) = self.last_centroid {
                let dx = (cx - prev_cx) * 1.15;
                let dy = (cy - prev_cy) * 1.15;
                if dx.hypot(dy) >= 0.9 {
                    self.last_centroid = Some((cx, cy));
                    let (last_vx, last_vy) = self.virtual_points.last().copied().unwrap_or((0.0, 0.0));
                    self.virtual_points.push((last_vx + dx, last_vy + dy));

                    // KRİTİK DÜZELTME: Her gerçek harekette 280ms değerlendirme sayacını kur!
                    // Böylece parmak kalktığında I2C veri akışı kesilse bile 280ms sonra %100 algılar!
                    unsafe {
                        SetTimer(self.hud_hwnd, 1003, 280, null_mut());
                    }
                }
            }

            unsafe {
                InvalidateRect(self.hud_hwnd, null(), 0);
            }
        } else if self.prev_contact_count > 0 {
            log_line(&format!(
                "✋ [PARMAKLAR KALKTI] Toplam çizim noktası: {}",
                self.virtual_points.len()
            ));
            self.prev_contact_count = 0;
            self.last_centroid = None;
            unsafe {
                InvalidateRect(self.hud_hwnd, null(), 0);
                // Parmak kalkışı doğrudan yakalandıysa 80ms içinde anında değerlendir!
                SetTimer(self.hud_hwnd, 1003, 80, null_mut());
            }
        }
    }

    pub fn project_points_to_canvas(&self) -> Vec<POINT> {
        if self.virtual_points.is_empty() {
            return Vec::new();
        }

        let view_left = 48.0f32;
        let view_right = 492.0f32;
        let view_top = 96.0f32;
        let view_bottom = 362.0f32;
        let view_w = view_right - view_left;
        let view_h = view_bottom - view_top;
        let center_x = (view_left + view_right) * 0.5;
        let center_y = (view_top + view_bottom) * 0.5;

        let mut min_x = f32::MAX;
        let mut max_x = f32::MIN;
        let mut min_y = f32::MAX;
        let mut max_y = f32::MIN;
        for &(vx, vy) in &self.virtual_points {
            if vx < min_x { min_x = vx; }
            if vx > max_x { max_x = vx; }
            if vy < min_y { min_y = vy; }
            if vy > max_y { max_y = vy; }
        }

        let span_w = (max_x - min_x).max(1.0);
        let span_h = (max_y - min_y).max(1.0);
        let scale = (view_w / span_w).min(view_h / span_h).min(1.0);

        let mut origin_x = center_x;
        let mut origin_y = center_y;

        if scale < 1.0 {
            let mid_x = (min_x + max_x) * 0.5;
            let mid_y = (min_y + max_y) * 0.5;
            origin_x = center_x - mid_x * scale;
            origin_y = center_y - mid_y * scale;
        } else {
            if origin_x + min_x < view_left {
                origin_x = view_left - min_x;
            } else if origin_x + max_x > view_right {
                origin_x = view_right - max_x;
            }

            if origin_y + min_y < view_top {
                origin_y = view_top - min_y;
            } else if origin_y + max_y > view_bottom {
                origin_y = view_bottom - max_y;
            }
        }

        self.virtual_points
            .iter()
            .map(|&(vx, vy)| POINT {
                x: (origin_x + vx * scale).round() as i32,
                y: (origin_y + vy * scale).round() as i32,
            })
            .collect()
    }

    pub fn evaluate_drawing(&mut self) {
        self.last_mouse_pt = None;
        if self.virtual_points.len() < 6 {
            return;
        }

        let total_len = path_length(&self.virtual_points);
        log_line(&format!(
            "🔍 [ÇİZİM DEĞERLENDİRİLİYOR] Nokta Sayısı: {}, Uzunluk: {:.1}",
            self.virtual_points.len(),
            total_len
        ));

        if total_len < 30.0 {
            log_line("⚠️ [ÇİZİM ÇOK KISA] Değerlendirme iptal edildi, tuval temizlendi.");
            self.status_text = "⚠️ Çok kısa çizgi. Lütfen harfi biraz daha belirgin çizin.".to_string();
            self.virtual_points.clear();
            unsafe { InvalidateRect(self.hud_hwnd, null(), 0); }
            return;
        }

        let norm_candidate = normalize_stroke(&self.virtual_points);

        // --- 1. ÖĞRETME (KALİBRASYON) MODU ---
        if self.is_teaching_mode {
            let target_ch = self.teaching_letter;
            let mut found = false;
            for r in &mut self.rules {
                if r.letter == target_ch {
                    r.custom_points = norm_candidate.clone();
                    found = true;
                    break;
                }
            }
            if !found {
                self.rules.push(GestureRule {
                    letter: target_ch,
                    name: format!("Özel ({})", target_ch),
                    app: "notepad.exe".to_string(),
                    param: "".to_string(),
                    custom_points: norm_candidate,
                });
            }
            save_rules_to_config("config.json", self.idle_timeout_ms, &self.rules);
            log_line(&format!(
                "✅ [KALİBRASYON KAYDEDİLDİ] '{}' harfi config.json'a yazıldı!",
                target_ch
            ));
            self.is_teaching_mode = false;
            self.virtual_points.clear();
            self.status_text = format!(
                "✅ '{}' harfi kendi çiziminizle kaydedildi! Şimdi çizerek deneyin.",
                target_ch
            );
            unsafe { InvalidateRect(self.hud_hwnd, null(), 0); }
            return;
        }

        // --- 2. HİBRİT (GEOMETRİK + $1 UNISTROKE) TANIMA ---
        let geo_scores = compute_geometric_scores(&self.virtual_points);
        let mut final_scores: Vec<(char, f32)> = vec![
            ('C', 0.0),
            ('V', 0.0),
            ('M', 0.0),
            ('S', 0.0),
            ('W', 0.0),
        ];

        // A. Geometrik puanları yükle
        for &(ch, g_sc) in &geo_scores {
            if let Some(entry) = final_scores.iter_mut().find(|e| e.0 == ch) {
                entry.1 = entry.1.max(g_sc);
            }
        }

        // B. Kullanıcının öğrettiği özel şablonları kontrol et (+10% öncelik)
        for r in &self.rules {
            if r.custom_points.len() == NUM_RESAMPLE_POINTS {
                let sc = (template_similarity(&norm_candidate, &r.custom_points) + 10.0).min(100.0);
                if let Some(entry) = final_scores.iter_mut().find(|e| e.0 == r.letter) {
                    entry.1 = entry.1.max(sc);
                } else {
                    final_scores.push((r.letter, sc));
                }
            }
        }

        // C. Yerleşik $1 Unistroke şablonlarını kontrol et
        for bt in &self.builtins {
            let sc = template_similarity(&norm_candidate, &bt.points);
            if let Some(entry) = final_scores.iter_mut().find(|e| e.0 == bt.letter) {
                entry.1 = entry.1.max(sc);
            }
        }

        // Log tablosu oluştur
        let score_summary: Vec<String> = final_scores
            .iter()
            .map(|(c, s)| format!("{}:{:.0}%", c, s))
            .collect();
        log_line(&format!("📊 [SKORLAR] {}", score_summary.join(" | ")));

        let mut best_letter: Option<char> = None;
        let mut best_score: f32 = 0.0;
        for &(ch, sc) in &final_scores {
            if sc > best_score {
                best_score = sc;
                best_letter = Some(ch);
            }
        }

        if let Some(letter) = best_letter {
            if best_score >= 54.0 {
                let app_name = self
                    .rules
                    .iter()
                    .find(|r| r.letter == letter)
                    .map(|r| r.name.clone())
                    .unwrap_or_else(|| letter.to_string());
                log_line(&format!(
                    "🎉 [HARF TANINDI] '{}' (%{:.0}) -> {} başlatılıyor!",
                    letter, best_score, app_name
                ));
                self.status_text = format!(
                    "🎉 '{}' Harfi Tanındı (%{:.0} Eşleşme) → {} Açılıyor...",
                    letter, best_score, app_name
                );
                self.launch_by_letter(letter);
                return;
            }
        }

        log_line(&format!(
            "❌ [EŞLEŞME YOK] En yakın: {:?} (%{:.0})",
            best_letter, best_score
        ));
        self.status_text = format!(
            "💡 Tam eşleşmedi (En yakın: %{:.0}). Tekrar çizin veya 'T' ile kendi çiziminizi öğretin.",
            best_score
        );
        self.virtual_points.clear();
        unsafe {
            InvalidateRect(self.hud_hwnd, null(), 0);
        }
    }

    fn launch_by_letter(&mut self, ch: char) {
        let rule = self.rules.iter().find(|r| r.letter == ch);
        if let Some(r) = rule {
            let app_str = r.app.clone();
            let param_str = r.param.clone();
            unsafe {
                InvalidateRect(self.hud_hwnd, null(), 0);
                SetTimer(self.hud_hwnd, 1002, 700, null_mut());
            }
            spawn_shell_open(app_str, param_str);
        }
    }
}

// ============================================================================
// 5. WIN32 HUD WINDOW PROCEDURE & TRAY MENU
// ============================================================================
unsafe extern "system" fn hud_wnd_proc(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match msg {
            WM_PAINT => {
                let mut ps: PAINTSTRUCT = std::mem::zeroed();
                let hdc = BeginPaint(hwnd, &mut ps);

                let w = 540;
                let h = 450;
                let mem_dc = CreateCompatibleDC(hdc);
                let mem_bmp = CreateCompatibleBitmap(hdc, w, h);
                let old_bmp = SelectObject(mem_dc, mem_bmp);

                // 1. Ana Kart Arka Planı (#15181F)
                let bg_brush = CreateSolidBrush(rgb(21, 24, 31));
                let border_pen = CreatePen(PS_SOLID, 2, rgb(52, 62, 82));
                let old_b = SelectObject(mem_dc, bg_brush);
                let old_p = SelectObject(mem_dc, border_pen);
                RoundRect(mem_dc, 0, 0, w, h, 24, 24);
                SelectObject(mem_dc, old_b);
                SelectObject(mem_dc, old_p);
                DeleteObject(bg_brush);
                DeleteObject(border_pen);

                // 2. Çizim Tuvali Alanı (#0B0D12)
                let canvas_brush = CreateSolidBrush(rgb(11, 13, 18));
                let canvas_pen = CreatePen(PS_SOLID, 1, rgb(38, 45, 60));
                let old_b = SelectObject(mem_dc, canvas_brush);
                let old_p = SelectObject(mem_dc, canvas_pen);
                RoundRect(mem_dc, 20, 68, w - 20, h - 55, 16, 16);
                SelectObject(mem_dc, old_b);
                SelectObject(mem_dc, old_p);
                DeleteObject(canvas_brush);
                DeleteObject(canvas_pen);

                // Tuval Nokta Izgarası
                let dot_pen = CreatePen(PS_SOLID, 1, rgb(28, 34, 46));
                let old_p = SelectObject(mem_dc, dot_pen);
                for x in (50..w - 50).step_by(40) {
                    for y in (95..h - 75).step_by(40) {
                        MoveToEx(mem_dc, x, y, null_mut());
                        LineTo(mem_dc, x + 2, y + 2);
                    }
                }
                SelectObject(mem_dc, old_p);
                DeleteObject(dot_pen);

                if let Some(ref app) = GLOBAL_APP {
                    // 3. [🎯 Harf Öğret (T)] Butonu
                    let teach_bg = if app.is_teaching_mode {
                        CreateSolidBrush(rgb(180, 120, 20))
                    } else {
                        CreateSolidBrush(rgb(32, 40, 56))
                    };
                    let teach_pen = CreatePen(PS_SOLID, 1, rgb(70, 90, 120));
                    let old_b = SelectObject(mem_dc, teach_bg);
                    let old_p = SelectObject(mem_dc, teach_pen);
                    RoundRect(mem_dc, 330, 12, 478, 40, 10, 10);
                    SelectObject(mem_dc, old_b);
                    SelectObject(mem_dc, old_p);
                    DeleteObject(teach_bg);
                    DeleteObject(teach_pen);

                    // 4. FOSFORLU NEON ÇİZGİLER (Auto-Fit İzdüşüm ile)
                    let canvas_pts = app.project_points_to_canvas();
                    if canvas_pts.len() >= 2 {
                        let glow_color = if app.is_teaching_mode {
                            rgb(230, 160, 20)
                        } else {
                            rgb(0, 185, 145)
                        };
                        let core_color = if app.is_teaching_mode {
                            rgb(255, 225, 90)
                        } else {
                            rgb(70, 255, 200)
                        };

                        let glow_pen = CreatePen(PS_SOLID, 9, glow_color);
                        let old_p = SelectObject(mem_dc, glow_pen);
                        MoveToEx(mem_dc, canvas_pts[0].x, canvas_pts[0].y, null_mut());
                        for p in &canvas_pts[1..] {
                            LineTo(mem_dc, p.x, p.y);
                        }
                        SelectObject(mem_dc, old_p);
                        DeleteObject(glow_pen);

                        let core_pen = CreatePen(PS_SOLID, 3, core_color);
                        let old_p = SelectObject(mem_dc, core_pen);
                        MoveToEx(mem_dc, canvas_pts[0].x, canvas_pts[0].y, null_mut());
                        for p in &canvas_pts[1..] {
                            LineTo(mem_dc, p.x, p.y);
                        }
                        SelectObject(mem_dc, old_p);
                        DeleteObject(core_pen);
                    }

                    // Çizim Ucu İmleç Noktası
                    if let Some(&last_pt) = canvas_pts.last() {
                        let dot_brush = CreateSolidBrush(rgb(120, 255, 220));
                        let dot_pen2 = CreatePen(PS_SOLID, 1, rgb(255, 255, 255));
                        let old_b = SelectObject(mem_dc, dot_brush);
                        let old_p = SelectObject(mem_dc, dot_pen2);
                        Ellipse(mem_dc, last_pt.x - 5, last_pt.y - 5, last_pt.x + 5, last_pt.y + 5);
                        SelectObject(mem_dc, old_b);
                        SelectObject(mem_dc, old_p);
                        DeleteObject(dot_brush);
                        DeleteObject(dot_pen2);
                    }

                    SetBkMode(mem_dc, TRANSPARENT);

                    // Başlık: touchpa(inte)d + Fırça Darbesi
                    let title_font = CreateFontW(
                        20, 0, 0, 0, 700, 0, 0, 0, 1, 0, 0, 2, 0,
                        to_wide("Segoe UI").as_ptr(),
                    );
                    let old_f = SelectObject(mem_dc, title_font);

                    let w_p1 = to_wide("touchpa");
                    let w_p2 = to_wide("(inte)");
                    let w_p3 = to_wide("d");
                    let mut sz1 = SIZE::default();
                    let mut sz2 = SIZE::default();
                    let mut sz3 = SIZE::default();
                    GetTextExtentPoint32W(mem_dc, w_p1.as_ptr(), (w_p1.len() - 1) as i32, &mut sz1);
                    GetTextExtentPoint32W(mem_dc, w_p2.as_ptr(), (w_p2.len() - 1) as i32, &mut sz2);
                    GetTextExtentPoint32W(mem_dc, w_p3.as_ptr(), (w_p3.len() - 1) as i32, &mut sz3);

                    let base_x = 24;
                    let brush_x1 = base_x + sz1.cx - 4;
                    let brush_x2 = base_x + sz1.cx + sz2.cx + sz3.cx + 8;

                    let brush_outer = CreatePen(PS_SOLID, 5, rgb(0, 145, 115));
                    let old_p = SelectObject(mem_dc, brush_outer);
                    MoveToEx(mem_dc, brush_x1, 38, null_mut());
                    LineTo(mem_dc, (brush_x1 + brush_x2) / 2, 36);
                    LineTo(mem_dc, brush_x2, 37);
                    SelectObject(mem_dc, old_p);
                    DeleteObject(brush_outer);

                    let brush_inner = CreatePen(PS_SOLID, 2, rgb(70, 255, 200));
                    let old_p = SelectObject(mem_dc, brush_inner);
                    MoveToEx(mem_dc, brush_x1 + 3, 38, null_mut());
                    LineTo(mem_dc, (brush_x1 + brush_x2) / 2, 36);
                    LineTo(mem_dc, brush_x2 - 2, 37);
                    SelectObject(mem_dc, old_p);
                    DeleteObject(brush_inner);

                    SetTextColor(mem_dc, rgb(235, 240, 250));
                    let mut r1 = RECT { left: base_x, top: 10, right: base_x + sz1.cx + 2, bottom: 38 };
                    DrawTextW(mem_dc, w_p1.as_ptr(), -1, &mut r1, DT_LEFT | DT_SINGLELINE | DT_VCENTER);

                    SetTextColor(mem_dc, rgb(70, 255, 200));
                    let mut r2 = RECT { left: base_x + sz1.cx, top: 10, right: base_x + sz1.cx + sz2.cx + 2, bottom: 38 };
                    DrawTextW(mem_dc, w_p2.as_ptr(), -1, &mut r2, DT_LEFT | DT_SINGLELINE | DT_VCENTER);

                    SetTextColor(mem_dc, rgb(235, 240, 250));
                    let mut r3 = RECT { left: base_x + sz1.cx + sz2.cx, top: 10, right: base_x + sz1.cx + sz2.cx + sz3.cx + 2, bottom: 38 };
                    DrawTextW(mem_dc, w_p3.as_ptr(), -1, &mut r3, DT_LEFT | DT_SINGLELINE | DT_VCENTER);

                    // Kapatma Butonu [✕]
                    SetTextColor(mem_dc, rgb(235, 95, 95));
                    let mut close_rect = RECT { left: 485, top: 12, right: 522, bottom: 40 };
                    let close_text = to_wide("✕");
                    DrawTextW(mem_dc, close_text.as_ptr(), -1, &mut close_rect, DT_CENTER | DT_SINGLELINE | DT_VCENTER);
                    SelectObject(mem_dc, old_f);
                    DeleteObject(title_font);

                    // [🎯 Harf Öğret (T)] Buton Yazısı
                    let btn_font = CreateFontW(
                        13, 0, 0, 0, 600, 0, 0, 0, 1, 0, 0, 2, 0,
                        to_wide("Segoe UI").as_ptr(),
                    );
                    let old_f = SelectObject(mem_dc, btn_font);
                    SetTextColor(mem_dc, rgb(240, 245, 255));
                    let mut teach_rect = RECT { left: 332, top: 12, right: 476, bottom: 40 };
                    let teach_label = if app.is_teaching_mode {
                        format!("🎯 Öğretiliyor: '{}'", app.teaching_letter)
                    } else {
                        "🎯 Harf Öğret (T)".to_string()
                    };
                    let teach_wide = to_wide(&teach_label);
                    DrawTextW(mem_dc, teach_wide.as_ptr(), -1, &mut teach_rect, DT_CENTER | DT_SINGLELINE | DT_VCENTER);
                    SelectObject(mem_dc, old_f);
                    DeleteObject(btn_font);

                    // Durum Metni
                    let status_font = CreateFontW(
                        14, 0, 0, 0, 600, 0, 0, 0, 1, 0, 0, 2, 0,
                        to_wide("Segoe UI").as_ptr(),
                    );
                    let old_f = SelectObject(mem_dc, status_font);
                    let status_color = if app.status_text.contains("Tanındı") || app.status_text.contains("kaydedildi") {
                        rgb(65, 255, 135)
                    } else if app.is_teaching_mode {
                        rgb(255, 215, 80)
                    } else {
                        rgb(155, 170, 195)
                    };
                    SetTextColor(mem_dc, status_color);
                    let mut st_rect = RECT { left: 24, top: 43, right: w - 24, bottom: 65 };
                    let st_wide = to_wide(&app.status_text);
                    DrawTextW(mem_dc, st_wide.as_ptr(), -1, &mut st_rect, DT_LEFT | DT_SINGLELINE | DT_VCENTER);
                    SelectObject(mem_dc, old_f);
                    DeleteObject(status_font);

                    // Alt Bar: Aktif Kısayollar, Parmak Sayısı ve Kalan Süre
                    let foot_font = CreateFontW(
                        13, 0, 0, 0, 500, 0, 0, 0, 1, 0, 0, 2, 0,
                        to_wide("Segoe UI").as_ptr(),
                    );
                    let old_f = SelectObject(mem_dc, foot_font);
                    SetTextColor(mem_dc, rgb(120, 135, 160));
                    let mut foot_rect = RECT { left: 20, top: h - 46, right: w - 20, bottom: h - 14 };

                    let elapsed_ms = app.last_activity_time.elapsed().as_millis();
                    let rem_sec = app.idle_timeout_ms.saturating_sub(elapsed_ms).div_ceil(1000);

                    let finger_badge = if app.active_finger_count > 0 {
                        format!("🖐️ {}P | ", app.active_finger_count)
                    } else {
                        "".to_string()
                    };
                    let shortcuts: Vec<String> = app
                        .rules
                        .iter()
                        .take(3)
                        .map(|r| format!("[{}: {}]", r.letter, r.name))
                        .collect();
                    let footer_str = format!(
                        "{}{}  [ESC: Kapat ({}sn)]",
                        finger_badge,
                        shortcuts.join("  "),
                        rem_sec
                    );
                    let foot_wide = to_wide(&footer_str);
                    DrawTextW(mem_dc, foot_wide.as_ptr(), -1, &mut foot_rect, DT_CENTER | DT_SINGLELINE | DT_VCENTER);
                    SelectObject(mem_dc, old_f);
                    DeleteObject(foot_font);
                }

                BitBlt(hdc, 0, 0, w, h, mem_dc, 0, 0, 0x00CC0020);
                SelectObject(mem_dc, old_bmp);
                DeleteObject(mem_bmp);
                DeleteDC(mem_dc);

                EndPaint(hwnd, &ps);
                0
            }

            WM_LBUTTONDOWN => {
                let x = (lparam & 0xFFFF) as i16 as i32;
                let y = ((lparam >> 16) & 0xFFFF) as i16 as i32;

                if let Some(ref mut app) = GLOBAL_APP {
                    app.last_activity_time = Instant::now();
                    if x >= 482 && x <= 530 && y >= 8 && y <= 45 {
                        app.hide_hud("[✕] Butonuna Tıklandı");
                        return 0;
                    }
                    if x >= 328 && x <= 480 && y >= 8 && y <= 45 {
                        app.toggle_teaching_mode();
                        return 0;
                    }
                    if y >= 68 && y <= 390 {
                        app.mouse_left_down = true;
                        app.last_mouse_pt = None;
                        if app.virtual_points.is_empty() {
                            app.virtual_points.push((0.0, 0.0));
                        }
                    }
                }
                0
            }

            WM_LBUTTONUP => {
                if let Some(ref mut app) = GLOBAL_APP {
                    if app.mouse_left_down {
                        app.mouse_left_down = false;
                        app.last_mouse_pt = None;
                        SetTimer(hwnd, 1003, 120, null_mut());
                    }
                }
                0
            }

            // Global Kısayollar: 1 = Ctrl+Space, 2 = ESC (HUD açıkken), 3 = T (HUD açıkken)
            WM_HOTKEY => {
                if let Some(ref mut app) = GLOBAL_APP {
                    if wparam == 1 {
                        if app.is_hud_visible {
                            app.hide_hud("Ctrl+Space Kısayolu");
                        } else {
                            app.show_hud("Ctrl+Space Kısayolu");
                        }
                    } else if wparam == 2 && app.is_hud_visible {
                        app.hide_hud("ESC Tuşu (Global Hotkey)");
                    } else if wparam == 3 && app.is_hud_visible {
                        app.toggle_teaching_mode();
                    }
                }
                0
            }

            WM_INPUT => {
                let mut size: UINT = 0;
                let header_size = std::mem::size_of::<RAWINPUTHEADER>() as UINT;
                GetRawInputData(lparam as HRAWINPUT, RID_INPUT, null_mut(), &mut size, header_size);

                if size > 0 {
                    let mut buf: Vec<u8> = vec![0; size as usize];
                    let r = GetRawInputData(
                        lparam as HRAWINPUT,
                        RID_INPUT,
                        buf.as_mut_ptr() as *mut _,
                        &mut size,
                        header_size,
                    );
                    if r != u32::MAX && buf.len() >= header_size as usize {
                        let header = &*(buf.as_ptr() as *const RAWINPUTHEADER);

                        if header.dwType == RIM_TYPEHID && buf.len() >= header_size as usize + 8 {
                            let offset = header_size as usize;
                            let dw_size_hid = u32::from_le_bytes([
                                buf[offset],
                                buf[offset + 1],
                                buf[offset + 2],
                                buf[offset + 3],
                            ]) as usize;
                            let dw_count = u32::from_le_bytes([
                                buf[offset + 4],
                                buf[offset + 5],
                                buf[offset + 6],
                                buf[offset + 7],
                            ]) as usize;
                            let raw_start = offset + 8;

                            if dw_size_hid > 0 && dw_count > 0 && buf.len() >= raw_start + dw_size_hid * dw_count {
                                if let Some(ref mut app) = GLOBAL_APP {
                                    let need_init = match app.ptp_cache {
                                        Some(ref c) => c.h_device != header.hDevice,
                                        None => true,
                                    };
                                    if need_init {
                                        app.ptp_cache = PtpDeviceCache::from_device(header.hDevice);
                                    }

                                    for idx in 0..dw_count {
                                        let rep_slice = &buf[raw_start + idx * dw_size_hid
                                            ..raw_start + (idx + 1) * dw_size_hid];
                                        let contacts = if let Some(ref cache) = app.ptp_cache {
                                            cache.parse_report(rep_slice)
                                        } else {
                                            Vec::new()
                                        };
                                        app.process_ptp_contacts(&contacts);
                                    }
                                }
                            }
                        }
                    }
                }
                0
            }

            WM_TIMER => {
                if wparam == 1003 {
                    // Parmak kalktı veya hareket bitti -> Çizimi değerlendir
                    KillTimer(hwnd, 1003);
                    if let Some(ref mut app) = GLOBAL_APP {
                        app.evaluate_drawing();
                    }
                } else if wparam == 1002 {
                    // Uygulama açıldıktan sonra kapat
                    KillTimer(hwnd, 1002);
                    if let Some(ref mut app) = GLOBAL_APP {
                        app.hide_hud("Harf Tanındı & Uygulama Başlatıldı");
                    }
                } else if wparam == 1005 {
                    // 100ms Watchdog: ESC kontrolü ve 10sn otomatik kapanma sayacı
                    if let Some(ref mut app) = GLOBAL_APP {
                        if app.is_hud_visible {
                            if (GetAsyncKeyState(VK_ESCAPE as i32) as u16 & 0x8000) != 0 {
                                app.hide_hud("ESC Tuşu (AsyncKey)");
                                return 0;
                            }
                            let elapsed = app.last_activity_time.elapsed().as_millis();
                            if elapsed >= app.idle_timeout_ms {
                                app.hide_hud(&format!("{}sn Hareketsizlik Süresi Doldu", app.idle_timeout_ms / 1000));
                                return 0;
                            }
                            InvalidateRect(hwnd, null(), 0);
                        }
                    }
                }
                0
            }

            WM_KEYDOWN => {
                let vk = wparam as u32;
                if let Some(ref mut app) = GLOBAL_APP {
                    app.last_activity_time = Instant::now();
                    if vk == VK_ESCAPE {
                        app.hide_hud("ESC Tuşu (WM_KEYDOWN)");
                    } else if vk == VK_T && !app.is_teaching_mode {
                        app.toggle_teaching_mode();
                    } else if app.is_teaching_mode && (0x41..=0x5A).contains(&vk) {
                        let ch = (vk as u8) as char;
                        app.teaching_letter = ch;
                        log_line(&format!("🎯 [HEDEF HARF SEÇİLDİ] '{}'", ch));
                        app.status_text = format!(
                            "🎯 ÖĞRETME MODU: '{}' harfi seçildi! Şimdi touchpad'de '{}' şeklini çizin.",
                            ch, ch
                        );
                        InvalidateRect(hwnd, null(), 0);
                    } else if vk == 0x20 || vk == 0x0D {
                        app.evaluate_drawing();
                    }
                }
                0
            }

            WM_TRAYICON => {
                let event = (lparam & 0xFFFF) as UINT;
                if event == WM_LBUTTONUP || event == WM_LBUTTONDBLCLK {
                    if let Some(ref mut app) = GLOBAL_APP {
                        app.show_hud("Sistem Tepsisi Sol Tık");
                    }
                } else if event == WM_RBUTTONUP {
                    let mut pt = POINT::default();
                    GetCursorPos(&mut pt);
                    let h_menu = CreatePopupMenu();
                    if !h_menu.is_null() {
                        let is_paused = GLOBAL_APP.as_ref().map(|a| a.is_paused).unwrap_or(false);
                        AppendMenuW(h_menu, MF_STRING, 101, to_wide("✨ Çizim Ekranını Aç (Ctrl+Space)").as_ptr());
                        AppendMenuW(h_menu, MF_STRING, 102, to_wide("🎯 Harf Öğret / Kalibre Et (T)").as_ptr());
                        AppendMenuW(h_menu, MF_STRING, 103, to_wide("📂 config.json Ayarlarını Aç").as_ptr());
                        AppendMenuW(h_menu, MF_SEPARATOR, 0, null());
                        let pause_txt = if is_paused {
                            "▶️ Çift Tık Algılamayı Başlat"
                        } else {
                            "⏸️ Çift Tık Algılamayı Duraklat"
                        };
                        AppendMenuW(h_menu, MF_STRING, 104, to_wide(pause_txt).as_ptr());
                        AppendMenuW(h_menu, MF_SEPARATOR, 0, null());
                        AppendMenuW(h_menu, MF_STRING, 105, to_wide("🚪 Çıkış (Tamamen Kapat)").as_ptr());

                        SetForegroundWindow(hwnd);
                        TrackPopupMenu(
                            h_menu,
                            TPM_BOTTOMALIGN | TPM_LEFTALIGN | TPM_RIGHTBUTTON,
                            pt.x,
                            pt.y,
                            0,
                            hwnd,
                            null(),
                        );
                        DestroyMenu(h_menu);
                    }
                }
                0
            }

            WM_COMMAND => {
                let cmd_id = wparam & 0xFFFF;
                match cmd_id {
                    101 => {
                        if let Some(ref mut app) = GLOBAL_APP {
                            app.show_hud("Tepsi Menüsü: Aç");
                        }
                    }
                    102 => {
                        if let Some(ref mut app) = GLOBAL_APP {
                            app.show_hud("Tepsi Menüsü: Harf Öğret");
                            if !app.is_teaching_mode {
                                app.toggle_teaching_mode();
                            }
                        }
                    }
                    103 => {
                        spawn_shell_open("notepad.exe".to_string(), "config.json".to_string());
                    }
                    104 => {
                        if let Some(ref mut app) = GLOBAL_APP {
                            app.is_paused = !app.is_paused;
                        }
                    }
                    105 => {
                        remove_tray_icon(hwnd);
                        PostQuitMessage(0);
                    }
                    _ => {}
                }
                0
            }

            WM_DESTROY => {
                remove_tray_icon(hwnd);
                if !MOUSE_HOOK.is_null() {
                    UnhookWindowsHookEx(MOUSE_HOOK);
                }
                PostQuitMessage(0);
                0
            }

            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }
}

// ============================================================================
// 6. GLOBAL LOW-LEVEL MOUSE ISOLATION & FALLBACK (WH_MOUSE_LL)
// ============================================================================
static mut MOUSE_HOOK: HANDLE = null_mut();
static mut LAST_RBUTTON_TAP: Option<Instant> = None;

unsafe extern "system" fn mouse_ll_proc(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    unsafe {
        if n_code >= 0 {
            let msg = w_param as UINT;
            let (is_open, is_paused, ptp_active) = if let Some(ref app) = GLOBAL_APP {
                let recent_ptp = app
                    .last_ptp_packet_time
                    .map(|t| t.elapsed().as_millis() < 160)
                    .unwrap_or(false);
                (
                    app.is_hud_visible,
                    app.is_paused,
                    app.active_finger_count > 0 || recent_ptp,
                )
            } else {
                (false, false, false)
            };

            if !is_open {
                if !is_paused && (msg == WM_RBUTTONDOWN || msg == WM_MBUTTONDOWN) {
                    let now = Instant::now();
                    if let Some(last) = LAST_RBUTTON_TAP {
                        let dur = now.duration_since(last).as_millis();
                        if dur >= 30 && dur <= 500 {
                            LAST_RBUTTON_TAP = None;
                            if let Some(ref mut app) = GLOBAL_APP {
                                app.show_hud("WH_MOUSE_LL 2-Parmak Çift Tık");
                            }
                            return 1;
                        }
                    }
                    LAST_RBUTTON_TAP = Some(now);
                }
            } else {
                let ms = &*(l_param as *const MSLLHOOKSTRUCT);

                if msg == WM_MOUSEWHEEL || msg == WM_MOUSEHWHEEL {
                    if !ptp_active {
                        if let Some(ref mut app) = GLOBAL_APP {
                            app.last_activity_time = Instant::now();
                            let delta = ((ms.mouseData >> 16) as i16) as f32;
                            let dx = if msg == WM_MOUSEHWHEEL { delta * 0.25 } else { 0.0 };
                            let dy = if msg == WM_MOUSEWHEEL { -delta * 0.25 } else { 0.0 };
                            if dx.hypot(dy) >= 0.5 {
                                if app.virtual_points.is_empty() {
                                    app.virtual_points.push((0.0, 0.0));
                                }
                                let (lx, ly) = app.virtual_points.last().copied().unwrap_or((0.0, 0.0));
                                app.virtual_points.push((lx + dx, ly + dy));
                                InvalidateRect(app.hud_hwnd, null(), 0);
                                SetTimer(app.hud_hwnd, 1003, 300, null_mut());
                            }
                        }
                    }
                    return 1;
                }

                if msg == WM_RBUTTONDOWN || msg == WM_RBUTTONUP {
                    return 1;
                }

                if ptp_active && msg == WM_MOUSEMOVE {
                    return 1;
                }

                if !ptp_active && msg == WM_MOUSEMOVE {
                    if let Some(ref mut app) = GLOBAL_APP {
                        if app.mouse_left_down {
                            if let Some(prev) = app.last_mouse_pt {
                                let dx = (ms.pt.x - prev.x) as f32 * 0.80;
                                let dy = (ms.pt.y - prev.y) as f32 * 0.80;
                                if dx.hypot(dy) >= 1.2 && dx.hypot(dy) < 180.0 {
                                    app.last_activity_time = Instant::now();
                                    if app.virtual_points.is_empty() {
                                        app.virtual_points.push((0.0, 0.0));
                                    }
                                    let (lx, ly) = app.virtual_points.last().copied().unwrap_or((0.0, 0.0));
                                    app.virtual_points.push((lx + dx, ly + dy));
                                    InvalidateRect(app.hud_hwnd, null(), 0);
                                }
                            }
                            app.last_mouse_pt = Some(ms.pt);
                        } else {
                            app.last_mouse_pt = None;
                        }
                    }
                }
            }
        }
        CallNextHookEx(null_mut(), n_code, w_param, l_param)
    }
}

// ============================================================================
// 7. SYSTEM TRAY ICON HELPER
// ============================================================================
fn add_tray_icon(hwnd: HWND) {
    unsafe {
        let mut nid: NOTIFYICONDATAW = std::mem::zeroed();
        nid.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
        nid.hWnd = hwnd;
        nid.uID = 1;
        nid.uFlags = NIF_MESSAGE | NIF_ICON | NIF_TIP | NIF_INFO;
        nid.uCallbackMessage = WM_TRAYICON;
        nid.hIcon = LoadIconW(null_mut(), 32512 as *const u16);
        nid.dwInfoFlags = NIIF_INFO;

        let tip = to_wide("touchpa(inte)d — 2 Parmak Çift Dokunuş veya Ctrl+Space");
        for (i, &ch) in tip.iter().take(127).enumerate() {
            nid.szTip[i] = ch;
        }
        let info_title = to_wide("🎨 touchpa(inte)d Hazır");
        for (i, &ch) in info_title.iter().take(63).enumerate() {
            nid.szInfoTitle[i] = ch;
        }
        let info_body = to_wide("Touchpad'de 2 parmak çift dokunuş (Tık-Tık) veya Ctrl+Space ile harf çizebilirsiniz.");
        for (i, &ch) in info_body.iter().take(255).enumerate() {
            nid.szInfo[i] = ch;
        }
        Shell_NotifyIconW(NIM_ADD, &nid);
    }
}

fn remove_tray_icon(hwnd: HWND) {
    unsafe {
        let mut nid: NOTIFYICONDATAW = std::mem::zeroed();
        nid.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
        nid.hWnd = hwnd;
        nid.uID = 1;
        Shell_NotifyIconW(NIM_DELETE, &nid);
    }
}

// ============================================================================
// 8. MAIN ENTRYPOINT
// ============================================================================
fn main() {
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(dir) = exe_path.parent() {
            let _ = std::env::set_current_dir(dir);
        }
    }

    let mutex_name = to_wide("Local\\TouchPainted_SingleInstance_Mutex");
    let _instance_mutex = unsafe { CreateMutexW(null_mut(), 1, mutex_name.as_ptr()) };
    if !_instance_mutex.is_null() && unsafe { GetLastError() } == ERROR_ALREADY_EXISTS {
        let hud_class = to_wide("TouchPaintedHUD");
        let existing_hwnd = unsafe { FindWindowW(hud_class.as_ptr(), null()) };
        if !existing_hwnd.is_null() {
            unsafe { PostMessageW(existing_hwnd, WM_COMMAND, 101, 0); }
        }
        return;
    }

    let enable_log = std::path::Path::new("touchpad_log.txt").exists()
        || std::env::args().any(|a| a == "--log");
    if enable_log {
        unsafe { LOG_TO_FILE = true; }
        let _ = fs::write(
            "touchpad_log.txt",
            "=== TOUCHPAINTED CANLI LOG ===\n",
        );
    }
    log_line("🚀 Uygulama başlatıldı (Rust, Doğrudan PTP 0x000D:0x0005 + Geometrik & $1 Tanıyıcı)");

    let h_instance = unsafe { GetModuleHandleW(null()) };

    let screen_w = unsafe { GetSystemMetrics(0) };
    let screen_h = unsafe { GetSystemMetrics(1) };
    let win_w = 540;
    let win_h = 450;
    let pos_x = (screen_w - win_w) / 2;
    let pos_y = (screen_h - win_h) / 2;

    let mut app = AppState::new(pos_x, pos_y, win_w, win_h);

    let hud_class = to_wide("TouchPaintedHUD");
    let wc_hud = WNDCLASSEXW {
        cbSize: std::mem::size_of::<WNDCLASSEXW>() as UINT,
        style: 0,
        lpfnWndProc: hud_wnd_proc,
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: h_instance,
        hIcon: unsafe { LoadIconW(null_mut(), 32512 as *const u16) },
        hCursor: unsafe { LoadCursorW(null_mut(), 32512 as *const u16) },
        hbrBackground: null_mut(),
        lpszMenuName: null(),
        lpszClassName: hud_class.as_ptr(),
        hIconSm: null_mut(),
    };
    unsafe { RegisterClassExW(&wc_hud); }

    let hud_hwnd = unsafe {
        CreateWindowExW(
            WS_EX_TOPMOST | WS_EX_TOOLWINDOW,
            hud_class.as_ptr(),
            to_wide("touchpainted").as_ptr(),
            WS_POPUP,
            pos_x,
            pos_y,
            win_w,
            win_h,
            null_mut(),
            null_mut(),
            h_instance,
            null_mut(),
        )
    };

    app.hud_hwnd = hud_hwnd;

    unsafe {
        RegisterHotKey(hud_hwnd, 1, MOD_CONTROL, VK_SPACE);
        MOUSE_HOOK = SetWindowsHookExW(WH_MOUSE_LL, mouse_ll_proc, null_mut(), 0);

        let rids = [RAWINPUTDEVICE {
            usUsagePage: 0x000D,
            usUsage: 0x0005,
            dwFlags: RIDEV_INPUTSINK | RIDEV_DEVNOTIFY,
            hwndTarget: hud_hwnd,
        }];
        let reg_ok = RegisterRawInputDevices(
            rids.as_ptr(),
            rids.len() as UINT,
            std::mem::size_of::<RAWINPUTDEVICE>() as UINT,
        );
        log_line(&format!("📡 RegisterRawInputDevices sonucu: {}", reg_ok));

        add_tray_icon(hud_hwnd);
        GLOBAL_APP = Some(app);
    }

    if let Some(a) = unsafe { &mut GLOBAL_APP } {
        a.show_hud("İlk Başlangıç Hoş Geldin Ekranı (4sn)");
    }

    let mut msg: MSG = unsafe { std::mem::zeroed() };
    loop {
        let b_ret = unsafe { GetMessageW(&mut msg, null_mut(), 0, 0) };
        if b_ret == 0 || b_ret == -1 {
            break;
        }
        unsafe {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}
