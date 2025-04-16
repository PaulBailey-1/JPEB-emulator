# Calling Conventions

The bcc compiler uses `r1` as the stack pointer and `r2` as the base pointer, so they are callee-saved.  
Everything else is caller saved.

Return values should be placed in `r3`.

When the compiler calls a function, it places the return address in `r7`.  
If your function plans to return to the caller, it should be careful to not lose this value.

If you keep the return address in `r7`, you can return to the caller with `jalr r7, r7`
