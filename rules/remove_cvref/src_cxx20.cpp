#include <type_traits>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> using t1 = std::remove_cvref<T1>;

template <typename T1> using t2 = typename std::remove_cvref<T1>::type;
