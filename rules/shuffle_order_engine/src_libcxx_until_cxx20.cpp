#include <cstddef>
#include <istream>
#include <ostream>
#include <random>
#include <utility>

template <typename T1, std::size_t T2, typename T3,
          std::__enable_if_t<std::__is_seed_sequence_v<
                                 T3, std::shuffle_order_engine<T1, T2>> &&
                                 !std::is_convertible<T3, T1>::value,
                             int>
              T4 = 0,
          typename std::enable_if<(!(std::is_same<T3, const T1>::value)),
                                  int>::type T5 = 0>
std::shuffle_order_engine<T1, T2> f22(T3 &a0) {
  return std::shuffle_order_engine<T1, T2>(a0);
}
