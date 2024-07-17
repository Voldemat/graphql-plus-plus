#pragma once

#include <rapidjson/allocators.h>
#include <rapidjson/document.h>
#include <rapidjson/encodings.h>
#include <rapidjson/stringbuffer.h>
#include <rapidjson/writer.h>

using JSONValue = rapidjson::GenericValue<
    rapidjson::UTF8<char>,
    rapidjson::MemoryPoolAllocator<rapidjson::CrtAllocator>>;
using JSONObjectEntry =
    rapidjson::GenericMember<rapidjson::UTF8<>,
                             rapidjson::MemoryPoolAllocator<>>;

using JSONArray = rapidjson::GenericArray<
    true, rapidjson::GenericValue<
               rapidjson::UTF8<char>,
               rapidjson::MemoryPoolAllocator<rapidjson::CrtAllocator>>>;

using JSONWriter = rapidjson::Writer<rapidjson::StringBuffer>;
using JSONObject = rapidjson::GenericObject<
    true, rapidjson::GenericValue<
               rapidjson::UTF8<char>,
               rapidjson::MemoryPoolAllocator<rapidjson::CrtAllocator>>>;
