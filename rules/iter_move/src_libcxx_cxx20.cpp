#include <iterator>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> using t1 = std::iter_rvalue_reference_t<T1>;
