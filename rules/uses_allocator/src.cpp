#include "../common/init.h"
#include "../common/traits.h"
#include <memory>
#include <tuple>
#include <utility>

template <typename T1, typename T2> using t1 = std::uses_allocator<T1, T2>;
