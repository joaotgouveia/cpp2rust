#include <iosfwd>
#include <stdexcept>
#include <utility>

using t1 = std::domain_error;

std::domain_error f1(const std::string &a0) { return std::domain_error(a0); }

std::domain_error f2(const char *a0) { return std::domain_error(a0); }

std::domain_error f3(const std::domain_error &a0) {
  return std::domain_error(a0);
}

std::domain_error &f4(std::domain_error &a0, const std::domain_error &a1) {
  return a0.operator=(a1);
}

std::domain_error f5(std::domain_error &&a0) {
  return std::domain_error(std::move(a0));
}

std::domain_error &f6(std::domain_error &a0, std::domain_error &&a1) {
  return a0.operator=(std::move(a1));
}

const char *f7(const std::domain_error &a0) { return a0.what(); }

using t2 = std::invalid_argument;

std::invalid_argument f8(const std::string &a0) {
  return std::invalid_argument(a0);
}

std::invalid_argument f9(const char *a0) { return std::invalid_argument(a0); }

std::invalid_argument f10(const std::invalid_argument &a0) {
  return std::invalid_argument(a0);
}

std::invalid_argument &f11(std::invalid_argument &a0,
                           const std::invalid_argument &a1) {
  return a0.operator=(a1);
}

std::invalid_argument f12(std::invalid_argument &&a0) {
  return std::invalid_argument(std::move(a0));
}

std::invalid_argument &f13(std::invalid_argument &a0,
                           std::invalid_argument &&a1) {
  return a0.operator=(std::move(a1));
}

const char *f14(const std::invalid_argument &a0) { return a0.what(); }

using t3 = std::length_error;

std::length_error f15(const std::string &a0) { return std::length_error(a0); }

std::length_error f16(const char *a0) { return std::length_error(a0); }

std::length_error f17(const std::length_error &a0) {
  return std::length_error(a0);
}

std::length_error &f18(std::length_error &a0, const std::length_error &a1) {
  return a0.operator=(a1);
}

std::length_error f19(std::length_error &&a0) {
  return std::length_error(std::move(a0));
}

std::length_error &f20(std::length_error &a0, std::length_error &&a1) {
  return a0.operator=(std::move(a1));
}

const char *f21(const std::length_error &a0) { return a0.what(); }

using t4 = std::logic_error;

std::logic_error f22(const std::string &a0) { return std::logic_error(a0); }

std::logic_error f23(const char *a0) { return std::logic_error(a0); }

std::logic_error f24(std::logic_error &&a0) {
  return std::logic_error(std::move(a0));
}

std::logic_error &f25(std::logic_error &a0, std::logic_error &&a1) {
  return a0.operator=(std::move(a1));
}

std::logic_error f26(const std::logic_error &a0) {
  return std::logic_error(a0);
}

std::logic_error &f27(std::logic_error &a0, const std::logic_error &a1) {
  return a0.operator=(a1);
}

const char *f28(const std::logic_error &a0) { return a0.what(); }

using t5 = std::out_of_range;

std::out_of_range f29(const std::string &a0) { return std::out_of_range(a0); }

std::out_of_range f30(const char *a0) { return std::out_of_range(a0); }

std::out_of_range f31(const std::out_of_range &a0) {
  return std::out_of_range(a0);
}

std::out_of_range &f32(std::out_of_range &a0, const std::out_of_range &a1) {
  return a0.operator=(a1);
}

std::out_of_range f33(std::out_of_range &&a0) {
  return std::out_of_range(std::move(a0));
}

std::out_of_range &f34(std::out_of_range &a0, std::out_of_range &&a1) {
  return a0.operator=(std::move(a1));
}

const char *f35(const std::out_of_range &a0) { return a0.what(); }

using t6 = std::overflow_error;

std::overflow_error f36(const std::string &a0) {
  return std::overflow_error(a0);
}

std::overflow_error f37(const char *a0) { return std::overflow_error(a0); }

std::overflow_error f38(const std::overflow_error &a0) {
  return std::overflow_error(a0);
}

std::overflow_error &f39(std::overflow_error &a0,
                         const std::overflow_error &a1) {
  return a0.operator=(a1);
}

std::overflow_error f40(std::overflow_error &&a0) {
  return std::overflow_error(std::move(a0));
}

std::overflow_error &f41(std::overflow_error &a0, std::overflow_error &&a1) {
  return a0.operator=(std::move(a1));
}

const char *f42(const std::overflow_error &a0) { return a0.what(); }

using t7 = std::range_error;

std::range_error f43(const std::string &a0) { return std::range_error(a0); }

std::range_error f44(const char *a0) { return std::range_error(a0); }

std::range_error f45(const std::range_error &a0) {
  return std::range_error(a0);
}

std::range_error &f46(std::range_error &a0, const std::range_error &a1) {
  return a0.operator=(a1);
}

std::range_error f47(std::range_error &&a0) {
  return std::range_error(std::move(a0));
}

std::range_error &f48(std::range_error &a0, std::range_error &&a1) {
  return a0.operator=(std::move(a1));
}

const char *f49(const std::range_error &a0) { return a0.what(); }

using t8 = std::runtime_error;

std::runtime_error f50(const std::string &a0) { return std::runtime_error(a0); }

std::runtime_error f51(const char *a0) { return std::runtime_error(a0); }

std::runtime_error f52(std::runtime_error &&a0) {
  return std::runtime_error(std::move(a0));
}

std::runtime_error &f53(std::runtime_error &a0, std::runtime_error &&a1) {
  return a0.operator=(std::move(a1));
}

std::runtime_error f54(const std::runtime_error &a0) {
  return std::runtime_error(a0);
}

std::runtime_error &f55(std::runtime_error &a0, const std::runtime_error &a1) {
  return a0.operator=(a1);
}

const char *f56(const std::runtime_error &a0) { return a0.what(); }

using t9 = std::underflow_error;

std::underflow_error f57(const std::string &a0) {
  return std::underflow_error(a0);
}

std::underflow_error f58(const char *a0) { return std::underflow_error(a0); }

std::underflow_error f59(const std::underflow_error &a0) {
  return std::underflow_error(a0);
}

std::underflow_error &f60(std::underflow_error &a0,
                          const std::underflow_error &a1) {
  return a0.operator=(a1);
}

std::underflow_error f61(std::underflow_error &&a0) {
  return std::underflow_error(std::move(a0));
}

std::underflow_error &f62(std::underflow_error &a0, std::underflow_error &&a1) {
  return a0.operator=(std::move(a1));
}

const char *f63(const std::underflow_error &a0) { return a0.what(); }
