#include <type_traits>
#include <utility>

template <typename T1, typename T2> bool f1(const std::is_base_of<T1, T2> &a0) {
  return static_cast<bool>(a0);
}
