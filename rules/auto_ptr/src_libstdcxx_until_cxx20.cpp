#include <memory>
#include <utility>

template <
    typename T1, typename T2,
    typename std::enable_if<(!(std::is_same<T2, T1>::value)), int>::type T3 = 0>
std::auto_ptr<T1> f17(std::auto_ptr<T2> &a0) {
  return {a0};
}

template <
    typename T1, typename T2,
    typename std::enable_if<(!(std::is_same<T2, T1>::value)), int>::type T3 = 0>
std::auto_ptr<T1> &f18(std::auto_ptr<T1> &a0, std::auto_ptr<T2> &a1) {
  return a0.template operator= <T2>(a1);
}
