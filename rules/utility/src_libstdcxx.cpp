#include <algorithm>
#include <utility>

using t1 = std::_Swallow_assign;

std::_Swallow_assign f1() { return std::_Swallow_assign(); }

std::_Swallow_assign f2(const std::_Swallow_assign &a0) {
  return std::_Swallow_assign(a0);
}

std::_Swallow_assign f3(std::_Swallow_assign &&a0) {
  return std::_Swallow_assign(std::move(a0));
}

std::_Swallow_assign &f4(std::_Swallow_assign &a0,
                         const std::_Swallow_assign &a1) {
  return a0.operator=(a1);
}

std::_Swallow_assign &f5(std::_Swallow_assign &a0, std::_Swallow_assign &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
const std::_Swallow_assign &f6(const std::_Swallow_assign &a0, const T1 &a1) {
  return a0.template operator= <T1>(a1);
}
