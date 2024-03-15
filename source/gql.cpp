#include <string>

#include "gql/gql.hpp"

#include <fmt/core.h>

exported_class::exported_class()
    : m_name {fmt::format("{}", "gql")}
{
}

auto exported_class::name() const -> char const*
{
  return m_name.c_str();
}
