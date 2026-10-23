#include <array>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, std::size_t T2>
void f41(std::array<T1, T2> &a0, std::array<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}
