#include <cstddef>
#include <initializer_list>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> std::size_t f7(const std::initializer_list<T1> &a0) {
  return a0.size();
}
