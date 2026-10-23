#include <type_traits>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> using t3 = std::result_of_t<T1>;
