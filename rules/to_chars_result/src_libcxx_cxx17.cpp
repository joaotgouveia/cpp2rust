#include <charconv>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

using t1 = std::to_chars_result;

std::to_chars_result f1() { return std::to_chars_result(); }

std::to_chars_result f2(const std::to_chars_result &a0) {
  return std::to_chars_result(a0);
}

std::to_chars_result f3(std::to_chars_result &&a0) {
  return std::to_chars_result(std::move(a0));
}

std::to_chars_result &f4(std::to_chars_result &a0,
                         const std::to_chars_result &a1) {
  return a0.operator=(a1);
}

std::to_chars_result &f5(std::to_chars_result &a0, std::to_chars_result &&a1) {
  return a0.operator=(std::move(a1));
}
