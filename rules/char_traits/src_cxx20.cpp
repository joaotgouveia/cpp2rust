#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <ios>
#include <iosfwd>
#include <utility>

using t26 = std::char_traits<char>::comparison_category;

using t27 = std::char_traits<wchar_t>::comparison_category;

using t28 = std::char_traits<char16_t>::comparison_category;

using t29 = std::char_traits<char8_t>;

using t30 = std::char_traits<char8_t>::char_type;

using t31 = std::char_traits<char8_t>::int_type;

using t32 = std::char_traits<char8_t>::pos_type;

using t33 = std::char_traits<char8_t>::off_type;

using t34 = std::char_traits<char8_t>::state_type;

using t35 = std::char_traits<char8_t>::comparison_category;

void f139(std::char_traits<char8_t>::char_type &a0,
          const std::char_traits<char8_t>::char_type &a1) {
  return std::char_traits<char8_t>::assign(a0, a1);
}

bool f140(const std::char_traits<char8_t>::char_type &a0,
          const std::char_traits<char8_t>::char_type &a1) {
  return std::char_traits<char8_t>::eq(a0, a1);
}

bool f141(const std::char_traits<char8_t>::char_type &a0,
          const std::char_traits<char8_t>::char_type &a1) {
  return std::char_traits<char8_t>::lt(a0, a1);
}

int f142(const std::char_traits<char8_t>::char_type *a0,
         const std::char_traits<char8_t>::char_type *a1, std::size_t a2) {
  return std::char_traits<char8_t>::compare(a0, a1, a2);
}

std::size_t f143(const std::char_traits<char8_t>::char_type *a0) {
  return std::char_traits<char8_t>::length(a0);
}

const std::char_traits<char8_t>::char_type *
f144(const std::char_traits<char8_t>::char_type *a0, std::size_t a1,
     const std::char_traits<char8_t>::char_type &a2) {
  return std::char_traits<char8_t>::find(a0, a1, a2);
}

std::char_traits<char8_t>::char_type *
f145(std::char_traits<char8_t>::char_type *a0,
     const std::char_traits<char8_t>::char_type *a1, std::size_t a2) {
  return std::char_traits<char8_t>::move(a0, a1, a2);
}

std::char_traits<char8_t>::char_type *
f146(std::char_traits<char8_t>::char_type *a0,
     const std::char_traits<char8_t>::char_type *a1, std::size_t a2) {
  return std::char_traits<char8_t>::copy(a0, a1, a2);
}

std::char_traits<char8_t>::char_type *
f147(std::char_traits<char8_t>::char_type *a0, std::size_t a1,
     std::char_traits<char8_t>::char_type a2) {
  return std::char_traits<char8_t>::assign(a0, a1, a2);
}

std::char_traits<char8_t>::char_type
f148(const std::char_traits<char8_t>::int_type &a0) {
  return std::char_traits<char8_t>::to_char_type(a0);
}

std::char_traits<char8_t>::int_type
f149(const std::char_traits<char8_t>::char_type &a0) {
  return std::char_traits<char8_t>::to_int_type(a0);
}

bool f150(const std::char_traits<char8_t>::int_type &a0,
          const std::char_traits<char8_t>::int_type &a1) {
  return std::char_traits<char8_t>::eq_int_type(a0, a1);
}

std::char_traits<char8_t>::int_type f151() {
  return std::char_traits<char8_t>::eof();
}

std::char_traits<char8_t>::int_type
f152(const std::char_traits<char8_t>::int_type &a0) {
  return std::char_traits<char8_t>::not_eof(a0);
}

std::char_traits<char8_t> f153() { return std::char_traits<char8_t>(); }

std::char_traits<char8_t> f154(const std::char_traits<char8_t> &a0) {
  return std::char_traits<char8_t>(a0);
}

std::char_traits<char8_t> f155(std::char_traits<char8_t> &&a0) {
  return std::char_traits<char8_t>(std::move(a0));
}

std::char_traits<char8_t> &f156(std::char_traits<char8_t> &a0,
                                const std::char_traits<char8_t> &a1) {
  return a0.operator=(a1);
}

std::char_traits<char8_t> &f157(std::char_traits<char8_t> &a0,
                                std::char_traits<char8_t> &&a1) {
  return a0.operator=(std::move(a1));
}

std::char_traits<char8_t>::pos_type f158() {
  return std::char_traits<char8_t>::pos_type();
}

std::char_traits<char8_t>::pos_type f159(std::streamoff a0) {
  return std::char_traits<char8_t>::pos_type(a0);
}

std::char_traits<char8_t>::pos_type
f160(const std::char_traits<char8_t>::pos_type &a0) {
  return std::char_traits<char8_t>::pos_type(a0);
}

std::char_traits<char8_t>::pos_type &
f161(std::char_traits<char8_t>::pos_type &a0,
     const std::char_traits<char8_t>::pos_type &a1) {
  return a0.operator=(a1);
}

std::streamoff f162(const std::char_traits<char8_t>::pos_type &a0) {
  return static_cast<std::streamoff>(a0);
}

void f163(std::char_traits<char8_t>::pos_type &a0, std::mbstate_t a1) {
  return a0.state(std::move(a1));
}

std::mbstate_t f164(const std::char_traits<char8_t>::pos_type &a0) {
  return a0.state();
}

std::char_traits<char8_t>::pos_type &
f165(std::char_traits<char8_t>::pos_type &a0, std::streamoff a1) {
  return a0.operator+=(a1);
}

std::char_traits<char8_t>::pos_type &
f166(std::char_traits<char8_t>::pos_type &a0, std::streamoff a1) {
  return a0.operator-=(a1);
}

std::char_traits<char8_t>::pos_type
f167(const std::char_traits<char8_t>::pos_type &a0, std::streamoff a1) {
  return a0.operator+(a1);
}

std::char_traits<char8_t>::pos_type
f168(const std::char_traits<char8_t>::pos_type &a0, std::streamoff a1) {
  return a0.operator-(a1);
}

std::streamoff f169(const std::char_traits<char8_t>::pos_type &a0,
                    const std::char_traits<char8_t>::pos_type &a1) {
  return a0.operator-(a1);
}

using t36 = std::char_traits<char32_t>::comparison_category;
