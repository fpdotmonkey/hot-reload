#include <stdio.h>
#include <unistd.h>

// This tells GCC to make a section named `.interp` and store
// `/lib64/ld-linux-x86-64.so.2` (the path of the dynamic linker) in it.
//
// (Normally it would do it itself, but since we're going to be using the
// `-shared` flag, it won't.)
const char interpreter[] __attribute__((section(".interp"))) =
  "/lib64/ld-linux-x86-64.so.2";

void
greet(const char* name)
{
  printf("Hello, %s!\n", name);
}

// Normally, we'd link with an object file that has its own entry point,
// and *then* calls `main`, but since we're using the `-shared` flag, we're
// linking to *another* object file, and we need to provide our own entry point.
//
// Unlike main, this one does not return an `int`, and we can never return from
// it, we need to call `_exit` or we'll crash.
void
entry()
{
  greet("rain");
  _exit(0);
}
