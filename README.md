# Retrieve data from current window

What to retrieve:

- Window title
- Exec path
- Window class (window name)

## Error handling packages

- `anyhow`: Use when don't care about error type, you just want it to be easy. This is common in application-like code.
- `thiserror`: Use when you care about designing your own dedicated error type(s) so that the caller receives exactly the information that you choose in the event of failure. This most often applies to library-like code.

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


