#include <algorithm>
#include <random>
#include <utility>

template <typename T1, std::size_t T2>
bool f14(const std::shuffle_order_engine<T1, T2> &a0,
         const std::shuffle_order_engine<T1, T2> &a1) {
  return operator==(a0, a1);
}
