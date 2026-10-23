#include <algorithm>
#include <cstddef>
#include <cxxabi.h>
#include <exception>
#include <typeindex>
#include <utility>

using t1 = std::type_index;

std::type_index f1(const std::type_info &a0) { return std::type_index(a0); }

bool f2(const std::type_index &a0, const std::type_index &a1) {
  return a0.operator==(a1);
}

bool f3(const std::type_index &a0, const std::type_index &a1) {
  return a0.operator<(a1);
}

bool f4(const std::type_index &a0, const std::type_index &a1) {
  return a0.operator<=(a1);
}

bool f5(const std::type_index &a0, const std::type_index &a1) {
  return a0.operator>(a1);
}

bool f6(const std::type_index &a0, const std::type_index &a1) {
  return a0.operator>=(a1);
}

std::size_t f7(const std::type_index &a0) { return a0.hash_code(); }

const char *f8(const std::type_index &a0) { return a0.name(); }

std::type_index f9(const std::type_index &a0) { return std::type_index(a0); }

std::type_index f10(std::type_index &&a0) {
  return std::type_index(std::move(a0));
}

std::type_index &f11(std::type_index &a0, const std::type_index &a1) {
  return a0.operator=(a1);
}

std::type_index &f12(std::type_index &a0, std::type_index &&a1) {
  return a0.operator=(std::move(a1));
}
