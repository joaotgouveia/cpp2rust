#include <algorithm>
#include <utility>

template <typename T1> using t9 = std::const_iterator<T1>;

template <typename T1> using t10 = std::const_sentinel<T1>;

template <typename T1> using t11 = std::iter_const_reference_t<T1>;

template <typename T1>
std::const_iterator<T1> f54(T1 a0)
  requires(std::input_iterator<T1>)
{
  return std::make_const_iterator<T1>(std::move(a0));
}

template <typename T1>
std::const_sentinel<T1> f55(T1 a0)
  requires(std::semiregular<T1>)
{
  return std::make_const_sentinel<T1>(std::move(a0));
}
