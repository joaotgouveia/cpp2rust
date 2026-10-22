#include "../common/init.h"
#include "../common/traits.h"
#include <codecvt>
#include <cstddef>
#include <ios>
#include <utility>

using t33 = std::codecvt<char16_t, char, std::mbstate_t>;

using t34 = std::codecvt<char16_t, char, __mbstate_t>::intern_type;

using t35 = std::codecvt<char16_t, char, __mbstate_t>::extern_type;

using t36 = std::codecvt<char16_t, char, __mbstate_t>::state_type;

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

int f61(const std::codecvt<char16_t, char, std::mbstate_t> &a0) {
  return a0.encoding();
}

bool f62(const std::codecvt<char16_t, char, std::mbstate_t> &a0) {
  return a0.always_noconv();
}

int f63(const std::codecvt<char16_t, char, std::mbstate_t> &a0,
        std::codecvt<char16_t, char, __mbstate_t>::state_type &a1,
        const std::codecvt<char16_t, char, __mbstate_t>::extern_type *a2,
        const std::codecvt<char16_t, char, __mbstate_t>::extern_type *a3,
        std::size_t a4) {
  return a0.length(a1, a2, a3, a4);
}

int f64(const std::codecvt<char16_t, char, std::mbstate_t> &a0) {
  return a0.max_length();
}

long f65(const std::codecvt<char16_t, char, std::mbstate_t> &a0) {
  return a0.use_count();
}

const std::locale::id &f66() {
  return std::codecvt<char16_t, char, __mbstate_t>::id;
}

using t37 = std::codecvt<char32_t, char, std::mbstate_t>;

using t38 = std::codecvt<char32_t, char, __mbstate_t>::intern_type;

using t39 = std::codecvt<char32_t, char, __mbstate_t>::extern_type;

using t40 = std::codecvt<char32_t, char, __mbstate_t>::state_type;

std::codecvt_base::result
f67(const std::codecvt<char32_t, char, std::mbstate_t> &a0,
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
f68(const std::codecvt<char32_t, char, std::mbstate_t> &a0,
    std::codecvt<char32_t, char, __mbstate_t>::state_type &a1,
    std::codecvt<char32_t, char, __mbstate_t>::extern_type *a2,
    std::codecvt<char32_t, char, __mbstate_t>::extern_type *a3,
    std::codecvt<char32_t, char, __mbstate_t>::extern_type *&a4) {
  return a0.unshift(a1, a2, a3, a4);
}

std::codecvt_base::result
f69(const std::codecvt<char32_t, char, std::mbstate_t> &a0,
    std::codecvt<char32_t, char, __mbstate_t>::state_type &a1,
    const std::codecvt<char32_t, char, __mbstate_t>::extern_type *a2,
    const std::codecvt<char32_t, char, __mbstate_t>::extern_type *a3,
    const std::codecvt<char32_t, char, __mbstate_t>::extern_type *&a4,
    std::codecvt<char32_t, char, __mbstate_t>::intern_type *a5,
    std::codecvt<char32_t, char, __mbstate_t>::intern_type *a6,
    std::codecvt<char32_t, char, __mbstate_t>::intern_type *&a7) {
  return a0.in(a1, a2, a3, a4, a5, a6, a7);
}

int f70(const std::codecvt<char32_t, char, std::mbstate_t> &a0) {
  return a0.encoding();
}

bool f71(const std::codecvt<char32_t, char, std::mbstate_t> &a0) {
  return a0.always_noconv();
}

int f72(const std::codecvt<char32_t, char, std::mbstate_t> &a0,
        std::codecvt<char32_t, char, __mbstate_t>::state_type &a1,
        const std::codecvt<char32_t, char, __mbstate_t>::extern_type *a2,
        const std::codecvt<char32_t, char, __mbstate_t>::extern_type *a3,
        std::size_t a4) {
  return a0.length(a1, a2, a3, a4);
}

int f73(const std::codecvt<char32_t, char, std::mbstate_t> &a0) {
  return a0.max_length();
}

long f74(const std::codecvt<char32_t, char, std::mbstate_t> &a0) {
  return a0.use_count();
}

const std::locale::id &f75() {
  return std::codecvt<char32_t, char, __mbstate_t>::id;
}
