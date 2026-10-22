#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <cstddef>
#include <ios>
#include <iosfwd>
#include <string>
#include <utility>

using t10 = std::char_traits<char>::comparison_category;

using t11 = std::char_traits<char8_t>;

std::char_traits<char8_t> f29() { return std::char_traits<char8_t>(); }

std::char_traits<char8_t> f30(const std::char_traits<char8_t> &a0) {
  return std::char_traits<char8_t>(a0);
}

std::char_traits<char8_t> f31(std::char_traits<char8_t> &&a0) {
  return std::char_traits<char8_t>(std::move(a0));
}

std::char_traits<char8_t> &f32(std::char_traits<char8_t> &a0,
                               const std::char_traits<char8_t> &a1) {
  return a0.operator=(a1);
}

std::char_traits<char8_t> &f33(std::char_traits<char8_t> &a0,
                               std::char_traits<char8_t> &&a1) {
  return a0.operator=(std::move(a1));
}
