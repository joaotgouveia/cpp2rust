#include <codecvt>
#include <cstddef>
#include <ios>
#include <utility>

std::codecvt_base::result
f48(const std::codecvt<char, char, std::mbstate_t> &a0,
    std::codecvt<char, char, __mbstate_t>::state_type &a1,
    const std::codecvt<char, char, __mbstate_t>::intern_type *a2,
    const std::codecvt<char, char, __mbstate_t>::intern_type *a3,
    const std::codecvt<char, char, __mbstate_t>::intern_type *&a4,
    std::codecvt<char, char, __mbstate_t>::extern_type *a5,
    std::codecvt<char, char, __mbstate_t>::extern_type *a6,
    std::codecvt<char, char, __mbstate_t>::extern_type *&a7) {
  return a0.out(a1, a2, a3, a4, a5, a6, a7);
}

std::codecvt_base::result
f49(const std::codecvt<char, char, std::mbstate_t> &a0,
    std::codecvt<char, char, __mbstate_t>::state_type &a1,
    std::codecvt<char, char, __mbstate_t>::extern_type *a2,
    std::codecvt<char, char, __mbstate_t>::extern_type *a3,
    std::codecvt<char, char, __mbstate_t>::extern_type *&a4) {
  return a0.unshift(a1, a2, a3, a4);
}

std::codecvt_base::result
f50(const std::codecvt<char, char, std::mbstate_t> &a0,
    std::codecvt<char, char, __mbstate_t>::state_type &a1,
    const std::codecvt<char, char, __mbstate_t>::extern_type *a2,
    const std::codecvt<char, char, __mbstate_t>::extern_type *a3,
    const std::codecvt<char, char, __mbstate_t>::extern_type *&a4,
    std::codecvt<char, char, __mbstate_t>::intern_type *a5,
    std::codecvt<char, char, __mbstate_t>::intern_type *a6,
    std::codecvt<char, char, __mbstate_t>::intern_type *&a7) {
  return a0.in(a1, a2, a3, a4, a5, a6, a7);
}

int f51(const std::codecvt<char, char, std::mbstate_t> &a0,
        std::codecvt<char, char, __mbstate_t>::state_type &a1,
        const std::codecvt<char, char, __mbstate_t>::extern_type *a2,
        const std::codecvt<char, char, __mbstate_t>::extern_type *a3,
        std::size_t a4) {
  return a0.length(a1, a2, a3, a4);
}

long f52(const std::codecvt<char, char, std::mbstate_t> &a0) {
  return a0.use_count();
}

std::codecvt_base::result
f53(const std::codecvt<wchar_t, char, std::mbstate_t> &a0,
    std::codecvt<wchar_t, char, __mbstate_t>::state_type &a1,
    const std::codecvt<wchar_t, char, __mbstate_t>::intern_type *a2,
    const std::codecvt<wchar_t, char, __mbstate_t>::intern_type *a3,
    const std::codecvt<wchar_t, char, __mbstate_t>::intern_type *&a4,
    std::codecvt<wchar_t, char, __mbstate_t>::extern_type *a5,
    std::codecvt<wchar_t, char, __mbstate_t>::extern_type *a6,
    std::codecvt<wchar_t, char, __mbstate_t>::extern_type *&a7) {
  return a0.out(a1, a2, a3, a4, a5, a6, a7);
}

std::codecvt_base::result
f54(const std::codecvt<wchar_t, char, std::mbstate_t> &a0,
    std::codecvt<wchar_t, char, __mbstate_t>::state_type &a1,
    std::codecvt<wchar_t, char, __mbstate_t>::extern_type *a2,
    std::codecvt<wchar_t, char, __mbstate_t>::extern_type *a3,
    std::codecvt<wchar_t, char, __mbstate_t>::extern_type *&a4) {
  return a0.unshift(a1, a2, a3, a4);
}

std::codecvt_base::result
f55(const std::codecvt<wchar_t, char, std::mbstate_t> &a0,
    std::codecvt<wchar_t, char, __mbstate_t>::state_type &a1,
    const std::codecvt<wchar_t, char, __mbstate_t>::extern_type *a2,
    const std::codecvt<wchar_t, char, __mbstate_t>::extern_type *a3,
    const std::codecvt<wchar_t, char, __mbstate_t>::extern_type *&a4,
    std::codecvt<wchar_t, char, __mbstate_t>::intern_type *a5,
    std::codecvt<wchar_t, char, __mbstate_t>::intern_type *a6,
    std::codecvt<wchar_t, char, __mbstate_t>::intern_type *&a7) {
  return a0.in(a1, a2, a3, a4, a5, a6, a7);
}

int f56(const std::codecvt<wchar_t, char, std::mbstate_t> &a0,
        std::codecvt<wchar_t, char, __mbstate_t>::state_type &a1,
        const std::codecvt<wchar_t, char, __mbstate_t>::extern_type *a2,
        const std::codecvt<wchar_t, char, __mbstate_t>::extern_type *a3,
        std::size_t a4) {
  return a0.length(a1, a2, a3, a4);
}

long f57(const std::codecvt<wchar_t, char, std::mbstate_t> &a0) {
  return a0.use_count();
}

std::codecvt_base::result
f58(const std::codecvt<char16_t, char, std::mbstate_t> &a0,
    std::codecvt<char16_t, char, __mbstate_t>::state_type &a1,
    const std::codecvt<char16_t, char, __mbstate_t>::intern_type *a2,
    const std::codecvt<char16_t, char, __mbstate_t>::intern_type *a3,
    const std::codecvt<char16_t, char, __mbstate_t>::intern_type *&a4,
    std::codecvt<char16_t, char, __mbstate_t>::extern_type *a5,
    std::codecvt<char16_t, char, __mbstate_t>::extern_type *a6,
    std::codecvt<char16_t, char, __mbstate_t>::extern_type *&a7) {
  return a0.out(a1, a2, a3, a4, a5, a6, a7);
}

std::codecvt_base::result
f59(const std::codecvt<char16_t, char, std::mbstate_t> &a0,
    std::codecvt<char16_t, char, __mbstate_t>::state_type &a1,
    std::codecvt<char16_t, char, __mbstate_t>::extern_type *a2,
    std::codecvt<char16_t, char, __mbstate_t>::extern_type *a3,
    std::codecvt<char16_t, char, __mbstate_t>::extern_type *&a4) {
  return a0.unshift(a1, a2, a3, a4);
}

std::codecvt_base::result
f60(const std::codecvt<char16_t, char, std::mbstate_t> &a0,
    std::codecvt<char16_t, char, __mbstate_t>::state_type &a1,
    const std::codecvt<char16_t, char, __mbstate_t>::extern_type *a2,
    const std::codecvt<char16_t, char, __mbstate_t>::extern_type *a3,
    const std::codecvt<char16_t, char, __mbstate_t>::extern_type *&a4,
    std::codecvt<char16_t, char, __mbstate_t>::intern_type *a5,
    std::codecvt<char16_t, char, __mbstate_t>::intern_type *a6,
    std::codecvt<char16_t, char, __mbstate_t>::intern_type *&a7) {
  return a0.in(a1, a2, a3, a4, a5, a6, a7);
}

int f61(const std::codecvt<char16_t, char, std::mbstate_t> &a0,
        std::codecvt<char16_t, char, __mbstate_t>::state_type &a1,
        const std::codecvt<char16_t, char, __mbstate_t>::extern_type *a2,
        const std::codecvt<char16_t, char, __mbstate_t>::extern_type *a3,
        std::size_t a4) {
  return a0.length(a1, a2, a3, a4);
}

long f62(const std::codecvt<char16_t, char, std::mbstate_t> &a0) {
  return a0.use_count();
}

std::codecvt_base::result
f63(const std::codecvt<char32_t, char, std::mbstate_t> &a0,
    std::codecvt<char32_t, char, __mbstate_t>::state_type &a1,
    const std::codecvt<char32_t, char, __mbstate_t>::intern_type *a2,
    const std::codecvt<char32_t, char, __mbstate_t>::intern_type *a3,
    const std::codecvt<char32_t, char, __mbstate_t>::intern_type *&a4,
    std::codecvt<char32_t, char, __mbstate_t>::extern_type *a5,
    std::codecvt<char32_t, char, __mbstate_t>::extern_type *a6,
    std::codecvt<char32_t, char, __mbstate_t>::extern_type *&a7) {
  return a0.out(a1, a2, a3, a4, a5, a6, a7);
}

std::codecvt_base::result
f64(const std::codecvt<char32_t, char, std::mbstate_t> &a0,
    std::codecvt<char32_t, char, __mbstate_t>::state_type &a1,
    std::codecvt<char32_t, char, __mbstate_t>::extern_type *a2,
    std::codecvt<char32_t, char, __mbstate_t>::extern_type *a3,
    std::codecvt<char32_t, char, __mbstate_t>::extern_type *&a4) {
  return a0.unshift(a1, a2, a3, a4);
}

std::codecvt_base::result
f65(const std::codecvt<char32_t, char, std::mbstate_t> &a0,
    std::codecvt<char32_t, char, __mbstate_t>::state_type &a1,
    const std::codecvt<char32_t, char, __mbstate_t>::extern_type *a2,
    const std::codecvt<char32_t, char, __mbstate_t>::extern_type *a3,
    const std::codecvt<char32_t, char, __mbstate_t>::extern_type *&a4,
    std::codecvt<char32_t, char, __mbstate_t>::intern_type *a5,
    std::codecvt<char32_t, char, __mbstate_t>::intern_type *a6,
    std::codecvt<char32_t, char, __mbstate_t>::intern_type *&a7) {
  return a0.in(a1, a2, a3, a4, a5, a6, a7);
}

int f66(const std::codecvt<char32_t, char, std::mbstate_t> &a0,
        std::codecvt<char32_t, char, __mbstate_t>::state_type &a1,
        const std::codecvt<char32_t, char, __mbstate_t>::extern_type *a2,
        const std::codecvt<char32_t, char, __mbstate_t>::extern_type *a3,
        std::size_t a4) {
  return a0.length(a1, a2, a3, a4);
}

long f67(const std::codecvt<char32_t, char, std::mbstate_t> &a0) {
  return a0.use_count();
}
