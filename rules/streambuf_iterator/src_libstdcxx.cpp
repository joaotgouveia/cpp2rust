#include <ios>
#include <utility>

template <typename T1, typename T2>
typename __gnu_cxx::__enable_if<std::__is_char<T1>::__value, void>::__type
f1(std::istreambuf_iterator<T1> &a0, T2 a1) {
  return std::advance<T1, T2>(a0, std::move(a1));
}

template <typename T1>
typename __gnu_cxx::__enable_if<std::__is_char<T1>::__value,
                                std::ostreambuf_iterator<T1>>::__type
f2(std::istreambuf_iterator<T1> a0, std::istreambuf_iterator<T1> a1,
   std::ostreambuf_iterator<T1> a2) {
  return std::copy<T1>(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1>
typename __gnu_cxx::__enable_if<std::__is_char<T1>::__value,
                                std::istreambuf_iterator<T1>>::__type
f3(std::istreambuf_iterator<T1> a0, std::istreambuf_iterator<T1> a1,
   const T1 &a2) {
  return std::find<T1>(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2>
bool f4(const std::istreambuf_iterator<T1, T2> &a0,
        const std::istreambuf_iterator<T1, T2> &a1) {
  return std::operator== <T1, T2>(a0, a1);
}
