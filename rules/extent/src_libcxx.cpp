#include <type_traits>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, std::size_t T2> using t2 = std::extent<T1, T2>;

template <typename T1, std::size_t T2>
typename std::extent<T1, T2>::value_type f3(const std::extent<T1, T2> &a0) {
  return static_cast<typename std::extent<T1, T2>::value_type>(a0);
}
