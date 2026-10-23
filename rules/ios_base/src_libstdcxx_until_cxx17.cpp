#include <ios>
#include <iosfwd>
#include <string>
#include <utility>

using t11 = std::ios_base::io_state;

using t12 = std::ios_base::open_mode;

using t13 = std::ios_base::seek_dir;

using t14 = std::ios_base::streampos;

using t15 = std::ios_base::streamoff;

std::ios_base::streampos f76() { return std::ios_base::streampos(); }

std::ios_base::streampos f77(std::streamoff a0) {
  return std::ios_base::streampos(a0);
}

std::ios_base::streampos f78(const std::ios_base::streampos &a0) {
  return std::ios_base::streampos(a0);
}

std::ios_base::streampos &f79(std::ios_base::streampos &a0,
                              const std::ios_base::streampos &a1) {
  return a0.operator=(a1);
}

std::streamoff f80(const std::ios_base::streampos &a0) {
  return static_cast<std::streamoff>(a0);
}

void f81(std::ios_base::streampos &a0, std::mbstate_t a1) {
  return a0.state(std::move(a1));
}

std::mbstate_t f82(const std::ios_base::streampos &a0) { return a0.state(); }

std::ios_base::streampos &f83(std::ios_base::streampos &a0, std::streamoff a1) {
  return a0.operator+=(a1);
}

std::ios_base::streampos &f84(std::ios_base::streampos &a0, std::streamoff a1) {
  return a0.operator-=(a1);
}

std::ios_base::streampos f85(const std::ios_base::streampos &a0,
                             std::streamoff a1) {
  return a0.operator+(a1);
}

std::ios_base::streampos f86(const std::ios_base::streampos &a0,
                             std::streamoff a1) {
  return a0.operator-(a1);
}

std::streamoff f87(const std::ios_base::streampos &a0,
                   const std::ios_base::streampos &a1) {
  return a0.operator-(a1);
}
