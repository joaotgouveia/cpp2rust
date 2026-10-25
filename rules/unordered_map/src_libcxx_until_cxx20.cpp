#include <initializer_list>
#include <unordered_map>
#include <utility>

template <typename T1>
bool f177(const std::__hash_map_const_iterator<T1> &a0,
          const std::__hash_map_const_iterator<T1> &a1) {
  return operator!=(a0, a1);
}

template <typename T1>
bool f178(const std::__hash_map_iterator<T1> &a0,
          const std::__hash_map_iterator<T1> &a1) {
  return operator!=(a0, a1);
}
