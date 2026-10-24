#include <codecvt>
#include <cstddef>
#include <ios>
#include <utility>

std::codecvt_base::result
f90(const std::codecvt<char16_t, char8_t, std::mbstate_t> &a0,
    std::codecvt<char16_t, char8_t, __mbstate_t>::state_type &a1,
    const std::codecvt<char16_t, char8_t, __mbstate_t>::intern_type *a2,
    const std::codecvt<char16_t, char8_t, __mbstate_t>::intern_type *a3,
    const std::codecvt<char16_t, char8_t, __mbstate_t>::intern_type *&a4,
    std::codecvt<char16_t, char8_t, __mbstate_t>::extern_type *a5,
    std::codecvt<char16_t, char8_t, __mbstate_t>::extern_type *a6,
    std::codecvt<char16_t, char8_t, __mbstate_t>::extern_type *&a7) {
  return a0.out(a1, a2, a3, a4, a5, a6, a7);
}

std::codecvt_base::result
f91(const std::codecvt<char16_t, char8_t, std::mbstate_t> &a0,
    std::codecvt<char16_t, char8_t, __mbstate_t>::state_type &a1,
    std::codecvt<char16_t, char8_t, __mbstate_t>::extern_type *a2,
    std::codecvt<char16_t, char8_t, __mbstate_t>::extern_type *a3,
    std::codecvt<char16_t, char8_t, __mbstate_t>::extern_type *&a4) {
  return a0.unshift(a1, a2, a3, a4);
}

std::codecvt_base::result
f92(const std::codecvt<char16_t, char8_t, std::mbstate_t> &a0,
    std::codecvt<char16_t, char8_t, __mbstate_t>::state_type &a1,
    const std::codecvt<char16_t, char8_t, __mbstate_t>::extern_type *a2,
    const std::codecvt<char16_t, char8_t, __mbstate_t>::extern_type *a3,
    const std::codecvt<char16_t, char8_t, __mbstate_t>::extern_type *&a4,
    std::codecvt<char16_t, char8_t, __mbstate_t>::intern_type *a5,
    std::codecvt<char16_t, char8_t, __mbstate_t>::intern_type *a6,
    std::codecvt<char16_t, char8_t, __mbstate_t>::intern_type *&a7) {
  return a0.in(a1, a2, a3, a4, a5, a6, a7);
}

int f93(const std::codecvt<char16_t, char8_t, std::mbstate_t> &a0,
        std::codecvt<char16_t, char8_t, __mbstate_t>::state_type &a1,
        const std::codecvt<char16_t, char8_t, __mbstate_t>::extern_type *a2,
        const std::codecvt<char16_t, char8_t, __mbstate_t>::extern_type *a3,
        std::size_t a4) {
  return a0.length(a1, a2, a3, a4);
}

long f94(const std::codecvt<char16_t, char8_t, std::mbstate_t> &a0) {
  return a0.use_count();
}

std::codecvt_base::result
f95(const std::codecvt<char32_t, char8_t, std::mbstate_t> &a0,
    std::codecvt<char32_t, char8_t, __mbstate_t>::state_type &a1,
    const std::codecvt<char32_t, char8_t, __mbstate_t>::intern_type *a2,
    const std::codecvt<char32_t, char8_t, __mbstate_t>::intern_type *a3,
    const std::codecvt<char32_t, char8_t, __mbstate_t>::intern_type *&a4,
    std::codecvt<char32_t, char8_t, __mbstate_t>::extern_type *a5,
    std::codecvt<char32_t, char8_t, __mbstate_t>::extern_type *a6,
    std::codecvt<char32_t, char8_t, __mbstate_t>::extern_type *&a7) {
  return a0.out(a1, a2, a3, a4, a5, a6, a7);
}

std::codecvt_base::result
f96(const std::codecvt<char32_t, char8_t, std::mbstate_t> &a0,
    std::codecvt<char32_t, char8_t, __mbstate_t>::state_type &a1,
    std::codecvt<char32_t, char8_t, __mbstate_t>::extern_type *a2,
    std::codecvt<char32_t, char8_t, __mbstate_t>::extern_type *a3,
    std::codecvt<char32_t, char8_t, __mbstate_t>::extern_type *&a4) {
  return a0.unshift(a1, a2, a3, a4);
}

std::codecvt_base::result
f97(const std::codecvt<char32_t, char8_t, std::mbstate_t> &a0,
    std::codecvt<char32_t, char8_t, __mbstate_t>::state_type &a1,
    const std::codecvt<char32_t, char8_t, __mbstate_t>::extern_type *a2,
    const std::codecvt<char32_t, char8_t, __mbstate_t>::extern_type *a3,
    const std::codecvt<char32_t, char8_t, __mbstate_t>::extern_type *&a4,
    std::codecvt<char32_t, char8_t, __mbstate_t>::intern_type *a5,
    std::codecvt<char32_t, char8_t, __mbstate_t>::intern_type *a6,
    std::codecvt<char32_t, char8_t, __mbstate_t>::intern_type *&a7) {
  return a0.in(a1, a2, a3, a4, a5, a6, a7);
}

int f98(const std::codecvt<char32_t, char8_t, std::mbstate_t> &a0,
        std::codecvt<char32_t, char8_t, __mbstate_t>::state_type &a1,
        const std::codecvt<char32_t, char8_t, __mbstate_t>::extern_type *a2,
        const std::codecvt<char32_t, char8_t, __mbstate_t>::extern_type *a3,
        std::size_t a4) {
  return a0.length(a1, a2, a3, a4);
}

long f99(const std::codecvt<char32_t, char8_t, std::mbstate_t> &a0) {
  return a0.use_count();
}
