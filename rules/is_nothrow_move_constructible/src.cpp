#include <type_traits>
#include <utility>

template <typename T1> using t1 = std::is_nothrow_move_constructible<T1>;

template <typename T1>
bool f1(const std::is_nothrow_move_constructible<T1> &a0) {
  return static_cast<bool>(a0);
}
