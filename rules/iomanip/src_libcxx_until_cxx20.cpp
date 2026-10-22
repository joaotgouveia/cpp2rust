#include "../common/init.h"
#include "../common/traits.h"
#include <iomanip>
#include <ios>
#include <utility>

decltype(std::resetiosflags(std::declval<std::ios_base::fmtflags>()))
f30(std::ios_base::fmtflags a0) {
  return std::resetiosflags(a0);
}

decltype(std::setbase(std::declval<int>())) f31(int a0) {
  return std::setbase(a0);
}

decltype(std::setiosflags(std::declval<std::ios_base::fmtflags>()))
f32(std::ios_base::fmtflags a0) {
  return std::setiosflags(a0);
}

decltype(std::setprecision(std::declval<int>())) f33(int a0) {
  return std::setprecision(a0);
}

decltype(std::setw(std::declval<int>())) f34(int a0) { return std::setw(a0); }
