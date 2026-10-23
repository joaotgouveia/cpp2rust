#include <algorithm>
#include <ios>
#include <iosfwd>
#include <utility>

using t28 = std::char_traits<wchar_t>::comparison_category;

using t29 = std::char_traits<char16_t>::comparison_category;

using t30 = std::char_traits<char8_t>::char_type;

using t31 = std::char_traits<char8_t>::int_type;

using t32 = std::char_traits<char8_t>::pos_type;

using t33 = std::char_traits<char8_t>::off_type;

using t34 = std::char_traits<char8_t>::state_type;

using t35 = std::char_traits<char8_t>::comparison_category;

void f96(std::char_traits<char8_t>::char_type &a0,
         const std::char_traits<char8_t>::char_type &a1) {
  return std::char_traits<char8_t>::assign(a0, a1);
}

bool f97(const std::char_traits<char8_t>::char_type &a0,
         const std::char_traits<char8_t>::char_type &a1) {
  return std::char_traits<char8_t>::eq(a0, a1);
}

bool f98(const std::char_traits<char8_t>::char_type &a0,
         const std::char_traits<char8_t>::char_type &a1) {
  return std::char_traits<char8_t>::lt(a0, a1);
}

int f99(const std::char_traits<char8_t>::char_type *a0,
        const std::char_traits<char8_t>::char_type *a1, std::size_t a2) {
  return std::char_traits<char8_t>::compare(a0, a1, a2);
}

std::size_t f100(const std::char_traits<char8_t>::char_type *a0) {
  return std::char_traits<char8_t>::length(a0);
}

const std::char_traits<char8_t>::char_type *
f101(const std::char_traits<char8_t>::char_type *a0, std::size_t a1,
     const std::char_traits<char8_t>::char_type &a2) {
  return std::char_traits<char8_t>::find(a0, a1, a2);
}

std::char_traits<char8_t>::char_type *
f102(std::char_traits<char8_t>::char_type *a0,
     const std::char_traits<char8_t>::char_type *a1, std::size_t a2) {
  return std::char_traits<char8_t>::move(a0, a1, a2);
}

std::char_traits<char8_t>::char_type *
f103(std::char_traits<char8_t>::char_type *a0,
     const std::char_traits<char8_t>::char_type *a1, std::size_t a2) {
  return std::char_traits<char8_t>::copy(a0, a1, a2);
}

std::char_traits<char8_t>::char_type *
f104(std::char_traits<char8_t>::char_type *a0, std::size_t a1,
     std::char_traits<char8_t>::char_type a2) {
  return std::char_traits<char8_t>::assign(a0, a1, a2);
}

std::char_traits<char8_t>::char_type
f105(const std::char_traits<char8_t>::int_type &a0) {
  return std::char_traits<char8_t>::to_char_type(a0);
}

std::char_traits<char8_t>::int_type
f106(const std::char_traits<char8_t>::char_type &a0) {
  return std::char_traits<char8_t>::to_int_type(a0);
}

bool f107(const std::char_traits<char8_t>::int_type &a0,
          const std::char_traits<char8_t>::int_type &a1) {
  return std::char_traits<char8_t>::eq_int_type(a0, a1);
}

std::char_traits<char8_t>::int_type f108() {
  return std::char_traits<char8_t>::eof();
}

std::char_traits<char8_t>::int_type
f109(const std::char_traits<char8_t>::int_type &a0) {
  return std::char_traits<char8_t>::not_eof(a0);
}

using t36 = std::char_traits<char32_t>::comparison_category;
