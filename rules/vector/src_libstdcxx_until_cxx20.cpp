#include <algorithm>
#include <utility>
#include <vector>

template <typename T1, typename T2, typename T3,
          typename T4 = std::__enable_if_t<std::is_convertible<
              T3, typename std::vector<T1, T2>::pointer>::value>>
typename std::vector<T1, T2>::iterator
f120(const __gnu_cxx::__normal_iterator<T3, std::vector<T1, T2>> &a0) {
  return typename std::vector<T1, T2>::iterator(a0);
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::__enable_if_t<std::is_convertible<
              T3, typename std::vector<T1, T2>::const_pointer>::value>>
typename std::vector<T1, T2>::const_iterator
f121(const __gnu_cxx::__normal_iterator<T3, std::vector<T1, T2>> &a0) {
  return typename std::vector<T1, T2>::const_iterator(a0);
}
