#include "../common/init.h"
#include "../common/traits.h"
#include <locale>
#include <utility>

char *f1(const char *a0, const char *a1) {
  return bind_textdomain_codeset(a0, a1);
}

char *f2(const char *a0, const char *a1) { return bindtextdomain(a0, a1); }

char *f3(const char *a0, const char *a1, int a2) {
  return dcgettext(a0, a1, a2);
}

char *f4(const char *a0, const char *a1, const char *a2, unsigned long a3,
         int a4) {
  return dcngettext(a0, a1, a2, a3, a4);
}

char *f5(const char *a0, const char *a1) { return dgettext(a0, a1); }

char *f6(const char *a0, const char *a1, const char *a2, unsigned long a3) {
  return dngettext(a0, a1, a2, a3);
}

char *f7(const char *a0) { return gettext(a0); }

char *f8(const char *a0, const char *a1, unsigned long a2) {
  return ngettext(a0, a1, a2);
}

char *f9(const char *a0) { return textdomain(a0); }
