#include <algorithm>
#include <condition_variable>
#include <utility>

template <typename T1, typename T2>
void f43(std::unique_ptr<T1, T2> &a0, std::unique_ptr<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}
