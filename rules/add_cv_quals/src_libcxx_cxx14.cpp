#include <type_traits>
#include <utility>

template <typename T1> using t1 = std::add_const_t<T1>;

template <typename T1> using t2 = std::add_cv_t<T1>;

template <typename T1> using t3 = std::add_volatile_t<T1>;
