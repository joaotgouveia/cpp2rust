#include <barrier>
#include <utility>

template <typename T1> using t1 = std::reference_wrapper<T1>;

template <typename T1> using t2 = typename std::reference_wrapper<T1>::type;

template <typename T1>
std::reference_wrapper<T1> f1(const std::reference_wrapper<T1> &a0) {
  return std::reference_wrapper<T1>(a0);
}

template <typename T1>
std::reference_wrapper<T1> &f2(std::reference_wrapper<T1> &a0,
                               const std::reference_wrapper<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1> T1 &f3(const std::reference_wrapper<T1> &a0) {
  return static_cast<T1 &>(a0);
}

template <typename T1> T1 &f4(const std::reference_wrapper<T1> &a0) {
  return a0.get();
}

template <typename T1, typename T2,
          typename T3 = typename std::enable_if<!std::is_same<
              std::reference_wrapper<T1>,
              typename std::remove_cv<typename std::remove_reference<
                  T2>::type>::type>::value>::type>
std::reference_wrapper<T1> f5(T2 &&a0)
  requires(!(std::is_same<T2, const std::reference_wrapper<T1> &>::value))
{
  return std::reference_wrapper<T1>(std::forward<T2>(a0));
}

template <typename T1, typename... T2>
auto f6(const std::reference_wrapper<T1> &a0, T2 &&...a1) {
  return a0.template operator()<T2...>(std::forward<T2>(a1)...);
}
