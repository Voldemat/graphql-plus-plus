#include <string>

#include "graphql--/graphql--.hpp"

#include <catch2/catch_test_macros.hpp>

TEST_CASE("Name is graphql--", "[library]")
{
  auto const exported = exported_class {};
  REQUIRE(std::string("graphql--") == exported.name());
}
