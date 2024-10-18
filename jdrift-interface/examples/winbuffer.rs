use windows::Win32::Graphics::Gdi::{GetStockObject, PAINTSTRUCT};
use windows::Win32::Graphics::Gdi::{BLACK_BRUSH, HDC};
use windows::core::{s, w, PCWSTR};
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::UI::WindowsAndMessaging::WNDCLASSEXW;
use windows::Win32::Graphics::Gdi::{GetDC, ValidateRect, HBRUSH};
use std::ffi::c_void;
use windows::Win32::UI::WindowsAndMessaging::RegisterClassExW;
use windows::Win32::UI::WindowsAndMessaging::CreateWindowExW;
use windows::Win32::UI::WindowsAndMessaging::WINDOW_EX_STYLE;
use windows::Win32::UI::WindowsAndMessaging::WINDOW_STYLE;
use windows::Win32::UI::WindowsAndMessaging::ShowWindow;
use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::Foundation::WPARAM;
use windows::Win32::Foundation::LPARAM;
use windows::Win32::Foundation::LRESULT;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::Foundation::GetLastError;
use windows::Win32::UI::WindowsAndMessaging::{WS_OVERLAPPEDWINDOW, WS_VISIBLE};
use std::ptr::{null, null_mut};
use windows::Win32::UI::WindowsAndMessaging::DefWindowProcW;
use windows::Win32::UI::WindowsAndMessaging::WS_EX_WINDOWEDGE;
use windows::Win32::UI::WindowsAndMessaging::SetWindowLongW;
use windows::Win32::UI::WindowsAndMessaging::GWL_STYLE;
use windows::Win32::UI::WindowsAndMessaging::WNDCLASS_STYLES;
use windows::Win32::UI::Controls::{BP_PAINTPARAMS, MARGINS};
use windows::Win32::Graphics::Dwm::DwmExtendFrameIntoClientArea;
use windows::Win32::UI::WindowsAndMessaging::NCCALCSIZE_PARAMS;
use windows::Win32::UI::Controls::BP_PAINTPARAMS_FLAGS;
use windows::Win32::UI::Controls::BeginBufferedPaint;
use windows::Win32::UI::Controls::BP_BUFFERFORMAT;
use windows::Win32::UI::WindowsAndMessaging::GetWindowRect;
use windows::Win32::UI::WindowsAndMessaging::SetWindowPos;
use windows::Win32::UI::WindowsAndMessaging::GetWindowLongW;
use windows::Win32::Graphics::Gdi::BeginPaint;
use windows::Win32::UI::Controls::{BPPF_NOCLIP, BPPF_ERASE, BPBF_TOPDOWNDIB};
use windows::Win32::Graphics::Gdi::CreateSolidBrush;
use windows::Win32::Graphics::Gdi::FillRect;
use windows::Win32::Graphics::Gdi::DeleteObject;
use windows::Win32::UI::Controls::BufferedPaintSetAlpha;
use windows::Win32::UI::Controls::EndBufferedPaint;
use windows::Win32::Graphics::Gdi::EndPaint;

unsafe extern "system" fn proc(hwnd: HWND, umsg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match umsg {
        WM_NCCALCSIZE => {
            let ptr = lparam.0 as *mut NCCALCSIZE_PARAMS;
            let cond = wparam.0 == 1;

            if cond {
                dbg!(*ptr);
            }

            let r = DefWindowProcW(hwnd, umsg, wparam, lparam);
            dbg!(*ptr);
            LRESULT(0)
        },
        WM_CREATE => {
            let mut rect = RECT { ..Default::default() };
            GetWindowRect(hwnd, &mut rect);

            SetWindowPos(hwnd, None, rect.left, rect.top, rect.right - rect.left, rect.bottom - rect.top, SWP_FRAMECHANGED);
            LRESULT(0)
        },
        // WM_NCPAINT => {
        //     let dc = GetDC(hwnd);
        //     let rect = RECT {
        //         top: 10,
        //         left: 10,
        //         right: 10,
        //         bottom: 10
        //     };
        //     let params = BP_PAINTPARAMS {
        //         cbSize: size_of::<BP_PAINTPARAMS>() as u32,
        //         dwFlags: BP_PAINTPARAMS_FLAGS(0),
        //         prcExclude: null(),
        //         pBlendFunction: null()
        //     };
        //     let hdc = null_mut();
        //     BeginBufferedPaint(dc, &rect, BP_BUFFERFORMAT(0), Some(&params), hdc);
        //     LRESULT(0)
        // },
        WM_MOUSEHWHEEL => {
            // dbg!(GET_WHEEL_DELTA_WPARAM(wparam));
            LRESULT(0)
        },
        WM_PAINT => {
            let mut ps = PAINTSTRUCT { ..Default::default() };
            let painter = BeginPaint(hwnd, &mut ps);
            let mut rc = ps.rcPaint;
            let params = BP_PAINTPARAMS {
                cbSize: size_of::<BP_PAINTPARAMS>() as u32,
                dwFlags: BP_PAINTPARAMS_FLAGS(BPPF_NOCLIP.0 | BPPF_ERASE.0),
                ..Default::default()
            };
            let mem = HDC { ..Default::default() };
            let buffered = BeginBufferedPaint(ps, &mut rc, BPBF_TOPDOWNDIB, &params, &mem);
            let brush = CreateSolidBrush(0x0000FFFF);
            FillRect(ps, &mut rc, brush);
            DeleteObject(brush);
            BufferedPaintSetAlpha(buffered, &mut rc, 255);
            EndBufferedPaint(buffered, true);
            EndPaint(hwnd, &ps);

            LRESULT(0)
        },
        // WM_MOUSEACTIVATE => {
        //     LRESULT(0)
        // },
        // WM_NCHITTEST => {
        //     LRESULT(0)
        // },
        _ => DefWindowProcW(hwnd, umsg, wparam, lparam)
    }
}

unsafe fn main_unsafe() {
    let instance = GetModuleHandleW(None).unwrap().into();
    let class_name = w!("WINDOW_CLASS_QUARTZ");
    let class = WNDCLASSEXW {
        cbSize: size_of::<WNDCLASSEXW>() as u32,
        style: WNDCLASS_STYLES(0),
        lpfnWndProc: Some(proc),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: instance,
        hIcon: HICON(null_mut()),
        hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),
        hbrBackground: HBRUSH(GetStockObject(BLACK_BRUSH).0),
        lpszMenuName: PCWSTR(null()),
        lpszClassName: class_name,
        hIconSm: HICON(null_mut())
    };

    let class_result = RegisterClassExW(&class);
    if class_result == 0 {
        println!("failed to make class");
        dbg!(GetLastError());
        return;
    }

    let hwnd = CreateWindowExW(
        WINDOW_EX_STYLE::default(),
        class_name,
        w!("hello world"),
        WS_THICKFRAME,
        100,
        100,
        500,
        300,
        None,
        None,
        instance,
        None
    ).unwrap();

    let mut ws = GetWindowLongW(hwnd, GWL_STYLE);
    ws = ws & !(WS_CAPTION.0 as i32);
    SetWindowLongW(hwnd, GWL_STYLE, ws);
    ShowWindow(hwnd, SHOW_WINDOW_CMD(5));

    let client = MARGINS {
        cxLeftWidth: 0,
        cxRightWidth: 0,
        cyTopHeight: 0,
        cyBottomHeight: 0
    };

    DwmExtendFrameIntoClientArea(hwnd, &client);

    let mut message = MSG::default();

    while GetMessageW(&mut message, None, 0, 0).into() {
        match message.message {
            WM_PAINT => {
                println!("painting")
            },
            _ => {}
        };
        DispatchMessageW(&message);
    }
}

fn main() {
    unsafe { main_unsafe() };
}