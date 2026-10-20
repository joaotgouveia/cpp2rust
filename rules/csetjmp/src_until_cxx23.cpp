#include "../common/init.h"
#include "../common/traits.h"
#include <csetjmp>
#include <utility>

void f3(jmp_buf a0, int a1) { return longjmp(a0, a1); }

void f4(jmp_buf a0, int a1) { return longjmp(a0, a1); }
