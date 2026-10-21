#include "../common/init.h"
#include "../common/traits.h"
#include <cctype>
#include <stdlib.h>
#include <utility>

int f1(int a0) { return isalnum(a0); }

int f2(int a0, locale_t a1) { return isalnum_l(a0, a1); }

int f3(int a0) { return isalpha(a0); }

int f4(int a0, locale_t a1) { return isalpha_l(a0, a1); }

int f5(int a0) { return isascii(a0); }

int f6(int a0) { return isblank(a0); }

int f7(int a0, locale_t a1) { return isblank_l(a0, a1); }

int f8(int a0) { return iscntrl(a0); }

int f9(int a0, locale_t a1) { return iscntrl_l(a0, a1); }

int f10(int a0, int a1) { return isctype(a0, a1); }

int f11(int a0) { return isdigit(a0); }

int f12(int a0, locale_t a1) { return isdigit_l(a0, a1); }

int f13(int a0) { return isgraph(a0); }

int f14(int a0, locale_t a1) { return isgraph_l(a0, a1); }

int f15(int a0) { return islower(a0); }

int f16(int a0, locale_t a1) { return islower_l(a0, a1); }

int f17(int a0) { return isprint(a0); }

int f18(int a0, locale_t a1) { return isprint_l(a0, a1); }

int f19(int a0) { return ispunct(a0); }

int f20(int a0, locale_t a1) { return ispunct_l(a0, a1); }

int f21(int a0) { return isspace(a0); }

int f22(int a0, locale_t a1) { return isspace_l(a0, a1); }

int f23(int a0) { return isupper(a0); }

int f24(int a0, locale_t a1) { return isupper_l(a0, a1); }

int f25(int a0) { return isxdigit(a0); }

int f26(int a0, locale_t a1) { return isxdigit_l(a0, a1); }

int f27(int a0) { return toascii(a0); }

int f28(int a0) { return tolower(a0); }

int f29(int a0, locale_t a1) { return tolower_l(a0, a1); }

int f30(int a0) { return toupper(a0); }

int f31(int a0, locale_t a1) { return toupper_l(a0, a1); }
