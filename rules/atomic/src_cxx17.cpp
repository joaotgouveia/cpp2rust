#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <atomic>
#include <utility>

bool f197() { return std::atomic<char>::is_always_lock_free; }

bool f198() { return std::atomic<int>::is_always_lock_free; }

bool f199() { return std::atomic<long long>::is_always_lock_free; }

bool f200() { return std::atomic<long>::is_always_lock_free; }

bool f201() { return std::atomic<short>::is_always_lock_free; }

bool f202() { return std::atomic<wchar_t>::is_always_lock_free; }

bool f203() { return std::atomic<bool>::is_always_lock_free; }

bool f204() { return std::atomic<unsigned char>::is_always_lock_free; }

bool f205() { return std::atomic<unsigned int>::is_always_lock_free; }

bool f206() { return std::atomic<unsigned long long>::is_always_lock_free; }

bool f207() { return std::atomic<unsigned long>::is_always_lock_free; }

bool f208() { return std::atomic<char16_t>::is_always_lock_free; }

bool f209() { return std::atomic<signed char>::is_always_lock_free; }

bool f210() { return std::atomic<unsigned short>::is_always_lock_free; }

bool f211() { return std::atomic<char32_t>::is_always_lock_free; }

template <typename T1> bool f212() {
  return std::atomic<T1 *>::is_always_lock_free;
}

template <typename T1> bool f213() {
  return std::atomic<T1>::is_always_lock_free;
}
