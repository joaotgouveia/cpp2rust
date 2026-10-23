#include <memory>
#include <utility>

template <typename T1, typename T2>
std::auto_ptr<T1> f20(std::auto_ptr<T2> &a0)
  requires(!(std::is_same<T2, T1>::value))
{
  return {a0};
}

template <typename T1, typename T2>
std::auto_ptr<T1> &f21(std::auto_ptr<T1> &a0, std::auto_ptr<T2> &a1)
  requires(!(std::is_same<T2, T1>::value))
{
  return a0.template operator= <T2>(a1);
}
