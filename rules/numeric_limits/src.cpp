#include <limits>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> using t1 = std::numeric_limits<const T1>;

template <typename T1> using t2 = std::numeric_limits<volatile T1>;

template <typename T1> using t3 = std::numeric_limits<const volatile T1>;

template <typename T1> using t4 = std::numeric_limits<T1>;
