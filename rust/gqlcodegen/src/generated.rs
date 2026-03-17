pub enum EUserField {
    Name,
    Email,
    CreatedAt,
}

impl libgql::executor::GQLEnum for EUserField {
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "NAME" => Ok(Self::Name),
            "EMAIL" => Ok(Self::Email),
            "CREATED_AT" => Ok(Self::CreatedAt),
            _ => Err(format!("Unexpected value {} for enum EUserField", s)),
        }
    }
}

pub enum EFileField {
    Name,
    Mimetype,
    SizeInBytes,
    AuthorName,
    Tags,
    CreatedAt,
}

impl libgql::executor::GQLEnum for EFileField {
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "NAME" => Ok(Self::Name),
            "MIMETYPE" => Ok(Self::Mimetype),
            "SIZE_IN_BYTES" => Ok(Self::SizeInBytes),
            "AUTHOR_NAME" => Ok(Self::AuthorName),
            "TAGS" => Ok(Self::Tags),
            "CREATED_AT" => Ok(Self::CreatedAt),
            _ => Err(format!("Unexpected value {} for enum EFileField", s)),
        }
    }
}

pub enum EGroupField {
    Name,
    CreatedAt,
    LimitOfDownloadsPerDay,
}

impl libgql::executor::GQLEnum for EGroupField {
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "NAME" => Ok(Self::Name),
            "CREATED_AT" => Ok(Self::CreatedAt),
            "LIMIT_OF_DOWNLOADS_PER_DAY" => Ok(Self::LimitOfDownloadsPerDay),
            _ => Err(format!("Unexpected value {} for enum EGroupField", s)),
        }
    }
}

pub enum EDealColumnType {
    List,
    Number,
    Date,
}

impl libgql::executor::GQLEnum for EDealColumnType {
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "LIST" => Ok(Self::List),
            "NUMBER" => Ok(Self::Number),
            "DATE" => Ok(Self::Date),
            _ => {
                Err(format!("Unexpected value {} for enum EDealColumnType", s))
            }
        }
    }
}

pub enum EGroupUsersField {
    Name,
    Email,
}

impl libgql::executor::GQLEnum for EGroupUsersField {
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "NAME" => Ok(Self::Name),
            "EMAIL" => Ok(Self::Email),
            _ => {
                Err(format!("Unexpected value {} for enum EGroupUsersField", s))
            }
        }
    }
}

pub enum ESortDirection {
    Asc,
    Dsc,
}

impl libgql::executor::GQLEnum for ESortDirection {
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "ASC" => Ok(Self::Asc),
            "DSC" => Ok(Self::Dsc),
            _ => Err(format!("Unexpected value {} for enum ESortDirection", s)),
        }
    }
}

pub enum EUsersTagField {
    Tag,
    UsersCount,
    CreatedAt,
}

impl libgql::executor::GQLEnum for EUsersTagField {
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "TAG" => Ok(Self::Tag),
            "USERS_COUNT" => Ok(Self::UsersCount),
            "CREATED_AT" => Ok(Self::CreatedAt),
            _ => Err(format!("Unexpected value {} for enum EUsersTagField", s)),
        }
    }
}

pub struct GetUsersSortBy {
    pub field: EUserField,
    pub direction: ESortDirection,
}

impl libgql::executor::GQLInput<super::scalar::ExampleScalar>
    for GetUsersSortBy
{
    fn from_variables(
        variables: &libgql::executor::Values<super::scalar::ExampleScalar>,
    ) -> Result<Self, String> {
        Ok(GetUsersSortBy{
            field: match variables.get("field").map(|v| match v {
            libgql::executor::Value::Null => None,
            libgql::executor::Value::NonNullable(n) => Some(n)
        }).flatten().ok_or("GetUsersSortBy: Required field field is missing or null")? {libgql::executor::NonNullableValue::Literal(l) => match l {libgql::executor::LiteralValue::Scalar(scalar) => <EUserField as libgql::executor::GQLEnum>::from_str(<super::scalar::ExampleScalar as libgql::executor::Scalar>::get_str(scalar).ok_or("Unexpected non-string scalar for enum: EUserField".to_string())?),
        libgql::executor::LiteralValue::Object(_, o) => Err(format!("Unexpected object value for scalar field: {:?}", o))
        },
        libgql::executor::NonNullableValue::Array(a) => Err(format!("field: Unexpected array value for literal: {:?}", a))}?,
            direction: match variables.get("direction").map(|v| match v {
            libgql::executor::Value::Null => None,
            libgql::executor::Value::NonNullable(n) => Some(n)
        }).flatten().ok_or("GetUsersSortBy: Required field direction is missing or null")? {libgql::executor::NonNullableValue::Literal(l) => match l {libgql::executor::LiteralValue::Scalar(scalar) => <ESortDirection as libgql::executor::GQLEnum>::from_str(<super::scalar::ExampleScalar as libgql::executor::Scalar>::get_str(scalar).ok_or("Unexpected non-string scalar for enum: ESortDirection".to_string())?),
        libgql::executor::LiteralValue::Object(_, o) => Err(format!("Unexpected object value for scalar field: {:?}", o))
        },
        libgql::executor::NonNullableValue::Array(a) => Err(format!("direction: Unexpected array value for literal: {:?}", a))}?
        })
    }
}

pub struct GetGroupsSortBy {
    pub field: EGroupField,
    pub direction: ESortDirection,
}

impl libgql::executor::GQLInput<super::scalar::ExampleScalar>
    for GetGroupsSortBy
{
    fn from_variables(
        variables: &libgql::executor::Values<super::scalar::ExampleScalar>,
    ) -> Result<Self, String> {
        Ok(GetGroupsSortBy{
            field: match variables.get("field").map(|v| match v {
            libgql::executor::Value::Null => None,
            libgql::executor::Value::NonNullable(n) => Some(n)
        }).flatten().ok_or("GetGroupsSortBy: Required field field is missing or null")? {libgql::executor::NonNullableValue::Literal(l) => match l {libgql::executor::LiteralValue::Scalar(scalar) => <EGroupField as libgql::executor::GQLEnum>::from_str(<super::scalar::ExampleScalar as libgql::executor::Scalar>::get_str(scalar).ok_or("Unexpected non-string scalar for enum: EGroupField".to_string())?),
        libgql::executor::LiteralValue::Object(_, o) => Err(format!("Unexpected object value for scalar field: {:?}", o))
        },
        libgql::executor::NonNullableValue::Array(a) => Err(format!("field: Unexpected array value for literal: {:?}", a))}?,
            direction: match variables.get("direction").map(|v| match v {
            libgql::executor::Value::Null => None,
            libgql::executor::Value::NonNullable(n) => Some(n)
        }).flatten().ok_or("GetGroupsSortBy: Required field direction is missing or null")? {libgql::executor::NonNullableValue::Literal(l) => match l {libgql::executor::LiteralValue::Scalar(scalar) => <ESortDirection as libgql::executor::GQLEnum>::from_str(<super::scalar::ExampleScalar as libgql::executor::Scalar>::get_str(scalar).ok_or("Unexpected non-string scalar for enum: ESortDirection".to_string())?),
        libgql::executor::LiteralValue::Object(_, o) => Err(format!("Unexpected object value for scalar field: {:?}", o))
        },
        libgql::executor::NonNullableValue::Array(a) => Err(format!("direction: Unexpected array value for literal: {:?}", a))}?
        })
    }
}

pub struct NumberRange {
    pub start_at: Option<f32>,
    pub end_at: Option<f32>,
}

impl libgql::executor::GQLInput<super::scalar::ExampleScalar> for NumberRange {
    fn from_variables(
        variables: &libgql::executor::Values<super::scalar::ExampleScalar>,
    ) -> Result<Self, String> {
        Ok(NumberRange{
            start_at: variables.get("startAt").map(|v| match v {
            libgql::executor::Value::Null => None,
            libgql::executor::Value::NonNullable(n) => Some(n)
        }).flatten().map(|v| match v {libgql::executor::NonNullableValue::Literal(l) => match l {libgql::executor::LiteralValue::Scalar(scalar) => <f32 as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_scalar(scalar),
        libgql::executor::LiteralValue::Object(_, o) => Err(format!("Unexpected object value for scalar field: {:?}", o))
        },
        libgql::executor::NonNullableValue::Array(a) => Err(format!("startAt: Unexpected array value for literal: {:?}", a))}).transpose()?,
            end_at: variables.get("endAt").map(|v| match v {
            libgql::executor::Value::Null => None,
            libgql::executor::Value::NonNullable(n) => Some(n)
        }).flatten().map(|v| match v {libgql::executor::NonNullableValue::Literal(l) => match l {libgql::executor::LiteralValue::Scalar(scalar) => <f32 as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_scalar(scalar),
        libgql::executor::LiteralValue::Object(_, o) => Err(format!("Unexpected object value for scalar field: {:?}", o))
        },
        libgql::executor::NonNullableValue::Array(a) => Err(format!("endAt: Unexpected array value for literal: {:?}", a))}).transpose()?
        })
    }
}

pub struct TagIn {
    pub parent_tag_id: Option<uuid::Uuid>,
    pub tag: String,
}

impl libgql::executor::GQLInput<super::scalar::ExampleScalar> for TagIn {
    fn from_variables(
        variables: &libgql::executor::Values<super::scalar::ExampleScalar>,
    ) -> Result<Self, String> {
        Ok(TagIn{
            parent_tag_id: variables.get("parentTagId").map(|v| match v {
            libgql::executor::Value::Null => None,
            libgql::executor::Value::NonNullable(n) => Some(n)
        }).flatten().map(|v| match v {libgql::executor::NonNullableValue::Literal(l) => match l {libgql::executor::LiteralValue::Scalar(scalar) => <uuid::Uuid as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_scalar(scalar),
        libgql::executor::LiteralValue::Object(_, o) => Err(format!("Unexpected object value for scalar field: {:?}", o))
        },
        libgql::executor::NonNullableValue::Array(a) => Err(format!("parentTagId: Unexpected array value for literal: {:?}", a))}).transpose()?,
            tag: match variables.get("tag").map(|v| match v {
            libgql::executor::Value::Null => None,
            libgql::executor::Value::NonNullable(n) => Some(n)
        }).flatten().ok_or("TagIn: Required field tag is missing or null")? {libgql::executor::NonNullableValue::Literal(l) => match l {libgql::executor::LiteralValue::Scalar(scalar) => <String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_scalar(scalar),
        libgql::executor::LiteralValue::Object(_, o) => Err(format!("Unexpected object value for scalar field: {:?}", o))
        },
        libgql::executor::NonNullableValue::Array(a) => Err(format!("tag: Unexpected array value for literal: {:?}", a))}?
        })
    }
}

pub struct FileSortBy {
    pub field: EFileField,
    pub direction: ESortDirection,
}

impl libgql::executor::GQLInput<super::scalar::ExampleScalar> for FileSortBy {
    fn from_variables(
        variables: &libgql::executor::Values<super::scalar::ExampleScalar>,
    ) -> Result<Self, String> {
        Ok(FileSortBy{
            field: match variables.get("field").map(|v| match v {
            libgql::executor::Value::Null => None,
            libgql::executor::Value::NonNullable(n) => Some(n)
        }).flatten().ok_or("FileSortBy: Required field field is missing or null")? {libgql::executor::NonNullableValue::Literal(l) => match l {libgql::executor::LiteralValue::Scalar(scalar) => <EFileField as libgql::executor::GQLEnum>::from_str(<super::scalar::ExampleScalar as libgql::executor::Scalar>::get_str(scalar).ok_or("Unexpected non-string scalar for enum: EFileField".to_string())?),
        libgql::executor::LiteralValue::Object(_, o) => Err(format!("Unexpected object value for scalar field: {:?}", o))
        },
        libgql::executor::NonNullableValue::Array(a) => Err(format!("field: Unexpected array value for literal: {:?}", a))}?,
            direction: match variables.get("direction").map(|v| match v {
            libgql::executor::Value::Null => None,
            libgql::executor::Value::NonNullable(n) => Some(n)
        }).flatten().ok_or("FileSortBy: Required field direction is missing or null")? {libgql::executor::NonNullableValue::Literal(l) => match l {libgql::executor::LiteralValue::Scalar(scalar) => <ESortDirection as libgql::executor::GQLEnum>::from_str(<super::scalar::ExampleScalar as libgql::executor::Scalar>::get_str(scalar).ok_or("Unexpected non-string scalar for enum: ESortDirection".to_string())?),
        libgql::executor::LiteralValue::Object(_, o) => Err(format!("Unexpected object value for scalar field: {:?}", o))
        },
        libgql::executor::NonNullableValue::Array(a) => Err(format!("direction: Unexpected array value for literal: {:?}", a))}?
        })
    }
}

pub struct GroupIn {
    pub tag_ids: Vec<uuid::Uuid>,
    pub limit_of_downloads_per_day: i32,
    pub name: String,
}

impl libgql::executor::GQLInput<super::scalar::ExampleScalar> for GroupIn {
    fn from_variables(
        variables: &libgql::executor::Values<super::scalar::ExampleScalar>,
    ) -> Result<Self, String> {
        Ok(GroupIn{
            tag_ids: match variables.get("tagIds").map(|v| match v {
            libgql::executor::Value::Null => None,
            libgql::executor::Value::NonNullable(n) => Some(n)
        }).flatten().ok_or("GroupIn: Required field tagIds is missing or null")? {libgql::executor::NonNullableValue::Array(array) =>array.iter().map(|element| match element {
        libgql::executor::Value::Null => Err("Unexpected null in non-nullable array".to_string()),
        libgql::executor::Value::NonNullable(n) => match n {libgql::executor::NonNullableValue::Literal(l) => match l {libgql::executor::LiteralValue::Scalar(scalar) => <uuid::Uuid as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_scalar(scalar),
        libgql::executor::LiteralValue::Object(_, o) => Err(format!("Unexpected object value for scalar field: {:?}", o))
        },
        libgql::executor::NonNullableValue::Array(a) => Err(format!("tagIds: Unexpected array value for literal: {:?}", a))}})
        .collect::<Result<Vec<_>, String>>(),
        libgql::executor::NonNullableValue::Literal(l) => Err(format!("tagIds: Unexpected literal value for array: {:?}", l))}?,
            limit_of_downloads_per_day: match variables.get("limitOfDownloadsPerDay").map(|v| match v {
            libgql::executor::Value::Null => None,
            libgql::executor::Value::NonNullable(n) => Some(n)
        }).flatten().ok_or("GroupIn: Required field limitOfDownloadsPerDay is missing or null")? {libgql::executor::NonNullableValue::Literal(l) => match l {libgql::executor::LiteralValue::Scalar(scalar) => <i32 as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_scalar(scalar),
        libgql::executor::LiteralValue::Object(_, o) => Err(format!("Unexpected object value for scalar field: {:?}", o))
        },
        libgql::executor::NonNullableValue::Array(a) => Err(format!("limitOfDownloadsPerDay: Unexpected array value for literal: {:?}", a))}?,
            name: match variables.get("name").map(|v| match v {
            libgql::executor::Value::Null => None,
            libgql::executor::Value::NonNullable(n) => Some(n)
        }).flatten().ok_or("GroupIn: Required field name is missing or null")? {libgql::executor::NonNullableValue::Literal(l) => match l {libgql::executor::LiteralValue::Scalar(scalar) => <String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_scalar(scalar),
        libgql::executor::LiteralValue::Object(_, o) => Err(format!("Unexpected object value for scalar field: {:?}", o))
        },
        libgql::executor::NonNullableValue::Array(a) => Err(format!("name: Unexpected array value for literal: {:?}", a))}?
        })
    }
}

pub struct MultipartUploadFileIn {
    pub name: String,
    pub initial_parts_count: i32,
    pub part_size_in_bytes: i64,
    pub size_in_bytes: i64,
    pub tag_ids: Vec<uuid::Uuid>,
}

impl libgql::executor::GQLInput<super::scalar::ExampleScalar>
    for MultipartUploadFileIn
{
    fn from_variables(
        variables: &libgql::executor::Values<super::scalar::ExampleScalar>,
    ) -> Result<Self, String> {
        Ok(MultipartUploadFileIn{
            name: match variables.get("name").map(|v| match v {
            libgql::executor::Value::Null => None,
            libgql::executor::Value::NonNullable(n) => Some(n)
        }).flatten().ok_or("MultipartUploadFileIn: Required field name is missing or null")? {libgql::executor::NonNullableValue::Literal(l) => match l {libgql::executor::LiteralValue::Scalar(scalar) => <String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_scalar(scalar),
        libgql::executor::LiteralValue::Object(_, o) => Err(format!("Unexpected object value for scalar field: {:?}", o))
        },
        libgql::executor::NonNullableValue::Array(a) => Err(format!("name: Unexpected array value for literal: {:?}", a))}?,
            initial_parts_count: match variables.get("initialPartsCount").map(|v| match v {
            libgql::executor::Value::Null => None,
            libgql::executor::Value::NonNullable(n) => Some(n)
        }).flatten().ok_or("MultipartUploadFileIn: Required field initialPartsCount is missing or null")? {libgql::executor::NonNullableValue::Literal(l) => match l {libgql::executor::LiteralValue::Scalar(scalar) => <i32 as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_scalar(scalar),
        libgql::executor::LiteralValue::Object(_, o) => Err(format!("Unexpected object value for scalar field: {:?}", o))
        },
        libgql::executor::NonNullableValue::Array(a) => Err(format!("initialPartsCount: Unexpected array value for literal: {:?}", a))}?,
            part_size_in_bytes: match variables.get("partSizeInBytes").map(|v| match v {
            libgql::executor::Value::Null => None,
            libgql::executor::Value::NonNullable(n) => Some(n)
        }).flatten().ok_or("MultipartUploadFileIn: Required field partSizeInBytes is missing or null")? {libgql::executor::NonNullableValue::Literal(l) => match l {libgql::executor::LiteralValue::Scalar(scalar) => <i64 as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_scalar(scalar),
        libgql::executor::LiteralValue::Object(_, o) => Err(format!("Unexpected object value for scalar field: {:?}", o))
        },
        libgql::executor::NonNullableValue::Array(a) => Err(format!("partSizeInBytes: Unexpected array value for literal: {:?}", a))}?,
            size_in_bytes: match variables.get("sizeInBytes").map(|v| match v {
            libgql::executor::Value::Null => None,
            libgql::executor::Value::NonNullable(n) => Some(n)
        }).flatten().ok_or("MultipartUploadFileIn: Required field sizeInBytes is missing or null")? {libgql::executor::NonNullableValue::Literal(l) => match l {libgql::executor::LiteralValue::Scalar(scalar) => <i64 as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_scalar(scalar),
        libgql::executor::LiteralValue::Object(_, o) => Err(format!("Unexpected object value for scalar field: {:?}", o))
        },
        libgql::executor::NonNullableValue::Array(a) => Err(format!("sizeInBytes: Unexpected array value for literal: {:?}", a))}?,
            tag_ids: match variables.get("tagIds").map(|v| match v {
            libgql::executor::Value::Null => None,
            libgql::executor::Value::NonNullable(n) => Some(n)
        }).flatten().ok_or("MultipartUploadFileIn: Required field tagIds is missing or null")? {libgql::executor::NonNullableValue::Array(array) =>array.iter().map(|element| match element {
        libgql::executor::Value::Null => Err("Unexpected null in non-nullable array".to_string()),
        libgql::executor::Value::NonNullable(n) => match n {libgql::executor::NonNullableValue::Literal(l) => match l {libgql::executor::LiteralValue::Scalar(scalar) => <uuid::Uuid as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_scalar(scalar),
        libgql::executor::LiteralValue::Object(_, o) => Err(format!("Unexpected object value for scalar field: {:?}", o))
        },
        libgql::executor::NonNullableValue::Array(a) => Err(format!("tagIds: Unexpected array value for literal: {:?}", a))}})
        .collect::<Result<Vec<_>, String>>(),
        libgql::executor::NonNullableValue::Literal(l) => Err(format!("tagIds: Unexpected literal value for array: {:?}", l))}?
        })
    }
}

pub struct UserIn {
    pub name: String,
    pub email: String,
    pub group_ids: Vec<uuid::Uuid>,
}

impl libgql::executor::GQLInput<super::scalar::ExampleScalar> for UserIn {
    fn from_variables(
        variables: &libgql::executor::Values<super::scalar::ExampleScalar>,
    ) -> Result<Self, String> {
        Ok(UserIn{
            name: match variables.get("name").map(|v| match v {
            libgql::executor::Value::Null => None,
            libgql::executor::Value::NonNullable(n) => Some(n)
        }).flatten().ok_or("UserIn: Required field name is missing or null")? {libgql::executor::NonNullableValue::Literal(l) => match l {libgql::executor::LiteralValue::Scalar(scalar) => <String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_scalar(scalar),
        libgql::executor::LiteralValue::Object(_, o) => Err(format!("Unexpected object value for scalar field: {:?}", o))
        },
        libgql::executor::NonNullableValue::Array(a) => Err(format!("name: Unexpected array value for literal: {:?}", a))}?,
            email: match variables.get("email").map(|v| match v {
            libgql::executor::Value::Null => None,
            libgql::executor::Value::NonNullable(n) => Some(n)
        }).flatten().ok_or("UserIn: Required field email is missing or null")? {libgql::executor::NonNullableValue::Literal(l) => match l {libgql::executor::LiteralValue::Scalar(scalar) => <String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_scalar(scalar),
        libgql::executor::LiteralValue::Object(_, o) => Err(format!("Unexpected object value for scalar field: {:?}", o))
        },
        libgql::executor::NonNullableValue::Array(a) => Err(format!("email: Unexpected array value for literal: {:?}", a))}?,
            group_ids: match variables.get("groupIds").map(|v| match v {
            libgql::executor::Value::Null => None,
            libgql::executor::Value::NonNullable(n) => Some(n)
        }).flatten().ok_or("UserIn: Required field groupIds is missing or null")? {libgql::executor::NonNullableValue::Array(array) =>array.iter().map(|element| match element {
        libgql::executor::Value::Null => Err("Unexpected null in non-nullable array".to_string()),
        libgql::executor::Value::NonNullable(n) => match n {libgql::executor::NonNullableValue::Literal(l) => match l {libgql::executor::LiteralValue::Scalar(scalar) => <uuid::Uuid as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_scalar(scalar),
        libgql::executor::LiteralValue::Object(_, o) => Err(format!("Unexpected object value for scalar field: {:?}", o))
        },
        libgql::executor::NonNullableValue::Array(a) => Err(format!("groupIds: Unexpected array value for literal: {:?}", a))}})
        .collect::<Result<Vec<_>, String>>(),
        libgql::executor::NonNullableValue::Literal(l) => Err(format!("groupIds: Unexpected literal value for array: {:?}", l))}?
        })
    }
}

pub struct Filter {
    pub list_values: Option<Vec<String>>,
    pub column_id: uuid::Uuid,
    pub number_range: Option<NumberRange>,
    pub date_range: Option<FilterDateRange>,
}

impl libgql::executor::GQLInput<super::scalar::ExampleScalar> for Filter {
    fn from_variables(
        variables: &libgql::executor::Values<super::scalar::ExampleScalar>,
    ) -> Result<Self, String> {
        Ok(Filter{
            list_values: variables.get("listValues").map(|v| match v {
            libgql::executor::Value::Null => None,
            libgql::executor::Value::NonNullable(n) => Some(n)
        }).flatten().map(|v| match v {libgql::executor::NonNullableValue::Array(array) =>array.iter().map(|element| match element {
        libgql::executor::Value::Null => Err("Unexpected null in non-nullable array".to_string()),
        libgql::executor::Value::NonNullable(n) => match n {libgql::executor::NonNullableValue::Literal(l) => match l {libgql::executor::LiteralValue::Scalar(scalar) => <String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_scalar(scalar),
        libgql::executor::LiteralValue::Object(_, o) => Err(format!("Unexpected object value for scalar field: {:?}", o))
        },
        libgql::executor::NonNullableValue::Array(a) => Err(format!("listValues: Unexpected array value for literal: {:?}", a))}})
        .collect::<Result<Vec<_>, String>>(),
        libgql::executor::NonNullableValue::Literal(l) => Err(format!("listValues: Unexpected literal value for array: {:?}", l))}).transpose()?,
            column_id: match variables.get("columnId").map(|v| match v {
            libgql::executor::Value::Null => None,
            libgql::executor::Value::NonNullable(n) => Some(n)
        }).flatten().ok_or("Filter: Required field columnId is missing or null")? {libgql::executor::NonNullableValue::Literal(l) => match l {libgql::executor::LiteralValue::Scalar(scalar) => <uuid::Uuid as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_scalar(scalar),
        libgql::executor::LiteralValue::Object(_, o) => Err(format!("Unexpected object value for scalar field: {:?}", o))
        },
        libgql::executor::NonNullableValue::Array(a) => Err(format!("columnId: Unexpected array value for literal: {:?}", a))}?,
            number_range: variables.get("numberRange").map(|v| match v {
            libgql::executor::Value::Null => None,
            libgql::executor::Value::NonNullable(n) => Some(n)
        }).flatten().map(|v| match v {libgql::executor::NonNullableValue::Literal(l) => match l {libgql::executor::LiteralValue::Object(_, o) => <NumberRange as libgql::executor::GQLInput<super::scalar::ExampleScalar>>::from_variables(o),
        libgql::executor::LiteralValue::Scalar(scalar) => Err(format!("Unexpected scalar value for input field: {:?}", scalar))
        },
        libgql::executor::NonNullableValue::Array(a) => Err(format!("numberRange: Unexpected array value for literal: {:?}", a))}).transpose()?,
            date_range: variables.get("dateRange").map(|v| match v {
            libgql::executor::Value::Null => None,
            libgql::executor::Value::NonNullable(n) => Some(n)
        }).flatten().map(|v| match v {libgql::executor::NonNullableValue::Literal(l) => match l {libgql::executor::LiteralValue::Object(_, o) => <FilterDateRange as libgql::executor::GQLInput<super::scalar::ExampleScalar>>::from_variables(o),
        libgql::executor::LiteralValue::Scalar(scalar) => Err(format!("Unexpected scalar value for input field: {:?}", scalar))
        },
        libgql::executor::NonNullableValue::Array(a) => Err(format!("dateRange: Unexpected array value for literal: {:?}", a))}).transpose()?
        })
    }
}

pub struct EventFiltersIn {
    pub event_file_tags_edited: bool,
    pub event_file_downloaded: bool,
    pub event_file_deleted: bool,
    pub event_tag_approval_is_requested: bool,
    pub event_file_download_requested: bool,
    pub event_file_uploaded: bool,
}

impl libgql::executor::GQLInput<super::scalar::ExampleScalar>
    for EventFiltersIn
{
    fn from_variables(
        variables: &libgql::executor::Values<super::scalar::ExampleScalar>,
    ) -> Result<Self, String> {
        Ok(EventFiltersIn{
            event_file_tags_edited: match variables.get("eventFileTagsEdited").map(|v| match v {
            libgql::executor::Value::Null => None,
            libgql::executor::Value::NonNullable(n) => Some(n)
        }).flatten().ok_or("EventFiltersIn: Required field eventFileTagsEdited is missing or null")? {libgql::executor::NonNullableValue::Literal(l) => match l {libgql::executor::LiteralValue::Scalar(scalar) => <bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_scalar(scalar),
        libgql::executor::LiteralValue::Object(_, o) => Err(format!("Unexpected object value for scalar field: {:?}", o))
        },
        libgql::executor::NonNullableValue::Array(a) => Err(format!("eventFileTagsEdited: Unexpected array value for literal: {:?}", a))}?,
            event_file_downloaded: match variables.get("eventFileDownloaded").map(|v| match v {
            libgql::executor::Value::Null => None,
            libgql::executor::Value::NonNullable(n) => Some(n)
        }).flatten().ok_or("EventFiltersIn: Required field eventFileDownloaded is missing or null")? {libgql::executor::NonNullableValue::Literal(l) => match l {libgql::executor::LiteralValue::Scalar(scalar) => <bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_scalar(scalar),
        libgql::executor::LiteralValue::Object(_, o) => Err(format!("Unexpected object value for scalar field: {:?}", o))
        },
        libgql::executor::NonNullableValue::Array(a) => Err(format!("eventFileDownloaded: Unexpected array value for literal: {:?}", a))}?,
            event_file_deleted: match variables.get("eventFileDeleted").map(|v| match v {
            libgql::executor::Value::Null => None,
            libgql::executor::Value::NonNullable(n) => Some(n)
        }).flatten().ok_or("EventFiltersIn: Required field eventFileDeleted is missing or null")? {libgql::executor::NonNullableValue::Literal(l) => match l {libgql::executor::LiteralValue::Scalar(scalar) => <bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_scalar(scalar),
        libgql::executor::LiteralValue::Object(_, o) => Err(format!("Unexpected object value for scalar field: {:?}", o))
        },
        libgql::executor::NonNullableValue::Array(a) => Err(format!("eventFileDeleted: Unexpected array value for literal: {:?}", a))}?,
            event_tag_approval_is_requested: match variables.get("eventTagApprovalIsRequested").map(|v| match v {
            libgql::executor::Value::Null => None,
            libgql::executor::Value::NonNullable(n) => Some(n)
        }).flatten().ok_or("EventFiltersIn: Required field eventTagApprovalIsRequested is missing or null")? {libgql::executor::NonNullableValue::Literal(l) => match l {libgql::executor::LiteralValue::Scalar(scalar) => <bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_scalar(scalar),
        libgql::executor::LiteralValue::Object(_, o) => Err(format!("Unexpected object value for scalar field: {:?}", o))
        },
        libgql::executor::NonNullableValue::Array(a) => Err(format!("eventTagApprovalIsRequested: Unexpected array value for literal: {:?}", a))}?,
            event_file_download_requested: match variables.get("eventFileDownloadRequested").map(|v| match v {
            libgql::executor::Value::Null => None,
            libgql::executor::Value::NonNullable(n) => Some(n)
        }).flatten().ok_or("EventFiltersIn: Required field eventFileDownloadRequested is missing or null")? {libgql::executor::NonNullableValue::Literal(l) => match l {libgql::executor::LiteralValue::Scalar(scalar) => <bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_scalar(scalar),
        libgql::executor::LiteralValue::Object(_, o) => Err(format!("Unexpected object value for scalar field: {:?}", o))
        },
        libgql::executor::NonNullableValue::Array(a) => Err(format!("eventFileDownloadRequested: Unexpected array value for literal: {:?}", a))}?,
            event_file_uploaded: match variables.get("eventFileUploaded").map(|v| match v {
            libgql::executor::Value::Null => None,
            libgql::executor::Value::NonNullable(n) => Some(n)
        }).flatten().ok_or("EventFiltersIn: Required field eventFileUploaded is missing or null")? {libgql::executor::NonNullableValue::Literal(l) => match l {libgql::executor::LiteralValue::Scalar(scalar) => <bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_scalar(scalar),
        libgql::executor::LiteralValue::Object(_, o) => Err(format!("Unexpected object value for scalar field: {:?}", o))
        },
        libgql::executor::NonNullableValue::Array(a) => Err(format!("eventFileUploaded: Unexpected array value for literal: {:?}", a))}?
        })
    }
}

pub struct GetGroupUsersSortBy {
    pub direction: ESortDirection,
    pub field: EGroupUsersField,
}

impl libgql::executor::GQLInput<super::scalar::ExampleScalar>
    for GetGroupUsersSortBy
{
    fn from_variables(
        variables: &libgql::executor::Values<super::scalar::ExampleScalar>,
    ) -> Result<Self, String> {
        Ok(GetGroupUsersSortBy{
            direction: match variables.get("direction").map(|v| match v {
            libgql::executor::Value::Null => None,
            libgql::executor::Value::NonNullable(n) => Some(n)
        }).flatten().ok_or("GetGroupUsersSortBy: Required field direction is missing or null")? {libgql::executor::NonNullableValue::Literal(l) => match l {libgql::executor::LiteralValue::Scalar(scalar) => <ESortDirection as libgql::executor::GQLEnum>::from_str(<super::scalar::ExampleScalar as libgql::executor::Scalar>::get_str(scalar).ok_or("Unexpected non-string scalar for enum: ESortDirection".to_string())?),
        libgql::executor::LiteralValue::Object(_, o) => Err(format!("Unexpected object value for scalar field: {:?}", o))
        },
        libgql::executor::NonNullableValue::Array(a) => Err(format!("direction: Unexpected array value for literal: {:?}", a))}?,
            field: match variables.get("field").map(|v| match v {
            libgql::executor::Value::Null => None,
            libgql::executor::Value::NonNullable(n) => Some(n)
        }).flatten().ok_or("GetGroupUsersSortBy: Required field field is missing or null")? {libgql::executor::NonNullableValue::Literal(l) => match l {libgql::executor::LiteralValue::Scalar(scalar) => <EGroupUsersField as libgql::executor::GQLEnum>::from_str(<super::scalar::ExampleScalar as libgql::executor::Scalar>::get_str(scalar).ok_or("Unexpected non-string scalar for enum: EGroupUsersField".to_string())?),
        libgql::executor::LiteralValue::Object(_, o) => Err(format!("Unexpected object value for scalar field: {:?}", o))
        },
        libgql::executor::NonNullableValue::Array(a) => Err(format!("field: Unexpected array value for literal: {:?}", a))}?
        })
    }
}

pub struct DateRange {
    pub end_at: chrono::DateTime<chrono::Utc>,
    pub start_at: chrono::DateTime<chrono::Utc>,
}

impl libgql::executor::GQLInput<super::scalar::ExampleScalar> for DateRange {
    fn from_variables(
        variables: &libgql::executor::Values<super::scalar::ExampleScalar>,
    ) -> Result<Self, String> {
        Ok(DateRange{
            end_at: match variables.get("endAt").map(|v| match v {
            libgql::executor::Value::Null => None,
            libgql::executor::Value::NonNullable(n) => Some(n)
        }).flatten().ok_or("DateRange: Required field endAt is missing or null")? {libgql::executor::NonNullableValue::Literal(l) => match l {libgql::executor::LiteralValue::Scalar(scalar) => <chrono::DateTime<chrono::Utc> as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_scalar(scalar),
        libgql::executor::LiteralValue::Object(_, o) => Err(format!("Unexpected object value for scalar field: {:?}", o))
        },
        libgql::executor::NonNullableValue::Array(a) => Err(format!("endAt: Unexpected array value for literal: {:?}", a))}?,
            start_at: match variables.get("startAt").map(|v| match v {
            libgql::executor::Value::Null => None,
            libgql::executor::Value::NonNullable(n) => Some(n)
        }).flatten().ok_or("DateRange: Required field startAt is missing or null")? {libgql::executor::NonNullableValue::Literal(l) => match l {libgql::executor::LiteralValue::Scalar(scalar) => <chrono::DateTime<chrono::Utc> as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_scalar(scalar),
        libgql::executor::LiteralValue::Object(_, o) => Err(format!("Unexpected object value for scalar field: {:?}", o))
        },
        libgql::executor::NonNullableValue::Array(a) => Err(format!("startAt: Unexpected array value for literal: {:?}", a))}?
        })
    }
}

pub struct UsersTagSortBy {
    pub direction: ESortDirection,
    pub field: EUsersTagField,
}

impl libgql::executor::GQLInput<super::scalar::ExampleScalar>
    for UsersTagSortBy
{
    fn from_variables(
        variables: &libgql::executor::Values<super::scalar::ExampleScalar>,
    ) -> Result<Self, String> {
        Ok(UsersTagSortBy{
            direction: match variables.get("direction").map(|v| match v {
            libgql::executor::Value::Null => None,
            libgql::executor::Value::NonNullable(n) => Some(n)
        }).flatten().ok_or("UsersTagSortBy: Required field direction is missing or null")? {libgql::executor::NonNullableValue::Literal(l) => match l {libgql::executor::LiteralValue::Scalar(scalar) => <ESortDirection as libgql::executor::GQLEnum>::from_str(<super::scalar::ExampleScalar as libgql::executor::Scalar>::get_str(scalar).ok_or("Unexpected non-string scalar for enum: ESortDirection".to_string())?),
        libgql::executor::LiteralValue::Object(_, o) => Err(format!("Unexpected object value for scalar field: {:?}", o))
        },
        libgql::executor::NonNullableValue::Array(a) => Err(format!("direction: Unexpected array value for literal: {:?}", a))}?,
            field: match variables.get("field").map(|v| match v {
            libgql::executor::Value::Null => None,
            libgql::executor::Value::NonNullable(n) => Some(n)
        }).flatten().ok_or("UsersTagSortBy: Required field field is missing or null")? {libgql::executor::NonNullableValue::Literal(l) => match l {libgql::executor::LiteralValue::Scalar(scalar) => <EUsersTagField as libgql::executor::GQLEnum>::from_str(<super::scalar::ExampleScalar as libgql::executor::Scalar>::get_str(scalar).ok_or("Unexpected non-string scalar for enum: EUsersTagField".to_string())?),
        libgql::executor::LiteralValue::Object(_, o) => Err(format!("Unexpected object value for scalar field: {:?}", o))
        },
        libgql::executor::NonNullableValue::Array(a) => Err(format!("field: Unexpected array value for literal: {:?}", a))}?
        })
    }
}

pub struct FilterDateRange {
    pub start_at: Option<chrono::DateTime<chrono::Utc>>,
    pub end_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl libgql::executor::GQLInput<super::scalar::ExampleScalar>
    for FilterDateRange
{
    fn from_variables(
        variables: &libgql::executor::Values<super::scalar::ExampleScalar>,
    ) -> Result<Self, String> {
        Ok(FilterDateRange{
            start_at: variables.get("startAt").map(|v| match v {
            libgql::executor::Value::Null => None,
            libgql::executor::Value::NonNullable(n) => Some(n)
        }).flatten().map(|v| match v {libgql::executor::NonNullableValue::Literal(l) => match l {libgql::executor::LiteralValue::Scalar(scalar) => <chrono::DateTime<chrono::Utc> as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_scalar(scalar),
        libgql::executor::LiteralValue::Object(_, o) => Err(format!("Unexpected object value for scalar field: {:?}", o))
        },
        libgql::executor::NonNullableValue::Array(a) => Err(format!("startAt: Unexpected array value for literal: {:?}", a))}).transpose()?,
            end_at: variables.get("endAt").map(|v| match v {
            libgql::executor::Value::Null => None,
            libgql::executor::Value::NonNullable(n) => Some(n)
        }).flatten().map(|v| match v {libgql::executor::NonNullableValue::Literal(l) => match l {libgql::executor::LiteralValue::Scalar(scalar) => <chrono::DateTime<chrono::Utc> as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_scalar(scalar),
        libgql::executor::LiteralValue::Object(_, o) => Err(format!("Unexpected object value for scalar field: {:?}", o))
        },
        libgql::executor::NonNullableValue::Array(a) => Err(format!("endAt: Unexpected array value for literal: {:?}", a))}).transpose()?
        })
    }
}

pub struct PutUploadFileIn {
    pub size_in_bytes: i64,
    pub tag_ids: Vec<uuid::Uuid>,
    pub name: String,
}

impl libgql::executor::GQLInput<super::scalar::ExampleScalar>
    for PutUploadFileIn
{
    fn from_variables(
        variables: &libgql::executor::Values<super::scalar::ExampleScalar>,
    ) -> Result<Self, String> {
        Ok(PutUploadFileIn{
            size_in_bytes: match variables.get("sizeInBytes").map(|v| match v {
            libgql::executor::Value::Null => None,
            libgql::executor::Value::NonNullable(n) => Some(n)
        }).flatten().ok_or("PutUploadFileIn: Required field sizeInBytes is missing or null")? {libgql::executor::NonNullableValue::Literal(l) => match l {libgql::executor::LiteralValue::Scalar(scalar) => <i64 as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_scalar(scalar),
        libgql::executor::LiteralValue::Object(_, o) => Err(format!("Unexpected object value for scalar field: {:?}", o))
        },
        libgql::executor::NonNullableValue::Array(a) => Err(format!("sizeInBytes: Unexpected array value for literal: {:?}", a))}?,
            tag_ids: match variables.get("tagIds").map(|v| match v {
            libgql::executor::Value::Null => None,
            libgql::executor::Value::NonNullable(n) => Some(n)
        }).flatten().ok_or("PutUploadFileIn: Required field tagIds is missing or null")? {libgql::executor::NonNullableValue::Array(array) =>array.iter().map(|element| match element {
        libgql::executor::Value::Null => Err("Unexpected null in non-nullable array".to_string()),
        libgql::executor::Value::NonNullable(n) => match n {libgql::executor::NonNullableValue::Literal(l) => match l {libgql::executor::LiteralValue::Scalar(scalar) => <uuid::Uuid as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_scalar(scalar),
        libgql::executor::LiteralValue::Object(_, o) => Err(format!("Unexpected object value for scalar field: {:?}", o))
        },
        libgql::executor::NonNullableValue::Array(a) => Err(format!("tagIds: Unexpected array value for literal: {:?}", a))}})
        .collect::<Result<Vec<_>, String>>(),
        libgql::executor::NonNullableValue::Literal(l) => Err(format!("tagIds: Unexpected literal value for array: {:?}", l))}?,
            name: match variables.get("name").map(|v| match v {
            libgql::executor::Value::Null => None,
            libgql::executor::Value::NonNullable(n) => Some(n)
        }).flatten().ok_or("PutUploadFileIn: Required field name is missing or null")? {libgql::executor::NonNullableValue::Literal(l) => match l {libgql::executor::LiteralValue::Scalar(scalar) => <String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_scalar(scalar),
        libgql::executor::LiteralValue::Object(_, o) => Err(format!("Unexpected object value for scalar field: {:?}", o))
        },
        libgql::executor::NonNullableValue::Array(a) => Err(format!("name: Unexpected array value for literal: {:?}", a))}?
        })
    }
}

pub struct EventFileDownloaded {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub file: File,
}

pub struct DealColumn {
    pub r#type: EDealColumnType,
    pub available_values: Vec<String>,
    pub name: String,
    pub id: uuid::Uuid,
}

pub struct ErrorUnknownSessionId {
    pub a: Option<bool>,
}

pub struct ErrorUnknownUsers {
    pub user_ids: Vec<uuid::Uuid>,
}

pub struct ErrorAlreadyExists {
    pub a: Option<bool>,
}

pub struct ErrorAlreadyPending {
    pub a: Option<bool>,
}

pub struct UrlObject {
    pub uvalue: url::Url,
}

pub struct StringObject {
    pub svalue: String,
}

pub struct ErrorNoDealTag {
    pub a: Option<bool>,
}

pub struct EventsList {
    pub events: Vec<Event>,
}

pub struct ErrorUnknownGroupIds {
    pub group_ids: Vec<uuid::Uuid>,
}

pub struct File {
    pub mime_type: Option<String>,
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub filename: String,
    pub preview_url: Option<url::Url>,
    pub size_in_bytes: i64,
    pub user: User,
}

pub struct DealInfo {
    pub stage_to_worktypes_map: Vec<StageToWorktypesMapEntry>,
    pub values: Vec<DealEntry>,
}

pub struct UsersTag {
    pub tag: Tag,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub users_count: i32,
}

pub struct ErrorInvalidLimitOfDownloadsPerDay {
    pub a: Option<bool>,
}

pub struct ErrorUnknownParentId {
    pub a: Option<bool>,
}

pub struct StageToWorktypesMapEntry {
    pub stage: Tag,
    pub worktypes: Vec<Tag>,
}

pub struct EventFileDownloadRequested {
    pub file: File,
    pub decision: Option<bool>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub user: User,
}

pub struct ErrorFilesChangeForbidden {
    pub ids: Vec<uuid::Uuid>,
}

pub struct TagList {
    pub list: Vec<Tag>,
}

pub struct ErrorInvalidOTPCode {
    pub a: Option<bool>,
}

pub struct ErrorInvalidCredentials {
    pub a: Option<bool>,
}

pub struct ErrorUnknownUser {
    pub a: Option<bool>,
}

pub struct EventFileUploaded {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub file: File,
}

pub struct Group {
    pub name: String,
    pub limit_of_downloads_per_day: i32,
    pub id: uuid::Uuid,
    pub first_10_tags: Vec<Tag>,
}

pub struct UploadUrl {
    pub url: url::Url,
    pub headers: Vec<StringEntry>,
}

pub struct DatetimeObject {
    pub dvalue: chrono::DateTime<chrono::Utc>,
}

pub struct ErrorFileNotUploaded {
    pub a: Option<bool>,
}

pub struct ErrorMultipartUploadFileIsTooBig {
    pub a: Option<bool>,
}

pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub ten_groups: Vec<Group>,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub struct ErrorInvalidToken {
    pub a: Option<bool>,
}

pub struct ErrorNotFound {
    pub a: Option<bool>,
}

pub struct EventTagApprovalIsRequested {
    pub already_in_catalog: bool,
    pub tag: Tag,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub author: User,
}

pub struct FilesDealInfo {
    pub unset_columns: Vec<DealColumn>,
    pub deal_info: DealInfo,
    pub deal_name: Tag,
}

pub struct ErrorPutUploadFileIsTooBig {
    pub a: Option<bool>,
}

pub struct UsersList {
    pub users: Vec<User>,
}

pub struct Tag {
    pub has_children: bool,
    pub is_favourite: bool,
    pub is_approved: bool,
    pub value: Option<TagValue>,
    pub tag: String,
    pub id: uuid::Uuid,
}

pub struct DealEntry {
    pub column_name: String,
    pub value: Tag,
}

pub struct ErrorChangeForbidden {
    pub a: Option<bool>,
}

pub struct PendingUser {
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub groups: Vec<Group>,
    pub name: String,
    pub ttl: f32,
}

pub struct ErrorInvalidUserName {
    pub a: Option<bool>,
}

pub struct EventFileDeleted {
    pub file: File,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub struct Query {
    pub is_tag_exists: bool,
    pub get_group_users_and_users: GetGroupUsersAndUsersResponse,
    pub get_pending_users: Vec<PendingUser>,
    pub get_deals: Vec<String>,
    pub get_favourite_tags: Vec<Tag>,
    pub get_tag_children: GetTagsResponse,
    pub get_tags: GetTagsResponse,
    pub get_group_users_total: GetGroupUsersTotalResponse,
    pub is_allowed_to_download: IsAllowedToDownloadResponse,
    pub get_files_deal_info: FilesDealInfoOrError,
    pub get_users_total: i32,
    pub get_group_tags: GetGroupTagsResponse,
    pub get_my_tags: Vec<Tag>,
    pub retrieve_file: RetrieveFileResponse,
    pub get_popular_tags: Vec<Tag>,
    pub get_tag_info: GetTagInfoResponse,
    pub get_path_to_tag: GetPathToTagResponse,
    pub get_me: User,
    pub get_files: GetFilesResponse,
    pub get_my_tags_count: i32,
    pub get_events: GetEventsResponse,
    pub get_users: Vec<User>,
    pub get_deal_columns: Vec<DealColumn>,
    pub search_tags: Vec<Tag>,
    pub get_deal_info: GetDealInfoResponse,
    pub get_group_users: GetGroupUsersResponse,
    pub get_uploaded_files: Vec<SearchFile>,
    pub get_users_tags_count: i32,
    pub get_tags_count: IntObjectOrErrorUnknownTags,
    pub get_next_multipart_upload_urls: GetNextMultipartUploadUrlsResponse,
    pub get_groups: Vec<Group>,
    pub retrieve_group: RetrieveGroupResponse,
    pub get_file_url: GetFileURLResponse,
    pub get_uploaded_files_count: i32,
    pub get_files_count: IntObjectOrErrorUnknownTags,
    pub get_groups_total: i32,
    pub get_users_tags: Vec<UsersTag>,
}

pub struct ErrorGroupNotFound {
    pub a: Option<bool>,
}

pub struct ErrorInvalidPassword {
    pub a: Option<bool>,
}

pub struct ErrorDateRangeIsInvalid {
    pub a: Option<bool>,
}

pub struct ErrorMultipartUploadFilePartSizeIsTooBig {
    pub a: Option<bool>,
}

pub struct ErrorUnknownGroups {
    pub group_ids: Vec<uuid::Uuid>,
}

pub struct FloatObject {
    pub fvalue: f32,
}

pub struct SearchFile {
    pub file: File,
    pub tags: Vec<Tag>,
}

pub struct TagInfo {
    pub tag: String,
    pub parent_tag: Option<Tag>,
}

pub struct ErrorAlreadyDone {
    pub a: Option<bool>,
}

pub struct ErrorUnknownFile {
    pub a: Option<bool>,
}

pub struct GroupUserList {
    pub users: Vec<GroupUser>,
}

pub struct StringEntry {
    pub key: String,
    pub value: String,
}

pub struct UploadUrlList {
    pub urls: Vec<UploadUrl>,
}

pub struct ErrorOTPTokenExpired {
    pub a: Option<bool>,
}

pub struct ErrorCantAddAutotags {
    pub a: Option<bool>,
}

pub struct ErrorEmailCollision {
    pub a: Option<bool>,
}

pub struct IntObject {
    pub ivalue: i32,
}

pub struct PutUploadSession {
    pub upload_url: UploadUrl,
    pub id: uuid::Uuid,
}

pub struct ErrorMultipartUploadFilePartSizeIsTooSmall {
    pub a: Option<bool>,
}

pub struct MultipartUploadSession {
    pub initial_upload_ur_ls: Vec<UploadUrl>,
    pub id: uuid::Uuid,
}

pub struct ErrorUnknownTags {
    pub tag_ids: Vec<uuid::Uuid>,
}

pub struct ErrorInvalidEmail {
    pub a: Option<bool>,
}

pub struct GroupUser {
    pub user: User,
    pub in_group: bool,
}

pub struct SearchFileList {
    pub files: Vec<SearchFile>,
}

pub struct ErrorUnknownFiles {
    pub ids: Vec<uuid::Uuid>,
}

pub struct ErrorAlreadyApprovedByAdmin {
    pub a: Option<bool>,
}

pub struct OTPToken {
    pub token: String,
}

pub struct StringList {
    pub values: Vec<String>,
}

pub struct ErrorInvalidGroupName {
    pub a: Option<bool>,
}

pub struct ErrorMultipartUploadFileIsTooLight {
    pub a: Option<bool>,
}

pub struct ErrorOTPCodeExpired {
    pub a: Option<bool>,
}

pub struct Mutation {
    pub change_password: Option<ErrorInvalidCredentials>,
    pub edit_group: Option<EditGroupError>,
    pub create_multipart_file_session: CreateMultipartFileSessionResponse,
    pub create_tag: Option<CreateTagError>,
    pub send_otp_code: Option<ErrorInvalidCredentials>,
    pub decide_on_download_request: Option<DecideOnDownloadRequestError>,
    pub delete_files: Option<DeleteFilesError>,
    pub commit_put_file_session: Option<CommitPutFileSessionResponse>,
    pub create_group: Option<CreateGroupError>,
    pub create_put_file_session: CreatePutFileSessionResponse,
    pub logout: (),
    pub delete_user: Option<ErrorNotFound>,
    pub delete_file: Option<DeleteFileError>,
    pub approve_tag: Option<ApproveTagError>,
    pub confirm_user: Option<ConfirmUserError>,
    pub delete_tag: Option<ErrorNotFound>,
    pub delete_group: Option<ErrorGroupNotFound>,
    pub update_file: Option<UpdateFileError>,
    pub set_tag_is_favourite: Option<ErrorAlreadyDoneOrUnknownTags>,
    pub confirm_otp_code: ConfirmOTPCodeResponse,
    pub remove_user_from_group: Option<ErrorGroupNotFoundOrErrorNotFound>,
    pub create_user: Option<CreateUserError>,
    pub delete_pending_user: Option<ErrorNotFound>,
    pub add_tags_to_files: Option<AddTagsToFilesError>,
    pub edit_tag: Option<EditTagError>,
    pub update_files_autotags: Option<ErrorCantAddAutotags>,
    pub login: Option<ErrorInvalidCredentials>,
    pub commit_multipart_file_session:
        Option<CommitMultipartFileSessionResponse>,
    pub reset_password: Option<ResetPasswordError>,
    pub add_user_to_group: Option<ErrorGroupNotFoundOrErrorNotFound>,
}

pub struct BooleanObject {
    pub bvalue: bool,
}

pub struct EventFileTagsEdited {
    pub added_tags: Vec<Tag>,
    pub file: File,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub removed_tags: Vec<Tag>,
}

pub enum AddTagsToFilesError {
    ErrorUnknownFiles(ErrorUnknownFiles),
    ErrorUnknownTags(ErrorUnknownTags),
}

pub enum DecideOnDownloadRequestError {
    ErrorAlreadyDone(ErrorAlreadyDone),
    ErrorUnknownUser(ErrorUnknownUser),
    ErrorUnknownFile(ErrorUnknownFile),
    ErrorNotFound(ErrorNotFound),
}

pub enum CreatePutFileSessionResponse {
    ErrorPutUploadFileIsTooBig(ErrorPutUploadFileIsTooBig),
    PutUploadSession(PutUploadSession),
    ErrorUnknownTags(ErrorUnknownTags),
    ErrorNoDealTag(ErrorNoDealTag),
}

pub enum EditGroupError {
    ErrorInvalidLimitOfDownloadsPerDay(ErrorInvalidLimitOfDownloadsPerDay),
    ErrorInvalidGroupName(ErrorInvalidGroupName),
    ErrorGroupNotFound(ErrorGroupNotFound),
    ErrorAlreadyExists(ErrorAlreadyExists),
    ErrorUnknownTags(ErrorUnknownTags),
}

pub enum GetTagInfoResponse {
    TagInfo(TagInfo),
    ErrorNotFound(ErrorNotFound),
}

pub enum ResetPasswordError {
    ErrorInvalidPassword(ErrorInvalidPassword),
    ErrorOTPTokenExpired(ErrorOTPTokenExpired),
}

pub enum GetFilesResponse {
    SearchFileList(SearchFileList),
    ErrorUnknownTags(ErrorUnknownTags),
}

pub enum DeleteFileError {
    ErrorUnknownFile(ErrorUnknownFile),
    ErrorChangeForbidden(ErrorChangeForbidden),
}

pub enum TagValue {
    DatetimeObject(DatetimeObject),
    FloatObject(FloatObject),
    StringObject(StringObject),
}

pub enum GetDealInfoResponse {
    ErrorNotFound(ErrorNotFound),
    DealInfo(DealInfo),
}

pub enum RetrieveFileResponse {
    SearchFile(SearchFile),
    ErrorUnknownFile(ErrorUnknownFile),
}

pub enum RetrieveGroupResponse {
    ErrorNotFound(ErrorNotFound),
    Group(Group),
}

pub enum CreateGroupError {
    ErrorInvalidLimitOfDownloadsPerDay(ErrorInvalidLimitOfDownloadsPerDay),
    ErrorAlreadyExists(ErrorAlreadyExists),
    ErrorInvalidGroupName(ErrorInvalidGroupName),
    ErrorUnknownTags(ErrorUnknownTags),
    ErrorUnknownUsers(ErrorUnknownUsers),
}

pub enum IsAllowedToDownloadResponse {
    ErrorUnknownFile(ErrorUnknownFile),
    BooleanObject(BooleanObject),
}

pub enum CommitMultipartFileSessionResponse {
    ErrorFileNotUploaded(ErrorFileNotUploaded),
    File(File),
    ErrorUnknownSessionId(ErrorUnknownSessionId),
}

pub enum FilesDealInfoOrError {
    ErrorCantAddAutotags(ErrorCantAddAutotags),
    FilesDealInfo(FilesDealInfo),
}

pub enum Event {
    EventFileUploaded(EventFileUploaded),
    EventFileDownloadRequested(EventFileDownloadRequested),
    EventFileDeleted(EventFileDeleted),
    EventFileDownloaded(EventFileDownloaded),
    EventFileTagsEdited(EventFileTagsEdited),
    EventTagApprovalIsRequested(EventTagApprovalIsRequested),
}

pub enum ConfirmUserError {
    ErrorInvalidToken(ErrorInvalidToken),
    ErrorInvalidPassword(ErrorInvalidPassword),
}

pub enum CreateMultipartFileSessionResponse {
    ErrorUnknownTags(ErrorUnknownTags),
    ErrorMultipartUploadFilePartSizeIsTooBig(
        ErrorMultipartUploadFilePartSizeIsTooBig,
    ),
    ErrorMultipartUploadFileIsTooBig(ErrorMultipartUploadFileIsTooBig),
    ErrorMultipartUploadFileIsTooLight(ErrorMultipartUploadFileIsTooLight),
    ErrorMultipartUploadFilePartSizeIsTooSmall(
        ErrorMultipartUploadFilePartSizeIsTooSmall,
    ),
    ErrorNoDealTag(ErrorNoDealTag),
    MultipartUploadSession(MultipartUploadSession),
}

pub enum GetPathToTagResponse {
    ErrorUnknownTags(ErrorUnknownTags),
    StringList(StringList),
}

pub enum ConfirmOTPCodeResponse {
    ErrorInvalidOTPCode(ErrorInvalidOTPCode),
    ErrorOTPCodeExpired(ErrorOTPCodeExpired),
    OTPToken(OTPToken),
}

pub enum GetEventsResponse {
    ErrorDateRangeIsInvalid(ErrorDateRangeIsInvalid),
    EventsList(EventsList),
}

pub enum GetGroupTagsResponse {
    TagList(TagList),
    ErrorNotFound(ErrorNotFound),
}

pub enum GetFileURLResponse {
    UrlObject(UrlObject),
    ErrorUnknownFile(ErrorUnknownFile),
}

pub enum GetNextMultipartUploadUrlsResponse {
    UploadUrlList(UploadUrlList),
    ErrorUnknownSessionId(ErrorUnknownSessionId),
}

pub enum GetGroupUsersAndUsersResponse {
    GroupUserList(GroupUserList),
    ErrorGroupNotFound(ErrorGroupNotFound),
}

pub enum CommitPutFileSessionResponse {
    ErrorFileNotUploaded(ErrorFileNotUploaded),
    ErrorUnknownSessionId(ErrorUnknownSessionId),
    File(File),
}

pub enum ApproveTagError {
    ErrorAlreadyExists(ErrorAlreadyExists),
    ErrorUnknownGroupIds(ErrorUnknownGroupIds),
    ErrorNotFound(ErrorNotFound),
}

pub enum ErrorAlreadyDoneOrUnknownTags {
    ErrorAlreadyDone(ErrorAlreadyDone),
    ErrorUnknownTags(ErrorUnknownTags),
}

pub enum IntObjectOrErrorUnknownTags {
    IntObject(IntObject),
    ErrorUnknownTags(ErrorUnknownTags),
}

pub enum GetTagsResponse {
    TagList(TagList),
    ErrorUnknownTags(ErrorUnknownTags),
}

pub enum UpdateFileError {
    ErrorUnknownFile(ErrorUnknownFile),
    ErrorChangeForbidden(ErrorChangeForbidden),
    ErrorUnknownTags(ErrorUnknownTags),
}

pub enum GetGroupUsersResponse {
    UsersList(UsersList),
    ErrorNotFound(ErrorNotFound),
}

pub enum GetGroupUsersTotalResponse {
    ErrorNotFound(ErrorNotFound),
    IntObject(IntObject),
}

pub enum CreateTagError {
    ErrorAlreadyExists(ErrorAlreadyExists),
    ErrorUnknownParentId(ErrorUnknownParentId),
}

pub enum DeleteFilesError {
    ErrorUnknownFiles(ErrorUnknownFiles),
    ErrorFilesChangeForbidden(ErrorFilesChangeForbidden),
}

pub enum ErrorGroupNotFoundOrErrorNotFound {
    ErrorGroupNotFound(ErrorGroupNotFound),
    ErrorNotFound(ErrorNotFound),
}

pub enum CreateUserError {
    ErrorEmailCollision(ErrorEmailCollision),
    ErrorInvalidEmail(ErrorInvalidEmail),
    ErrorUnknownGroups(ErrorUnknownGroups),
    ErrorInvalidUserName(ErrorInvalidUserName),
    ErrorAlreadyPending(ErrorAlreadyPending),
}

pub enum EditTagError {
    ErrorUnknownParentId(ErrorUnknownParentId),
    ErrorAlreadyApprovedByAdmin(ErrorAlreadyApprovedByAdmin),
    ErrorAlreadyExists(ErrorAlreadyExists),
    ErrorNotFound(ErrorNotFound),
}

