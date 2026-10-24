#include <memory>
#include <utility>

template <typename T1>
T1 *f7(typename std::conditional<std::is_void<T1>::value, std::__nat, T1>::type
           &a0) {
  return std::pointer_traits<T1 *>::pointer_to(a0);
}
