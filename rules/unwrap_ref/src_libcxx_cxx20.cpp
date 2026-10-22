#include "../common/init.h"
#include "../common/traits.h"
#include <type_traits>
#include <utility>

template <typename T1> using t1 = std::unwrap_ref_decay_t<T1>;

template <typename T1> using t2 = std::unwrap_reference_t<T1>;
