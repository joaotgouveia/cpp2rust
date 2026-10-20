#include "../common/init.h"
#include "../common/traits.h"
#include <clocale>
#include <stdlib.h>
#include <utility>

locale_t f1(locale_t a0) { return duplocale(a0); }

void f2(locale_t a0) { return freelocale(a0); }

using t1 = struct lconv;

struct lconv *f3() { return localeconv(); }

locale_t f4(int a0, const char *a1, locale_t a2) {
  return newlocale(a0, a1, a2);
}

char *f5(int a0, const char *a1) { return setlocale(a0, a1); }

locale_t f6(locale_t a0) { return uselocale(a0); }
