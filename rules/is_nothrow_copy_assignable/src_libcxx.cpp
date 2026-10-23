#include <type_traits>
#include <utility>

template <typename T1> bool f1(const std::is_nothrow_copy_assignable<T1> &a0) {
  return static_cast<bool>(a0);
}
