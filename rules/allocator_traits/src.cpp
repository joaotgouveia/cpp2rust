#include "../common/init.h"
#include "../common/traits.h"
#include <memory>
#include <string>
#include <utility>

template <typename T1> using t1 = std::allocator_traits<T1>;

template <typename T1>
using t2 = typename std::allocator_traits<T1>::allocator_type;

template <typename T1>
using t3 = typename std::allocator_traits<T1>::value_type;

template <typename T1> using t4 = typename std::allocator_traits<T1>::pointer;

template <typename T1>
using t5 = typename std::allocator_traits<T1>::const_pointer;

template <typename T1>
using t6 = typename std::allocator_traits<T1>::void_pointer;

template <typename T1>
using t7 = typename std::allocator_traits<T1>::const_void_pointer;

template <typename T1>
using t8 = typename std::allocator_traits<T1>::difference_type;

template <typename T1> using t9 = typename std::allocator_traits<T1>::size_type;

template <typename T1>
using t10 =
    typename std::allocator_traits<T1>::propagate_on_container_copy_assignment;

template <typename T1>
using t11 =
    typename std::allocator_traits<T1>::propagate_on_container_move_assignment;

template <typename T1>
using t12 = typename std::allocator_traits<T1>::is_always_equal;
