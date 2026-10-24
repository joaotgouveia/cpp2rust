#include <ios>
#include <iosfwd>
#include <sstream>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
std::basic_stringbuf<T1, T2, T3> f34(const std::basic_string<T1, T2, T4> &a0,
                                     std::ios_base::openmode a1)
  requires(!(std::is_same<T4, T3>::value))
{
  return std::basic_stringbuf<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T4> f35(const std::basic_stringbuf<T1, T2, T3> &a0,
                                  const T4 &a1)
  requires(std::__allocator_like<T4>)
{
  return a0.template str<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f36(std::basic_stringbuf<T1, T2, T3> &a0,
         const std::basic_string<T1, T2, T4> &a1)
  requires((!std::is_same_v<T4, T3>)) && (std::__allocator_like<T4>) &&
          (!(std::is_same<T4, T3>::value))
{
  return a0.template str<T4>(a1);
}
