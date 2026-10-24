#include <algorithm>
#include <any>
#include <utility>

template <
    typename T1,
    typename T2 = std::enable_if_t<!std::is_same_v<std::decay_t<T1>, std::any>,
                                   std::decay_t<T1>>,
    typename T3 = std::__conditional_t<
        std::integral_constant<
            bool,
            std::is_nothrow_move_constructible<
                std::enable_if_t<!std::is_same_v<std::decay_t<T1>, std::any>,
                                 std::decay_t<T1>>>::value &&
                (sizeof(std::enable_if_t<
                        !std::is_same_v<std::decay_t<T1>, std::any>,
                        std::decay_t<T1>>) <= sizeof(std::any::_Storage)) &&
                (alignof(std::enable_if_t<
                         !std::is_same_v<std::decay_t<T1>, std::any>,
                         std::decay_t<T1>>) <=
                 alignof(std::any::_Storage))>::value,
        std::any::_Manager_internal<std::enable_if_t<
            !std::is_same_v<std::decay_t<T1>, std::any>, std::decay_t<T1>>>,
        std::any::_Manager_external<std::enable_if_t<
            !std::is_same_v<std::decay_t<T1>, std::any>, std::decay_t<T1>>>>,
    std::enable_if_t<
        std::is_copy_constructible_v<std::enable_if_t<
            !std::is_same_v<std::decay_t<T1>, std::any>, std::decay_t<T1>>> &&
            !std::__is_in_place_type_v<std::enable_if_t<
                !std::is_same_v<std::decay_t<T1>, std::any>, std::decay_t<T1>>>,
        bool>
        T4 = true>
std::any f30(T1 &&a0)
  requires(!(std::is_same<T1, const std::any &>::value) &&
           !(std::is_same<T1, std::any>::value))
{
  return std::any(std::forward<T1>(a0));
}

template <typename T1>
std::enable_if_t<
    std::is_copy_constructible<std::enable_if_t<
        !std::is_same_v<std::decay_t<T1>, std::any>, std::decay_t<T1>>>::value,
    std::any &>
f31(std::any &a0, T1 &&a1)
  requires(!(std::is_same<T1, const std::any &>::value) &&
           !(std::is_same<T1, std::any>::value))
{
  return a0.template operator= <T1>(std::forward<T1>(a1));
}
