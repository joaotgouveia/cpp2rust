#include <type_traits>
#include <utility>

template <typename T1> using t1 = std::add_rvalue_reference<T1>;

template <typename T1> using t2 = typename std::add_rvalue_reference<T1>::type;
