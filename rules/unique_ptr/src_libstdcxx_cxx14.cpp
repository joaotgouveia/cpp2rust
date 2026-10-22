#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <condition_variable>
#include <utility>

template <typename T1> auto f63(std::size_t a0) {
  return std::make_unique<T1>(a0);
}
