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

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for BooleanObject {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("BooleanObject".to_string(), libgql::executor::Values::from_iter([("bvalue".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(self.bvalue)?))),
        ])))
    }
}

pub struct DatetimeObject {
    pub dvalue: chrono::DateTime<chrono::Utc>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for DatetimeObject {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("DatetimeObject".to_string(), libgql::executor::Values::from_iter([("dvalue".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<chrono::DateTime<chrono::Utc> as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(self.dvalue)?))),
        ])))
    }
}

pub struct DealColumn {
    pub available_values: Vec<String>,
    pub id: uuid::Uuid,
    pub name: String,
    pub r#type: EDealColumnType,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for DealColumn {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("DealColumn".to_string(), libgql::executor::Values::from_iter([("availableValues".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Array(self.available_values.into_iter().map(|element| Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(element)?)))).collect::<Result<Vec<_>, String>>()?))),
        ("id".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<uuid::Uuid as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(self.id)?))),
        ("name".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(self.name)?))),
        ("type".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<EDealColumnType as libgql::executor::GQLEnum<super::scalar::ExampleScalar>>::to_literal_value(self.r#type)?))),
        ])))
    }
}

pub struct DealEntry {
    pub column_name: String,
    pub value: Tag,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for DealEntry {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("DealEntry".to_string(), libgql::executor::Values::from_iter([("columnName".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(self.column_name)?))),
        ("value".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(self.value)?.into()))),
        ])))
    }
}

pub struct DealInfo {
    pub stage_to_worktypes_map: Vec<StageToWorktypesMapEntry>,
    pub values: Vec<DealEntry>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for DealInfo {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("DealInfo".to_string(), libgql::executor::Values::from_iter([("stageToWorktypesMap".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Array(self.stage_to_worktypes_map.into_iter().map(|element| Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(element)?.into())))).collect::<Result<Vec<_>, String>>()?))),
        ("values".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Array(self.values.into_iter().map(|element| Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(element)?.into())))).collect::<Result<Vec<_>, String>>()?))),
        ])))
    }
}

pub struct ErrorAlreadyApprovedByAdmin {
    pub a: Option<bool>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorAlreadyApprovedByAdmin {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorAlreadyApprovedByAdmin".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorAlreadyDone {
    pub a: Option<bool>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorAlreadyDone {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorAlreadyDone".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorAlreadyExists {
    pub a: Option<bool>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorAlreadyExists {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorAlreadyExists".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorAlreadyPending {
    pub a: Option<bool>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorAlreadyPending {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorAlreadyPending".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorCantAddAutotags {
    pub a: Option<bool>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorCantAddAutotags {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorCantAddAutotags".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorChangeForbidden {
    pub a: Option<bool>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorChangeForbidden {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorChangeForbidden".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorDateRangeIsInvalid {
    pub a: Option<bool>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorDateRangeIsInvalid {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorDateRangeIsInvalid".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorEmailCollision {
    pub a: Option<bool>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorEmailCollision {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorEmailCollision".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorFileNotUploaded {
    pub a: Option<bool>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorFileNotUploaded {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorFileNotUploaded".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorFilesChangeForbidden {
    pub ids: Vec<uuid::Uuid>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorFilesChangeForbidden {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorFilesChangeForbidden".to_string(), libgql::executor::Values::from_iter([("ids".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Array(self.ids.into_iter().map(|element| Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<uuid::Uuid as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(element)?)))).collect::<Result<Vec<_>, String>>()?))),
        ])))
    }
}

pub struct ErrorGroupNotFound {
    pub a: Option<bool>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorGroupNotFound {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorGroupNotFound".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorInvalidCredentials {
    pub a: Option<bool>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorInvalidCredentials {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorInvalidCredentials".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorInvalidEmail {
    pub a: Option<bool>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorInvalidEmail {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorInvalidEmail".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorInvalidGroupName {
    pub a: Option<bool>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorInvalidGroupName {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorInvalidGroupName".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorInvalidLimitOfDownloadsPerDay {
    pub a: Option<bool>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorInvalidLimitOfDownloadsPerDay {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorInvalidLimitOfDownloadsPerDay".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorInvalidOTPCode {
    pub a: Option<bool>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorInvalidOTPCode {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorInvalidOTPCode".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorInvalidPassword {
    pub a: Option<bool>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorInvalidPassword {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorInvalidPassword".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorInvalidToken {
    pub a: Option<bool>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorInvalidToken {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorInvalidToken".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorInvalidUserName {
    pub a: Option<bool>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorInvalidUserName {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorInvalidUserName".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorMultipartUploadFileIsTooBig {
    pub a: Option<bool>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorMultipartUploadFileIsTooBig {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorMultipartUploadFileIsTooBig".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorMultipartUploadFileIsTooLight {
    pub a: Option<bool>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorMultipartUploadFileIsTooLight {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorMultipartUploadFileIsTooLight".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorMultipartUploadFilePartSizeIsTooBig {
    pub a: Option<bool>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorMultipartUploadFilePartSizeIsTooBig {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorMultipartUploadFilePartSizeIsTooBig".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorMultipartUploadFilePartSizeIsTooSmall {
    pub a: Option<bool>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorMultipartUploadFilePartSizeIsTooSmall {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorMultipartUploadFilePartSizeIsTooSmall".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorNoDealTag {
    pub a: Option<bool>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorNoDealTag {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorNoDealTag".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorNotFound {
    pub a: Option<bool>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorNotFound {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorNotFound".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorOTPCodeExpired {
    pub a: Option<bool>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorOTPCodeExpired {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorOTPCodeExpired".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorOTPTokenExpired {
    pub a: Option<bool>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorOTPTokenExpired {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorOTPTokenExpired".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorPutUploadFileIsTooBig {
    pub a: Option<bool>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorPutUploadFileIsTooBig {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorPutUploadFileIsTooBig".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorUnknownFile {
    pub a: Option<bool>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorUnknownFile {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorUnknownFile".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorUnknownFiles {
    pub ids: Vec<uuid::Uuid>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorUnknownFiles {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorUnknownFiles".to_string(), libgql::executor::Values::from_iter([("ids".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Array(self.ids.into_iter().map(|element| Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<uuid::Uuid as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(element)?)))).collect::<Result<Vec<_>, String>>()?))),
        ])))
    }
}

pub struct ErrorUnknownGroupIds {
    pub group_ids: Vec<uuid::Uuid>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorUnknownGroupIds {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorUnknownGroupIds".to_string(), libgql::executor::Values::from_iter([("groupIds".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Array(self.group_ids.into_iter().map(|element| Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<uuid::Uuid as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(element)?)))).collect::<Result<Vec<_>, String>>()?))),
        ])))
    }
}

pub struct ErrorUnknownGroups {
    pub group_ids: Vec<uuid::Uuid>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorUnknownGroups {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorUnknownGroups".to_string(), libgql::executor::Values::from_iter([("groupIds".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Array(self.group_ids.into_iter().map(|element| Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<uuid::Uuid as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(element)?)))).collect::<Result<Vec<_>, String>>()?))),
        ])))
    }
}

pub struct ErrorUnknownParentId {
    pub a: Option<bool>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorUnknownParentId {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorUnknownParentId".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorUnknownSessionId {
    pub a: Option<bool>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorUnknownSessionId {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorUnknownSessionId".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorUnknownTags {
    pub tag_ids: Vec<uuid::Uuid>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorUnknownTags {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorUnknownTags".to_string(), libgql::executor::Values::from_iter([("tagIds".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Array(self.tag_ids.into_iter().map(|element| Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<uuid::Uuid as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(element)?)))).collect::<Result<Vec<_>, String>>()?))),
        ])))
    }
}

pub struct ErrorUnknownUser {
    pub a: Option<bool>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorUnknownUser {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorUnknownUser".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorUnknownUsers {
    pub user_ids: Vec<uuid::Uuid>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorUnknownUsers {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorUnknownUsers".to_string(), libgql::executor::Values::from_iter([("userIds".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Array(self.user_ids.into_iter().map(|element| Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<uuid::Uuid as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(element)?)))).collect::<Result<Vec<_>, String>>()?))),
        ])))
    }
}

pub struct EventFileDeleted {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub file: File,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for EventFileDeleted {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("EventFileDeleted".to_string(), libgql::executor::Values::from_iter([("createdAt".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<chrono::DateTime<chrono::Utc> as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(self.created_at)?))),
        ("file".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(self.file)?.into()))),
        ])))
    }
}

pub struct EventFileDownloadRequested {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub decision: Option<bool>,
    pub file: File,
    pub user: User,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for EventFileDownloadRequested {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("EventFileDownloadRequested".to_string(), libgql::executor::Values::from_iter([("createdAt".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<chrono::DateTime<chrono::Utc> as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(self.created_at)?))),
        ("decision".to_string(), self.decision.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ("file".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(self.file)?.into()))),
        ("user".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(self.user)?.into()))),
        ])))
    }
}

pub struct EventFileDownloaded {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub file: File,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for EventFileDownloaded {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("EventFileDownloaded".to_string(), libgql::executor::Values::from_iter([("createdAt".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<chrono::DateTime<chrono::Utc> as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(self.created_at)?))),
        ("file".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(self.file)?.into()))),
        ])))
    }
}

pub struct EventFileTagsEdited {
    pub added_tags: Vec<Tag>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub file: File,
    pub removed_tags: Vec<Tag>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for EventFileTagsEdited {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("EventFileTagsEdited".to_string(), libgql::executor::Values::from_iter([("addedTags".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Array(self.added_tags.into_iter().map(|element| Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(element)?.into())))).collect::<Result<Vec<_>, String>>()?))),
        ("createdAt".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<chrono::DateTime<chrono::Utc> as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(self.created_at)?))),
        ("file".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(self.file)?.into()))),
        ("removedTags".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Array(self.removed_tags.into_iter().map(|element| Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(element)?.into())))).collect::<Result<Vec<_>, String>>()?))),
        ])))
    }
}

pub struct EventFileUploaded {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub file: File,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for EventFileUploaded {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("EventFileUploaded".to_string(), libgql::executor::Values::from_iter([("createdAt".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<chrono::DateTime<chrono::Utc> as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(self.created_at)?))),
        ("file".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(self.file)?.into()))),
        ])))
    }
}

pub struct EventTagApprovalIsRequested {
    pub already_in_catalog: bool,
    pub author: User,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub tag: Tag,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for EventTagApprovalIsRequested {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("EventTagApprovalIsRequested".to_string(), libgql::executor::Values::from_iter([("alreadyInCatalog".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(self.already_in_catalog)?))),
        ("author".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(self.author)?.into()))),
        ("createdAt".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<chrono::DateTime<chrono::Utc> as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(self.created_at)?))),
        ("tag".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(self.tag)?.into()))),
        ])))
    }
}

pub struct EventsList {
    pub events: Vec<Event>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for EventsList {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("EventsList".to_string(), libgql::executor::Values::from_iter([("events".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Array(self.events.into_iter().map(|element| Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(element)?.into())))).collect::<Result<Vec<_>, String>>()?))),
        ])))
    }
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

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for File {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("File".to_string(), libgql::executor::Values::from_iter([("createdAt".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<chrono::DateTime<chrono::Utc> as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(self.created_at)?))),
        ("filename".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(self.filename)?))),
        ("id".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<uuid::Uuid as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(self.id)?))),
        ("mimeType".to_string(), self.mime_type.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ("previewUrl".to_string(), self.preview_url.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<url::Url as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ("sizeInBytes".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<i64 as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(self.size_in_bytes)?))),
        ("user".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(self.user)?.into()))),
        ])))
    }
}

pub struct FilesDealInfo {
    pub deal_info: DealInfo,
    pub deal_name: Tag,
    pub unset_columns: Vec<DealColumn>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for FilesDealInfo {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("FilesDealInfo".to_string(), libgql::executor::Values::from_iter([("dealInfo".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(self.deal_info)?.into()))),
        ("dealName".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(self.deal_name)?.into()))),
        ("unsetColumns".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Array(self.unset_columns.into_iter().map(|element| Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(element)?.into())))).collect::<Result<Vec<_>, String>>()?))),
        ])))
    }
}

pub struct FloatObject {
    pub fvalue: f32,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for FloatObject {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("FloatObject".to_string(), libgql::executor::Values::from_iter([("fvalue".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<f32 as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(self.fvalue)?))),
        ])))
    }
}

pub struct Group {
    pub first_10_tags: Vec<Tag>,
    pub id: uuid::Uuid,
    pub limit_of_downloads_per_day: i32,
    pub name: String,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for Group {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("Group".to_string(), libgql::executor::Values::from_iter([("first10Tags".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Array(self.first_10_tags.into_iter().map(|element| Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(element)?.into())))).collect::<Result<Vec<_>, String>>()?))),
        ("id".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<uuid::Uuid as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(self.id)?))),
        ("limitOfDownloadsPerDay".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<i32 as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(self.limit_of_downloads_per_day)?))),
        ("name".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(self.name)?))),
        ])))
    }
}

pub struct GroupUser {
    pub in_group: bool,
    pub user: User,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for GroupUser {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("GroupUser".to_string(), libgql::executor::Values::from_iter([("inGroup".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(self.in_group)?))),
        ("user".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(self.user)?.into()))),
        ])))
    }
}

pub struct GroupUserList {
    pub users: Vec<GroupUser>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for GroupUserList {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("GroupUserList".to_string(), libgql::executor::Values::from_iter([("users".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Array(self.users.into_iter().map(|element| Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(element)?.into())))).collect::<Result<Vec<_>, String>>()?))),
        ])))
    }
}

pub struct IntObject {
    pub ivalue: i32,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for IntObject {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("IntObject".to_string(), libgql::executor::Values::from_iter([("ivalue".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<i32 as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(self.ivalue)?))),
        ])))
    }
}

pub struct MultipartUploadSession {
    pub id: uuid::Uuid,
    pub initial_upload_ur_ls: Vec<UploadUrl>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for MultipartUploadSession {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("MultipartUploadSession".to_string(), libgql::executor::Values::from_iter([("id".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<uuid::Uuid as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(self.id)?))),
        ("initialUploadURLs".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Array(self.initial_upload_ur_ls.into_iter().map(|element| Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(element)?.into())))).collect::<Result<Vec<_>, String>>()?))),
        ])))
    }
}

pub struct OTPToken {
    pub token: String,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for OTPToken {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("OTPToken".to_string(), libgql::executor::Values::from_iter([("token".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(self.token)?))),
        ])))
    }
}

pub struct PendingUser {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub email: String,
    pub groups: Vec<Group>,
    pub name: String,
    pub ttl: f32,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for PendingUser {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("PendingUser".to_string(), libgql::executor::Values::from_iter([("createdAt".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<chrono::DateTime<chrono::Utc> as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(self.created_at)?))),
        ("email".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(self.email)?))),
        ("groups".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Array(self.groups.into_iter().map(|element| Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(element)?.into())))).collect::<Result<Vec<_>, String>>()?))),
        ("name".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(self.name)?))),
        ("ttl".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<f32 as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(self.ttl)?))),
        ])))
    }
}

pub struct PutUploadSession {
    pub id: uuid::Uuid,
    pub upload_url: UploadUrl,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for PutUploadSession {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("PutUploadSession".to_string(), libgql::executor::Values::from_iter([("id".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<uuid::Uuid as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(self.id)?))),
        ("uploadURL".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(self.upload_url)?.into()))),
        ])))
    }
}

pub struct SearchFile {
    pub file: File,
    pub tags: Vec<Tag>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for SearchFile {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("SearchFile".to_string(), libgql::executor::Values::from_iter([("file".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(self.file)?.into()))),
        ("tags".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Array(self.tags.into_iter().map(|element| Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(element)?.into())))).collect::<Result<Vec<_>, String>>()?))),
        ])))
    }
}

pub struct SearchFileList {
    pub files: Vec<SearchFile>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for SearchFileList {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("SearchFileList".to_string(), libgql::executor::Values::from_iter([("files".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Array(self.files.into_iter().map(|element| Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(element)?.into())))).collect::<Result<Vec<_>, String>>()?))),
        ])))
    }
}

pub struct StageToWorktypesMapEntry {
    pub stage: Tag,
    pub worktypes: Vec<Tag>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for StageToWorktypesMapEntry {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("StageToWorktypesMapEntry".to_string(), libgql::executor::Values::from_iter([("stage".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(self.stage)?.into()))),
        ("worktypes".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Array(self.worktypes.into_iter().map(|element| Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(element)?.into())))).collect::<Result<Vec<_>, String>>()?))),
        ])))
    }
}

pub struct StringEntry {
    pub key: String,
    pub value: String,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for StringEntry {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("StringEntry".to_string(), libgql::executor::Values::from_iter([("key".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(self.key)?))),
        ("value".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(self.value)?))),
        ])))
    }
}

pub struct StringList {
    pub values: Vec<String>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for StringList {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("StringList".to_string(), libgql::executor::Values::from_iter([("values".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Array(self.values.into_iter().map(|element| Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(element)?)))).collect::<Result<Vec<_>, String>>()?))),
        ])))
    }
}

pub struct StringObject {
    pub svalue: String,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for StringObject {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("StringObject".to_string(), libgql::executor::Values::from_iter([("svalue".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(self.svalue)?))),
        ])))
    }
}

pub struct Tag {
    pub has_children: bool,
    pub id: uuid::Uuid,
    pub is_approved: bool,
    pub is_favourite: bool,
    pub tag: String,
    pub value: Option<TagValue>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for Tag {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("Tag".to_string(), libgql::executor::Values::from_iter([("hasChildren".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(self.has_children)?))),
        ("id".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<uuid::Uuid as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(self.id)?))),
        ("isApproved".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(self.is_approved)?))),
        ("isFavourite".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(self.is_favourite)?))),
        ("tag".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(self.tag)?))),
        ("value".to_string(), self.value.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(v)?.into())))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct TagInfo {
    pub parent_tag: Option<Tag>,
    pub tag: String,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for TagInfo {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("TagInfo".to_string(), libgql::executor::Values::from_iter([("parentTag".to_string(), self.parent_tag.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(v)?.into())))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ("tag".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(self.tag)?))),
        ])))
    }
}

pub struct TagList {
    pub list: Vec<Tag>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for TagList {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("TagList".to_string(), libgql::executor::Values::from_iter([("list".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Array(self.list.into_iter().map(|element| Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(element)?.into())))).collect::<Result<Vec<_>, String>>()?))),
        ])))
    }
}

pub struct UploadUrl {
    pub headers: Vec<StringEntry>,
    pub url: url::Url,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for UploadUrl {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("UploadUrl".to_string(), libgql::executor::Values::from_iter([("headers".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Array(self.headers.into_iter().map(|element| Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(element)?.into())))).collect::<Result<Vec<_>, String>>()?))),
        ("url".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<url::Url as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(self.url)?))),
        ])))
    }
}

pub struct UploadUrlList {
    pub urls: Vec<UploadUrl>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for UploadUrlList {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("UploadUrlList".to_string(), libgql::executor::Values::from_iter([("urls".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Array(self.urls.into_iter().map(|element| Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(element)?.into())))).collect::<Result<Vec<_>, String>>()?))),
        ])))
    }
}

pub struct UrlObject {
    pub uvalue: url::Url,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for UrlObject {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("UrlObject".to_string(), libgql::executor::Values::from_iter([("uvalue".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<url::Url as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(self.uvalue)?))),
        ])))
    }
}

pub struct User {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub email: String,
    pub id: uuid::Uuid,
    pub name: String,
    pub ten_groups: Vec<Group>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for User {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("User".to_string(), libgql::executor::Values::from_iter([("createdAt".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<chrono::DateTime<chrono::Utc> as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(self.created_at)?))),
        ("email".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(self.email)?))),
        ("id".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<uuid::Uuid as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(self.id)?))),
        ("name".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(self.name)?))),
        ("tenGroups".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Array(self.ten_groups.into_iter().map(|element| Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(element)?.into())))).collect::<Result<Vec<_>, String>>()?))),
        ])))
    }
}

pub struct UsersList {
    pub users: Vec<User>,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for UsersList {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("UsersList".to_string(), libgql::executor::Values::from_iter([("users".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Array(self.users.into_iter().map(|element| Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(element)?.into())))).collect::<Result<Vec<_>, String>>()?))),
        ])))
    }
}

pub struct UsersTag {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub tag: Tag,
    pub users_count: i32,
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for UsersTag {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("UsersTag".to_string(), libgql::executor::Values::from_iter([("createdAt".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<chrono::DateTime<chrono::Utc> as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(self.created_at)?))),
        ("tag".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(self.tag)?.into()))),
        ("usersCount".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<i32 as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(self.users_count)?))),
        ])))
    }
}

pub enum AddTagsToFilesError {
    ErrorUnknownFiles(ErrorUnknownFiles),
    ErrorUnknownTags(ErrorUnknownTags),
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for AddTagsToFilesError {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        match self {
        Self::ErrorUnknownFiles(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::ErrorUnknownTags(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        }
    }
}

pub enum ApproveTagError {
    ErrorAlreadyExists(ErrorAlreadyExists),
    ErrorNotFound(ErrorNotFound),
    ErrorUnknownGroupIds(ErrorUnknownGroupIds),
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ApproveTagError {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        match self {
        Self::ErrorAlreadyExists(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::ErrorNotFound(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::ErrorUnknownGroupIds(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        }
    }
}

pub enum CommitMultipartFileSessionResponse {
    ErrorFileNotUploaded(ErrorFileNotUploaded),
    ErrorUnknownSessionId(ErrorUnknownSessionId),
    File(File),
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for CommitMultipartFileSessionResponse {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        match self {
        Self::ErrorFileNotUploaded(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::ErrorUnknownSessionId(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::File(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        }
    }
}

pub enum CommitPutFileSessionResponse {
    ErrorFileNotUploaded(ErrorFileNotUploaded),
    ErrorUnknownSessionId(ErrorUnknownSessionId),
    File(File),
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for CommitPutFileSessionResponse {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        match self {
        Self::ErrorFileNotUploaded(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::ErrorUnknownSessionId(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::File(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        }
    }
}

pub enum ConfirmOTPCodeResponse {
    ErrorInvalidOTPCode(ErrorInvalidOTPCode),
    ErrorOTPCodeExpired(ErrorOTPCodeExpired),
    OTPToken(OTPToken),
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ConfirmOTPCodeResponse {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        match self {
        Self::ErrorInvalidOTPCode(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::ErrorOTPCodeExpired(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::OTPToken(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        }
    }
}

pub enum ConfirmUserError {
    ErrorInvalidPassword(ErrorInvalidPassword),
    ErrorInvalidToken(ErrorInvalidToken),
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ConfirmUserError {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        match self {
        Self::ErrorInvalidPassword(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::ErrorInvalidToken(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        }
    }
}

pub enum CreateGroupError {
    ErrorAlreadyExists(ErrorAlreadyExists),
    ErrorInvalidGroupName(ErrorInvalidGroupName),
    ErrorInvalidLimitOfDownloadsPerDay(ErrorInvalidLimitOfDownloadsPerDay),
    ErrorUnknownTags(ErrorUnknownTags),
    ErrorUnknownUsers(ErrorUnknownUsers),
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for CreateGroupError {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        match self {
        Self::ErrorAlreadyExists(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::ErrorInvalidGroupName(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::ErrorInvalidLimitOfDownloadsPerDay(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::ErrorUnknownTags(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::ErrorUnknownUsers(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        }
    }
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

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for CreateMultipartFileSessionResponse {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        match self {
        Self::ErrorMultipartUploadFileIsTooBig(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::ErrorMultipartUploadFileIsTooLight(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::ErrorMultipartUploadFilePartSizeIsTooBig(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::ErrorMultipartUploadFilePartSizeIsTooSmall(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::ErrorNoDealTag(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::ErrorUnknownTags(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::MultipartUploadSession(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        }
    }
}

pub enum CreatePutFileSessionResponse {
    ErrorNoDealTag(ErrorNoDealTag),
    ErrorPutUploadFileIsTooBig(ErrorPutUploadFileIsTooBig),
    ErrorUnknownTags(ErrorUnknownTags),
    PutUploadSession(PutUploadSession),
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for CreatePutFileSessionResponse {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        match self {
        Self::ErrorNoDealTag(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::ErrorPutUploadFileIsTooBig(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::ErrorUnknownTags(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::PutUploadSession(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        }
    }
}

pub enum CreateTagError {
    ErrorAlreadyExists(ErrorAlreadyExists),
    ErrorUnknownParentId(ErrorUnknownParentId),
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for CreateTagError {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        match self {
        Self::ErrorAlreadyExists(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::ErrorUnknownParentId(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        }
    }
}

pub enum CreateUserError {
    ErrorAlreadyPending(ErrorAlreadyPending),
    ErrorEmailCollision(ErrorEmailCollision),
    ErrorInvalidEmail(ErrorInvalidEmail),
    ErrorInvalidUserName(ErrorInvalidUserName),
    ErrorUnknownGroups(ErrorUnknownGroups),
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for CreateUserError {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        match self {
        Self::ErrorAlreadyPending(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::ErrorEmailCollision(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::ErrorInvalidEmail(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::ErrorInvalidUserName(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::ErrorUnknownGroups(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        }
    }
}

pub enum DecideOnDownloadRequestError {
    ErrorAlreadyDone(ErrorAlreadyDone),
    ErrorNotFound(ErrorNotFound),
    ErrorUnknownFile(ErrorUnknownFile),
    ErrorUnknownUser(ErrorUnknownUser),
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for DecideOnDownloadRequestError {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        match self {
        Self::ErrorAlreadyDone(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::ErrorNotFound(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::ErrorUnknownFile(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::ErrorUnknownUser(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        }
    }
}

pub enum DeleteFileError {
    ErrorChangeForbidden(ErrorChangeForbidden),
    ErrorUnknownFile(ErrorUnknownFile),
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for DeleteFileError {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        match self {
        Self::ErrorChangeForbidden(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::ErrorUnknownFile(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        }
    }
}

pub enum DeleteFilesError {
    ErrorFilesChangeForbidden(ErrorFilesChangeForbidden),
    ErrorUnknownFiles(ErrorUnknownFiles),
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for DeleteFilesError {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        match self {
        Self::ErrorFilesChangeForbidden(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::ErrorUnknownFiles(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        }
    }
}

pub enum EditGroupError {
    ErrorAlreadyExists(ErrorAlreadyExists),
    ErrorGroupNotFound(ErrorGroupNotFound),
    ErrorInvalidGroupName(ErrorInvalidGroupName),
    ErrorInvalidLimitOfDownloadsPerDay(ErrorInvalidLimitOfDownloadsPerDay),
    ErrorUnknownTags(ErrorUnknownTags),
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for EditGroupError {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        match self {
        Self::ErrorAlreadyExists(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::ErrorGroupNotFound(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::ErrorInvalidGroupName(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::ErrorInvalidLimitOfDownloadsPerDay(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::ErrorUnknownTags(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        }
    }
}

pub enum EditTagError {
    ErrorAlreadyApprovedByAdmin(ErrorAlreadyApprovedByAdmin),
    ErrorAlreadyExists(ErrorAlreadyExists),
    ErrorNotFound(ErrorNotFound),
    ErrorUnknownParentId(ErrorUnknownParentId),
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for EditTagError {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        match self {
        Self::ErrorAlreadyApprovedByAdmin(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::ErrorAlreadyExists(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::ErrorNotFound(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::ErrorUnknownParentId(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        }
    }
}

pub enum ErrorAlreadyDoneOrUnknownTags {
    ErrorAlreadyDone(ErrorAlreadyDone),
    ErrorUnknownTags(ErrorUnknownTags),
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorAlreadyDoneOrUnknownTags {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        match self {
        Self::ErrorAlreadyDone(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::ErrorUnknownTags(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        }
    }
}

pub enum ErrorGroupNotFoundOrErrorNotFound {
    ErrorGroupNotFound(ErrorGroupNotFound),
    ErrorNotFound(ErrorNotFound),
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorGroupNotFoundOrErrorNotFound {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        match self {
        Self::ErrorGroupNotFound(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::ErrorNotFound(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        }
    }
}

pub enum Event {
    EventFileDeleted(EventFileDeleted),
    EventFileDownloadRequested(EventFileDownloadRequested),
    EventFileDownloaded(EventFileDownloaded),
    EventFileTagsEdited(EventFileTagsEdited),
    EventFileUploaded(EventFileUploaded),
    EventTagApprovalIsRequested(EventTagApprovalIsRequested),
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for Event {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        match self {
        Self::EventFileDeleted(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::EventFileDownloadRequested(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::EventFileDownloaded(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::EventFileTagsEdited(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::EventFileUploaded(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::EventTagApprovalIsRequested(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        }
    }
}

pub enum FilesDealInfoOrError {
    ErrorCantAddAutotags(ErrorCantAddAutotags),
    FilesDealInfo(FilesDealInfo),
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for FilesDealInfoOrError {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        match self {
        Self::ErrorCantAddAutotags(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::FilesDealInfo(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        }
    }
}

pub enum GetDealInfoResponse {
    DealInfo(DealInfo),
    ErrorNotFound(ErrorNotFound),
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for GetDealInfoResponse {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        match self {
        Self::DealInfo(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::ErrorNotFound(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        }
    }
}

pub enum GetEventsResponse {
    ErrorDateRangeIsInvalid(ErrorDateRangeIsInvalid),
    EventsList(EventsList),
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for GetEventsResponse {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        match self {
        Self::ErrorDateRangeIsInvalid(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::EventsList(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        }
    }
}

pub enum GetFileURLResponse {
    ErrorUnknownFile(ErrorUnknownFile),
    UrlObject(UrlObject),
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for GetFileURLResponse {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        match self {
        Self::ErrorUnknownFile(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::UrlObject(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        }
    }
}

pub enum GetFilesResponse {
    ErrorUnknownTags(ErrorUnknownTags),
    SearchFileList(SearchFileList),
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for GetFilesResponse {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        match self {
        Self::ErrorUnknownTags(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::SearchFileList(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        }
    }
}

pub enum GetGroupTagsResponse {
    ErrorNotFound(ErrorNotFound),
    TagList(TagList),
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for GetGroupTagsResponse {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        match self {
        Self::ErrorNotFound(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::TagList(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        }
    }
}

pub enum GetGroupUsersAndUsersResponse {
    ErrorGroupNotFound(ErrorGroupNotFound),
    GroupUserList(GroupUserList),
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for GetGroupUsersAndUsersResponse {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        match self {
        Self::ErrorGroupNotFound(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::GroupUserList(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        }
    }
}

pub enum GetGroupUsersResponse {
    ErrorNotFound(ErrorNotFound),
    UsersList(UsersList),
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for GetGroupUsersResponse {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        match self {
        Self::ErrorNotFound(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::UsersList(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        }
    }
}

pub enum GetGroupUsersTotalResponse {
    ErrorNotFound(ErrorNotFound),
    IntObject(IntObject),
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for GetGroupUsersTotalResponse {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        match self {
        Self::ErrorNotFound(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::IntObject(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        }
    }
}

pub enum GetNextMultipartUploadUrlsResponse {
    ErrorUnknownSessionId(ErrorUnknownSessionId),
    UploadUrlList(UploadUrlList),
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for GetNextMultipartUploadUrlsResponse {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        match self {
        Self::ErrorUnknownSessionId(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::UploadUrlList(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        }
    }
}

pub enum GetPathToTagResponse {
    ErrorUnknownTags(ErrorUnknownTags),
    StringList(StringList),
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for GetPathToTagResponse {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        match self {
        Self::ErrorUnknownTags(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::StringList(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        }
    }
}

pub enum GetTagInfoResponse {
    ErrorNotFound(ErrorNotFound),
    TagInfo(TagInfo),
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for GetTagInfoResponse {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        match self {
        Self::ErrorNotFound(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::TagInfo(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        }
    }
}

pub enum GetTagsResponse {
    ErrorUnknownTags(ErrorUnknownTags),
    TagList(TagList),
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for GetTagsResponse {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        match self {
        Self::ErrorUnknownTags(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::TagList(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        }
    }
}

pub enum IntObjectOrErrorUnknownTags {
    ErrorUnknownTags(ErrorUnknownTags),
    IntObject(IntObject),
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for IntObjectOrErrorUnknownTags {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        match self {
        Self::ErrorUnknownTags(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::IntObject(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        }
    }
}

pub enum IsAllowedToDownloadResponse {
    BooleanObject(BooleanObject),
    ErrorUnknownFile(ErrorUnknownFile),
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for IsAllowedToDownloadResponse {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        match self {
        Self::BooleanObject(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::ErrorUnknownFile(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        }
    }
}

pub enum ResetPasswordError {
    ErrorInvalidPassword(ErrorInvalidPassword),
    ErrorOTPTokenExpired(ErrorOTPTokenExpired),
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ResetPasswordError {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        match self {
        Self::ErrorInvalidPassword(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::ErrorOTPTokenExpired(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        }
    }
}

pub enum RetrieveFileResponse {
    ErrorUnknownFile(ErrorUnknownFile),
    SearchFile(SearchFile),
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for RetrieveFileResponse {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        match self {
        Self::ErrorUnknownFile(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::SearchFile(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        }
    }
}

pub enum RetrieveGroupResponse {
    ErrorNotFound(ErrorNotFound),
    Group(Group),
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for RetrieveGroupResponse {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        match self {
        Self::ErrorNotFound(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::Group(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        }
    }
}

pub enum TagValue {
    DatetimeObject(DatetimeObject),
    FloatObject(FloatObject),
    StringObject(StringObject),
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for TagValue {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        match self {
        Self::DatetimeObject(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::FloatObject(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::StringObject(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        }
    }
}

pub enum UpdateFileError {
    ErrorChangeForbidden(ErrorChangeForbidden),
    ErrorUnknownFile(ErrorUnknownFile),
    ErrorUnknownTags(ErrorUnknownTags),
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for UpdateFileError {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        match self {
        Self::ErrorChangeForbidden(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::ErrorUnknownFile(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        Self::ErrorUnknownTags(item) => TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(item),
        }
    }
}