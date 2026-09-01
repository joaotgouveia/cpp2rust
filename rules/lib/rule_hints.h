// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#pragma once

#include <chrono>
#include <cstddef>
#include <cstdint>
#include <cwchar>
#include <filesystem>
#include <format>
#include <limits>
#include <ratio>
#include <execution>
#include <locale>
#include <memory>
#include <ostream>
#if __cplusplus >= 202002L
#include <ranges>
#endif
#include <string>
#include <type_traits>
#include <utility>
#include <variant>
#include <vector>

#include "rule_tags.h"

namespace Synthesis {
template <typename...> struct Slot;
using BindExisting = void;
using BindAny = void *;
using BindSelf = const void *;
} // namespace Synthesis

#define MARK_INVALID_ALLOCATOR                                                 \
  template <typename RebindT> struct rebind {                                  \
    static_assert(!std::is_same_v<RebindT, RebindT>);                          \
    using other = void;                                                        \
  };

#define ARG(...) __VA_ARGS__

#define DECLARE_HINT(name)                                                     \
  template <int = 0> struct [[clang::annotate(CPP2RUST_RULE_HINT_TAG)]] name

#define DECLARE_PARAMETERIZABLE_HINT(name)                                     \
  struct [[clang::annotate(CPP2RUST_RULE_HINT_TAG),                            \
           clang::annotate(CPP2RUST_PARAMETERIZABLE_RULE_TAG)]] name

#define DECLARE_PARAMETERIZABLE_BUILTIN_HINT(name, type)                       \
  using name [[clang::annotate(CPP2RUST_RULE_HINT_TAG),                        \
               clang::annotate(CPP2RUST_BUILTIN_RULE_TAG),                     \
               clang::annotate(CPP2RUST_PARAMETERIZABLE_RULE_TAG)]] = type;

#define DECLARE_BUILTIN_HINT(name, type)                                       \
  using name [[clang::annotate(CPP2RUST_RULE_HINT_TAG),                        \
               clang::annotate(CPP2RUST_BUILTIN_RULE_TAG)]] = type;

#define DECLARE_NON_TYPE_HINT(name, type, expr)                                \
  [[clang::annotate(CPP2RUST_RULE_HINT_TAG)]] constexpr type name = expr;

#define PROBE_ONLY [[clang::annotate(CPP2RUST_PROBE_ONLY_TAG)]]

#define DECLARE_SCALAR_HINT_BODY(name, builtin)                                \
  builtin value;                                                               \
  name() = default;                                                            \
  PROBE_ONLY constexpr name(builtin initial) : value(initial) {}               \
  PROBE_ONLY constexpr operator builtin() const { return value; }              \
  template <int Id> PROBE_ONLY constexpr operator name<Id>() const {           \
    return name<Id>(value);                                                    \
  }                                                                            \
  PROBE_ONLY constexpr name &operator=(builtin other) {                        \
    value = other;                                                             \
    return *this;                                                              \
  }                                                                            \
  PROBE_ONLY constexpr name &operator+=(builtin other) {                       \
    value = static_cast<builtin>(value + other);                               \
    return *this;                                                              \
  }                                                                            \
  PROBE_ONLY constexpr name &operator-=(builtin other) {                       \
    value = static_cast<builtin>(value - other);                               \
    return *this;                                                              \
  }                                                                            \
  PROBE_ONLY constexpr name &operator*=(builtin other) {                       \
    value = static_cast<builtin>(value * other);                               \
    return *this;                                                              \
  }                                                                            \
  PROBE_ONLY constexpr name &operator/=(builtin other) {                       \
    value = static_cast<builtin>(value / other);                               \
    return *this;                                                              \
  }                                                                            \
  PROBE_ONLY constexpr name &operator++() {                                    \
    ++value;                                                                   \
    return *this;                                                              \
  }                                                                            \
  PROBE_ONLY constexpr name operator++(int) {                                  \
    const name previous = *this;                                               \
    ++value;                                                                   \
    return previous;                                                           \
  }                                                                            \
  PROBE_ONLY constexpr name &operator--() {                                    \
    --value;                                                                   \
    return *this;                                                              \
  }                                                                            \
  PROBE_ONLY constexpr name operator--(int) {                                  \
    const name previous = *this;                                               \
    --value;                                                                   \
    return previous;                                                           \
  }

#define DECLARE_INTEGRAL_HINT_BODY(name, builtin)                              \
  DECLARE_SCALAR_HINT_BODY(name, builtin)                                      \
  PROBE_ONLY constexpr name &operator%=(builtin other) {                       \
    value = static_cast<builtin>(value % other);                               \
    return *this;                                                              \
  }                                                                            \
  PROBE_ONLY constexpr name &operator&=(builtin other) {                       \
    value = static_cast<builtin>(value & other);                               \
    return *this;                                                              \
  }                                                                            \
  PROBE_ONLY constexpr name &operator|=(builtin other) {                       \
    value = static_cast<builtin>(value | other);                               \
    return *this;                                                              \
  }                                                                            \
  PROBE_ONLY constexpr name &operator^=(builtin other) {                       \
    value = static_cast<builtin>(value ^ other);                               \
    return *this;                                                              \
  }                                                                            \
  PROBE_ONLY constexpr name &operator<<=(int other) {                          \
    value = static_cast<builtin>(value << other);                              \
    return *this;                                                              \
  }                                                                            \
  PROBE_ONLY constexpr name &operator>>=(int other) {                          \
    value = static_cast<builtin>(value >> other);                              \
    return *this;                                                              \
  }

#define DECLARE_ENUM_HINT_BODY(name, builtin)                                  \
  builtin value;                                                               \
  name() = default;                                                            \
  PROBE_ONLY constexpr name(builtin initial) : value(initial) {}               \
  PROBE_ONLY constexpr operator builtin() const { return value; }              \
  template <int Id> PROBE_ONLY constexpr operator name<Id>() const {           \
    return name<Id>(value);                                                    \
  }

DECLARE_HINT(Integer) { DECLARE_INTEGRAL_HINT_BODY(Integer, int) };

DECLARE_HINT(Long) { DECLARE_INTEGRAL_HINT_BODY(Long, long) };

DECLARE_HINT(Char) { DECLARE_INTEGRAL_HINT_BODY(Char, char) };

DECLARE_HINT(WChar) { DECLARE_INTEGRAL_HINT_BODY(WChar, wchar_t) };

#if __cplusplus >= 202002L
DECLARE_HINT(Char8) { DECLARE_INTEGRAL_HINT_BODY(Char8, char8_t) };
#endif

DECLARE_HINT(Char16) { DECLARE_INTEGRAL_HINT_BODY(Char16, char16_t) };

DECLARE_HINT(Char32) { DECLARE_INTEGRAL_HINT_BODY(Char32, char32_t) };

DECLARE_HINT(ErrorCodeEnum) {
  DECLARE_ENUM_HINT_BODY(ErrorCodeEnum, std::io_errc)
};

DECLARE_HINT(ErrorConditionEnum) {
  DECLARE_ENUM_HINT_BODY(ErrorConditionEnum, std::errc)
};

DECLARE_HINT(Double) { DECLARE_SCALAR_HINT_BODY(Double, double) };

DECLARE_HINT(Void){};

DECLARE_HINT(Ratio) : public std::ratio<1> {
  template <int Id> PROBE_ONLY operator Ratio<Id>() const;
};

DECLARE_HINT(Duration) : public std::chrono::nanoseconds {
  using std::chrono::nanoseconds::duration;
  template <int Id> PROBE_ONLY operator Duration<Id>() const;
};

DECLARE_HINT(CoarseDuration) : public std::chrono::seconds {
  using std::chrono::seconds::duration;
  template <int Id> PROBE_ONLY operator CoarseDuration<Id>() const;
};

DECLARE_HINT(Clock) : public std::chrono::system_clock {
  using duration = std::chrono::system_clock::duration;
  using rep = duration::rep;
  using period = duration::period;
  using time_point = std::chrono::time_point<Clock, duration>;

  static constexpr bool is_steady = false;
  static time_point now() noexcept;

  template <int Id> PROBE_ONLY operator Clock<Id>() const;
};

DECLARE_HINT(Path) : public std::filesystem::path {
  using std::filesystem::path::path;
  template <int Id> PROBE_ONLY operator Path<Id>() const;
};

#if __cplusplus >= 202002L
DECLARE_HINT(FormatContext) : public std::format_context {
  template <int Id> PROBE_ONLY operator FormatContext<Id>() const;
};
#endif

DECLARE_HINT(MbState) : public std::mbstate_t {
  template <int Id> PROBE_ONLY operator MbState<Id>() const;
};

#if defined(__linux__)
DECLARE_HINT(ExecutionPolicy) : public std::execution::parallel_policy {
  template <int Id> PROBE_ONLY operator ExecutionPolicy<Id>() const;
};
#endif

DECLARE_HINT(UnsignedInteger) {
  DECLARE_INTEGRAL_HINT_BODY(UnsignedInteger, unsigned)
};

DECLARE_HINT(Plain){};

DECLARE_HINT(Role) : private Plain<>{};

DECLARE_HINT(Assignable) : private Plain<>{};

DECLARE_HINT(Comparable) : private Plain<> {
  template <typename Other> bool operator==(const Other &) const;
  template <typename Other> bool operator!=(const Other &) const;
  template <typename Other> bool operator<(const Other &) const;
  template <typename Other> bool operator>(const Other &) const;
  template <typename Other> bool operator<=(const Other &) const;
  template <typename Other> bool operator>=(const Other &) const;
#if __cplusplus >= 202002L
  template <typename Other>
  std::strong_ordering operator<=>(const Other &) const;
#endif
};

DECLARE_HINT(MoveAssignable) : private Assignable<> {
  MoveAssignable() = default;
  MoveAssignable(MoveAssignable &&) noexcept;
  MoveAssignable(const MoveAssignable &) = delete;
  MoveAssignable &operator=(MoveAssignable &&) noexcept;
  MoveAssignable &operator=(const MoveAssignable &) = delete;
};

DECLARE_HINT(Hashable) : private Plain<> {
  template <typename Other> bool operator==(const Other &) const;
  template <typename Other> bool operator!=(const Other &) const;
};

template <int N> struct std::hash<Hashable<N>> {
  std::size_t operator()(const Hashable<N> &) const noexcept;
};

DECLARE_HINT(ConstAssignable) : private Assignable<> {
  const ConstAssignable &operator=(const ConstAssignable &) const;
  const ConstAssignable &operator=(ConstAssignable &&) const;
};

DECLARE_HINT(ConstSwappable) : private Assignable<> {
  void swap(const ConstSwappable &) const noexcept;
  friend void swap(const ConstSwappable &, const ConstSwappable &) noexcept;
};

template <typename U = Synthesis::Slot<Synthesis::BindExisting>>
DECLARE_PARAMETERIZABLE_HINT(AssignableFrom) : private Assignable<> {
  AssignableFrom &operator=(const U &);
  AssignableFrom &operator=(U &&);
  const AssignableFrom &operator=(const U &) const;
  const AssignableFrom &operator=(U &&) const;
};

DECLARE_HINT(NonConvertibleComparable) : private Comparable<> {
  template <typename Other> bool operator==(const Other &) const;
  template <typename Other> bool operator!=(const Other &) const;
  template <typename Other> bool operator<(const Other &) const;
  template <typename Other> bool operator>(const Other &) const;
  template <typename Other> bool operator<=(const Other &) const;
  template <typename Other> bool operator>=(const Other &) const;
#if __cplusplus >= 202002L
  template <typename Other>
  std::strong_ordering operator<=>(const Other &) const;
#endif
};

template <typename> struct is_non_convertible : std::false_type {};

template <auto N>
struct is_non_convertible<NonConvertibleComparable<N>> : std::true_type {};

template <typename Tp>
inline constexpr bool is_non_convertible_v = is_non_convertible<Tp>::value;

template <typename T>
using enable_unless_non_convertible_t = std::enable_if_t<!is_non_convertible_v<
    std::remove_cv_t<std::remove_pointer_t<std::remove_reference_t<T>>>>>;

DECLARE_HINT(ExplicitlyConvertible) : private Plain<> {
  template <typename Other, typename = enable_unless_non_convertible_t<Other>>
  explicit operator Other() const;
};

template <typename T = Synthesis::Slot<Synthesis::BindExisting>>
DECLARE_PARAMETERIZABLE_HINT(ConvertibleTo) : private ExplicitlyConvertible<> {
  operator T() const;
};

DECLARE_HINT(ImplicitlyConvertible) : private ExplicitlyConvertible<> {
  template <typename Other, typename = enable_unless_non_convertible_t<Other>>
  operator Other() const;
};

DECLARE_HINT(StringLike) : private Plain<> {
  template <typename CharT, typename Traits,
            typename = enable_unless_non_convertible_t<CharT>>
  operator std::basic_string<CharT, Traits>() const;

  template <typename CharT, typename Traits,
            typename = enable_unless_non_convertible_t<CharT>>
  operator std::basic_string_view<CharT, Traits>() const;
};

template <typename T1, typename T2>
using enable_any_convertible_t =
    std::enable_if_t<std::is_convertible_v<T1, T2> ||
                     std::is_convertible_v<T2, T1>>;

template <typename InnerT = Synthesis::Slot<
              Synthesis::BindExisting, Comparable<>, ExplicitlyConvertible<>,
              ImplicitlyConvertible<>, MoveAssignable<>, UnsignedInteger<>>,
          typename DiffT = Synthesis::Slot<Long<>>>
DECLARE_PARAMETERIZABLE_HINT(InputIterator) : private Plain<> {
  MARK_INVALID_ALLOCATOR

  using value_type = InnerT;
  using difference_type = DiffT;
  using pointer = const value_type *;
  using reference = const value_type &;
  using iterator_concept = std::input_iterator_tag;
  using iterator_category = std::input_iterator_tag;

  reference operator*() const;
  pointer operator->() const;
  InputIterator &operator++();
  InputIterator operator++(int);

  template <typename OInnerT, typename ODiffT,
            typename = enable_any_convertible_t<OInnerT, InnerT>,
            typename = enable_any_convertible_t<ODiffT, DiffT>>
  bool operator==(const InputIterator<OInnerT, ODiffT> &) const;

  template <typename OInnerT, typename ODiffT,
            typename = enable_any_convertible_t<OInnerT, InnerT>,
            typename = enable_any_convertible_t<ODiffT, DiffT>>
  bool operator!=(const InputIterator<OInnerT, ODiffT> &) const;

  template <typename OInnerT, typename ODiffT,
            typename = enable_any_convertible_t<OInnerT, InnerT>,
            typename = enable_any_convertible_t<ODiffT, DiffT>>
  bool operator<(const InputIterator<OInnerT, ODiffT> &) const;

  template <typename OInnerT, typename ODiffT,
            typename = enable_any_convertible_t<OInnerT, InnerT>,
            typename = enable_any_convertible_t<ODiffT, DiffT>>
  bool operator>(const InputIterator<OInnerT, ODiffT> &) const;

  template <typename OInnerT, typename ODiffT,
            typename = enable_any_convertible_t<OInnerT, InnerT>,
            typename = enable_any_convertible_t<ODiffT, DiffT>>
  bool operator<=(const InputIterator<OInnerT, ODiffT> &) const;

  template <typename OInnerT, typename ODiffT,
            typename = enable_any_convertible_t<OInnerT, InnerT>,
            typename = enable_any_convertible_t<ODiffT, DiffT>>
  bool operator>=(const InputIterator<OInnerT, ODiffT> &) const;

#if __cplusplus >= 202002L
  template <typename OInnerT, typename ODiffT,
            typename = enable_any_convertible_t<OInnerT, InnerT>,
            typename = enable_any_convertible_t<ODiffT, DiffT>>
  std::strong_ordering operator<=>(const InputIterator<OInnerT, ODiffT> &)
      const;
#endif

  template <typename OInnerT, typename ODiffT,
            typename = enable_any_convertible_t<OInnerT, InnerT>,
            typename = enable_any_convertible_t<ODiffT, DiffT>>
  operator InputIterator<OInnerT, ODiffT>() const;
};

template <typename InnerT = Synthesis::Slot<
              Synthesis::BindExisting, Comparable<>, ExplicitlyConvertible<>,
              ImplicitlyConvertible<>, MoveAssignable<>>,
          typename DiffT = Synthesis::Slot<Long<>>>
DECLARE_PARAMETERIZABLE_HINT(Iterator) : private InputIterator<InnerT, DiffT> {
  MARK_INVALID_ALLOCATOR

  using value_type = InnerT;
  using difference_type = DiffT;
  using pointer = value_type *;
  using reference = value_type &;
  using iterator_category = std::random_access_iterator_tag;
#if __cplusplus >= 202002L
  using iterator_concept = std::contiguous_iterator_tag;
#endif

  reference operator*() const;
  pointer operator->() const;
  reference operator[](difference_type) const;

  Iterator &operator++();
  Iterator operator++(int) const;
  Iterator &operator--();
  Iterator operator--(int) const;

  Iterator &operator+=(difference_type);
  Iterator &operator-=(difference_type);
  Iterator operator+(difference_type) const;
  Iterator operator-(difference_type) const;
  difference_type operator-(const Iterator &) const;
  friend Iterator operator+(difference_type, const Iterator &);

  template <typename OInnerT, typename ODiffT,
            typename = enable_any_convertible_t<OInnerT, InnerT>,
            typename = enable_any_convertible_t<ODiffT, DiffT>>
  bool operator==(const Iterator<OInnerT, ODiffT> &) const;

  template <typename OInnerT, typename ODiffT,
            typename = enable_any_convertible_t<OInnerT, InnerT>,
            typename = enable_any_convertible_t<ODiffT, DiffT>>
  bool operator!=(const Iterator<OInnerT, ODiffT> &) const;

  template <typename OInnerT, typename ODiffT,
            typename = enable_any_convertible_t<OInnerT, InnerT>,
            typename = enable_any_convertible_t<ODiffT, DiffT>>
  bool operator<(const Iterator<OInnerT, ODiffT> &) const;

  template <typename OInnerT, typename ODiffT,
            typename = enable_any_convertible_t<OInnerT, InnerT>,
            typename = enable_any_convertible_t<ODiffT, DiffT>>
  bool operator>(const Iterator<OInnerT, ODiffT> &) const;

  template <typename OInnerT, typename ODiffT,
            typename = enable_any_convertible_t<OInnerT, InnerT>,
            typename = enable_any_convertible_t<ODiffT, DiffT>>
  bool operator<=(const Iterator<OInnerT, ODiffT> &) const;

  template <typename OInnerT, typename ODiffT,
            typename = enable_any_convertible_t<OInnerT, InnerT>,
            typename = enable_any_convertible_t<ODiffT, DiffT>>
  bool operator>=(const Iterator<OInnerT, ODiffT> &) const;

#if __cplusplus >= 202002L
  template <typename OInnerT, typename ODiffT,
            typename = enable_any_convertible_t<OInnerT, InnerT>,
            typename = enable_any_convertible_t<ODiffT, DiffT>>
  std::strong_ordering operator<=>(const Iterator<OInnerT, ODiffT> &) const;
#endif

#if defined(__linux__)
  template <typename Other = InnerT,
            typename = std::enable_if_t<std::is_same_v<Other, bool>>>
  operator std::_Bit_const_iterator() const;

  template <typename Other = InnerT,
            typename = std::enable_if_t<std::is_same_v<Other, bool>>>
  operator std::_Bit_iterator() const;
#endif

  template <typename OInnerT, typename ODiffT,
            typename = enable_any_convertible_t<OInnerT, InnerT>,
            typename = enable_any_convertible_t<ODiffT, DiffT>>
  operator Iterator<OInnerT, ODiffT>() const;
};

template <typename InnerT = Synthesis::Slot<
              Synthesis::BindExisting, Comparable<>, ExplicitlyConvertible<>,
              ImplicitlyConvertible<>, MoveAssignable<>>,
          typename DiffT = Synthesis::Slot<Long<>>>
DECLARE_PARAMETERIZABLE_HINT(BidirectionalIterator)
    : private InputIterator<InnerT, DiffT> {
  MARK_INVALID_ALLOCATOR

  using value_type = InnerT;
  using difference_type = DiffT;
  using pointer = value_type *;
  using reference = value_type &;
  using iterator_category = std::bidirectional_iterator_tag;
#if __cplusplus >= 202002L
  using iterator_concept = std::bidirectional_iterator_tag;
#endif

  reference operator*() const;
  pointer operator->() const;

  BidirectionalIterator &operator++();
  BidirectionalIterator operator++(int);
  BidirectionalIterator &operator--();
  BidirectionalIterator operator--(int);

  bool operator==(const BidirectionalIterator &) const;
  bool operator!=(const BidirectionalIterator &) const;
};

template <typename R = Synthesis::Slot<Integer<>, Synthesis::BindSelf>>
DECLARE_PARAMETERIZABLE_HINT(Callable) : private Plain<> {
  using is_transparent = void;

  template <typename... Args> R operator()(Args...) noexcept;
  template <typename... Args> R operator()(Args...) const noexcept;

  template <typename T> operator std::default_delete<T>() const;
};

template <typename A = Synthesis::Slot<Synthesis::BindExisting>>
DECLARE_PARAMETERIZABLE_HINT(UnaryCallable) : private Callable<> {
  int operator()(const A &) const noexcept;
};

template <typename K = Synthesis::Slot<Synthesis::BindExisting>>
DECLARE_PARAMETERIZABLE_HINT(Hasher) : private UnaryCallable<> {
  using is_transparent = void;
  std::size_t operator()(const K &) const noexcept;
};

template <typename A = Synthesis::Slot<Synthesis::BindExisting>>
DECLARE_PARAMETERIZABLE_HINT(BinaryPredicate) : private Callable<> {
  using is_transparent = void;
  bool operator()(const A &, const A &) const noexcept;
};

template <typename T = Synthesis::Slot<Synthesis::BindExisting, Plain<>>>
DECLARE_PARAMETERIZABLE_HINT(Allocator) : private Role<> {
  using value_type = T;

  Allocator() noexcept;
  template <typename U> Allocator(const Allocator<U> &) noexcept;

  T *allocate(std::size_t);
  void deallocate(T *, std::size_t);

  template <typename U> bool operator==(const Allocator<U> &) const noexcept;
  template <typename U> bool operator!=(const Allocator<U> &) const noexcept;
};

DECLARE_HINT(BoolConstant) : private Role<> {
  static constexpr bool value = false;
  using value_type = bool;
  constexpr operator value_type() const noexcept { return value; }
  constexpr value_type operator()() const noexcept { return value; }
};

template <typename T = Synthesis::Slot<Synthesis::BindExisting>>
DECLARE_PARAMETERIZABLE_HINT(Range) : private Plain<> {
  T *begin();
  T *end();

  const T *begin() const;
  const T *end() const;
};

#if __cplusplus >= 202002L
template <typename T = Synthesis::Slot<Synthesis::BindExisting, Plain<>>>
DECLARE_PARAMETERIZABLE_HINT(View)
    : public std::ranges::view_interface<View<T>> {
  T *begin();
  T *end();

  const T *begin() const;
  const T *end() const;
};
#endif

template <typename T1 = Synthesis::Slot<Synthesis::BindExisting, ImplicitlyConvertible<>>,
          typename T2 = Synthesis::Slot<ImplicitlyConvertible<>>>
DECLARE_PARAMETERIZABLE_BUILTIN_HINT(PairOf, ARG(std::pair<T1, T2>))

template <typename InnerT = Synthesis::Slot<
              Synthesis::BindExisting, Comparable<>, ExplicitlyConvertible<>,
              ImplicitlyConvertible<>, MoveAssignable<>>,
          typename SizeT = Synthesis::Slot<Long<>>,
          typename DiffT = Synthesis::Slot<Long<>>>
DECLARE_PARAMETERIZABLE_HINT(Container) : private Range<InnerT> {
  MARK_INVALID_ALLOCATOR

  using value_type = InnerT;
  using size_type = SizeT;
  using difference_type = DiffT;
  using reference = value_type &;
  using const_reference = const value_type &;
  using iterator = Iterator<value_type, DiffT>;
  using const_iterator = Iterator<const value_type>;
  using reverse_iterator = Iterator<value_type>;
  using const_reverse_iterator = Iterator<const value_type>;

  reference operator[](size_type) const;

  void push_back(const value_type &);
  void push_back(value_type &&);
  void push_front(const value_type &);
  void push_front(value_type &&);

  iterator insert(const_iterator, const InnerT &);
  iterator insert(const_iterator, InnerT &&);
  iterator insert(const_iterator, size_type, const InnerT &);
  template <typename InputIt> iterator insert(const_iterator, InputIt, InputIt);
  iterator insert(const_iterator, std::initializer_list<InnerT>);

  iterator begin() noexcept;
  const_iterator begin() const noexcept;
  const_iterator cbegin() const noexcept;
  reverse_iterator rbegin() noexcept;

  iterator end() noexcept;
  const_iterator end() const noexcept;
  const_iterator cend() const noexcept;
  reverse_iterator rend() noexcept;

  size_type size() const noexcept;
  bool empty() const noexcept;

  reference front();
  reference back();
  reference at(size_type);
  InnerT *data() noexcept;

  void clear() noexcept;
  void pop_back();
  template <typename... Args> reference emplace_back(Args && ...);
  iterator erase(const_iterator);
  void resize(size_type);
};

template <typename CharT = Synthesis::Slot<Synthesis::BindExisting>,
          typename IntT = Synthesis::Slot<Integer<>>>
DECLARE_PARAMETERIZABLE_HINT(CharTraits) : private Role<> {
  using char_type = CharT;
  using int_type = IntT;
  using off_type = std::streamoff;
  using pos_type = std::streampos;
  using state_type = std::mbstate_t;

  static void assign(char_type &, const char_type &);
  static char_type *assign(char_type *, std::size_t, char_type);

  static bool eq(char_type, char_type);
  static bool lt(char_type, char_type);
  static int compare(const char_type *, const char_type *, std::size_t);

  static char_type *move(char_type *, const char_type *, std::size_t);
  static char_type *copy(char_type *, const char_type *, std::size_t);

  static std::size_t length(const char_type *);

  static const char_type *find(const char_type *, std::size_t,
                               const char_type &);

  static char_type to_char_type(int_type);
  static int_type to_int_type(char_type);
  static bool eq_int_type(int_type, int_type);

  static int_type eof();
  static int_type not_eof(int_type);
};

template <typename T = Synthesis::Slot<Synthesis::BindExisting>>
DECLARE_PARAMETERIZABLE_HINT(Derived) : T{};

DECLARE_HINT(Mutex) : private Role<> {
  void lock();
  void unlock();

  bool try_lock();
  template <typename Duration> bool try_lock_for(Duration);
  template <typename Duration> bool try_lock_until(Duration);
};

DECLARE_HINT(OutputStream) : private Role<>, public std::ostream {
  OutputStream();
  template <typename T> OutputStream &operator<<(const T &);
};

template <typename InternT = Synthesis::Slot<Synthesis::BindExisting, Char<>, WChar<>>,
          typename ExternT = Synthesis::Slot<Char<>>>
DECLARE_PARAMETERIZABLE_HINT(Facet)
    : private Role<>, public std::codecvt<InternT, ExternT, std::mbstate_t>{};

DECLARE_HINT(SeedSequence) : private Role<> {
  using result_type = std::uint_least32_t;
  template <typename RandomIt> void generate(RandomIt, RandomIt);
};

template <typename T = Synthesis::Slot<Synthesis::BindExisting, UnsignedInteger<>>>
DECLARE_PARAMETERIZABLE_HINT(NumberGenerator) : private Role<> {
  using result_type = T;
  static constexpr result_type min() { return T{}; }
  static constexpr result_type max() { return std::numeric_limits<T>::max(); }
  result_type operator()();
  void seed();
  void seed(result_type);
  template <typename Sseq> void seed(Sseq &);
  void discard(unsigned long long);
  bool operator==(const NumberGenerator &) const;
  bool operator!=(const NumberGenerator &) const;
};

template <typename T =
              Synthesis::Slot<Synthesis::BindExisting, ImplicitlyConvertible<>>>
DECLARE_PARAMETERIZABLE_BUILTIN_HINT(Const, const T)

template <typename T = Synthesis::Slot<Synthesis::BindExisting>>
DECLARE_PARAMETERIZABLE_BUILTIN_HINT(Pointer, T *)

template <typename T = Synthesis::Slot<Synthesis::BindExisting, Plain<>>>
DECLARE_PARAMETERIZABLE_HINT(SmartPointer) : private Role<> {
  using element_type = T;
  using pointer = T *;
  pointer get() const noexcept;
  pointer release() noexcept;
  void reset(pointer = nullptr) noexcept;
};

DECLARE_BUILTIN_HINT(CString, const char *)

template <typename T = Synthesis::Slot<Synthesis::BindExisting, Char<>>>
DECLARE_PARAMETERIZABLE_HINT(OutputIterator) : private Plain<> {
  using value_type = T;
  using difference_type = long;
  using pointer = T *;
  using reference = T &;
  using iterator_category = std::output_iterator_tag;

  reference operator*() const;
  OutputIterator &operator++();
  OutputIterator operator++(int);
};

template <typename T = Synthesis::Slot<Synthesis::BindExisting, Plain<>>>
DECLARE_PARAMETERIZABLE_BUILTIN_HINT(Array, T[1])

template <typename T = Synthesis::Slot<Synthesis::BindExisting>>
DECLARE_PARAMETERIZABLE_BUILTIN_HINT(DefaultDelete, std::default_delete<T>)

DECLARE_NON_TYPE_HINT(NonNullInteger, int, 1)

DECLARE_NON_TYPE_HINT(IndexInteger, std::size_t, 0)

DECLARE_NON_TYPE_HINT(TrueValue, bool, true)
DECLARE_NON_TYPE_HINT(LargeInteger, int, 64)

#if __cplusplus >= 202002L
DECLARE_NON_TYPE_HINT(SizedSubRangeKind, std::ranges::subrange_kind,
                      std::ranges::subrange_kind::sized)
#endif

namespace std {

template <int N> struct __is_integral_helper<Integer<N>> : true_type {};
template <int N> struct __is_integral_helper<Long<N>> : true_type {};
template <int N> struct __is_integral_helper<Char<N>> : true_type {};
template <int N> struct __is_integral_helper<WChar<N>> : true_type {};
#if __cplusplus >= 202002L
template <int N> struct __is_integral_helper<Char8<N>> : true_type {};
#endif
template <int N> struct __is_integral_helper<Char16<N>> : true_type {};
template <int N> struct __is_integral_helper<Char32<N>> : true_type {};
template <int N> struct __is_integral_helper<UnsignedInteger<N>> : true_type {};
template <int N> struct __is_floating_point_helper<Double<N>> : true_type {};

template <int N> struct is_integral<Integer<N>> : true_type {};
template <int N> struct is_integral<Long<N>> : true_type {};
template <int N> struct is_integral<Char<N>> : true_type {};
template <int N> struct is_integral<WChar<N>> : true_type {};
#if __cplusplus >= 202002L
template <int N> struct is_integral<Char8<N>> : true_type {};
#endif
template <int N> struct is_integral<Char16<N>> : true_type {};
template <int N> struct is_integral<Char32<N>> : true_type {};
template <int N> struct is_integral<UnsignedInteger<N>> : true_type {};
template <int N> struct is_floating_point<Double<N>> : true_type {};
template <int N> struct is_void<Void<N>> : true_type {};
template <int N> struct __is_ratio<Ratio<N>> : true_type {};
template <int N> inline constexpr bool __is_ratio_v<Ratio<N>> = true;

template <int N> struct is_signed<Integer<N>> : true_type {};
template <int N> struct is_signed<Long<N>> : true_type {};
template <int N> struct is_signed<Char<N>> : true_type {};
template <int N> struct is_signed<WChar<N>> : true_type {};
template <int N> struct is_signed<Double<N>> : true_type {};
#if __cplusplus >= 202002L
template <int N> struct is_unsigned<Char8<N>> : true_type {};
#endif
template <int N> struct is_unsigned<Char16<N>> : true_type {};
template <int N> struct is_unsigned<Char32<N>> : true_type {};
template <int N> struct is_unsigned<UnsignedInteger<N>> : true_type {};

template <int N> struct numeric_limits<Integer<N>> : numeric_limits<int> {};
template <int N> struct numeric_limits<Long<N>> : numeric_limits<long> {};
template <int N> struct numeric_limits<Char<N>> : numeric_limits<char> {};
template <int N> struct numeric_limits<WChar<N>> : numeric_limits<wchar_t> {};
#if __cplusplus >= 202002L
template <int N> struct numeric_limits<Char8<N>> : numeric_limits<char8_t> {};
#endif
template <int N>
struct numeric_limits<Char16<N>> : numeric_limits<char16_t> {};
template <int N>
struct numeric_limits<Char32<N>> : numeric_limits<char32_t> {};
template <int N>
struct numeric_limits<UnsignedInteger<N>> : numeric_limits<unsigned> {};
template <int N> struct numeric_limits<Double<N>> : numeric_limits<double> {};

template <int N> struct hash<Integer<N>> {
  std::size_t operator()(const Integer<N> &) const noexcept;
};
template <int N> struct hash<Long<N>> {
  std::size_t operator()(const Long<N> &) const noexcept;
};
template <int N> struct hash<Char<N>> {
  std::size_t operator()(const Char<N> &) const noexcept;
};
template <int N> struct hash<WChar<N>> {
  std::size_t operator()(const WChar<N> &) const noexcept;
};
#if __cplusplus >= 202002L
template <int N> struct hash<Char8<N>> {
  std::size_t operator()(const Char8<N> &) const noexcept;
};
#endif
template <int N> struct hash<Char16<N>> {
  std::size_t operator()(const Char16<N> &) const noexcept;
};
template <int N> struct hash<Char32<N>> {
  std::size_t operator()(const Char32<N> &) const noexcept;
};
template <int N> struct hash<UnsignedInteger<N>> {
  std::size_t operator()(const UnsignedInteger<N> &) const noexcept;
};
template <int N> struct hash<Double<N>> {
  std::size_t operator()(const Double<N> &) const noexcept;
};

template <int N> struct is_same<Char<N>, char> : true_type {};
template <int N> struct is_same<char, Char<N>> : true_type {};
template <int N> inline constexpr bool is_same_v<Char<N>, char> = true;
template <int N> inline constexpr bool is_same_v<char, Char<N>> = true;
template <int N> struct is_same<WChar<N>, wchar_t> : true_type {};
template <int N> struct is_same<wchar_t, WChar<N>> : true_type {};
template <int N> inline constexpr bool is_same_v<WChar<N>, wchar_t> = true;
template <int N> inline constexpr bool is_same_v<wchar_t, WChar<N>> = true;
#if __cplusplus >= 202002L
template <int N> struct is_same<Char8<N>, char8_t> : true_type {};
template <int N> struct is_same<char8_t, Char8<N>> : true_type {};
template <int N> inline constexpr bool is_same_v<Char8<N>, char8_t> = true;
template <int N> inline constexpr bool is_same_v<char8_t, Char8<N>> = true;
#endif
template <int N> struct is_same<Char16<N>, char16_t> : true_type {};
template <int N> struct is_same<char16_t, Char16<N>> : true_type {};
template <int N> inline constexpr bool is_same_v<Char16<N>, char16_t> = true;
template <int N> inline constexpr bool is_same_v<char16_t, Char16<N>> = true;
template <int N> struct is_same<Char32<N>, char32_t> : true_type {};
template <int N> struct is_same<char32_t, Char32<N>> : true_type {};
template <int N> inline constexpr bool is_same_v<Char32<N>, char32_t> = true;
template <int N> inline constexpr bool is_same_v<char32_t, Char32<N>> = true;

template <int N> struct make_signed<Integer<N>> {
  using type = Integer<N>;
};
template <int N> struct make_signed<Long<N>> {
  using type = Long<N>;
};
template <int N> struct make_signed<Char<N>> {
  using type = Char<N>;
};
template <int N> struct make_signed<WChar<N>> {
  using type = WChar<N>;
};
template <int N> struct make_signed<UnsignedInteger<N>> {
  using type = Integer<N>;
};
template <int N> struct make_unsigned<Char<N>> {
  using type = Char<N>;
};
template <int N> struct make_unsigned<WChar<N>> {
  using type = WChar<N>;
};
#if __cplusplus >= 202002L
template <int N> struct make_unsigned<Char8<N>> {
  using type = Char8<N>;
};
#endif
template <int N> struct make_unsigned<Char16<N>> {
  using type = Char16<N>;
};
template <int N> struct make_unsigned<Char32<N>> {
  using type = Char32<N>;
};
template <int N> struct make_unsigned<Integer<N>> {
  using type = UnsignedInteger<N>;
};
template <int N> struct make_unsigned<Long<N>> {
  using type = UnsignedInteger<N>;
};
template <int N> struct make_unsigned<UnsignedInteger<N>> {
  using type = UnsignedInteger<N>;
};

template <int N> struct is_error_code_enum<ErrorCodeEnum<N>> : true_type {};
template <int N>
struct is_error_condition_enum<ErrorConditionEnum<N>> : true_type {};

template <int N> struct __byte_operand<Integer<N>> {
  using __type = byte;
};
template <int N> struct __byte_operand<Long<N>> {
  using __type = byte;
};
template <int N> struct __byte_operand<Char<N>> {
  using __type = byte;
};
template <int N> struct __byte_operand<WChar<N>> {
  using __type = byte;
};
#if __cplusplus >= 202002L
template <int N> struct __byte_operand<Char8<N>> {
  using __type = byte;
};
#endif
template <int N> struct __byte_operand<Char16<N>> {
  using __type = byte;
};
template <int N> struct __byte_operand<Char32<N>> {
  using __type = byte;
};
template <int N> struct __byte_operand<UnsignedInteger<N>> {
  using __type = byte;
};

template <int N> struct __is_integer<Integer<N>> {
  enum { __value = 1 };
  typedef __true_type __type;
};

template <int N> struct __is_integer<Long<N>> {
  enum { __value = 1 };
  typedef __true_type __type;
};

template <int N> struct __is_integer<Char<N>> {
  enum { __value = 1 };
  typedef __true_type __type;
};

template <int N> struct __is_integer<WChar<N>> {
  enum { __value = 1 };
  typedef __true_type __type;
};

#if __cplusplus >= 202002L
template <int N> struct __is_integer<Char8<N>> {
  enum { __value = 1 };
  typedef __true_type __type;
};
#endif

template <int N> struct __is_integer<Char16<N>> {
  enum { __value = 1 };
  typedef __true_type __type;
};

template <int N> struct __is_integer<Char32<N>> {
  enum { __value = 1 };
  typedef __true_type __type;
};

template <int N> struct __is_integer<UnsignedInteger<N>> {
  enum { __value = 1 };
  typedef __true_type __type;
};
} // namespace std

namespace std::chrono {

template <int N> struct __is_duration<Duration<N>> : true_type {};
template <int N> struct __is_duration<CoarseDuration<N>> : true_type {};

} // namespace std::chrono

namespace __gnu_cxx {

template <int N> struct __promote<Double<N>, false> {
  typedef double __type;
};

template <int N>
struct __numeric_traits_integer<Integer<N>> : __numeric_traits_integer<int> {};
template <int N>
struct __numeric_traits_integer<Long<N>> : __numeric_traits_integer<long> {};
template <int N>
struct __numeric_traits_integer<Char<N>> : __numeric_traits_integer<char> {};
template <int N>
struct __numeric_traits_integer<WChar<N>>
    : __numeric_traits_integer<wchar_t> {};
#if __cplusplus >= 202002L
template <int N>
struct __numeric_traits_integer<Char8<N>>
    : __numeric_traits_integer<char8_t> {};
#endif
template <int N>
struct __numeric_traits_integer<Char16<N>>
    : __numeric_traits_integer<char16_t> {};
template <int N>
struct __numeric_traits_integer<Char32<N>>
    : __numeric_traits_integer<char32_t> {};
template <int N>
struct __numeric_traits_integer<UnsignedInteger<N>>
    : __numeric_traits_integer<unsigned> {};

} // namespace __gnu_cxx

#if defined(__linux__)
namespace __pstl::execution {

template <int N>
struct is_execution_policy<ExecutionPolicy<N>> : std::true_type {};

} // namespace __pstl::execution
#endif

#define Plain Plain<__COUNTER__>
#define Comparable Comparable<__COUNTER__>
#define MoveAssignable MoveAssignable<__COUNTER__>
#define ConstAssignable ConstAssignable<__COUNTER__>
#define Hashable Hashable<__COUNTER__>
#define ConstSwappable ConstSwappable<__COUNTER__>
#define NonConvertibleComparable NonConvertibleComparable<__COUNTER__>
#define ExplicitlyConvertible ExplicitlyConvertible<__COUNTER__>
#define ImplicitlyConvertible ImplicitlyConvertible<__COUNTER__>
#define StringLike StringLike<__COUNTER__>
#define BoolConstant BoolConstant<__COUNTER__>
#define Mutex Mutex<__COUNTER__>
#define OutputStream OutputStream<__COUNTER__>
#define SeedSequence SeedSequence<__COUNTER__>
#define Integer Integer<__COUNTER__>
#define Long Long<__COUNTER__>
#define Char Char<__COUNTER__>
#define WChar WChar<__COUNTER__>
#define UnsignedInteger UnsignedInteger<__COUNTER__>
#if __cplusplus >= 202002L
#define Char8 Char8<__COUNTER__>
#endif
#define Char16 Char16<__COUNTER__>
#define Char32 Char32<__COUNTER__>
#define Double Double<__COUNTER__>
#define ErrorCodeEnum ErrorCodeEnum<__COUNTER__>
#define ErrorConditionEnum ErrorConditionEnum<__COUNTER__>
#define Void Void<__COUNTER__>
#define Ratio Ratio<__COUNTER__>
#define Duration Duration<__COUNTER__>
#define CoarseDuration CoarseDuration<__COUNTER__>
#define Clock Clock<__COUNTER__>
#define Path Path<__COUNTER__>
#if __cplusplus >= 202002L
#define FormatContext FormatContext<__COUNTER__>
#endif
#define MbState MbState<__COUNTER__>
#define ExecutionPolicy ExecutionPolicy<__COUNTER__>
