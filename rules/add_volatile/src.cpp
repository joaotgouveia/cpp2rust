#include <type_traits>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> using t1 = std::add_volatile<T1>;

template <typename T1> using t2 = typename std::add_volatile<T1>::type;
