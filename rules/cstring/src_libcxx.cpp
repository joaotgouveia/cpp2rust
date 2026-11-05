// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cstddef>
#include <cwchar>

const wchar_t *f2011(const wchar_t *a0, wchar_t a1) { return ::wcschr(a0, a1); }

wchar_t *f2012(wchar_t *a0, wchar_t a1) { return ::wcschr(a0, a1); }

const wchar_t *f2013(const wchar_t *a0, wchar_t a1) { return ::wcsrchr(a0, a1); }

wchar_t *f2014(wchar_t *a0, wchar_t a1) { return ::wcsrchr(a0, a1); }

const wchar_t *f2015(const wchar_t *a0, const wchar_t *a1) { return ::wcspbrk(a0, a1); }

wchar_t *f2016(wchar_t *a0, const wchar_t *a1) { return ::wcspbrk(a0, a1); }

const wchar_t *f2017(const wchar_t *a0, const wchar_t *a1) { return ::wcsstr(a0, a1); }

wchar_t *f2018(wchar_t *a0, const wchar_t *a1) { return ::wcsstr(a0, a1); }

const wchar_t *f2019(const wchar_t *a0, wchar_t a1, std::size_t a2) {
  return ::wmemchr(a0, a1, a2);
}

wchar_t *f2020(wchar_t *a0, wchar_t a1, std::size_t a2) {
  return ::wmemchr(a0, a1, a2);
}
