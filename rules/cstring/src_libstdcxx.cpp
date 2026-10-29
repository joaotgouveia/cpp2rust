// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cstddef>
#include <cwchar>

wchar_t *f2011(const wchar_t *a0, wchar_t a1) { return ::wcschr(a0, a1); }

wchar_t *f2012(const wchar_t *a0, wchar_t a1) { return ::wcsrchr(a0, a1); }

wchar_t *f2013(const wchar_t *a0, const wchar_t *a1) { return ::wcspbrk(a0, a1); }

wchar_t *f2014(const wchar_t *a0, const wchar_t *a1) { return ::wcsstr(a0, a1); }

wchar_t *f2015(const wchar_t *a0, wchar_t a1, std::size_t a2) {
  return ::wmemchr(a0, a1, a2);
}
