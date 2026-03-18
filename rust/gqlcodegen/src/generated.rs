pub enum EUsersTagField {
    Tag,
    UsersCount,
    CreatedAt,
}

impl libgql::executor::GQLEnum<super::scalar::ExampleScalar> for EUsersTagField {
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
        "TAG" => Ok(Self::Tag),
        "USERS_COUNT" => Ok(Self::UsersCount),
        "CREATED_AT" => Ok(Self::CreatedAt),
        _ => Err(format!("Unexpected value {} for enum EUsersTagField", s))
        }
    }

    fn to_str(self: &Self) -> Result<&str, String> {
        match self {
        Self::Tag => Ok("TAG"),
        Self::UsersCount => Ok("USERS_COUNT"),
        Self::CreatedAt => Ok("CREATED_AT"),
        }
    }
}

pub enum ESortDirection {
    Asc,
    Dsc,
}

impl libgql::executor::GQLEnum<super::scalar::ExampleScalar> for ESortDirection {
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
        "ASC" => Ok(Self::Asc),
        "DSC" => Ok(Self::Dsc),
        _ => Err(format!("Unexpected value {} for enum ESortDirection", s))
        }
    }

    fn to_str(self: &Self) -> Result<&str, String> {
        match self {
        Self::Asc => Ok("ASC"),
        Self::Dsc => Ok("DSC"),
        }
    }
}

pub enum EGroupField {
    Name,
    CreatedAt,
    LimitOfDownloadsPerDay,
}

impl libgql::executor::GQLEnum<super::scalar::ExampleScalar> for EGroupField {
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
        "NAME" => Ok(Self::Name),
        "CREATED_AT" => Ok(Self::CreatedAt),
        "LIMIT_OF_DOWNLOADS_PER_DAY" => Ok(Self::LimitOfDownloadsPerDay),
        _ => Err(format!("Unexpected value {} for enum EGroupField", s))
        }
    }

    fn to_str(self: &Self) -> Result<&str, String> {
        match self {
        Self::Name => Ok("NAME"),
        Self::CreatedAt => Ok("CREATED_AT"),
        Self::LimitOfDownloadsPerDay => Ok("LIMIT_OF_DOWNLOADS_PER_DAY"),
        }
    }
}

pub enum EDealColumnType {
    List,
    Number,
    Date,
}

impl libgql::executor::GQLEnum<super::scalar::ExampleScalar> for EDealColumnType {
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
        "LIST" => Ok(Self::List),
        "NUMBER" => Ok(Self::Number),
        "DATE" => Ok(Self::Date),
        _ => Err(format!("Unexpected value {} for enum EDealColumnType", s))
        }
    }

    fn to_str(self: &Self) -> Result<&str, String> {
        match self {
        Self::List => Ok("LIST"),
        Self::Number => Ok("NUMBER"),
        Self::Date => Ok("DATE"),
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

impl libgql::executor::GQLEnum<super::scalar::ExampleScalar> for EFileField {
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
        "NAME" => Ok(Self::Name),
        "MIMETYPE" => Ok(Self::Mimetype),
        "SIZE_IN_BYTES" => Ok(Self::SizeInBytes),
        "AUTHOR_NAME" => Ok(Self::AuthorName),
        "TAGS" => Ok(Self::Tags),
        "CREATED_AT" => Ok(Self::CreatedAt),
        _ => Err(format!("Unexpected value {} for enum EFileField", s))
        }
    }

    fn to_str(self: &Self) -> Result<&str, String> {
        match self {
        Self::Name => Ok("NAME"),
        Self::Mimetype => Ok("MIMETYPE"),
        Self::SizeInBytes => Ok("SIZE_IN_BYTES"),
        Self::AuthorName => Ok("AUTHOR_NAME"),
        Self::Tags => Ok("TAGS"),
        Self::CreatedAt => Ok("CREATED_AT"),
        }
    }
}

pub enum EUserField {
    Name,
    Email,
    CreatedAt,
}

impl libgql::executor::GQLEnum<super::scalar::ExampleScalar> for EUserField {
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
        "NAME" => Ok(Self::Name),
        "EMAIL" => Ok(Self::Email),
        "CREATED_AT" => Ok(Self::CreatedAt),
        _ => Err(format!("Unexpected value {} for enum EUserField", s))
        }
    }

    fn to_str(self: &Self) -> Result<&str, String> {
        match self {
        Self::Name => Ok("NAME"),
        Self::Email => Ok("EMAIL"),
        Self::CreatedAt => Ok("CREATED_AT"),
        }
    }
}

pub enum EGroupUsersField {
    Name,
    Email,
}

impl libgql::executor::GQLEnum<super::scalar::ExampleScalar> for EGroupUsersField {
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
        "NAME" => Ok(Self::Name),
        "EMAIL" => Ok(Self::Email),
        _ => Err(format!("Unexpected value {} for enum EGroupUsersField", s))
        }
    }

    fn to_str(self: &Self) -> Result<&str, String> {
        match self {
        Self::Name => Ok("NAME"),
        Self::Email => Ok("EMAIL"),
        }
    }
}

pub struct TagIn {
    pub tag: String,
    pub parent_tag_id: Option<uuid::Uuid>,
}

impl libgql::executor::GQLInput<super::scalar::ExampleScalar> for TagIn {
    fn from_variables(variables: &libgql::executor::Values<super::scalar::ExampleScalar>) -> Result<Self, String> {
        Ok(TagIn{
            tag: variables.get("tag").map(libgql::executor::Value::to_non_nullable_option).flatten()
        .ok_or("TagIn: Required field tag is missing or null".to_string()).map(|v| v.get_literal().ok_or("Unexpected array value for literal".to_string()).map(<String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value).flatten()).flatten()?,
            parent_tag_id: variables.get("parentTagId").map(libgql::executor::Value::to_non_nullable_option).flatten()
        .map(|v| v.get_literal().ok_or("Unexpected array value for literal".to_string()).map(<uuid::Uuid as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value).flatten()).transpose()?
        })
    }
}

pub struct PutUploadFileIn {
    pub tag_ids: Vec<uuid::Uuid>,
    pub name: String,
    pub size_in_bytes: i64,
}

impl libgql::executor::GQLInput<super::scalar::ExampleScalar> for PutUploadFileIn {
    fn from_variables(variables: &libgql::executor::Values<super::scalar::ExampleScalar>) -> Result<Self, String> {
        Ok(PutUploadFileIn{
            tag_ids: variables.get("tagIds").map(libgql::executor::Value::to_non_nullable_option).flatten()
        .ok_or("PutUploadFileIn: Required field tagIds is missing or null".to_string()).map(|v| libgql::executor::ast::extract_array(v, |element: &libgql::executor::Value<super::scalar::ExampleScalar>| element.to_non_nullable_option().ok_or("Unexpected null in non-nullable array".to_string()).map(|v| v.get_literal().ok_or("Unexpected array value for literal".to_string()).map(<uuid::Uuid as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value).flatten()).flatten())).flatten()?,
            name: variables.get("name").map(libgql::executor::Value::to_non_nullable_option).flatten()
        .ok_or("PutUploadFileIn: Required field name is missing or null".to_string()).map(|v| v.get_literal().ok_or("Unexpected array value for literal".to_string()).map(<String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value).flatten()).flatten()?,
            size_in_bytes: variables.get("sizeInBytes").map(libgql::executor::Value::to_non_nullable_option).flatten()
        .ok_or("PutUploadFileIn: Required field sizeInBytes is missing or null".to_string()).map(|v| v.get_literal().ok_or("Unexpected array value for literal".to_string()).map(<i64 as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value).flatten()).flatten()?
        })
    }
}

pub struct FileSortBy {
    pub field: EFileField,
    pub direction: ESortDirection,
}

impl libgql::executor::GQLInput<super::scalar::ExampleScalar> for FileSortBy {
    fn from_variables(variables: &libgql::executor::Values<super::scalar::ExampleScalar>) -> Result<Self, String> {
        Ok(FileSortBy{
            field: variables.get("field").map(libgql::executor::Value::to_non_nullable_option).flatten()
        .ok_or("FileSortBy: Required field field is missing or null".to_string()).map(|v| v.get_literal().ok_or("Unexpected array value for literal".to_string()).map(<EFileField as libgql::executor::GQLEnum<super::scalar::ExampleScalar>>::from_literal_value).flatten()).flatten()?,
            direction: variables.get("direction").map(libgql::executor::Value::to_non_nullable_option).flatten()
        .ok_or("FileSortBy: Required field direction is missing or null".to_string()).map(|v| v.get_literal().ok_or("Unexpected array value for literal".to_string()).map(<ESortDirection as libgql::executor::GQLEnum<super::scalar::ExampleScalar>>::from_literal_value).flatten()).flatten()?
        })
    }
}

pub struct Filter {
    pub number_range: Option<NumberRange>,
    pub column_id: uuid::Uuid,
    pub date_range: Option<FilterDateRange>,
    pub list_values: Option<Vec<String>>,
}

impl libgql::executor::GQLInput<super::scalar::ExampleScalar> for Filter {
    fn from_variables(variables: &libgql::executor::Values<super::scalar::ExampleScalar>) -> Result<Self, String> {
        Ok(Filter{
            number_range: variables.get("numberRange").map(libgql::executor::Value::to_non_nullable_option).flatten()
        .map(|v| v.get_literal().ok_or("Unexpected array value for literal".to_string()).map(<NumberRange as libgql::executor::GQLInput<super::scalar::ExampleScalar>>::from_literal_value).flatten()).transpose()?,
            column_id: variables.get("columnId").map(libgql::executor::Value::to_non_nullable_option).flatten()
        .ok_or("Filter: Required field columnId is missing or null".to_string()).map(|v| v.get_literal().ok_or("Unexpected array value for literal".to_string()).map(<uuid::Uuid as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value).flatten()).flatten()?,
            date_range: variables.get("dateRange").map(libgql::executor::Value::to_non_nullable_option).flatten()
        .map(|v| v.get_literal().ok_or("Unexpected array value for literal".to_string()).map(<FilterDateRange as libgql::executor::GQLInput<super::scalar::ExampleScalar>>::from_literal_value).flatten()).transpose()?,
            list_values: variables.get("listValues").map(libgql::executor::Value::to_non_nullable_option).flatten()
        .map(|v| libgql::executor::ast::extract_array(v, |element: &libgql::executor::Value<super::scalar::ExampleScalar>| element.to_non_nullable_option().ok_or("Unexpected null in non-nullable array".to_string()).map(|v| v.get_literal().ok_or("Unexpected array value for literal".to_string()).map(<String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value).flatten()).flatten())).transpose()?
        })
    }
}

pub struct UserIn {
    pub name: String,
    pub email: String,
    pub group_ids: Vec<uuid::Uuid>,
}

impl libgql::executor::GQLInput<super::scalar::ExampleScalar> for UserIn {
    fn from_variables(variables: &libgql::executor::Values<super::scalar::ExampleScalar>) -> Result<Self, String> {
        Ok(UserIn{
            name: variables.get("name").map(libgql::executor::Value::to_non_nullable_option).flatten()
        .ok_or("UserIn: Required field name is missing or null".to_string()).map(|v| v.get_literal().ok_or("Unexpected array value for literal".to_string()).map(<String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value).flatten()).flatten()?,
            email: variables.get("email").map(libgql::executor::Value::to_non_nullable_option).flatten()
        .ok_or("UserIn: Required field email is missing or null".to_string()).map(|v| v.get_literal().ok_or("Unexpected array value for literal".to_string()).map(<String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value).flatten()).flatten()?,
            group_ids: variables.get("groupIds").map(libgql::executor::Value::to_non_nullable_option).flatten()
        .ok_or("UserIn: Required field groupIds is missing or null".to_string()).map(|v| libgql::executor::ast::extract_array(v, |element: &libgql::executor::Value<super::scalar::ExampleScalar>| element.to_non_nullable_option().ok_or("Unexpected null in non-nullable array".to_string()).map(|v| v.get_literal().ok_or("Unexpected array value for literal".to_string()).map(<uuid::Uuid as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value).flatten()).flatten())).flatten()?
        })
    }
}

pub struct GetUsersSortBy {
    pub field: EUserField,
    pub direction: ESortDirection,
}

impl libgql::executor::GQLInput<super::scalar::ExampleScalar> for GetUsersSortBy {
    fn from_variables(variables: &libgql::executor::Values<super::scalar::ExampleScalar>) -> Result<Self, String> {
        Ok(GetUsersSortBy{
            field: variables.get("field").map(libgql::executor::Value::to_non_nullable_option).flatten()
        .ok_or("GetUsersSortBy: Required field field is missing or null".to_string()).map(|v| v.get_literal().ok_or("Unexpected array value for literal".to_string()).map(<EUserField as libgql::executor::GQLEnum<super::scalar::ExampleScalar>>::from_literal_value).flatten()).flatten()?,
            direction: variables.get("direction").map(libgql::executor::Value::to_non_nullable_option).flatten()
        .ok_or("GetUsersSortBy: Required field direction is missing or null".to_string()).map(|v| v.get_literal().ok_or("Unexpected array value for literal".to_string()).map(<ESortDirection as libgql::executor::GQLEnum<super::scalar::ExampleScalar>>::from_literal_value).flatten()).flatten()?
        })
    }
}

pub struct GetGroupUsersSortBy {
    pub direction: ESortDirection,
    pub field: EGroupUsersField,
}

impl libgql::executor::GQLInput<super::scalar::ExampleScalar> for GetGroupUsersSortBy {
    fn from_variables(variables: &libgql::executor::Values<super::scalar::ExampleScalar>) -> Result<Self, String> {
        Ok(GetGroupUsersSortBy{
            direction: variables.get("direction").map(libgql::executor::Value::to_non_nullable_option).flatten()
        .ok_or("GetGroupUsersSortBy: Required field direction is missing or null".to_string()).map(|v| v.get_literal().ok_or("Unexpected array value for literal".to_string()).map(<ESortDirection as libgql::executor::GQLEnum<super::scalar::ExampleScalar>>::from_literal_value).flatten()).flatten()?,
            field: variables.get("field").map(libgql::executor::Value::to_non_nullable_option).flatten()
        .ok_or("GetGroupUsersSortBy: Required field field is missing or null".to_string()).map(|v| v.get_literal().ok_or("Unexpected array value for literal".to_string()).map(<EGroupUsersField as libgql::executor::GQLEnum<super::scalar::ExampleScalar>>::from_literal_value).flatten()).flatten()?
        })
    }
}

pub struct UsersTagSortBy {
    pub direction: ESortDirection,
    pub field: EUsersTagField,
}

impl libgql::executor::GQLInput<super::scalar::ExampleScalar> for UsersTagSortBy {
    fn from_variables(variables: &libgql::executor::Values<super::scalar::ExampleScalar>) -> Result<Self, String> {
        Ok(UsersTagSortBy{
            direction: variables.get("direction").map(libgql::executor::Value::to_non_nullable_option).flatten()
        .ok_or("UsersTagSortBy: Required field direction is missing or null".to_string()).map(|v| v.get_literal().ok_or("Unexpected array value for literal".to_string()).map(<ESortDirection as libgql::executor::GQLEnum<super::scalar::ExampleScalar>>::from_literal_value).flatten()).flatten()?,
            field: variables.get("field").map(libgql::executor::Value::to_non_nullable_option).flatten()
        .ok_or("UsersTagSortBy: Required field field is missing or null".to_string()).map(|v| v.get_literal().ok_or("Unexpected array value for literal".to_string()).map(<EUsersTagField as libgql::executor::GQLEnum<super::scalar::ExampleScalar>>::from_literal_value).flatten()).flatten()?
        })
    }
}

pub struct EventFiltersIn {
    pub event_file_download_requested: bool,
    pub event_file_downloaded: bool,
    pub event_file_uploaded: bool,
    pub event_tag_approval_is_requested: bool,
    pub event_file_tags_edited: bool,
    pub event_file_deleted: bool,
}

impl libgql::executor::GQLInput<super::scalar::ExampleScalar> for EventFiltersIn {
    fn from_variables(variables: &libgql::executor::Values<super::scalar::ExampleScalar>) -> Result<Self, String> {
        Ok(EventFiltersIn{
            event_file_download_requested: variables.get("eventFileDownloadRequested").map(libgql::executor::Value::to_non_nullable_option).flatten()
        .ok_or("EventFiltersIn: Required field eventFileDownloadRequested is missing or null".to_string()).map(|v| v.get_literal().ok_or("Unexpected array value for literal".to_string()).map(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value).flatten()).flatten()?,
            event_file_downloaded: variables.get("eventFileDownloaded").map(libgql::executor::Value::to_non_nullable_option).flatten()
        .ok_or("EventFiltersIn: Required field eventFileDownloaded is missing or null".to_string()).map(|v| v.get_literal().ok_or("Unexpected array value for literal".to_string()).map(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value).flatten()).flatten()?,
            event_file_uploaded: variables.get("eventFileUploaded").map(libgql::executor::Value::to_non_nullable_option).flatten()
        .ok_or("EventFiltersIn: Required field eventFileUploaded is missing or null".to_string()).map(|v| v.get_literal().ok_or("Unexpected array value for literal".to_string()).map(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value).flatten()).flatten()?,
            event_tag_approval_is_requested: variables.get("eventTagApprovalIsRequested").map(libgql::executor::Value::to_non_nullable_option).flatten()
        .ok_or("EventFiltersIn: Required field eventTagApprovalIsRequested is missing or null".to_string()).map(|v| v.get_literal().ok_or("Unexpected array value for literal".to_string()).map(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value).flatten()).flatten()?,
            event_file_tags_edited: variables.get("eventFileTagsEdited").map(libgql::executor::Value::to_non_nullable_option).flatten()
        .ok_or("EventFiltersIn: Required field eventFileTagsEdited is missing or null".to_string()).map(|v| v.get_literal().ok_or("Unexpected array value for literal".to_string()).map(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value).flatten()).flatten()?,
            event_file_deleted: variables.get("eventFileDeleted").map(libgql::executor::Value::to_non_nullable_option).flatten()
        .ok_or("EventFiltersIn: Required field eventFileDeleted is missing or null".to_string()).map(|v| v.get_literal().ok_or("Unexpected array value for literal".to_string()).map(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value).flatten()).flatten()?
        })
    }
}

pub struct NumberRange {
    pub end_at: Option<f32>,
    pub start_at: Option<f32>,
}

impl libgql::executor::GQLInput<super::scalar::ExampleScalar> for NumberRange {
    fn from_variables(variables: &libgql::executor::Values<super::scalar::ExampleScalar>) -> Result<Self, String> {
        Ok(NumberRange{
            end_at: variables.get("endAt").map(libgql::executor::Value::to_non_nullable_option).flatten()
        .map(|v| v.get_literal().ok_or("Unexpected array value for literal".to_string()).map(<f32 as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value).flatten()).transpose()?,
            start_at: variables.get("startAt").map(libgql::executor::Value::to_non_nullable_option).flatten()
        .map(|v| v.get_literal().ok_or("Unexpected array value for literal".to_string()).map(<f32 as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value).flatten()).transpose()?
        })
    }
}

pub struct DateRange {
    pub start_at: chrono::DateTime<chrono::Utc>,
    pub end_at: chrono::DateTime<chrono::Utc>,
}

impl libgql::executor::GQLInput<super::scalar::ExampleScalar> for DateRange {
    fn from_variables(variables: &libgql::executor::Values<super::scalar::ExampleScalar>) -> Result<Self, String> {
        Ok(DateRange{
            start_at: variables.get("startAt").map(libgql::executor::Value::to_non_nullable_option).flatten()
        .ok_or("DateRange: Required field startAt is missing or null".to_string()).map(|v| v.get_literal().ok_or("Unexpected array value for literal".to_string()).map(<chrono::DateTime<chrono::Utc> as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value).flatten()).flatten()?,
            end_at: variables.get("endAt").map(libgql::executor::Value::to_non_nullable_option).flatten()
        .ok_or("DateRange: Required field endAt is missing or null".to_string()).map(|v| v.get_literal().ok_or("Unexpected array value for literal".to_string()).map(<chrono::DateTime<chrono::Utc> as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value).flatten()).flatten()?
        })
    }
}

pub struct GetGroupsSortBy {
    pub direction: ESortDirection,
    pub field: EGroupField,
}

impl libgql::executor::GQLInput<super::scalar::ExampleScalar> for GetGroupsSortBy {
    fn from_variables(variables: &libgql::executor::Values<super::scalar::ExampleScalar>) -> Result<Self, String> {
        Ok(GetGroupsSortBy{
            direction: variables.get("direction").map(libgql::executor::Value::to_non_nullable_option).flatten()
        .ok_or("GetGroupsSortBy: Required field direction is missing or null".to_string()).map(|v| v.get_literal().ok_or("Unexpected array value for literal".to_string()).map(<ESortDirection as libgql::executor::GQLEnum<super::scalar::ExampleScalar>>::from_literal_value).flatten()).flatten()?,
            field: variables.get("field").map(libgql::executor::Value::to_non_nullable_option).flatten()
        .ok_or("GetGroupsSortBy: Required field field is missing or null".to_string()).map(|v| v.get_literal().ok_or("Unexpected array value for literal".to_string()).map(<EGroupField as libgql::executor::GQLEnum<super::scalar::ExampleScalar>>::from_literal_value).flatten()).flatten()?
        })
    }
}

pub struct MultipartUploadFileIn {
    pub part_size_in_bytes: i64,
    pub size_in_bytes: i64,
    pub name: String,
    pub initial_parts_count: i32,
    pub tag_ids: Vec<uuid::Uuid>,
}

impl libgql::executor::GQLInput<super::scalar::ExampleScalar> for MultipartUploadFileIn {
    fn from_variables(variables: &libgql::executor::Values<super::scalar::ExampleScalar>) -> Result<Self, String> {
        Ok(MultipartUploadFileIn{
            part_size_in_bytes: variables.get("partSizeInBytes").map(libgql::executor::Value::to_non_nullable_option).flatten()
        .ok_or("MultipartUploadFileIn: Required field partSizeInBytes is missing or null".to_string()).map(|v| v.get_literal().ok_or("Unexpected array value for literal".to_string()).map(<i64 as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value).flatten()).flatten()?,
            size_in_bytes: variables.get("sizeInBytes").map(libgql::executor::Value::to_non_nullable_option).flatten()
        .ok_or("MultipartUploadFileIn: Required field sizeInBytes is missing or null".to_string()).map(|v| v.get_literal().ok_or("Unexpected array value for literal".to_string()).map(<i64 as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value).flatten()).flatten()?,
            name: variables.get("name").map(libgql::executor::Value::to_non_nullable_option).flatten()
        .ok_or("MultipartUploadFileIn: Required field name is missing or null".to_string()).map(|v| v.get_literal().ok_or("Unexpected array value for literal".to_string()).map(<String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value).flatten()).flatten()?,
            initial_parts_count: variables.get("initialPartsCount").map(libgql::executor::Value::to_non_nullable_option).flatten()
        .ok_or("MultipartUploadFileIn: Required field initialPartsCount is missing or null".to_string()).map(|v| v.get_literal().ok_or("Unexpected array value for literal".to_string()).map(<i32 as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value).flatten()).flatten()?,
            tag_ids: variables.get("tagIds").map(libgql::executor::Value::to_non_nullable_option).flatten()
        .ok_or("MultipartUploadFileIn: Required field tagIds is missing or null".to_string()).map(|v| libgql::executor::ast::extract_array(v, |element: &libgql::executor::Value<super::scalar::ExampleScalar>| element.to_non_nullable_option().ok_or("Unexpected null in non-nullable array".to_string()).map(|v| v.get_literal().ok_or("Unexpected array value for literal".to_string()).map(<uuid::Uuid as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value).flatten()).flatten())).flatten()?
        })
    }
}

pub struct GroupIn {
    pub name: String,
    pub tag_ids: Vec<uuid::Uuid>,
    pub limit_of_downloads_per_day: i32,
}

impl libgql::executor::GQLInput<super::scalar::ExampleScalar> for GroupIn {
    fn from_variables(variables: &libgql::executor::Values<super::scalar::ExampleScalar>) -> Result<Self, String> {
        Ok(GroupIn{
            name: variables.get("name").map(libgql::executor::Value::to_non_nullable_option).flatten()
        .ok_or("GroupIn: Required field name is missing or null".to_string()).map(|v| v.get_literal().ok_or("Unexpected array value for literal".to_string()).map(<String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value).flatten()).flatten()?,
            tag_ids: variables.get("tagIds").map(libgql::executor::Value::to_non_nullable_option).flatten()
        .ok_or("GroupIn: Required field tagIds is missing or null".to_string()).map(|v| libgql::executor::ast::extract_array(v, |element: &libgql::executor::Value<super::scalar::ExampleScalar>| element.to_non_nullable_option().ok_or("Unexpected null in non-nullable array".to_string()).map(|v| v.get_literal().ok_or("Unexpected array value for literal".to_string()).map(<uuid::Uuid as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value).flatten()).flatten())).flatten()?,
            limit_of_downloads_per_day: variables.get("limitOfDownloadsPerDay").map(libgql::executor::Value::to_non_nullable_option).flatten()
        .ok_or("GroupIn: Required field limitOfDownloadsPerDay is missing or null".to_string()).map(|v| v.get_literal().ok_or("Unexpected array value for literal".to_string()).map(<i32 as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value).flatten()).flatten()?
        })
    }
}

pub struct FilterDateRange {
    pub end_at: Option<chrono::DateTime<chrono::Utc>>,
    pub start_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl libgql::executor::GQLInput<super::scalar::ExampleScalar> for FilterDateRange {
    fn from_variables(variables: &libgql::executor::Values<super::scalar::ExampleScalar>) -> Result<Self, String> {
        Ok(FilterDateRange{
            end_at: variables.get("endAt").map(libgql::executor::Value::to_non_nullable_option).flatten()
        .map(|v| v.get_literal().ok_or("Unexpected array value for literal".to_string()).map(<chrono::DateTime<chrono::Utc> as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value).flatten()).transpose()?,
            start_at: variables.get("startAt").map(libgql::executor::Value::to_non_nullable_option).flatten()
        .map(|v| v.get_literal().ok_or("Unexpected array value for literal".to_string()).map(<chrono::DateTime<chrono::Utc> as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value).flatten()).transpose()?
        })
    }
}

pub struct ErrorInvalidToken {
    pub a: Option<bool>,
}

pub struct StageToWorktypesMapEntry {
    pub stage: Tag,
    pub worktypes: Vec<Tag>,
}

pub struct ErrorMultipartUploadFilePartSizeIsTooBig {
    pub a: Option<bool>,
}

pub struct ErrorInvalidLimitOfDownloadsPerDay {
    pub a: Option<bool>,
}

pub struct EventFileDeleted {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub file: File,
}

pub struct DealColumn {
    pub available_values: Vec<String>,
    pub name: String,
    pub id: uuid::Uuid,
    pub r#type: EDealColumnType,
}

pub struct EventFileDownloadRequested {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub decision: Option<bool>,
    pub user: User,
    pub file: File,
}

pub struct File {
    pub size_in_bytes: i64,
    pub preview_url: Option<url::Url>,
    pub filename: String,
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub mime_type: Option<String>,
    pub user: User,
}

pub struct User {
    pub ten_groups: Vec<Group>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub email: String,
    pub id: uuid::Uuid,
    pub name: String,
}

pub struct EventFileUploaded {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub file: File,
}

pub struct ErrorMultipartUploadFileIsTooLight {
    pub a: Option<bool>,
}

pub struct OTPToken {
    pub token: String,
}

pub struct ErrorMultipartUploadFilePartSizeIsTooSmall {
    pub a: Option<bool>,
}

pub struct ErrorDateRangeIsInvalid {
    pub a: Option<bool>,
}

pub struct UploadUrlList {
    pub urls: Vec<UploadUrl>,
}

pub struct ErrorNoDealTag {
    pub a: Option<bool>,
}

pub struct EventFileDownloaded {
    pub file: File,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub struct Tag {
    pub has_children: bool,
    pub value: Option<TagValue>,
    pub tag: String,
    pub is_favourite: bool,
    pub id: uuid::Uuid,
    pub is_approved: bool,
}

pub struct ErrorInvalidGroupName {
    pub a: Option<bool>,
}

pub struct ErrorAlreadyApprovedByAdmin {
    pub a: Option<bool>,
}

pub struct ErrorInvalidOTPCode {
    pub a: Option<bool>,
}

pub struct ErrorUnknownGroupIds {
    pub group_ids: Vec<uuid::Uuid>,
}

pub struct StringEntry {
    pub value: String,
    pub key: String,
}

pub struct TagInfo {
    pub parent_tag: Option<Tag>,
    pub tag: String,
}

pub struct Mutation {
    pub create_group: Option<CreateGroupError>,
    pub confirm_user: Option<ConfirmUserError>,
    pub create_user: Option<CreateUserError>,
    pub reset_password: Option<ResetPasswordError>,
    pub send_otp_code: Option<ErrorInvalidCredentials>,
    pub delete_user: Option<ErrorNotFound>,
    pub create_put_file_session: CreatePutFileSessionResponse,
    pub edit_group: Option<EditGroupError>,
    pub commit_put_file_session: Option<CommitPutFileSessionResponse>,
    pub delete_files: Option<DeleteFilesError>,
    pub remove_user_from_group: Option<ErrorGroupNotFoundOrErrorNotFound>,
    pub update_file: Option<UpdateFileError>,
    pub delete_tag: Option<ErrorNotFound>,
    pub delete_group: Option<ErrorGroupNotFound>,
    pub edit_tag: Option<EditTagError>,
    pub confirm_otp_code: ConfirmOTPCodeResponse,
    pub change_password: Option<ErrorInvalidCredentials>,
    pub logout: (),
    pub create_tag: Option<CreateTagError>,
    pub update_files_autotags: Option<ErrorCantAddAutotags>,
    pub decide_on_download_request: Option<DecideOnDownloadRequestError>,
    pub login: Option<ErrorInvalidCredentials>,
    pub approve_tag: Option<ApproveTagError>,
    pub set_tag_is_favourite: Option<ErrorAlreadyDoneOrUnknownTags>,
    pub add_user_to_group: Option<ErrorGroupNotFoundOrErrorNotFound>,
    pub delete_pending_user: Option<ErrorNotFound>,
    pub commit_multipart_file_session: Option<CommitMultipartFileSessionResponse>,
    pub create_multipart_file_session: CreateMultipartFileSessionResponse,
    pub add_tags_to_files: Option<AddTagsToFilesError>,
    pub delete_file: Option<DeleteFileError>,
}

pub struct UploadUrl {
    pub headers: Vec<StringEntry>,
    pub url: url::Url,
}

pub struct DealInfo {
    pub values: Vec<DealEntry>,
    pub stage_to_worktypes_map: Vec<StageToWorktypesMapEntry>,
}

pub struct ErrorCantAddAutotags {
    pub a: Option<bool>,
}

pub struct ErrorNotFound {
    pub a: Option<bool>,
}

pub struct ErrorEmailCollision {
    pub a: Option<bool>,
}

pub struct EventTagApprovalIsRequested {
    pub author: User,
    pub already_in_catalog: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub tag: Tag,
}

pub struct ErrorGroupNotFound {
    pub a: Option<bool>,
}

pub struct ErrorOTPCodeExpired {
    pub a: Option<bool>,
}

pub struct ErrorUnknownUsers {
    pub user_ids: Vec<uuid::Uuid>,
}

pub struct SearchFile {
    pub file: File,
    pub tags: Vec<Tag>,
}

pub struct ErrorUnknownParentId {
    pub a: Option<bool>,
}

pub struct DealEntry {
    pub value: Tag,
    pub column_name: String,
}

pub struct StringList {
    pub values: Vec<String>,
}

pub struct ErrorFilesChangeForbidden {
    pub ids: Vec<uuid::Uuid>,
}

pub struct GroupUserList {
    pub users: Vec<GroupUser>,
}

pub struct ErrorUnknownFiles {
    pub ids: Vec<uuid::Uuid>,
}

pub struct ErrorMultipartUploadFileIsTooBig {
    pub a: Option<bool>,
}

pub struct ErrorChangeForbidden {
    pub a: Option<bool>,
}

pub struct ErrorInvalidCredentials {
    pub a: Option<bool>,
}

pub struct ErrorInvalidEmail {
    pub a: Option<bool>,
}

pub struct FilesDealInfo {
    pub deal_name: Tag,
    pub unset_columns: Vec<DealColumn>,
    pub deal_info: DealInfo,
}

pub struct MultipartUploadSession {
    pub id: uuid::Uuid,
    pub initial_upload_ur_ls: Vec<UploadUrl>,
}

pub struct TagList {
    pub list: Vec<Tag>,
}

pub struct ErrorAlreadyDone {
    pub a: Option<bool>,
}

pub struct ErrorUnknownUser {
    pub a: Option<bool>,
}

pub struct SearchFileList {
    pub files: Vec<SearchFile>,
}

pub struct BooleanObject {
    pub bvalue: bool,
}

pub struct ErrorInvalidUserName {
    pub a: Option<bool>,
}

pub struct ErrorOTPTokenExpired {
    pub a: Option<bool>,
}

pub struct EventsList {
    pub events: Vec<Event>,
}

pub struct ErrorFileNotUploaded {
    pub a: Option<bool>,
}

pub struct PutUploadSession {
    pub id: uuid::Uuid,
    pub upload_url: UploadUrl,
}

pub struct EventFileTagsEdited {
    pub removed_tags: Vec<Tag>,
    pub added_tags: Vec<Tag>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub file: File,
}

pub struct Group {
    pub limit_of_downloads_per_day: i32,
    pub first_10_tags: Vec<Tag>,
    pub name: String,
    pub id: uuid::Uuid,
}

pub struct Query {
    pub get_my_tags: Vec<Tag>,
    pub get_files: GetFilesResponse,
    pub get_me: User,
    pub get_path_to_tag: GetPathToTagResponse,
    pub get_next_multipart_upload_urls: GetNextMultipartUploadUrlsResponse,
    pub get_group_users: GetGroupUsersResponse,
    pub get_users: Vec<User>,
    pub get_group_tags: GetGroupTagsResponse,
    pub get_deals: Vec<String>,
    pub get_tags_count: IntObjectOrErrorUnknownTags,
    pub get_uploaded_files: Vec<SearchFile>,
    pub get_tags: GetTagsResponse,
    pub get_favourite_tags: Vec<Tag>,
    pub get_pending_users: Vec<PendingUser>,
    pub get_deal_info: GetDealInfoResponse,
    pub get_groups: Vec<Group>,
    pub get_deal_columns: Vec<DealColumn>,
    pub get_users_total: i32,
    pub get_files_count: IntObjectOrErrorUnknownTags,
    pub get_users_tags: Vec<UsersTag>,
    pub get_uploaded_files_count: i32,
    pub get_groups_total: i32,
    pub is_allowed_to_download: IsAllowedToDownloadResponse,
    pub get_events: GetEventsResponse,
    pub get_group_users_and_users: GetGroupUsersAndUsersResponse,
    pub get_my_tags_count: i32,
    pub retrieve_group: RetrieveGroupResponse,
    pub is_tag_exists: bool,
    pub get_file_url: GetFileURLResponse,
    pub get_popular_tags: Vec<Tag>,
    pub get_files_deal_info: FilesDealInfoOrError,
    pub get_tag_info: GetTagInfoResponse,
    pub retrieve_file: RetrieveFileResponse,
    pub search_tags: Vec<Tag>,
    pub get_group_users_total: GetGroupUsersTotalResponse,
    pub get_tag_children: GetTagsResponse,
    pub get_users_tags_count: i32,
}

pub struct GroupUser {
    pub in_group: bool,
    pub user: User,
}

pub struct ErrorUnknownGroups {
    pub group_ids: Vec<uuid::Uuid>,
}

pub struct PendingUser {
    pub email: String,
    pub ttl: f32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub name: String,
    pub groups: Vec<Group>,
}

pub struct UsersList {
    pub users: Vec<User>,
}

pub struct ErrorAlreadyExists {
    pub a: Option<bool>,
}

pub struct ErrorInvalidPassword {
    pub a: Option<bool>,
}

pub struct UsersTag {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub users_count: i32,
    pub tag: Tag,
}

pub struct ErrorAlreadyPending {
    pub a: Option<bool>,
}

pub struct DatetimeObject {
    pub dvalue: chrono::DateTime<chrono::Utc>,
}

pub struct ErrorPutUploadFileIsTooBig {
    pub a: Option<bool>,
}

pub struct ErrorUnknownSessionId {
    pub a: Option<bool>,
}

pub struct FloatObject {
    pub fvalue: f32,
}

pub struct IntObject {
    pub ivalue: i32,
}

pub struct StringObject {
    pub svalue: String,
}

pub struct ErrorUnknownFile {
    pub a: Option<bool>,
}

pub struct UrlObject {
    pub uvalue: url::Url,
}

pub struct ErrorUnknownTags {
    pub tag_ids: Vec<uuid::Uuid>,
}

pub enum ApproveTagError {
    ErrorUnknownGroupIds(ErrorUnknownGroupIds),
    ErrorAlreadyExists(ErrorAlreadyExists),
    ErrorNotFound(ErrorNotFound),
}

pub enum DeleteFilesError {
    ErrorFilesChangeForbidden(ErrorFilesChangeForbidden),
    ErrorUnknownFiles(ErrorUnknownFiles),
}

pub enum GetFilesResponse {
    ErrorUnknownTags(ErrorUnknownTags),
    SearchFileList(SearchFileList),
}

pub enum IntObjectOrErrorUnknownTags {
    ErrorUnknownTags(ErrorUnknownTags),
    IntObject(IntObject),
}

pub enum RetrieveFileResponse {
    ErrorUnknownFile(ErrorUnknownFile),
    SearchFile(SearchFile),
}

pub enum CommitMultipartFileSessionResponse {
    ErrorUnknownSessionId(ErrorUnknownSessionId),
    ErrorFileNotUploaded(ErrorFileNotUploaded),
    File(File),
}

pub enum EditGroupError {
    ErrorGroupNotFound(ErrorGroupNotFound),
    ErrorInvalidLimitOfDownloadsPerDay(ErrorInvalidLimitOfDownloadsPerDay),
    ErrorInvalidGroupName(ErrorInvalidGroupName),
    ErrorUnknownTags(ErrorUnknownTags),
    ErrorAlreadyExists(ErrorAlreadyExists),
}

pub enum GetGroupUsersResponse {
    UsersList(UsersList),
    ErrorNotFound(ErrorNotFound),
}

pub enum GetGroupUsersAndUsersResponse {
    GroupUserList(GroupUserList),
    ErrorGroupNotFound(ErrorGroupNotFound),
}

pub enum GetPathToTagResponse {
    StringList(StringList),
    ErrorUnknownTags(ErrorUnknownTags),
}

pub enum GetTagsResponse {
    ErrorUnknownTags(ErrorUnknownTags),
    TagList(TagList),
}

pub enum RetrieveGroupResponse {
    ErrorNotFound(ErrorNotFound),
    Group(Group),
}

pub enum DeleteFileError {
    ErrorUnknownFile(ErrorUnknownFile),
    ErrorChangeForbidden(ErrorChangeForbidden),
}

pub enum GetDealInfoResponse {
    DealInfo(DealInfo),
    ErrorNotFound(ErrorNotFound),
}

pub enum GetNextMultipartUploadUrlsResponse {
    ErrorUnknownSessionId(ErrorUnknownSessionId),
    UploadUrlList(UploadUrlList),
}

pub enum FilesDealInfoOrError {
    FilesDealInfo(FilesDealInfo),
    ErrorCantAddAutotags(ErrorCantAddAutotags),
}

pub enum ErrorAlreadyDoneOrUnknownTags {
    ErrorUnknownTags(ErrorUnknownTags),
    ErrorAlreadyDone(ErrorAlreadyDone),
}

pub enum DecideOnDownloadRequestError {
    ErrorUnknownFile(ErrorUnknownFile),
    ErrorNotFound(ErrorNotFound),
    ErrorAlreadyDone(ErrorAlreadyDone),
    ErrorUnknownUser(ErrorUnknownUser),
}

pub enum GetFileURLResponse {
    ErrorUnknownFile(ErrorUnknownFile),
    UrlObject(UrlObject),
}

pub enum GetGroupTagsResponse {
    ErrorNotFound(ErrorNotFound),
    TagList(TagList),
}

pub enum CreateGroupError {
    ErrorUnknownUsers(ErrorUnknownUsers),
    ErrorAlreadyExists(ErrorAlreadyExists),
    ErrorInvalidLimitOfDownloadsPerDay(ErrorInvalidLimitOfDownloadsPerDay),
    ErrorUnknownTags(ErrorUnknownTags),
    ErrorInvalidGroupName(ErrorInvalidGroupName),
}

pub enum ConfirmUserError {
    ErrorInvalidToken(ErrorInvalidToken),
    ErrorInvalidPassword(ErrorInvalidPassword),
}

pub enum AddTagsToFilesError {
    ErrorUnknownTags(ErrorUnknownTags),
    ErrorUnknownFiles(ErrorUnknownFiles),
}

pub enum CreateTagError {
    ErrorUnknownParentId(ErrorUnknownParentId),
    ErrorAlreadyExists(ErrorAlreadyExists),
}

pub enum ErrorGroupNotFoundOrErrorNotFound {
    ErrorNotFound(ErrorNotFound),
    ErrorGroupNotFound(ErrorGroupNotFound),
}

pub enum EditTagError {
    ErrorNotFound(ErrorNotFound),
    ErrorUnknownParentId(ErrorUnknownParentId),
    ErrorAlreadyExists(ErrorAlreadyExists),
    ErrorAlreadyApprovedByAdmin(ErrorAlreadyApprovedByAdmin),
}

pub enum CommitPutFileSessionResponse {
    ErrorUnknownSessionId(ErrorUnknownSessionId),
    File(File),
    ErrorFileNotUploaded(ErrorFileNotUploaded),
}

pub enum ConfirmOTPCodeResponse {
    OTPToken(OTPToken),
    ErrorInvalidOTPCode(ErrorInvalidOTPCode),
    ErrorOTPCodeExpired(ErrorOTPCodeExpired),
}

pub enum CreateUserError {
    ErrorAlreadyPending(ErrorAlreadyPending),
    ErrorInvalidUserName(ErrorInvalidUserName),
    ErrorUnknownGroups(ErrorUnknownGroups),
    ErrorInvalidEmail(ErrorInvalidEmail),
    ErrorEmailCollision(ErrorEmailCollision),
}

pub enum Event {
    EventTagApprovalIsRequested(EventTagApprovalIsRequested),
    EventFileDeleted(EventFileDeleted),
    EventFileDownloadRequested(EventFileDownloadRequested),
    EventFileTagsEdited(EventFileTagsEdited),
    EventFileDownloaded(EventFileDownloaded),
    EventFileUploaded(EventFileUploaded),
}

pub enum UpdateFileError {
    ErrorChangeForbidden(ErrorChangeForbidden),
    ErrorUnknownTags(ErrorUnknownTags),
    ErrorUnknownFile(ErrorUnknownFile),
}

pub enum GetGroupUsersTotalResponse {
    ErrorNotFound(ErrorNotFound),
    IntObject(IntObject),
}

pub enum GetEventsResponse {
    ErrorDateRangeIsInvalid(ErrorDateRangeIsInvalid),
    EventsList(EventsList),
}

pub enum IsAllowedToDownloadResponse {
    ErrorUnknownFile(ErrorUnknownFile),
    BooleanObject(BooleanObject),
}

pub enum TagValue {
    FloatObject(FloatObject),
    DatetimeObject(DatetimeObject),
    StringObject(StringObject),
}

pub enum CreateMultipartFileSessionResponse {
    ErrorMultipartUploadFileIsTooBig(ErrorMultipartUploadFileIsTooBig),
    ErrorNoDealTag(ErrorNoDealTag),
    ErrorMultipartUploadFileIsTooLight(ErrorMultipartUploadFileIsTooLight),
    ErrorMultipartUploadFilePartSizeIsTooBig(ErrorMultipartUploadFilePartSizeIsTooBig),
    MultipartUploadSession(MultipartUploadSession),
    ErrorUnknownTags(ErrorUnknownTags),
    ErrorMultipartUploadFilePartSizeIsTooSmall(ErrorMultipartUploadFilePartSizeIsTooSmall),
}

pub enum GetTagInfoResponse {
    ErrorNotFound(ErrorNotFound),
    TagInfo(TagInfo),
}

pub enum ResetPasswordError {
    ErrorInvalidPassword(ErrorInvalidPassword),
    ErrorOTPTokenExpired(ErrorOTPTokenExpired),
}

pub enum CreatePutFileSessionResponse {
    ErrorPutUploadFileIsTooBig(ErrorPutUploadFileIsTooBig),
    ErrorNoDealTag(ErrorNoDealTag),
    ErrorUnknownTags(ErrorUnknownTags),
    PutUploadSession(PutUploadSession),
}