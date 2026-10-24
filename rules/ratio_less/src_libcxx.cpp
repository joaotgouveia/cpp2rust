#include <ratio>
#include <utility>

template <typename T1, typename T2> bool f1(const std::ratio_less<T1, T2> &a0) {
  return static_cast<bool>(a0);
}
