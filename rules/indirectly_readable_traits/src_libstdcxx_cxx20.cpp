#include <algorithm>
#include <utility>

template <typename T1>
  requires(std::__detail::__has_member_element_type<T1> &&
           std::same_as<std::remove_cv_t<typename T1::element_type>,
                        std::remove_cv_t<typename T1::value_type>>)
using t6 = std::indirectly_readable_traits<T1>;
