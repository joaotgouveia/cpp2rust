#include <algorithm>
#include <atomic>
#include <utility>

bool f805() { return std::atomic<char>::is_always_lock_free; }

bool f806() { return std::atomic<int>::is_always_lock_free; }

bool f807() { return std::atomic<long long>::is_always_lock_free; }

bool f808() { return std::atomic<long>::is_always_lock_free; }

bool f809() { return std::atomic<short>::is_always_lock_free; }

bool f810() { return std::atomic<wchar_t>::is_always_lock_free; }

bool f811() { return std::atomic<bool>::is_always_lock_free; }

bool f812() { return std::atomic<unsigned char>::is_always_lock_free; }

bool f813() { return std::atomic<unsigned int>::is_always_lock_free; }

bool f814() { return std::atomic<unsigned long long>::is_always_lock_free; }

bool f815() { return std::atomic<unsigned long>::is_always_lock_free; }

bool f816() { return std::atomic<char16_t>::is_always_lock_free; }

bool f817() { return std::atomic<signed char>::is_always_lock_free; }

bool f818() { return std::atomic<unsigned short>::is_always_lock_free; }

bool f819() { return std::atomic<char32_t>::is_always_lock_free; }

template <typename T1> bool f820() {
  return std::atomic<T1 *>::is_always_lock_free;
}

template <typename T1> bool f821() {
  return std::atomic<T1>::is_always_lock_free;
}
