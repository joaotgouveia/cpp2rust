#include "../common/init.h"
#include "../common/traits.h"
#include <limits>
#include <utility>

template <typename T1> std::float_denorm_style f798() {
  return std::numeric_limits<T1>::has_denorm;
}

template <typename T1> bool f799() {
  return std::numeric_limits<T1>::has_denorm_loss;
}
