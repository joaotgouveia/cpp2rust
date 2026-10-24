#include <thread>
#include <utility>

bool f1(std::__thread_id a0, std::__thread_id a1) {
  return std::operator!=(std::move(a0), std::move(a1));
}

bool f2(std::__thread_id a0, std::__thread_id a1) {
  return std::operator<(std::move(a0), std::move(a1));
}

bool f3(std::__thread_id a0, std::__thread_id a1) {
  return std::operator<=(std::move(a0), std::move(a1));
}

bool f4(std::__thread_id a0, std::__thread_id a1) {
  return std::operator>(std::move(a0), std::move(a1));
}

bool f5(std::__thread_id a0, std::__thread_id a1) {
  return std::operator>=(std::move(a0), std::move(a1));
}
