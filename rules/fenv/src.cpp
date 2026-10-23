#include <fenv.h>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

int f1(int a0) { return feclearexcept(a0); }

int f2(fenv_t *a0) { return fegetenv(a0); }

int f3(fexcept_t *a0, int a1) { return fegetexceptflag(a0, a1); }

int f4() { return fegetround(); }

int f5(fenv_t *a0) { return feholdexcept(a0); }

using t1 = fenv_t;

int f6(int a0) { return feraiseexcept(a0); }

int f7(const fenv_t *a0) { return fesetenv(a0); }

int f8(const fexcept_t *a0, int a1) { return fesetexceptflag(a0, a1); }

int f9(int a0) { return fesetround(a0); }

int f10(int a0) { return fetestexcept(a0); }

int f11(const fenv_t *a0) { return feupdateenv(a0); }

using t2 = fexcept_t;
