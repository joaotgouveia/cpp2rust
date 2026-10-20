#include "../common/init.h"
#include "../common/traits.h"
#include <fenv.h>
#include <utility>

int f1(int a0) { return feclearexcept(a0); }

int f2(int a0) { return fedisableexcept(a0); }

int f3(int a0) { return feenableexcept(a0); }

int f4(fenv_t *a0) { return fegetenv(a0); }

int f5() { return fegetexcept(); }

int f6(fexcept_t *a0, int a1) { return fegetexceptflag(a0, a1); }

int f7(femode_t *a0) { return fegetmode(a0); }

int f8() { return fegetround(); }

int f9(fenv_t *a0) { return feholdexcept(a0); }

using t1 = femode_t;

using t2 = fenv_t;

int f10(int a0) { return feraiseexcept(a0); }

int f11(const fenv_t *a0) { return fesetenv(a0); }

int f12(int a0) { return fesetexcept(a0); }

int f13(const fexcept_t *a0, int a1) { return fesetexceptflag(a0, a1); }

int f14(const femode_t *a0) { return fesetmode(a0); }

int f15(int a0) { return fesetround(a0); }

int f16(int a0) { return fetestexcept(a0); }

int f17(const fexcept_t *a0, int a1) { return fetestexceptflag(a0, a1); }

int f18(const fenv_t *a0) { return feupdateenv(a0); }

using t3 = fexcept_t;
