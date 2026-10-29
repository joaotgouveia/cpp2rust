#include <ios>
#include <utility>

template <typename T1, typename T2>
void f1(std::istreambuf_iterator<T1> &a0, T2 a1) {
  return std::advance<T1, T2>(a0, std::move(a1));
}

template <typename T1>
std::ostreambuf_iterator<T1> f2(std::istreambuf_iterator<T1> a0,
                                std::istreambuf_iterator<T1> a1,
                                std::ostreambuf_iterator<T1> a2) {
  return std::copy<T1>(std::forward<std::istreambuf_iterator<T1>>(a0),
                       std::forward<std::istreambuf_iterator<T1>>(a1),
                       std::forward<std::ostreambuf_iterator<T1>>(a2));
}

template <typename T1>
std::istreambuf_iterator<T1> f3(std::istreambuf_iterator<T1> a0,
                                std::istreambuf_iterator<T1> a1, const T1 &a2) {
  return std::find<T1>(std::forward<std::istreambuf_iterator<T1>>(a0),
                       std::forward<std::istreambuf_iterator<T1>>(a1), a2);
}

template <typename T1, typename T2>
bool f4(const std::istreambuf_iterator<T1, T2> &a0,
        const std::istreambuf_iterator<T1, T2> &a1) {
  return std::operator== <T1, T2>(a0, a1);
}
