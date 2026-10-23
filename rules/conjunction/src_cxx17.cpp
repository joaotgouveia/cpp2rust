#include <type_traits>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename... T1> using t1 = std::conjunction<T1...>;
