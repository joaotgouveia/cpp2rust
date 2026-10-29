#include <algorithm>
#include <exception>
#include <typeinfo>
#include <utility>

using t1 = std::bad_cast;

std::bad_cast f1() { return std::bad_cast(); }

const char *f2(const std::bad_cast &a0) { return a0.what(); }

std::bad_cast f3(const std::bad_cast &a0) { return std::bad_cast(a0); }

std::bad_cast &f4(std::bad_cast &a0, const std::bad_cast &a1) {
  return a0.operator=(a1);
}

using t2 = std::bad_typeid;

std::bad_typeid f5() { return std::bad_typeid(); }

const char *f6(const std::bad_typeid &a0) { return a0.what(); }

std::bad_typeid f7(const std::bad_typeid &a0) { return std::bad_typeid(a0); }

std::bad_typeid &f8(std::bad_typeid &a0, const std::bad_typeid &a1) {
  return a0.operator=(a1);
}

using t3 = std::type_info;

const char *f9(const std::type_info &a0) { return a0.name(); }

bool f10(const std::type_info &a0, const std::type_info &a1) {
  return a0.before(a1);
}

bool f11(const std::type_info &a0, const std::type_info &a1) {
  return a0.operator==(a1);
}

std::size_t f12(const std::type_info &a0) { return a0.hash_code(); }
