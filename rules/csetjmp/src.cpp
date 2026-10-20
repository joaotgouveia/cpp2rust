#include "../common/init.h"
#include "../common/traits.h"
#include <csetjmp>
#include <utility>

using t1 = jmp_buf;

int f1(jmp_buf a0) { return setjmp(a0); }

using t2 = sigjmp_buf;

void f2(sigjmp_buf a0, int a1) { return siglongjmp(a0, a1); }
