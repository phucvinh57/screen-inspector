# Retrieve data from current window

What to retrieve:

- Window title
- Exec path
- Window class (window name)
- URL of browsers (not supported yet)

## Linux

- Get window raw id: `xprop -root _NET_ACTIVE_WINDOW` -> Result `_NET_ACTIVE_WINDOW(WINDOW): window id # 0xa00003`
- Get window id:
  - Get first hex number `0x1a0003`
  - Parse to int -> `10485763`
- Get window information by id: `LC_ALL=C.utf8 xprop -id <window_id> `. Then retrieve these information
  - WM_CLASS(STRING) : Window class (Window name)
  - WM_NAME(UTF8_STRING) | \_NET_WM_NAME(UTF8_STRING) = Window title
  - \_NET_WM_PID(CARDINAL): Window PID
- From PID, get exec path by readlink `/proc/<pid>/exe`

## Windows

- Get window id: [`GetForegroundWindow() -> HWND`](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/WindowsAndMessaging/fn.GetForegroundWindow.html)
- From window id:
  - Get PID: [`GetWindowThreadProcessId(HWND, *mut DWORD) -> BOOL`](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/WindowsAndMessaging/fn.GetWindowThreadProcessId.html)
  - Open process handle: [`OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, <PID>)`](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/fn.OpenProcess.html)
- Get exec path: [GetModuleFileNameW](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/LibraryLoader/fn.GetModuleFileNameA.html#) -> From exec path, get window name
- Get window title: [`GetWindowTextW(HWND, &title)`](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/WindowsAndMessaging/fn.GetWindowTextA.html)


## Mac

Need more research.

## About collect URLs from browsers

There are many ways:

- Access browser's shared memory/data directly. It's not only difficult to do, but also painful on different browsers.
- Use browser's extension. Extensions can communitcate to a process by [native messaging](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/Native_messaging) or via calling HTTP request to a private network (usually `localhost`). It's easy to do, but it's quite annoying to install extension for each browser for users.
- Catch browser's network requests, then parse URLs. It's crazy to do, exhausted parsing, mapping data and it's not reliable
