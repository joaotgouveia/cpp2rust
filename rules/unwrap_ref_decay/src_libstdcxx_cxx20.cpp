#include <type_traits>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> using t2 = typename std::unwrap_ref_decay<T1>::type;
