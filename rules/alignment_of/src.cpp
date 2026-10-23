#include <type_traits>
#include <utility>

template <typename T1> using t1 = std::alignment_of<T1>;

template <typename T1>
typename std::alignment_of<T1>::value_type f1(const std::alignment_of<T1> &a0) {
  return static_cast<typename std::alignment_of<T1>::value_type>(a0);
}
