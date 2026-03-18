pub enum EDealColumnType {
    List,
    Number,
    Date,
}

impl libgql::executor::GQLEnum<super::scalar::ExampleScalar> for EDealColumnType {
    fn from_string(s: String) -> Result<Self, String> {
        match s.as_str() {
        "LIST" => Ok(Self::List),
        "NUMBER" => Ok(Self::Number),
        "DATE" => Ok(Self::Date),
        _ => Err(format!("Unexpected value {} for enum EDealColumnType", s))
        }
    }

    fn to_str(self: Self) -> Result<&'static str, String> {
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
    fn from_string(s: String) -> Result<Self, String> {
        match s.as_str() {
        "NAME" => Ok(Self::Name),
        "MIMETYPE" => Ok(Self::Mimetype),
        "SIZE_IN_BYTES" => Ok(Self::SizeInBytes),
        "AUTHOR_NAME" => Ok(Self::AuthorName),
        "TAGS" => Ok(Self::Tags),
        "CREATED_AT" => Ok(Self::CreatedAt),
        _ => Err(format!("Unexpected value {} for enum EFileField", s))
        }
    }

    fn to_str(self: Self) -> Result<&'static str, String> {
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

pub enum EGroupField {
    Name,
    CreatedAt,
    LimitOfDownloadsPerDay,
}

impl libgql::executor::GQLEnum<super::scalar::ExampleScalar> for EGroupField {
    fn from_string(s: String) -> Result<Self, String> {
        match s.as_str() {
        "NAME" => Ok(Self::Name),
        "CREATED_AT" => Ok(Self::CreatedAt),
        "LIMIT_OF_DOWNLOADS_PER_DAY" => Ok(Self::LimitOfDownloadsPerDay),
        _ => Err(format!("Unexpected value {} for enum EGroupField", s))
        }
    }

    fn to_str(self: Self) -> Result<&'static str, String> {
        match self {
        Self::Name => Ok("NAME"),
        Self::CreatedAt => Ok("CREATED_AT"),
        Self::LimitOfDownloadsPerDay => Ok("LIMIT_OF_DOWNLOADS_PER_DAY"),
        }
    }
}

pub enum EGroupUsersField {
    Name,
    Email,
}

impl libgql::executor::GQLEnum<super::scalar::ExampleScalar> for EGroupUsersField {
    fn from_string(s: String) -> Result<Self, String> {
        match s.as_str() {
        "NAME" => Ok(Self::Name),
        "EMAIL" => Ok(Self::Email),
        _ => Err(format!("Unexpected value {} for enum EGroupUsersField", s))
        }
    }

    fn to_str(self: Self) -> Result<&'static str, String> {
        match self {
        Self::Name => Ok("NAME"),
        Self::Email => Ok("EMAIL"),
        }
    }
}

pub enum ESortDirection {
    Asc,
    Dsc,
}

impl libgql::executor::GQLEnum<super::scalar::ExampleScalar> for ESortDirection {
    fn from_string(s: String) -> Result<Self, String> {
        match s.as_str() {
        "ASC" => Ok(Self::Asc),
        "DSC" => Ok(Self::Dsc),
        _ => Err(format!("Unexpected value {} for enum ESortDirection", s))
        }
    }

    fn to_str(self: Self) -> Result<&'static str, String> {
        match self {
        Self::Asc => Ok("ASC"),
        Self::Dsc => Ok("DSC"),
        }
    }
}

pub enum EUserField {
    Name,
    Email,
    CreatedAt,
}

impl libgql::executor::GQLEnum<super::scalar::ExampleScalar> for EUserField {
    fn from_string(s: String) -> Result<Self, String> {
        match s.as_str() {
        "NAME" => Ok(Self::Name),
        "EMAIL" => Ok(Self::Email),
        "CREATED_AT" => Ok(Self::CreatedAt),
        _ => Err(format!("Unexpected value {} for enum EUserField", s))
        }
    }

    fn to_str(self: Self) -> Result<&'static str, String> {
        match self {
        Self::Name => Ok("NAME"),
        Self::Email => Ok("EMAIL"),
        Self::CreatedAt => Ok("CREATED_AT"),
        }
    }
}

pub enum EUsersTagField {
    Tag,
    UsersCount,
    CreatedAt,
}

impl libgql::executor::GQLEnum<super::scalar::ExampleScalar> for EUsersTagField {
    fn from_string(s: String) -> Result<Self, String> {
        match s.as_str() {
        "TAG" => Ok(Self::Tag),
        "USERS_COUNT" => Ok(Self::UsersCount),
        "CREATED_AT" => Ok(Self::CreatedAt),
        _ => Err(format!("Unexpected value {} for enum EUsersTagField", s))
        }
    }

    fn to_str(self: Self) -> Result<&'static str, String> {
        match self {
        Self::Tag => Ok("TAG"),
        Self::UsersCount => Ok("USERS_COUNT"),
        Self::CreatedAt => Ok("CREATED_AT"),
        }
    }
}

pub struct DateRange {
    pub end_at: chrono::DateTime<chrono::Utc>,
    pub start_at: chrono::DateTime<chrono::Utc>,
}

impl libgql::executor::GQLInput<super::scalar::ExampleScalar> for DateRange {
    fn from_variables(mut variables: libgql::executor::Values<super::scalar::ExampleScalar>) -> Result<Self, String> {
        Ok(DateRange{
            end_at: variables.remove("endAt")
                .map(libgql::executor::Value::to_non_nullable_option)
                .flatten()
                .ok_or("DateRange: Required field endAt is missing or null".to_string())
                .map(|v| 
                    v.get_literal()
                    .ok_or("Unexpected array value for literal".to_string())
                    .map(<chrono::DateTime<chrono::Utc> as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value)
                    .flatten()
                )
                .flatten()?,
            start_at: variables.remove("startAt")
                .map(libgql::executor::Value::to_non_nullable_option)
                .flatten()
                .ok_or("DateRange: Required field startAt is missing or null".to_string())
                .map(|v| 
                    v.get_literal()
                    .ok_or("Unexpected array value for literal".to_string())
                    .map(<chrono::DateTime<chrono::Utc> as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value)
                    .flatten()
                )
                .flatten()?
        })
    }
}

pub struct EventFiltersIn {
    pub event_file_deleted: bool,
    pub event_file_download_requested: bool,
    pub event_file_downloaded: bool,
    pub event_file_tags_edited: bool,
    pub event_file_uploaded: bool,
    pub event_tag_approval_is_requested: bool,
}

impl libgql::executor::GQLInput<super::scalar::ExampleScalar> for EventFiltersIn {
    fn from_variables(mut variables: libgql::executor::Values<super::scalar::ExampleScalar>) -> Result<Self, String> {
        Ok(EventFiltersIn{
            event_file_deleted: variables.remove("eventFileDeleted")
                .map(libgql::executor::Value::to_non_nullable_option)
                .flatten()
                .ok_or("EventFiltersIn: Required field eventFileDeleted is missing or null".to_string())
                .map(|v| 
                    v.get_literal()
                    .ok_or("Unexpected array value for literal".to_string())
                    .map(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value)
                    .flatten()
                )
                .flatten()?,
            event_file_download_requested: variables.remove("eventFileDownloadRequested")
                .map(libgql::executor::Value::to_non_nullable_option)
                .flatten()
                .ok_or("EventFiltersIn: Required field eventFileDownloadRequested is missing or null".to_string())
                .map(|v| 
                    v.get_literal()
                    .ok_or("Unexpected array value for literal".to_string())
                    .map(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value)
                    .flatten()
                )
                .flatten()?,
            event_file_downloaded: variables.remove("eventFileDownloaded")
                .map(libgql::executor::Value::to_non_nullable_option)
                .flatten()
                .ok_or("EventFiltersIn: Required field eventFileDownloaded is missing or null".to_string())
                .map(|v| 
                    v.get_literal()
                    .ok_or("Unexpected array value for literal".to_string())
                    .map(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value)
                    .flatten()
                )
                .flatten()?,
            event_file_tags_edited: variables.remove("eventFileTagsEdited")
                .map(libgql::executor::Value::to_non_nullable_option)
                .flatten()
                .ok_or("EventFiltersIn: Required field eventFileTagsEdited is missing or null".to_string())
                .map(|v| 
                    v.get_literal()
                    .ok_or("Unexpected array value for literal".to_string())
                    .map(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value)
                    .flatten()
                )
                .flatten()?,
            event_file_uploaded: variables.remove("eventFileUploaded")
                .map(libgql::executor::Value::to_non_nullable_option)
                .flatten()
                .ok_or("EventFiltersIn: Required field eventFileUploaded is missing or null".to_string())
                .map(|v| 
                    v.get_literal()
                    .ok_or("Unexpected array value for literal".to_string())
                    .map(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value)
                    .flatten()
                )
                .flatten()?,
            event_tag_approval_is_requested: variables.remove("eventTagApprovalIsRequested")
                .map(libgql::executor::Value::to_non_nullable_option)
                .flatten()
                .ok_or("EventFiltersIn: Required field eventTagApprovalIsRequested is missing or null".to_string())
                .map(|v| 
                    v.get_literal()
                    .ok_or("Unexpected array value for literal".to_string())
                    .map(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value)
                    .flatten()
                )
                .flatten()?
        })
    }
}

pub struct FileSortBy {
    pub direction: ESortDirection,
    pub field: EFileField,
}

impl libgql::executor::GQLInput<super::scalar::ExampleScalar> for FileSortBy {
    fn from_variables(mut variables: libgql::executor::Values<super::scalar::ExampleScalar>) -> Result<Self, String> {
        Ok(FileSortBy{
            direction: variables.remove("direction")
                .map(libgql::executor::Value::to_non_nullable_option)
                .flatten()
                .ok_or("FileSortBy: Required field direction is missing or null".to_string())
                .map(|v| 
                    v.get_literal()
                    .ok_or("Unexpected array value for literal".to_string())
                    .map(<ESortDirection as libgql::executor::GQLEnum<super::scalar::ExampleScalar>>::from_literal_value)
                    .flatten()
                )
                .flatten()?,
            field: variables.remove("field")
                .map(libgql::executor::Value::to_non_nullable_option)
                .flatten()
                .ok_or("FileSortBy: Required field field is missing or null".to_string())
                .map(|v| 
                    v.get_literal()
                    .ok_or("Unexpected array value for literal".to_string())
                    .map(<EFileField as libgql::executor::GQLEnum<super::scalar::ExampleScalar>>::from_literal_value)
                    .flatten()
                )
                .flatten()?
        })
    }
}

pub struct Filter {
    pub column_id: uuid::Uuid,
    pub date_range: Option<FilterDateRange>,
    pub list_values: Option<Vec<String>>,
    pub number_range: Option<NumberRange>,
}

impl libgql::executor::GQLInput<super::scalar::ExampleScalar> for Filter {
    fn from_variables(mut variables: libgql::executor::Values<super::scalar::ExampleScalar>) -> Result<Self, String> {
        Ok(Filter{
            column_id: variables.remove("columnId")
                .map(libgql::executor::Value::to_non_nullable_option)
                .flatten()
                .ok_or("Filter: Required field columnId is missing or null".to_string())
                .map(|v| 
                    v.get_literal()
                    .ok_or("Unexpected array value for literal".to_string())
                    .map(<uuid::Uuid as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value)
                    .flatten()
                )
                .flatten()?,
            date_range: variables.remove("dateRange")
                .map(libgql::executor::Value::to_non_nullable_option)
                .flatten()
                .map(|v| 
                    v.get_literal()
                    .ok_or("Unexpected array value for literal".to_string())
                    .map(<FilterDateRange as libgql::executor::GQLInput<super::scalar::ExampleScalar>>::from_literal_value)
                    .flatten()        
                ).transpose()?,
            list_values: variables.remove("listValues")
                .map(libgql::executor::Value::to_non_nullable_option)
                .flatten()
                .map(|v| libgql::executor::ast::extract_array(v, |element: libgql::executor::Value<super::scalar::ExampleScalar>| element.to_non_nullable_option().ok_or("Unexpected null in non-nullable array".to_string()).map(|v| 
                    v.get_literal()
                    .ok_or("Unexpected array value for literal".to_string())
                    .map(<String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value)
                    .flatten()).flatten())        
                ).transpose()?,
            number_range: variables.remove("numberRange")
                .map(libgql::executor::Value::to_non_nullable_option)
                .flatten()
                .map(|v| 
                    v.get_literal()
                    .ok_or("Unexpected array value for literal".to_string())
                    .map(<NumberRange as libgql::executor::GQLInput<super::scalar::ExampleScalar>>::from_literal_value)
                    .flatten()        
                ).transpose()?
        })
    }
}

pub struct FilterDateRange {
    pub end_at: Option<chrono::DateTime<chrono::Utc>>,
    pub start_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl libgql::executor::GQLInput<super::scalar::ExampleScalar> for FilterDateRange {
    fn from_variables(mut variables: libgql::executor::Values<super::scalar::ExampleScalar>) -> Result<Self, String> {
        Ok(FilterDateRange{
            end_at: variables.remove("endAt")
                .map(libgql::executor::Value::to_non_nullable_option)
                .flatten()
                .map(|v| 
                    v.get_literal()
                    .ok_or("Unexpected array value for literal".to_string())
                    .map(<chrono::DateTime<chrono::Utc> as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value)
                    .flatten()        
                ).transpose()?,
            start_at: variables.remove("startAt")
                .map(libgql::executor::Value::to_non_nullable_option)
                .flatten()
                .map(|v| 
                    v.get_literal()
                    .ok_or("Unexpected array value for literal".to_string())
                    .map(<chrono::DateTime<chrono::Utc> as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value)
                    .flatten()        
                ).transpose()?
        })
    }
}

pub struct GetGroupUsersSortBy {
    pub direction: ESortDirection,
    pub field: EGroupUsersField,
}

impl libgql::executor::GQLInput<super::scalar::ExampleScalar> for GetGroupUsersSortBy {
    fn from_variables(mut variables: libgql::executor::Values<super::scalar::ExampleScalar>) -> Result<Self, String> {
        Ok(GetGroupUsersSortBy{
            direction: variables.remove("direction")
                .map(libgql::executor::Value::to_non_nullable_option)
                .flatten()
                .ok_or("GetGroupUsersSortBy: Required field direction is missing or null".to_string())
                .map(|v| 
                    v.get_literal()
                    .ok_or("Unexpected array value for literal".to_string())
                    .map(<ESortDirection as libgql::executor::GQLEnum<super::scalar::ExampleScalar>>::from_literal_value)
                    .flatten()
                )
                .flatten()?,
            field: variables.remove("field")
                .map(libgql::executor::Value::to_non_nullable_option)
                .flatten()
                .ok_or("GetGroupUsersSortBy: Required field field is missing or null".to_string())
                .map(|v| 
                    v.get_literal()
                    .ok_or("Unexpected array value for literal".to_string())
                    .map(<EGroupUsersField as libgql::executor::GQLEnum<super::scalar::ExampleScalar>>::from_literal_value)
                    .flatten()
                )
                .flatten()?
        })
    }
}

pub struct GetGroupsSortBy {
    pub direction: ESortDirection,
    pub field: EGroupField,
}

impl libgql::executor::GQLInput<super::scalar::ExampleScalar> for GetGroupsSortBy {
    fn from_variables(mut variables: libgql::executor::Values<super::scalar::ExampleScalar>) -> Result<Self, String> {
        Ok(GetGroupsSortBy{
            direction: variables.remove("direction")
                .map(libgql::executor::Value::to_non_nullable_option)
                .flatten()
                .ok_or("GetGroupsSortBy: Required field direction is missing or null".to_string())
                .map(|v| 
                    v.get_literal()
                    .ok_or("Unexpected array value for literal".to_string())
                    .map(<ESortDirection as libgql::executor::GQLEnum<super::scalar::ExampleScalar>>::from_literal_value)
                    .flatten()
                )
                .flatten()?,
            field: variables.remove("field")
                .map(libgql::executor::Value::to_non_nullable_option)
                .flatten()
                .ok_or("GetGroupsSortBy: Required field field is missing or null".to_string())
                .map(|v| 
                    v.get_literal()
                    .ok_or("Unexpected array value for literal".to_string())
                    .map(<EGroupField as libgql::executor::GQLEnum<super::scalar::ExampleScalar>>::from_literal_value)
                    .flatten()
                )
                .flatten()?
        })
    }
}

pub struct GetUsersSortBy {
    pub direction: ESortDirection,
    pub field: EUserField,
}

impl libgql::executor::GQLInput<super::scalar::ExampleScalar> for GetUsersSortBy {
    fn from_variables(mut variables: libgql::executor::Values<super::scalar::ExampleScalar>) -> Result<Self, String> {
        Ok(GetUsersSortBy{
            direction: variables.remove("direction")
                .map(libgql::executor::Value::to_non_nullable_option)
                .flatten()
                .ok_or("GetUsersSortBy: Required field direction is missing or null".to_string())
                .map(|v| 
                    v.get_literal()
                    .ok_or("Unexpected array value for literal".to_string())
                    .map(<ESortDirection as libgql::executor::GQLEnum<super::scalar::ExampleScalar>>::from_literal_value)
                    .flatten()
                )
                .flatten()?,
            field: variables.remove("field")
                .map(libgql::executor::Value::to_non_nullable_option)
                .flatten()
                .ok_or("GetUsersSortBy: Required field field is missing or null".to_string())
                .map(|v| 
                    v.get_literal()
                    .ok_or("Unexpected array value for literal".to_string())
                    .map(<EUserField as libgql::executor::GQLEnum<super::scalar::ExampleScalar>>::from_literal_value)
                    .flatten()
                )
                .flatten()?
        })
    }
}

pub struct GroupIn {
    pub limit_of_downloads_per_day: i32,
    pub name: String,
    pub tag_ids: Vec<uuid::Uuid>,
}

impl libgql::executor::GQLInput<super::scalar::ExampleScalar> for GroupIn {
    fn from_variables(mut variables: libgql::executor::Values<super::scalar::ExampleScalar>) -> Result<Self, String> {
        Ok(GroupIn{
            limit_of_downloads_per_day: variables.remove("limitOfDownloadsPerDay")
                .map(libgql::executor::Value::to_non_nullable_option)
                .flatten()
                .ok_or("GroupIn: Required field limitOfDownloadsPerDay is missing or null".to_string())
                .map(|v| 
                    v.get_literal()
                    .ok_or("Unexpected array value for literal".to_string())
                    .map(<i32 as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value)
                    .flatten()
                )
                .flatten()?,
            name: variables.remove("name")
                .map(libgql::executor::Value::to_non_nullable_option)
                .flatten()
                .ok_or("GroupIn: Required field name is missing or null".to_string())
                .map(|v| 
                    v.get_literal()
                    .ok_or("Unexpected array value for literal".to_string())
                    .map(<String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value)
                    .flatten()
                )
                .flatten()?,
            tag_ids: variables.remove("tagIds")
                .map(libgql::executor::Value::to_non_nullable_option)
                .flatten()
                .ok_or("GroupIn: Required field tagIds is missing or null".to_string())
                .map(|v| libgql::executor::ast::extract_array(v, |element: libgql::executor::Value<super::scalar::ExampleScalar>| element.to_non_nullable_option().ok_or("Unexpected null in non-nullable array".to_string()).map(|v| 
                    v.get_literal()
                    .ok_or("Unexpected array value for literal".to_string())
                    .map(<uuid::Uuid as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value)
                    .flatten()).flatten())
                )
                .flatten()?
        })
    }
}

pub struct MultipartUploadFileIn {
    pub initial_parts_count: i32,
    pub name: String,
    pub part_size_in_bytes: i64,
    pub size_in_bytes: i64,
    pub tag_ids: Vec<uuid::Uuid>,
}

impl libgql::executor::GQLInput<super::scalar::ExampleScalar> for MultipartUploadFileIn {
    fn from_variables(mut variables: libgql::executor::Values<super::scalar::ExampleScalar>) -> Result<Self, String> {
        Ok(MultipartUploadFileIn{
            initial_parts_count: variables.remove("initialPartsCount")
                .map(libgql::executor::Value::to_non_nullable_option)
                .flatten()
                .ok_or("MultipartUploadFileIn: Required field initialPartsCount is missing or null".to_string())
                .map(|v| 
                    v.get_literal()
                    .ok_or("Unexpected array value for literal".to_string())
                    .map(<i32 as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value)
                    .flatten()
                )
                .flatten()?,
            name: variables.remove("name")
                .map(libgql::executor::Value::to_non_nullable_option)
                .flatten()
                .ok_or("MultipartUploadFileIn: Required field name is missing or null".to_string())
                .map(|v| 
                    v.get_literal()
                    .ok_or("Unexpected array value for literal".to_string())
                    .map(<String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value)
                    .flatten()
                )
                .flatten()?,
            part_size_in_bytes: variables.remove("partSizeInBytes")
                .map(libgql::executor::Value::to_non_nullable_option)
                .flatten()
                .ok_or("MultipartUploadFileIn: Required field partSizeInBytes is missing or null".to_string())
                .map(|v| 
                    v.get_literal()
                    .ok_or("Unexpected array value for literal".to_string())
                    .map(<i64 as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value)
                    .flatten()
                )
                .flatten()?,
            size_in_bytes: variables.remove("sizeInBytes")
                .map(libgql::executor::Value::to_non_nullable_option)
                .flatten()
                .ok_or("MultipartUploadFileIn: Required field sizeInBytes is missing or null".to_string())
                .map(|v| 
                    v.get_literal()
                    .ok_or("Unexpected array value for literal".to_string())
                    .map(<i64 as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value)
                    .flatten()
                )
                .flatten()?,
            tag_ids: variables.remove("tagIds")
                .map(libgql::executor::Value::to_non_nullable_option)
                .flatten()
                .ok_or("MultipartUploadFileIn: Required field tagIds is missing or null".to_string())
                .map(|v| libgql::executor::ast::extract_array(v, |element: libgql::executor::Value<super::scalar::ExampleScalar>| element.to_non_nullable_option().ok_or("Unexpected null in non-nullable array".to_string()).map(|v| 
                    v.get_literal()
                    .ok_or("Unexpected array value for literal".to_string())
                    .map(<uuid::Uuid as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value)
                    .flatten()).flatten())
                )
                .flatten()?
        })
    }
}

pub struct NumberRange {
    pub end_at: Option<f32>,
    pub start_at: Option<f32>,
}

impl libgql::executor::GQLInput<super::scalar::ExampleScalar> for NumberRange {
    fn from_variables(mut variables: libgql::executor::Values<super::scalar::ExampleScalar>) -> Result<Self, String> {
        Ok(NumberRange{
            end_at: variables.remove("endAt")
                .map(libgql::executor::Value::to_non_nullable_option)
                .flatten()
                .map(|v| 
                    v.get_literal()
                    .ok_or("Unexpected array value for literal".to_string())
                    .map(<f32 as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value)
                    .flatten()        
                ).transpose()?,
            start_at: variables.remove("startAt")
                .map(libgql::executor::Value::to_non_nullable_option)
                .flatten()
                .map(|v| 
                    v.get_literal()
                    .ok_or("Unexpected array value for literal".to_string())
                    .map(<f32 as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value)
                    .flatten()        
                ).transpose()?
        })
    }
}

pub struct PutUploadFileIn {
    pub name: String,
    pub size_in_bytes: i64,
    pub tag_ids: Vec<uuid::Uuid>,
}

impl libgql::executor::GQLInput<super::scalar::ExampleScalar> for PutUploadFileIn {
    fn from_variables(mut variables: libgql::executor::Values<super::scalar::ExampleScalar>) -> Result<Self, String> {
        Ok(PutUploadFileIn{
            name: variables.remove("name")
                .map(libgql::executor::Value::to_non_nullable_option)
                .flatten()
                .ok_or("PutUploadFileIn: Required field name is missing or null".to_string())
                .map(|v| 
                    v.get_literal()
                    .ok_or("Unexpected array value for literal".to_string())
                    .map(<String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value)
                    .flatten()
                )
                .flatten()?,
            size_in_bytes: variables.remove("sizeInBytes")
                .map(libgql::executor::Value::to_non_nullable_option)
                .flatten()
                .ok_or("PutUploadFileIn: Required field sizeInBytes is missing or null".to_string())
                .map(|v| 
                    v.get_literal()
                    .ok_or("Unexpected array value for literal".to_string())
                    .map(<i64 as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value)
                    .flatten()
                )
                .flatten()?,
            tag_ids: variables.remove("tagIds")
                .map(libgql::executor::Value::to_non_nullable_option)
                .flatten()
                .ok_or("PutUploadFileIn: Required field tagIds is missing or null".to_string())
                .map(|v| libgql::executor::ast::extract_array(v, |element: libgql::executor::Value<super::scalar::ExampleScalar>| element.to_non_nullable_option().ok_or("Unexpected null in non-nullable array".to_string()).map(|v| 
                    v.get_literal()
                    .ok_or("Unexpected array value for literal".to_string())
                    .map(<uuid::Uuid as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value)
                    .flatten()).flatten())
                )
                .flatten()?
        })
    }
}

pub struct TagIn {
    pub parent_tag_id: Option<uuid::Uuid>,
    pub tag: String,
}

impl libgql::executor::GQLInput<super::scalar::ExampleScalar> for TagIn {
    fn from_variables(mut variables: libgql::executor::Values<super::scalar::ExampleScalar>) -> Result<Self, String> {
        Ok(TagIn{
            parent_tag_id: variables.remove("parentTagId")
                .map(libgql::executor::Value::to_non_nullable_option)
                .flatten()
                .map(|v| 
                    v.get_literal()
                    .ok_or("Unexpected array value for literal".to_string())
                    .map(<uuid::Uuid as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value)
                    .flatten()        
                ).transpose()?,
            tag: variables.remove("tag")
                .map(libgql::executor::Value::to_non_nullable_option)
                .flatten()
                .ok_or("TagIn: Required field tag is missing or null".to_string())
                .map(|v| 
                    v.get_literal()
                    .ok_or("Unexpected array value for literal".to_string())
                    .map(<String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value)
                    .flatten()
                )
                .flatten()?
        })
    }
}

pub struct UserIn {
    pub email: String,
    pub group_ids: Vec<uuid::Uuid>,
    pub name: String,
}

impl libgql::executor::GQLInput<super::scalar::ExampleScalar> for UserIn {
    fn from_variables(mut variables: libgql::executor::Values<super::scalar::ExampleScalar>) -> Result<Self, String> {
        Ok(UserIn{
            email: variables.remove("email")
                .map(libgql::executor::Value::to_non_nullable_option)
                .flatten()
                .ok_or("UserIn: Required field email is missing or null".to_string())
                .map(|v| 
                    v.get_literal()
                    .ok_or("Unexpected array value for literal".to_string())
                    .map(<String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value)
                    .flatten()
                )
                .flatten()?,
            group_ids: variables.remove("groupIds")
                .map(libgql::executor::Value::to_non_nullable_option)
                .flatten()
                .ok_or("UserIn: Required field groupIds is missing or null".to_string())
                .map(|v| libgql::executor::ast::extract_array(v, |element: libgql::executor::Value<super::scalar::ExampleScalar>| element.to_non_nullable_option().ok_or("Unexpected null in non-nullable array".to_string()).map(|v| 
                    v.get_literal()
                    .ok_or("Unexpected array value for literal".to_string())
                    .map(<uuid::Uuid as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value)
                    .flatten()).flatten())
                )
                .flatten()?,
            name: variables.remove("name")
                .map(libgql::executor::Value::to_non_nullable_option)
                .flatten()
                .ok_or("UserIn: Required field name is missing or null".to_string())
                .map(|v| 
                    v.get_literal()
                    .ok_or("Unexpected array value for literal".to_string())
                    .map(<String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::from_literal_value)
                    .flatten()
                )
                .flatten()?
        })
    }
}

pub struct UsersTagSortBy {
    pub direction: ESortDirection,
    pub field: EUsersTagField,
}

impl libgql::executor::GQLInput<super::scalar::ExampleScalar> for UsersTagSortBy {
    fn from_variables(mut variables: libgql::executor::Values<super::scalar::ExampleScalar>) -> Result<Self, String> {
        Ok(UsersTagSortBy{
            direction: variables.remove("direction")
                .map(libgql::executor::Value::to_non_nullable_option)
                .flatten()
                .ok_or("UsersTagSortBy: Required field direction is missing or null".to_string())
                .map(|v| 
                    v.get_literal()
                    .ok_or("Unexpected array value for literal".to_string())
                    .map(<ESortDirection as libgql::executor::GQLEnum<super::scalar::ExampleScalar>>::from_literal_value)
                    .flatten()
                )
                .flatten()?,
            field: variables.remove("field")
                .map(libgql::executor::Value::to_non_nullable_option)
                .flatten()
                .ok_or("UsersTagSortBy: Required field field is missing or null".to_string())
                .map(|v| 
                    v.get_literal()
                    .ok_or("Unexpected array value for literal".to_string())
                    .map(<EUsersTagField as libgql::executor::GQLEnum<super::scalar::ExampleScalar>>::from_literal_value)
                    .flatten()
                )
                .flatten()?
        })
    }
}

pub struct BooleanObject {
    pub bvalue: bool,
}

pub struct DatetimeObject {
    pub dvalue: chrono::DateTime<chrono::Utc>,
}

pub struct DealColumn {
    pub available_values: Vec<String>,
    pub id: uuid::Uuid,
    pub name: String,
    pub r#type: EDealColumnType,
}

pub struct DealEntry {
    pub column_name: String,
    pub value: Tag,
}

pub struct DealInfo {
    pub stage_to_worktypes_map: Vec<StageToWorktypesMapEntry>,
    pub values: Vec<DealEntry>,
}

pub struct ErrorAlreadyApprovedByAdmin {
    pub a: Option<bool>,
}

pub struct ErrorAlreadyDone {
    pub a: Option<bool>,
}

pub struct ErrorAlreadyExists {
    pub a: Option<bool>,
}

pub struct ErrorAlreadyPending {
    pub a: Option<bool>,
}

pub struct ErrorCantAddAutotags {
    pub a: Option<bool>,
}

pub struct ErrorChangeForbidden {
    pub a: Option<bool>,
}

pub struct ErrorDateRangeIsInvalid {
    pub a: Option<bool>,
}

pub struct ErrorEmailCollision {
    pub a: Option<bool>,
}

pub struct ErrorFileNotUploaded {
    pub a: Option<bool>,
}

pub struct ErrorFilesChangeForbidden {
    pub ids: Vec<uuid::Uuid>,
}

pub struct ErrorGroupNotFound {
    pub a: Option<bool>,
}

pub struct ErrorInvalidCredentials {
    pub a: Option<bool>,
}

pub struct ErrorInvalidEmail {
    pub a: Option<bool>,
}

pub struct ErrorInvalidGroupName {
    pub a: Option<bool>,
}

pub struct ErrorInvalidLimitOfDownloadsPerDay {
    pub a: Option<bool>,
}

pub struct ErrorInvalidOTPCode {
    pub a: Option<bool>,
}

pub struct ErrorInvalidPassword {
    pub a: Option<bool>,
}

pub struct ErrorInvalidToken {
    pub a: Option<bool>,
}

pub struct ErrorInvalidUserName {
    pub a: Option<bool>,
}

pub struct ErrorMultipartUploadFileIsTooBig {
    pub a: Option<bool>,
}

pub struct ErrorMultipartUploadFileIsTooLight {
    pub a: Option<bool>,
}

pub struct ErrorMultipartUploadFilePartSizeIsTooBig {
    pub a: Option<bool>,
}

pub struct ErrorMultipartUploadFilePartSizeIsTooSmall {
    pub a: Option<bool>,
}

pub struct ErrorNoDealTag {
    pub a: Option<bool>,
}

pub struct ErrorNotFound {
    pub a: Option<bool>,
}

pub struct ErrorOTPCodeExpired {
    pub a: Option<bool>,
}

pub struct ErrorOTPTokenExpired {
    pub a: Option<bool>,
}

pub struct ErrorPutUploadFileIsTooBig {
    pub a: Option<bool>,
}

pub struct ErrorUnknownFile {
    pub a: Option<bool>,
}

pub struct ErrorUnknownFiles {
    pub ids: Vec<uuid::Uuid>,
}

pub struct ErrorUnknownGroupIds {
    pub group_ids: Vec<uuid::Uuid>,
}

pub struct ErrorUnknownGroups {
    pub group_ids: Vec<uuid::Uuid>,
}

pub struct ErrorUnknownParentId {
    pub a: Option<bool>,
}

pub struct ErrorUnknownSessionId {
    pub a: Option<bool>,
}

pub struct ErrorUnknownTags {
    pub tag_ids: Vec<uuid::Uuid>,
}

pub struct ErrorUnknownUser {
    pub a: Option<bool>,
}

pub struct ErrorUnknownUsers {
    pub user_ids: Vec<uuid::Uuid>,
}

pub struct EventFileDeleted {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub file: File,
}

pub struct EventFileDownloadRequested {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub decision: Option<bool>,
    pub file: File,
    pub user: User,
}

pub struct EventFileDownloaded {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub file: File,
}

pub struct EventFileTagsEdited {
    pub added_tags: Vec<Tag>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub file: File,
    pub removed_tags: Vec<Tag>,
}

pub struct EventFileUploaded {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub file: File,
}

pub struct EventTagApprovalIsRequested {
    pub already_in_catalog: bool,
    pub author: User,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub tag: Tag,
}

pub struct EventsList {
    pub events: Vec<Event>,
}

pub struct File {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub filename: String,
    pub id: uuid::Uuid,
    pub mime_type: Option<String>,
    pub preview_url: Option<url::Url>,
    pub size_in_bytes: i64,
    pub user: User,
}

pub struct FilesDealInfo {
    pub deal_info: DealInfo,
    pub deal_name: Tag,
    pub unset_columns: Vec<DealColumn>,
}

pub struct FloatObject {
    pub fvalue: f32,
}

pub struct Group {
    pub first_10_tags: Vec<Tag>,
    pub id: uuid::Uuid,
    pub limit_of_downloads_per_day: i32,
    pub name: String,
}

pub struct GroupUser {
    pub in_group: bool,
    pub user: User,
}

pub struct GroupUserList {
    pub users: Vec<GroupUser>,
}

pub struct IntObject {
    pub ivalue: i32,
}

pub struct MultipartUploadSession {
    pub id: uuid::Uuid,
    pub initial_upload_ur_ls: Vec<UploadUrl>,
}

pub struct Mutation {
    pub add_tags_to_files: Option<AddTagsToFilesError>,
    pub add_user_to_group: Option<ErrorGroupNotFoundOrErrorNotFound>,
    pub approve_tag: Option<ApproveTagError>,
    pub change_password: Option<ErrorInvalidCredentials>,
    pub commit_multipart_file_session: Option<CommitMultipartFileSessionResponse>,
    pub commit_put_file_session: Option<CommitPutFileSessionResponse>,
    pub confirm_otp_code: ConfirmOTPCodeResponse,
    pub confirm_user: Option<ConfirmUserError>,
    pub create_group: Option<CreateGroupError>,
    pub create_multipart_file_session: CreateMultipartFileSessionResponse,
    pub create_put_file_session: CreatePutFileSessionResponse,
    pub create_tag: Option<CreateTagError>,
    pub create_user: Option<CreateUserError>,
    pub decide_on_download_request: Option<DecideOnDownloadRequestError>,
    pub delete_file: Option<DeleteFileError>,
    pub delete_files: Option<DeleteFilesError>,
    pub delete_group: Option<ErrorGroupNotFound>,
    pub delete_pending_user: Option<ErrorNotFound>,
    pub delete_tag: Option<ErrorNotFound>,
    pub delete_user: Option<ErrorNotFound>,
    pub edit_group: Option<EditGroupError>,
    pub edit_tag: Option<EditTagError>,
    pub login: Option<ErrorInvalidCredentials>,
    pub logout: (),
    pub remove_user_from_group: Option<ErrorGroupNotFoundOrErrorNotFound>,
    pub reset_password: Option<ResetPasswordError>,
    pub send_otp_code: Option<ErrorInvalidCredentials>,
    pub set_tag_is_favourite: Option<ErrorAlreadyDoneOrUnknownTags>,
    pub update_file: Option<UpdateFileError>,
    pub update_files_autotags: Option<ErrorCantAddAutotags>,
}

pub struct OTPToken {
    pub token: String,
}

pub struct PendingUser {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub email: String,
    pub groups: Vec<Group>,
    pub name: String,
    pub ttl: f32,
}

pub struct PutUploadSession {
    pub id: uuid::Uuid,
    pub upload_url: UploadUrl,
}

pub struct Query {
    pub get_deal_columns: Vec<DealColumn>,
    pub get_deal_info: GetDealInfoResponse,
    pub get_deals: Vec<String>,
    pub get_events: GetEventsResponse,
    pub get_favourite_tags: Vec<Tag>,
    pub get_file_url: GetFileURLResponse,
    pub get_files: GetFilesResponse,
    pub get_files_count: IntObjectOrErrorUnknownTags,
    pub get_files_deal_info: FilesDealInfoOrError,
    pub get_group_tags: GetGroupTagsResponse,
    pub get_group_users: GetGroupUsersResponse,
    pub get_group_users_and_users: GetGroupUsersAndUsersResponse,
    pub get_group_users_total: GetGroupUsersTotalResponse,
    pub get_groups: Vec<Group>,
    pub get_groups_total: i32,
    pub get_me: User,
    pub get_my_tags: Vec<Tag>,
    pub get_my_tags_count: i32,
    pub get_next_multipart_upload_urls: GetNextMultipartUploadUrlsResponse,
    pub get_path_to_tag: GetPathToTagResponse,
    pub get_pending_users: Vec<PendingUser>,
    pub get_popular_tags: Vec<Tag>,
    pub get_tag_children: GetTagsResponse,
    pub get_tag_info: GetTagInfoResponse,
    pub get_tags: GetTagsResponse,
    pub get_tags_count: IntObjectOrErrorUnknownTags,
    pub get_uploaded_files: Vec<SearchFile>,
    pub get_uploaded_files_count: i32,
    pub get_users: Vec<User>,
    pub get_users_tags: Vec<UsersTag>,
    pub get_users_tags_count: i32,
    pub get_users_total: i32,
    pub is_allowed_to_download: IsAllowedToDownloadResponse,
    pub is_tag_exists: bool,
    pub retrieve_file: RetrieveFileResponse,
    pub retrieve_group: RetrieveGroupResponse,
    pub search_tags: Vec<Tag>,
}

pub struct SearchFile {
    pub file: File,
    pub tags: Vec<Tag>,
}

pub struct SearchFileList {
    pub files: Vec<SearchFile>,
}

pub struct StageToWorktypesMapEntry {
    pub stage: Tag,
    pub worktypes: Vec<Tag>,
}

pub struct StringEntry {
    pub key: String,
    pub value: String,
}

pub struct StringList {
    pub values: Vec<String>,
}

pub struct StringObject {
    pub svalue: String,
}

pub struct Tag {
    pub has_children: bool,
    pub id: uuid::Uuid,
    pub is_approved: bool,
    pub is_favourite: bool,
    pub tag: String,
    pub value: Option<TagValue>,
}

pub struct TagInfo {
    pub parent_tag: Option<Tag>,
    pub tag: String,
}

pub struct TagList {
    pub list: Vec<Tag>,
}

pub struct UploadUrl {
    pub headers: Vec<StringEntry>,
    pub url: url::Url,
}

pub struct UploadUrlList {
    pub urls: Vec<UploadUrl>,
}

pub struct UrlObject {
    pub uvalue: url::Url,
}

pub struct User {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub email: String,
    pub id: uuid::Uuid,
    pub name: String,
    pub ten_groups: Vec<Group>,
}

pub struct UsersList {
    pub users: Vec<User>,
}

pub struct UsersTag {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub tag: Tag,
    pub users_count: i32,
}

pub enum AddTagsToFilesError {
    ErrorUnknownFiles(ErrorUnknownFiles),
    ErrorUnknownTags(ErrorUnknownTags),
}

pub enum ApproveTagError {
    ErrorAlreadyExists(ErrorAlreadyExists),
    ErrorNotFound(ErrorNotFound),
    ErrorUnknownGroupIds(ErrorUnknownGroupIds),
}

pub enum CommitMultipartFileSessionResponse {
    ErrorFileNotUploaded(ErrorFileNotUploaded),
    ErrorUnknownSessionId(ErrorUnknownSessionId),
    File(File),
}

pub enum CommitPutFileSessionResponse {
    ErrorFileNotUploaded(ErrorFileNotUploaded),
    ErrorUnknownSessionId(ErrorUnknownSessionId),
    File(File),
}

pub enum ConfirmOTPCodeResponse {
    ErrorInvalidOTPCode(ErrorInvalidOTPCode),
    ErrorOTPCodeExpired(ErrorOTPCodeExpired),
    OTPToken(OTPToken),
}

pub enum ConfirmUserError {
    ErrorInvalidPassword(ErrorInvalidPassword),
    ErrorInvalidToken(ErrorInvalidToken),
}

pub enum CreateGroupError {
    ErrorAlreadyExists(ErrorAlreadyExists),
    ErrorInvalidGroupName(ErrorInvalidGroupName),
    ErrorInvalidLimitOfDownloadsPerDay(ErrorInvalidLimitOfDownloadsPerDay),
    ErrorUnknownTags(ErrorUnknownTags),
    ErrorUnknownUsers(ErrorUnknownUsers),
}

pub enum CreateMultipartFileSessionResponse {
    ErrorMultipartUploadFileIsTooBig(ErrorMultipartUploadFileIsTooBig),
    ErrorMultipartUploadFileIsTooLight(ErrorMultipartUploadFileIsTooLight),
    ErrorMultipartUploadFilePartSizeIsTooBig(ErrorMultipartUploadFilePartSizeIsTooBig),
    ErrorMultipartUploadFilePartSizeIsTooSmall(ErrorMultipartUploadFilePartSizeIsTooSmall),
    ErrorNoDealTag(ErrorNoDealTag),
    ErrorUnknownTags(ErrorUnknownTags),
    MultipartUploadSession(MultipartUploadSession),
}

pub enum CreatePutFileSessionResponse {
    ErrorNoDealTag(ErrorNoDealTag),
    ErrorPutUploadFileIsTooBig(ErrorPutUploadFileIsTooBig),
    ErrorUnknownTags(ErrorUnknownTags),
    PutUploadSession(PutUploadSession),
}

pub enum CreateTagError {
    ErrorAlreadyExists(ErrorAlreadyExists),
    ErrorUnknownParentId(ErrorUnknownParentId),
}

pub enum CreateUserError {
    ErrorAlreadyPending(ErrorAlreadyPending),
    ErrorEmailCollision(ErrorEmailCollision),
    ErrorInvalidEmail(ErrorInvalidEmail),
    ErrorInvalidUserName(ErrorInvalidUserName),
    ErrorUnknownGroups(ErrorUnknownGroups),
}

pub enum DecideOnDownloadRequestError {
    ErrorAlreadyDone(ErrorAlreadyDone),
    ErrorNotFound(ErrorNotFound),
    ErrorUnknownFile(ErrorUnknownFile),
    ErrorUnknownUser(ErrorUnknownUser),
}

pub enum DeleteFileError {
    ErrorChangeForbidden(ErrorChangeForbidden),
    ErrorUnknownFile(ErrorUnknownFile),
}

pub enum DeleteFilesError {
    ErrorFilesChangeForbidden(ErrorFilesChangeForbidden),
    ErrorUnknownFiles(ErrorUnknownFiles),
}

pub enum EditGroupError {
    ErrorAlreadyExists(ErrorAlreadyExists),
    ErrorGroupNotFound(ErrorGroupNotFound),
    ErrorInvalidGroupName(ErrorInvalidGroupName),
    ErrorInvalidLimitOfDownloadsPerDay(ErrorInvalidLimitOfDownloadsPerDay),
    ErrorUnknownTags(ErrorUnknownTags),
}

pub enum EditTagError {
    ErrorAlreadyApprovedByAdmin(ErrorAlreadyApprovedByAdmin),
    ErrorAlreadyExists(ErrorAlreadyExists),
    ErrorNotFound(ErrorNotFound),
    ErrorUnknownParentId(ErrorUnknownParentId),
}

pub enum ErrorAlreadyDoneOrUnknownTags {
    ErrorAlreadyDone(ErrorAlreadyDone),
    ErrorUnknownTags(ErrorUnknownTags),
}

pub enum ErrorGroupNotFoundOrErrorNotFound {
    ErrorGroupNotFound(ErrorGroupNotFound),
    ErrorNotFound(ErrorNotFound),
}

pub enum Event {
    EventFileDeleted(EventFileDeleted),
    EventFileDownloadRequested(EventFileDownloadRequested),
    EventFileDownloaded(EventFileDownloaded),
    EventFileTagsEdited(EventFileTagsEdited),
    EventFileUploaded(EventFileUploaded),
    EventTagApprovalIsRequested(EventTagApprovalIsRequested),
}

pub enum FilesDealInfoOrError {
    ErrorCantAddAutotags(ErrorCantAddAutotags),
    FilesDealInfo(FilesDealInfo),
}

pub enum GetDealInfoResponse {
    DealInfo(DealInfo),
    ErrorNotFound(ErrorNotFound),
}

pub enum GetEventsResponse {
    ErrorDateRangeIsInvalid(ErrorDateRangeIsInvalid),
    EventsList(EventsList),
}

pub enum GetFileURLResponse {
    ErrorUnknownFile(ErrorUnknownFile),
    UrlObject(UrlObject),
}

pub enum GetFilesResponse {
    ErrorUnknownTags(ErrorUnknownTags),
    SearchFileList(SearchFileList),
}

pub enum GetGroupTagsResponse {
    ErrorNotFound(ErrorNotFound),
    TagList(TagList),
}

pub enum GetGroupUsersAndUsersResponse {
    ErrorGroupNotFound(ErrorGroupNotFound),
    GroupUserList(GroupUserList),
}

pub enum GetGroupUsersResponse {
    ErrorNotFound(ErrorNotFound),
    UsersList(UsersList),
}

pub enum GetGroupUsersTotalResponse {
    ErrorNotFound(ErrorNotFound),
    IntObject(IntObject),
}

pub enum GetNextMultipartUploadUrlsResponse {
    ErrorUnknownSessionId(ErrorUnknownSessionId),
    UploadUrlList(UploadUrlList),
}

pub enum GetPathToTagResponse {
    ErrorUnknownTags(ErrorUnknownTags),
    StringList(StringList),
}

pub enum GetTagInfoResponse {
    ErrorNotFound(ErrorNotFound),
    TagInfo(TagInfo),
}

pub enum GetTagsResponse {
    ErrorUnknownTags(ErrorUnknownTags),
    TagList(TagList),
}

pub enum IntObjectOrErrorUnknownTags {
    ErrorUnknownTags(ErrorUnknownTags),
    IntObject(IntObject),
}

pub enum IsAllowedToDownloadResponse {
    BooleanObject(BooleanObject),
    ErrorUnknownFile(ErrorUnknownFile),
}

pub enum ResetPasswordError {
    ErrorInvalidPassword(ErrorInvalidPassword),
    ErrorOTPTokenExpired(ErrorOTPTokenExpired),
}

pub enum RetrieveFileResponse {
    ErrorUnknownFile(ErrorUnknownFile),
    SearchFile(SearchFile),
}

pub enum RetrieveGroupResponse {
    ErrorNotFound(ErrorNotFound),
    Group(Group),
}

pub enum TagValue {
    DatetimeObject(DatetimeObject),
    FloatObject(FloatObject),
    StringObject(StringObject),
}

pub enum UpdateFileError {
    ErrorChangeForbidden(ErrorChangeForbidden),
    ErrorUnknownFile(ErrorUnknownFile),
    ErrorUnknownTags(ErrorUnknownTags),
}