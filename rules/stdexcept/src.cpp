#include <stdexcept>
#include <string>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

using t1 = std::domain_error;

std::domain_error f1(const std::string &a0) { return std::domain_error(a0); }

std::domain_error f2(const char *a0) { return std::domain_error(a0); }

std::domain_error f3(const std::domain_error &a0) {
  return std::domain_error(a0);
}

std::domain_error &f4(std::domain_error &a0, const std::domain_error &a1) {
  return a0.operator=(a1);
}

const char *f5(const std::domain_error &a0) { return a0.what(); }

using t2 = std::invalid_argument;

std::invalid_argument f6(const std::string &a0) {
  return std::invalid_argument(a0);
}

std::invalid_argument f7(const char *a0) { return std::invalid_argument(a0); }

std::invalid_argument f8(const std::invalid_argument &a0) {
  return std::invalid_argument(a0);
}

std::invalid_argument &f9(std::invalid_argument &a0,
                          const std::invalid_argument &a1) {
  return a0.operator=(a1);
}

const char *f10(const std::invalid_argument &a0) { return a0.what(); }

using t3 = std::length_error;

std::length_error f11(const std::string &a0) { return std::length_error(a0); }

std::length_error f12(const char *a0) { return std::length_error(a0); }

std::length_error f13(const std::length_error &a0) {
  return std::length_error(a0);
}

std::length_error &f14(std::length_error &a0, const std::length_error &a1) {
  return a0.operator=(a1);
}

const char *f15(const std::length_error &a0) { return a0.what(); }

using t4 = std::logic_error;

std::logic_error f16(const std::string &a0) { return std::logic_error(a0); }

std::logic_error f17(const char *a0) { return std::logic_error(a0); }

std::logic_error f18(const std::logic_error &a0) {
  return std::logic_error(a0);
}

std::logic_error &f19(std::logic_error &a0, const std::logic_error &a1) {
  return a0.operator=(a1);
}

const char *f20(const std::logic_error &a0) { return a0.what(); }

using t5 = std::out_of_range;

std::out_of_range f21(const std::string &a0) { return std::out_of_range(a0); }

std::out_of_range f22(const char *a0) { return std::out_of_range(a0); }

std::out_of_range f23(const std::out_of_range &a0) {
  return std::out_of_range(a0);
}

std::out_of_range &f24(std::out_of_range &a0, const std::out_of_range &a1) {
  return a0.operator=(a1);
}

const char *f25(const std::out_of_range &a0) { return a0.what(); }

using t6 = std::overflow_error;

std::overflow_error f26(const std::string &a0) {
  return std::overflow_error(a0);
}

std::overflow_error f27(const char *a0) { return std::overflow_error(a0); }

std::overflow_error f28(const std::overflow_error &a0) {
  return std::overflow_error(a0);
}

std::overflow_error &f29(std::overflow_error &a0,
                         const std::overflow_error &a1) {
  return a0.operator=(a1);
}

const char *f30(const std::overflow_error &a0) { return a0.what(); }

using t7 = std::range_error;

std::range_error f31(const std::string &a0) { return std::range_error(a0); }

std::range_error f32(const char *a0) { return std::range_error(a0); }

std::range_error f33(const std::range_error &a0) {
  return std::range_error(a0);
}

std::range_error &f34(std::range_error &a0, const std::range_error &a1) {
  return a0.operator=(a1);
}

const char *f35(const std::range_error &a0) { return a0.what(); }

using t8 = std::runtime_error;

std::runtime_error f36(const std::string &a0) { return std::runtime_error(a0); }

std::runtime_error f37(const char *a0) { return std::runtime_error(a0); }

std::runtime_error f38(const std::runtime_error &a0) {
  return std::runtime_error(a0);
}

std::runtime_error &f39(std::runtime_error &a0, const std::runtime_error &a1) {
  return a0.operator=(a1);
}

const char *f40(const std::runtime_error &a0) { return a0.what(); }

using t9 = std::underflow_error;

std::underflow_error f41(const std::string &a0) {
  return std::underflow_error(a0);
}

std::underflow_error f42(const char *a0) { return std::underflow_error(a0); }

std::underflow_error f43(const std::underflow_error &a0) {
  return std::underflow_error(a0);
}

std::underflow_error &f44(std::underflow_error &a0,
                          const std::underflow_error &a1) {
  return a0.operator=(a1);
}

const char *f45(const std::underflow_error &a0) { return a0.what(); }
