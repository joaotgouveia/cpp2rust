#include <type_traits>
#include <utility>

template <typename T1> using t1 = std::add_lvalue_reference_t<T1>;

template <typename T1> using t2 = std::add_rvalue_reference_t<T1>;
