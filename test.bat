@echo off
if "%1" == "--release" (
  set TARGET=release
) else (
  set TARGET=debug
)
cls & cargo build %* & gdb --quiet target\%TARGET%\rjs_jit.exe < gdbscript.txt
