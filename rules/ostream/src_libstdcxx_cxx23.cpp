#include <barrier>
#include <ostream>
#include <utility>

template <typename... T1>
void f18(std::ostream &a0, std::format_string<T1...> a1, T1 &&...a2) {
  return std::print<T1...>(a0, std::forward<std::format_string<T1...>>(a1),
                           std::forward<T1>(a2)...);
}

void f19(std::ostream &a0) { return std::println(a0); }

template <typename... T1>
void f20(std::ostream &a0, std::format_string<T1...> a1, T1 &&...a2) {
  return std::println<T1...>(a0, std::forward<std::format_string<T1...>>(a1),
                             std::forward<T1>(a2)...);
}

void f21(std::ostream &a0, std::string_view a1, std::format_args a2) {
  return std::vprint_nonunicode(a0, std::move(a1), std::move(a2));
}

void f22(std::ostream &a0, std::string_view a1, std::format_args a2) {
  return std::vprint_unicode(a0, std::move(a1), std::move(a2));
}
