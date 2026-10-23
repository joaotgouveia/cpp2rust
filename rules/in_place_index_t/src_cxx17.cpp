#include <algorithm>
#include <utility>

template <std::size_t T1> using t1 = std::in_place_index_t<T1>;

template <std::size_t T1> std::in_place_index_t<T1> f1() {
  return std::in_place_index_t<T1>();
}
