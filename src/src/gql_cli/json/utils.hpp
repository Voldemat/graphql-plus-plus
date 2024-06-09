#pragma once

#include <rapidjson/allocators.h>
#include <rapidjson/document.h>
#include <rapidjson/encodings.h>

using JSONValue = rapidjson::GenericValue<
    rapidjson::UTF8<char>,
    rapidjson::MemoryPoolAllocator<rapidjson::CrtAllocator>>;
using JSONObjectEntry =
    rapidjson::GenericMember<rapidjson::UTF8<>,
                             rapidjson::MemoryPoolAllocator<>>;
