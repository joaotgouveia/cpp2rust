#include <iterator>
#include <utility>

template <typename T1>
  requires((!std::__has_member_difference_type<T1>))
using t10 = std::incrementable_traits<T1>;

template <typename T1> using t11 = std::iter_difference_t<T1>;
