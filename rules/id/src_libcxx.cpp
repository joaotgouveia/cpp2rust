#include <thread>
#include <utility>

bool f6(std::__thread_id a0, std::__thread_id a1) {
  return std::operator==(std::move(a0), std::move(a1));
}

using t1 = std::__thread_id;

std::__thread_id f7() { return std::__thread_id(); }

std::__thread_id f8(const std::__thread_id &a0) { return std::__thread_id(a0); }

std::__thread_id f9(std::__thread_id &&a0) {
  return std::__thread_id(std::move(a0));
}

std::__thread_id &f10(std::__thread_id &a0, const std::__thread_id &a1) {
  return a0.operator=(a1);
}

std::__thread_id &f11(std::__thread_id &a0, std::__thread_id &&a1) {
  return a0.operator=(std::move(a1));
}

std::__thread_id f12() { return std::this_thread::get_id(); }
