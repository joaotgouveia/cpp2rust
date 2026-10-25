#include <algorithm>
#include <istream>
#include <ranges>
#include <utility>

template <typename T1, typename T2, typename T3>
auto f26(std::ranges::basic_istream_view<T1, T2, T3> &a0)
  requires(std::ranges::input_range<
              std::ranges::basic_istream_view<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::same_as<std::ranges::basic_istream_view<T1, T2, T3>,
                        std::remove_cv_t<
                            std::ranges::basic_istream_view<T1, T2, T3>>>) &&
          (std::default_initializable<T1> &&
           std::ranges::__detail::__stream_extractable<T1, T2, T3>)
{
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3>
auto f27(const std::ranges::basic_istream_view<T1, T2, T3> &a0)
  requires(std::ranges::input_range<
              const std::ranges::basic_istream_view<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::same_as<std::ranges::basic_istream_view<T1, T2, T3>,
                        std::remove_cv_t<
                            std::ranges::basic_istream_view<T1, T2, T3>>>) &&
          (std::default_initializable<T1> &&
           std::ranges::__detail::__stream_extractable<T1, T2, T3>)
{
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3>
auto f28(std::ranges::basic_istream_view<T1, T2, T3> &a0)
  requires(std::ranges::input_range<
              std::ranges::basic_istream_view<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::same_as<std::ranges::basic_istream_view<T1, T2, T3>,
                        std::remove_cv_t<
                            std::ranges::basic_istream_view<T1, T2, T3>>>) &&
          (std::default_initializable<T1> &&
           std::ranges::__detail::__stream_extractable<T1, T2, T3>)
{
  return a0.cend();
}

template <typename T1, typename T2, typename T3>
auto f29(const std::ranges::basic_istream_view<T1, T2, T3> &a0)
  requires(std::ranges::input_range<
              const std::ranges::basic_istream_view<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::basic_istream_view<T1, T2, T3>> &&
           std::same_as<std::ranges::basic_istream_view<T1, T2, T3>,
                        std::remove_cv_t<
                            std::ranges::basic_istream_view<T1, T2, T3>>>) &&
          (std::default_initializable<T1> &&
           std::ranges::__detail::__stream_extractable<T1, T2, T3>)
{
  return a0.cend();
}
