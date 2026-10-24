#include <algorithm>
#include <atomic>
#include <utility>

bool f328() { return std::atomic<char>::is_always_lock_free; }

bool f329() { return std::atomic<int>::is_always_lock_free; }

bool f330() { return std::atomic<long long>::is_always_lock_free; }

bool f331() { return std::atomic<long>::is_always_lock_free; }

bool f332() { return std::atomic<short>::is_always_lock_free; }

bool f333() { return std::atomic<wchar_t>::is_always_lock_free; }

bool f334() { return std::atomic<bool>::is_always_lock_free; }

bool f335() { return std::atomic<unsigned char>::is_always_lock_free; }

bool f336() { return std::atomic<unsigned int>::is_always_lock_free; }

bool f337() { return std::atomic<unsigned long long>::is_always_lock_free; }

bool f338() { return std::atomic<unsigned long>::is_always_lock_free; }

bool f339() { return std::atomic<char16_t>::is_always_lock_free; }

bool f340() { return std::atomic<signed char>::is_always_lock_free; }

bool f341() { return std::atomic<unsigned short>::is_always_lock_free; }

bool f342() { return std::atomic<char32_t>::is_always_lock_free; }

template <typename T1> bool f343() {
  return std::atomic<T1 *>::is_always_lock_free;
}

template <typename T1> bool f344() {
  return std::atomic<T1>::is_always_lock_free;
}
