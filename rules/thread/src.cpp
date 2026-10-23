#include <future>
#include <ostream>
#include <thread>
#include <utility>

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f1(std::basic_ostream<T1, T2> &a0,
                               std::thread::id a1) {
  return std::operator<< <T1, T2>(a0, std::move(a1));
}
