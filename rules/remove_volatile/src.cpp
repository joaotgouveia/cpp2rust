#include "../common/init.h"
#include "../common/traits.h"
#include <type_traits>
#include <utility>

template <typename T1> using t1 = std::remove_volatile<T1>;

template <typename T1> using t2 = typename std::remove_volatile<T1>::type;
