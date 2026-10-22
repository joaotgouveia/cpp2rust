#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <cwchar>
#include <utility>

wchar_t *f1(wchar_t *a0, wchar_t a1) { return std::wcschr(a0, a1); }

wchar_t *f2(wchar_t *a0, const wchar_t *a1) { return std::wcspbrk(a0, a1); }

wchar_t *f3(wchar_t *a0, wchar_t a1) { return std::wcsrchr(a0, a1); }

wchar_t *f4(wchar_t *a0, const wchar_t *a1) { return std::wcsstr(a0, a1); }

wchar_t *f5(wchar_t *a0, wchar_t a1, std::size_t a2) {
  return std::wmemchr(a0, a1, a2);
}
