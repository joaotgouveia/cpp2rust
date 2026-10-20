#include "../common/init.h"
#include "../common/traits.h"
#include <cwchar>
#include <cwctype>
#include <stdlib.h>
#include <utility>

int f1(wint_t a0) { return iswalnum(a0); }

int f2(wint_t a0, locale_t a1) { return iswalnum_l(a0, a1); }

int f3(wint_t a0) { return iswalpha(a0); }

int f4(wint_t a0, locale_t a1) { return iswalpha_l(a0, a1); }

int f5(wint_t a0) { return iswblank(a0); }

int f6(wint_t a0, locale_t a1) { return iswblank_l(a0, a1); }

int f7(wint_t a0) { return iswcntrl(a0); }

int f8(wint_t a0, locale_t a1) { return iswcntrl_l(a0, a1); }

int f9(wint_t a0, wctype_t a1) { return iswctype(a0, a1); }

int f10(wint_t a0, wctype_t a1, locale_t a2) { return iswctype_l(a0, a1, a2); }

int f11(wint_t a0) { return iswdigit(a0); }

int f12(wint_t a0, locale_t a1) { return iswdigit_l(a0, a1); }

int f13(wint_t a0) { return iswgraph(a0); }

int f14(wint_t a0, locale_t a1) { return iswgraph_l(a0, a1); }

int f15(wint_t a0) { return iswlower(a0); }

int f16(wint_t a0, locale_t a1) { return iswlower_l(a0, a1); }

int f17(wint_t a0) { return iswprint(a0); }

int f18(wint_t a0, locale_t a1) { return iswprint_l(a0, a1); }

int f19(wint_t a0) { return iswpunct(a0); }

int f20(wint_t a0, locale_t a1) { return iswpunct_l(a0, a1); }

int f21(wint_t a0) { return iswspace(a0); }

int f22(wint_t a0, locale_t a1) { return iswspace_l(a0, a1); }

int f23(wint_t a0) { return iswupper(a0); }

int f24(wint_t a0, locale_t a1) { return iswupper_l(a0, a1); }

int f25(wint_t a0) { return iswxdigit(a0); }

int f26(wint_t a0, locale_t a1) { return iswxdigit_l(a0, a1); }

wint_t f27(wint_t a0, wctrans_t a1) { return towctrans(a0, a1); }

wint_t f28(wint_t a0) { return towlower(a0); }

wint_t f29(wint_t a0) { return towupper(a0); }

wctrans_t f30(const char *a0) { return wctrans(a0); }

using t1 = wctrans_t;

wctype_t f31(const char *a0) { return wctype(a0); }

using t2 = wctype_t;

wint_t f32(wint_t a0, wctrans_t a1, locale_t a2) {
  return towctrans_l(a0, a1, a2);
}

wint_t f33(wint_t a0, locale_t a1) { return towlower_l(a0, a1); }

wint_t f34(wint_t a0, locale_t a1) { return towupper_l(a0, a1); }

wctrans_t f35(const char *a0, locale_t a1) { return wctrans_l(a0, a1); }

wctype_t f36(const char *a0, locale_t a1) { return wctype_l(a0, a1); }
