#include <format>
#include <string.h>
#include <utility>

using t1 = std::format_error;

std::format_error f1(const std::string &a0) { return std::format_error(a0); }

std::format_error f2(const char *a0) { return std::format_error(a0); }

std::format_error f3(const std::format_error &a0) {
  return std::format_error(a0);
}

std::format_error &f4(std::format_error &a0, const std::format_error &a1) {
  return a0.operator=(a1);
}

const char *f5(const std::format_error &a0) { return a0.what(); }
