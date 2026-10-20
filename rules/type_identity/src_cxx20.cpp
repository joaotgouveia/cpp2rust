#include "../common/init.h"
#include "../common/traits.h"
#include <type_traits>
#include <utility>

template <typename T1> using t1 = std::type_identity<T1>;

template <typename T1> using t2 = typename std::type_identity<T1>::type;
