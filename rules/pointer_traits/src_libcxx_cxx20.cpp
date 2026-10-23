#include <memory>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> auto f3(const T1 &a0) { return std::to_address<T1>(a0); }

template <typename T1> auto f4(T1 *a0) { return std::to_address<T1>(a0); }
