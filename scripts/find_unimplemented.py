#!/usr/bin/env python3
"""List raylib + raygui FFI functions that aren't yet wrapped in the safe layer.

Scans `raylib-sys/raylib/src/raylib.h` and `raylib-sys/binding/raygui.h` for
`RLAPI`/`RAYGUIAPI` function declarations, then checks whether each function
name appears anywhere under `raylib/src/core/` or `raylib/src/rgui/`. Anything
that doesn't appear — and isn't in the hard-coded `wont_impl` list below — is
printed as a `- [ ] Name` line, ready to paste into a tracking checklist.

Run from the repo root:

    python3 scripts/find_unimplemented.py

No dependencies beyond the Python standard library.
"""

import os

# Functions we intentionally don't wrap in the safe layer.
wont_impl = [
    # Implemented in a C shim, so the scanner doesn't see it.
    "SetTraceLogCallback",
    # UTF-8 helpers
    "GetCodepointNext",
    "GetCodepointPrevious",
    "CodepointToUTF8",
    "LoadUTF8",
    "UnloadUTF8",
    # Text helpers — Rust's std string APIs cover these.
    "TextCopy",
    "TextIsEqual",
    "TextLength",
    "TextFormat",
    "TextSubtext",
    "TextReplace",
    "TextInsert",
    "TextJoin",
    "TextSplit",
    "TextAppend",
    "TextFindIndex",
    "TextToUpper",
    "TextToLower",
    "TextToPascal",
    "TextToSnake",
    "TextToCamel",
    "TextToInteger",
    "TextToFloat",
    # File helpers — Rust's std::fs / std::path cover these.
    "LoadFileData",
    "UnloadFileData",
    "SaveFileData",
    "LoadFileText",
    "UnloadFileText",
    "SaveFileText",
    "FileExists",
    "DirectoryExists",
    "GetFileExtension",
    "GetFileName",
    "GetFileNameWithoutExt",
    "GetDirectoryPath",
    "GetPrevDirectoryPath",
    "GetWorkingDirectory",
    "MakeDirectory",
    "ChangeDirectory",
    "IsFileNameValid",
    "GetFileModTime",
    "ComputeCRC32",
    "ComputeMD5",
    "ComputeSHA1",
    # Misc
    "MemRealloc",
]


def file_find(lib, src, dest, opener):
    print("=====", lib, "=====")

    src_files = []
    for entry in os.scandir(dest):
        if entry.is_file(follow_symlinks=True):
            with open(entry.path) as f:
                src_files.append("\n".join(f.readlines()))

    with open(src) as d:
        lines = [ln for ln in d.readlines() if ln.startswith(opener)]

    for line in lines:
        func_name = (
            [f for f in line.split(" ") if "(" in f][0]
            .split("(")[0]
            .replace("*", "")
        )

        found = func_name in wont_impl or any(
            "ffi::" + func_name in body for body in src_files
        )
        if not found:
            print("- [ ] " + func_name)
    print("")


file_find("Raylib", "./raylib-sys/raylib/src/raylib.h", "./raylib/src/core/", "RLAPI")
file_find("Raygui", "./raylib-sys/binding/raygui.h", "./raylib/src/rgui", "    RAYGUIAPI")
