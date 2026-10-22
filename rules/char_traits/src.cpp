#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <cstddef>
#include <string>
#include <utility>

using t1 = std::char_traits<char>;

using t2 = std::char_traits<char>::char_type;

using t3 = std::char_traits<char>::int_type;

using t4 = std::char_traits<char>::pos_type;

using t5 = std::char_traits<char>::off_type;

using t6 = std::char_traits<char>::state_type;

void f1(std::char_traits<char>::char_type &a0,
        const std::char_traits<char>::char_type &a1) {
  return std::char_traits<char>::assign(a0, a1);
}

int f2(const std::char_traits<char>::char_type *a0,
       const std::char_traits<char>::char_type *a1, std::size_t a2) {
  return std::char_traits<char>::compare(a0, a1, a2);
}

std::size_t f3(const std::char_traits<char>::char_type *a0) {
  return std::char_traits<char>::length(a0);
}

const std::char_traits<char>::char_type *
f4(const std::char_traits<char>::char_type *a0, std::size_t a1,
   const std::char_traits<char>::char_type &a2) {
  return std::char_traits<char>::find(a0, a1, a2);
}

std::char_traits<char>::char_type *
f5(std::char_traits<char>::char_type *a0,
   const std::char_traits<char>::char_type *a1, std::size_t a2) {
  return std::char_traits<char>::move(a0, a1, a2);
}

std::char_traits<char>::char_type *
f6(std::char_traits<char>::char_type *a0,
   const std::char_traits<char>::char_type *a1, std::size_t a2) {
  return std::char_traits<char>::copy(a0, a1, a2);
}

std::char_traits<char>::char_type *f7(std::char_traits<char>::char_type *a0,
                                      std::size_t a1,
                                      std::char_traits<char>::char_type a2) {
  return std::char_traits<char>::assign(a0, a1, a2);
}

std::char_traits<char>::int_type f8() { return std::char_traits<char>::eof(); }

std::char_traits<char> f9() { return std::char_traits<char>(); }

std::char_traits<char> f10(const std::char_traits<char> &a0) {
  return std::char_traits<char>(a0);
}

std::char_traits<char> f11(std::char_traits<char> &&a0) {
  return std::char_traits<char>(std::move(a0));
}

std::char_traits<char> &f12(std::char_traits<char> &a0,
                            const std::char_traits<char> &a1) {
  return a0.operator=(a1);
}

std::char_traits<char> &f13(std::char_traits<char> &a0,
                            std::char_traits<char> &&a1) {
  return a0.operator=(std::move(a1));
}

using t7 = std::char_traits<wchar_t>;

std::char_traits<wchar_t> f14() { return std::char_traits<wchar_t>(); }

std::char_traits<wchar_t> f15(const std::char_traits<wchar_t> &a0) {
  return std::char_traits<wchar_t>(a0);
}

std::char_traits<wchar_t> f16(std::char_traits<wchar_t> &&a0) {
  return std::char_traits<wchar_t>(std::move(a0));
}

std::char_traits<wchar_t> &f17(std::char_traits<wchar_t> &a0,
                               const std::char_traits<wchar_t> &a1) {
  return a0.operator=(a1);
}

std::char_traits<wchar_t> &f18(std::char_traits<wchar_t> &a0,
                               std::char_traits<wchar_t> &&a1) {
  return a0.operator=(std::move(a1));
}

using t8 = std::char_traits<char16_t>;

std::char_traits<char16_t> f19() { return std::char_traits<char16_t>(); }

std::char_traits<char16_t> f20(const std::char_traits<char16_t> &a0) {
  return std::char_traits<char16_t>(a0);
}

std::char_traits<char16_t> f21(std::char_traits<char16_t> &&a0) {
  return std::char_traits<char16_t>(std::move(a0));
}

std::char_traits<char16_t> &f22(std::char_traits<char16_t> &a0,
                                const std::char_traits<char16_t> &a1) {
  return a0.operator=(a1);
}

std::char_traits<char16_t> &f23(std::char_traits<char16_t> &a0,
                                std::char_traits<char16_t> &&a1) {
  return a0.operator=(std::move(a1));
}

using t9 = std::char_traits<char32_t>;

std::char_traits<char32_t> f24() { return std::char_traits<char32_t>(); }

std::char_traits<char32_t> f25(const std::char_traits<char32_t> &a0) {
  return std::char_traits<char32_t>(a0);
}

std::char_traits<char32_t> f26(std::char_traits<char32_t> &&a0) {
  return std::char_traits<char32_t>(std::move(a0));
}

std::char_traits<char32_t> &f27(std::char_traits<char32_t> &a0,
                                const std::char_traits<char32_t> &a1) {
  return a0.operator=(a1);
}

std::char_traits<char32_t> &f28(std::char_traits<char32_t> &a0,
                                std::char_traits<char32_t> &&a1) {
  return a0.operator=(std::move(a1));
}
