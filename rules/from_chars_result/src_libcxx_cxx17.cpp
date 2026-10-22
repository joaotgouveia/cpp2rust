#include "../common/init.h"
#include "../common/traits.h"
#include <charconv>
#include <utility>

using t1 = std::from_chars_result;

std::from_chars_result f1() { return std::from_chars_result(); }

std::from_chars_result f2(const std::from_chars_result &a0) {
  return std::from_chars_result(a0);
}

std::from_chars_result f3(std::from_chars_result &&a0) {
  return std::from_chars_result(std::move(a0));
}

std::from_chars_result &f4(std::from_chars_result &a0,
                           const std::from_chars_result &a1) {
  return a0.operator=(a1);
}

std::from_chars_result &f5(std::from_chars_result &a0,
                           std::from_chars_result &&a1) {
  return a0.operator=(std::move(a1));
}
