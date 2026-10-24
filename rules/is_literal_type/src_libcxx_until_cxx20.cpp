#include <type_traits>
#include <utility>

template <typename T1> using t2 = std::is_literal_type<T1>;

template <typename T1> bool f3(const std::is_literal_type<T1> &a0) {
  return static_cast<bool>(a0);
}
