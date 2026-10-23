#include <algorithm>
#include <utility>

template <typename T1> using t1 = std::in_place_type_t<T1>;

template <typename T1> std::in_place_type_t<T1> f1() {
  return std::in_place_type_t<T1>();
}
