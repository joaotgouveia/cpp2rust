#include <algorithm>
#include <memory>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> using t1 = std::pointer_traits<T1 *>;

template <typename T1> using t2 = typename std::pointer_traits<T1 *>::pointer;

template <typename T1>
using t3 = typename std::pointer_traits<T1 *>::element_type;

template <typename T1>
using t4 = typename std::pointer_traits<T1 *>::difference_type;

template <typename T1> using t5 = std::pointer_traits<T1>;
