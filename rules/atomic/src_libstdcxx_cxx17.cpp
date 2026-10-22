#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <atomic>
#include <utility>

bool f207() { return std::atomic<char>::is_always_lock_free; }

bool f208() { return std::atomic<int>::is_always_lock_free; }

bool f209() { return std::atomic<long long>::is_always_lock_free; }

bool f210() { return std::atomic<long>::is_always_lock_free; }

bool f211() { return std::atomic<short>::is_always_lock_free; }

bool f212() { return std::atomic<wchar_t>::is_always_lock_free; }

bool f213() { return std::atomic<bool>::is_always_lock_free; }

bool f214() { return std::atomic<unsigned char>::is_always_lock_free; }

bool f215() { return std::atomic<unsigned int>::is_always_lock_free; }

bool f216() { return std::atomic<unsigned long long>::is_always_lock_free; }

bool f217() { return std::atomic<unsigned long>::is_always_lock_free; }

bool f218() { return std::atomic<char16_t>::is_always_lock_free; }

bool f219() { return std::atomic<signed char>::is_always_lock_free; }

bool f220() { return std::atomic<unsigned short>::is_always_lock_free; }

bool f221() { return std::atomic<char32_t>::is_always_lock_free; }

template <typename T1> bool f222() {
  return std::atomic<T1 *>::is_always_lock_free;
}

template <typename T1> bool f223() {
  return std::atomic<T1>::is_always_lock_free;
}
