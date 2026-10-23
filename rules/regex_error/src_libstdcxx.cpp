#include <regex>
#include <utility>

using t1 = std::regex_constants::error_type;

using t2 = std::regex_error;

std::regex_error f1(std::regex_constants::error_type a0) {
  return std::regex_error(a0);
}

std::regex_constants::error_type f2(const std::regex_error &a0) {
  return a0.code();
}

std::regex_error f3(const std::regex_error &a0) { return std::regex_error(a0); }

std::regex_error &f4(std::regex_error &a0, const std::regex_error &a1) {
  return a0.operator=(a1);
}

const char *f5(const std::regex_error &a0) { return a0.what(); }
