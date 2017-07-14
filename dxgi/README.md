# dxgi-rs
[![Build status](https://ci.appveyor.com/api/projects/status/btppf53dydmnjhyy?svg=true)](https://ci.appveyor.com/project/GuildMasterInfinite/dxgi-rs)

Rust wrapper around the [DirectX Graphics Interface](https://msdn.microsoft.com/en-us/library/windows/desktop/hh404534(v=vs.85).aspx).

## Note on documentation
This library closely follows the original DXGI API. Any DXGI tutorials using C++ should still apply, as does the [official documentation](https://msdn.microsoft.com/en-us/library/windows/desktop/bb205169(v=vs.85).aspx).

In other words, the documentation of this crate is **pretty sparse**, because links to the original DXGI documentation are provided.

## Features
Safe wrappers around DXGI interfaces, providing access to **all** the DXGI features while staying "close to the metal".

## Requirements
This library requires [DXGI 1.2](https://msdn.microsoft.com/en-us/library/windows/desktop/hh404490(v=vs.85).aspx) or newer.
This means [Windows 7 with the Platform Update](https://msdn.microsoft.com/en-us/library/windows/desktop/jj863687(v=vs.85).aspx) or newer (Windows Vista is **not** supported).

The library automatically uses newer DXGI features at runtime, if available.

Building this library is supported only on Rust 1.19 or newer. It may work on older compilers, but no attempts will be made to support them.
