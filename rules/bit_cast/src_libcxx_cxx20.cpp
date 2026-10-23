#include <bit>
#include <utility>

template <typename T1, typename T2> T1 f1(const T2 &a0) {
  return std::bit_cast<T1, T2>(a0);
}
