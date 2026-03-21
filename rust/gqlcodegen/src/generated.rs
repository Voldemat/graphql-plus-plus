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

    fn to_str(self: &Self) -> Result<&'static str, String> {
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

    fn to_str(self: &Self) -> Result<&'static str, String> {
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

    fn to_str(self: &Self) -> Result<&'static str, String> {
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

    fn to_str(self: &Self) -> Result<&'static str, String> {
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

    fn to_str(self: &Self) -> Result<&'static str, String> {
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

    fn to_str(self: &Self) -> Result<&'static str, String> {
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

    fn to_str(self: &Self) -> Result<&'static str, String> {
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for BooleanObject {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for BooleanObject {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("BooleanObject".to_string(), libgql::executor::Values::from_iter([("bvalue".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&self.bvalue)?))),
        ])))
    }
}

pub struct DatetimeObject {
    pub dvalue: chrono::DateTime<chrono::Utc>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for DatetimeObject {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for DatetimeObject {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("DatetimeObject".to_string(), libgql::executor::Values::from_iter([("dvalue".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<chrono::DateTime<chrono::Utc> as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&self.dvalue)?))),
        ])))
    }
}

pub struct DealColumn {
    pub available_values: Vec<String>,
    pub id: uuid::Uuid,
    pub name: String,
    pub r#type: EDealColumnType,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for DealColumn {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for DealColumn {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("DealColumn".to_string(), libgql::executor::Values::from_iter([("availableValues".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Array(self.available_values.into_iter().map(|element| Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&element)?)))).collect::<Result<Vec<_>, String>>()?))),
        ("id".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<uuid::Uuid as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&self.id)?))),
        ("name".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&self.name)?))),
        ("type".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<EDealColumnType as libgql::executor::GQLEnum<super::scalar::ExampleScalar>>::to_literal_value(&self.r#type)?))),
        ])))
    }
}

pub struct DealEntry {
    pub column_name: String,
    pub value: Tag,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for DealEntry {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for DealEntry {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("DealEntry".to_string(), libgql::executor::Values::from_iter([("columnName".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&self.column_name)?))),
        ("value".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(self.value)?.into()))),
        ])))
    }
}

pub struct DealInfo {
    pub stage_to_worktypes_map: Vec<StageToWorktypesMapEntry>,
    pub values: Vec<DealEntry>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for DealInfo {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorAlreadyApprovedByAdmin {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorAlreadyApprovedByAdmin {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorAlreadyApprovedByAdmin".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorAlreadyDone {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorAlreadyDone {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorAlreadyDone {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorAlreadyDone".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorAlreadyExists {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorAlreadyExists {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorAlreadyExists {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorAlreadyExists".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorAlreadyPending {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorAlreadyPending {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorAlreadyPending {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorAlreadyPending".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorCantAddAutotags {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorCantAddAutotags {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorCantAddAutotags {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorCantAddAutotags".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorChangeForbidden {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorChangeForbidden {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorChangeForbidden {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorChangeForbidden".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorDateRangeIsInvalid {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorDateRangeIsInvalid {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorDateRangeIsInvalid {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorDateRangeIsInvalid".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorEmailCollision {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorEmailCollision {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorEmailCollision {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorEmailCollision".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorFileNotUploaded {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorFileNotUploaded {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorFileNotUploaded {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorFileNotUploaded".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorFilesChangeForbidden {
    pub ids: Vec<uuid::Uuid>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorFilesChangeForbidden {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorFilesChangeForbidden {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorFilesChangeForbidden".to_string(), libgql::executor::Values::from_iter([("ids".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Array(self.ids.into_iter().map(|element| Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<uuid::Uuid as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&element)?)))).collect::<Result<Vec<_>, String>>()?))),
        ])))
    }
}

pub struct ErrorGroupNotFound {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorGroupNotFound {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorGroupNotFound {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorGroupNotFound".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorInvalidCredentials {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorInvalidCredentials {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorInvalidCredentials {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorInvalidCredentials".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorInvalidEmail {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorInvalidEmail {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorInvalidEmail {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorInvalidEmail".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorInvalidGroupName {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorInvalidGroupName {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorInvalidGroupName {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorInvalidGroupName".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorInvalidLimitOfDownloadsPerDay {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorInvalidLimitOfDownloadsPerDay {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorInvalidLimitOfDownloadsPerDay {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorInvalidLimitOfDownloadsPerDay".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorInvalidOTPCode {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorInvalidOTPCode {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorInvalidOTPCode {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorInvalidOTPCode".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorInvalidPassword {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorInvalidPassword {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorInvalidPassword {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorInvalidPassword".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorInvalidToken {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorInvalidToken {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorInvalidToken {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorInvalidToken".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorInvalidUserName {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorInvalidUserName {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorInvalidUserName {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorInvalidUserName".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorMultipartUploadFileIsTooBig {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorMultipartUploadFileIsTooBig {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorMultipartUploadFileIsTooBig {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorMultipartUploadFileIsTooBig".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorMultipartUploadFileIsTooLight {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorMultipartUploadFileIsTooLight {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorMultipartUploadFileIsTooLight {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorMultipartUploadFileIsTooLight".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorMultipartUploadFilePartSizeIsTooBig {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorMultipartUploadFilePartSizeIsTooBig {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorMultipartUploadFilePartSizeIsTooBig {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorMultipartUploadFilePartSizeIsTooBig".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorMultipartUploadFilePartSizeIsTooSmall {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorMultipartUploadFilePartSizeIsTooSmall {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorMultipartUploadFilePartSizeIsTooSmall {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorMultipartUploadFilePartSizeIsTooSmall".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorNoDealTag {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorNoDealTag {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorNoDealTag {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorNoDealTag".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorNotFound {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorNotFound {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorNotFound {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorNotFound".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorOTPCodeExpired {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorOTPCodeExpired {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorOTPCodeExpired {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorOTPCodeExpired".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorOTPTokenExpired {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorOTPTokenExpired {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorOTPTokenExpired {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorOTPTokenExpired".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorPutUploadFileIsTooBig {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorPutUploadFileIsTooBig {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorPutUploadFileIsTooBig {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorPutUploadFileIsTooBig".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorUnknownFile {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorUnknownFile {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorUnknownFile {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorUnknownFile".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorUnknownFiles {
    pub ids: Vec<uuid::Uuid>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorUnknownFiles {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorUnknownFiles {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorUnknownFiles".to_string(), libgql::executor::Values::from_iter([("ids".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Array(self.ids.into_iter().map(|element| Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<uuid::Uuid as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&element)?)))).collect::<Result<Vec<_>, String>>()?))),
        ])))
    }
}

pub struct ErrorUnknownGroupIds {
    pub group_ids: Vec<uuid::Uuid>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorUnknownGroupIds {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorUnknownGroupIds {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorUnknownGroupIds".to_string(), libgql::executor::Values::from_iter([("groupIds".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Array(self.group_ids.into_iter().map(|element| Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<uuid::Uuid as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&element)?)))).collect::<Result<Vec<_>, String>>()?))),
        ])))
    }
}

pub struct ErrorUnknownGroups {
    pub group_ids: Vec<uuid::Uuid>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorUnknownGroups {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorUnknownGroups {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorUnknownGroups".to_string(), libgql::executor::Values::from_iter([("groupIds".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Array(self.group_ids.into_iter().map(|element| Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<uuid::Uuid as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&element)?)))).collect::<Result<Vec<_>, String>>()?))),
        ])))
    }
}

pub struct ErrorUnknownParentId {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorUnknownParentId {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorUnknownParentId {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorUnknownParentId".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorUnknownSessionId {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorUnknownSessionId {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorUnknownSessionId {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorUnknownSessionId".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorUnknownTags {
    pub tag_ids: Vec<uuid::Uuid>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorUnknownTags {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorUnknownTags {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorUnknownTags".to_string(), libgql::executor::Values::from_iter([("tagIds".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Array(self.tag_ids.into_iter().map(|element| Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<uuid::Uuid as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&element)?)))).collect::<Result<Vec<_>, String>>()?))),
        ])))
    }
}

pub struct ErrorUnknownUser {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorUnknownUser {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorUnknownUser {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorUnknownUser".to_string(), libgql::executor::Values::from_iter([("a".to_string(), self.a.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct ErrorUnknownUsers {
    pub user_ids: Vec<uuid::Uuid>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorUnknownUsers {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for ErrorUnknownUsers {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("ErrorUnknownUsers".to_string(), libgql::executor::Values::from_iter([("userIds".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Array(self.user_ids.into_iter().map(|element| Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<uuid::Uuid as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&element)?)))).collect::<Result<Vec<_>, String>>()?))),
        ])))
    }
}

pub struct EventFileDeleted {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub file: File,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for EventFileDeleted {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for EventFileDeleted {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("EventFileDeleted".to_string(), libgql::executor::Values::from_iter([("createdAt".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<chrono::DateTime<chrono::Utc> as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&self.created_at)?))),
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for EventFileDownloadRequested {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for EventFileDownloadRequested {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("EventFileDownloadRequested".to_string(), libgql::executor::Values::from_iter([("createdAt".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<chrono::DateTime<chrono::Utc> as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&self.created_at)?))),
        ("decision".to_string(), self.decision.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ("file".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(self.file)?.into()))),
        ("user".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(self.user)?.into()))),
        ])))
    }
}

pub struct EventFileDownloaded {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub file: File,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for EventFileDownloaded {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for EventFileDownloaded {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("EventFileDownloaded".to_string(), libgql::executor::Values::from_iter([("createdAt".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<chrono::DateTime<chrono::Utc> as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&self.created_at)?))),
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for EventFileTagsEdited {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for EventFileTagsEdited {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("EventFileTagsEdited".to_string(), libgql::executor::Values::from_iter([("addedTags".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Array(self.added_tags.into_iter().map(|element| Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(element)?.into())))).collect::<Result<Vec<_>, String>>()?))),
        ("createdAt".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<chrono::DateTime<chrono::Utc> as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&self.created_at)?))),
        ("file".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(self.file)?.into()))),
        ("removedTags".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Array(self.removed_tags.into_iter().map(|element| Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(element)?.into())))).collect::<Result<Vec<_>, String>>()?))),
        ])))
    }
}

pub struct EventFileUploaded {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub file: File,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for EventFileUploaded {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for EventFileUploaded {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("EventFileUploaded".to_string(), libgql::executor::Values::from_iter([("createdAt".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<chrono::DateTime<chrono::Utc> as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&self.created_at)?))),
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for EventTagApprovalIsRequested {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for EventTagApprovalIsRequested {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("EventTagApprovalIsRequested".to_string(), libgql::executor::Values::from_iter([("alreadyInCatalog".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&self.already_in_catalog)?))),
        ("author".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(self.author)?.into()))),
        ("createdAt".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<chrono::DateTime<chrono::Utc> as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&self.created_at)?))),
        ("tag".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(self.tag)?.into()))),
        ])))
    }
}

pub struct EventsList {
    pub events: Vec<Event>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for EventsList {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for File {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for File {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("File".to_string(), libgql::executor::Values::from_iter([("createdAt".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<chrono::DateTime<chrono::Utc> as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&self.created_at)?))),
        ("filename".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&self.filename)?))),
        ("id".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<uuid::Uuid as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&self.id)?))),
        ("mimeType".to_string(), self.mime_type.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ("previewUrl".to_string(), self.preview_url.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<url::Url as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&v)?)))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ("sizeInBytes".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<i64 as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&self.size_in_bytes)?))),
        ("user".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(self.user)?.into()))),
        ])))
    }
}

pub struct FilesDealInfo {
    pub deal_info: DealInfo,
    pub deal_name: Tag,
    pub unset_columns: Vec<DealColumn>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for FilesDealInfo {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for FloatObject {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for FloatObject {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("FloatObject".to_string(), libgql::executor::Values::from_iter([("fvalue".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<f32 as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&self.fvalue)?))),
        ])))
    }
}

pub struct Group {
    pub first_10_tags: Vec<Tag>,
    pub id: uuid::Uuid,
    pub limit_of_downloads_per_day: i32,
    pub name: String,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for Group {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for Group {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("Group".to_string(), libgql::executor::Values::from_iter([("first10Tags".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Array(self.first_10_tags.into_iter().map(|element| Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(element)?.into())))).collect::<Result<Vec<_>, String>>()?))),
        ("id".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<uuid::Uuid as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&self.id)?))),
        ("limitOfDownloadsPerDay".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<i32 as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&self.limit_of_downloads_per_day)?))),
        ("name".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&self.name)?))),
        ])))
    }
}

pub struct GroupUser {
    pub in_group: bool,
    pub user: User,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for GroupUser {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for GroupUser {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("GroupUser".to_string(), libgql::executor::Values::from_iter([("inGroup".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&self.in_group)?))),
        ("user".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(self.user)?.into()))),
        ])))
    }
}

pub struct GroupUserList {
    pub users: Vec<GroupUser>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for GroupUserList {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for IntObject {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for IntObject {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("IntObject".to_string(), libgql::executor::Values::from_iter([("ivalue".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<i32 as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&self.ivalue)?))),
        ])))
    }
}

pub struct MultipartUploadSession {
    pub id: uuid::Uuid,
    pub initial_upload_ur_ls: Vec<UploadUrl>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for MultipartUploadSession {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for MultipartUploadSession {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("MultipartUploadSession".to_string(), libgql::executor::Values::from_iter([("id".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<uuid::Uuid as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&self.id)?))),
        ("initialUploadURLs".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Array(self.initial_upload_ur_ls.into_iter().map(|element| Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(element)?.into())))).collect::<Result<Vec<_>, String>>()?))),
        ])))
    }
}

async fn mutation_add_tags_to_files(context: &(), file_ids: &Vec<uuid::Uuid>, tag_ids: &Vec<uuid::Uuid>) -> Result<Option<AddTagsToFilesError>, String> {
    todo!()
}

fn mutation_add_tags_to_files_wrapper<'args>(context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let file_ids = variables.get("fileIds").unwrap().downcast_ref::<Vec<uuid::Uuid>>().unwrap();
    let tag_ids = variables.get("tagIds").unwrap().downcast_ref::<Vec<uuid::Uuid>>().unwrap();
    Box::pin(async move {
        mutation_add_tags_to_files(context, file_ids, tag_ids).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn mutation_add_user_to_group(context: &(), group_id: &uuid::Uuid, user_id: &uuid::Uuid) -> Result<Option<ErrorGroupNotFoundOrErrorNotFound>, String> {
    todo!()
}

fn mutation_add_user_to_group_wrapper<'args>(context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let group_id = variables.get("groupId").unwrap().downcast_ref::<uuid::Uuid>().unwrap();
    let user_id = variables.get("userId").unwrap().downcast_ref::<uuid::Uuid>().unwrap();
    Box::pin(async move {
        mutation_add_user_to_group(context, group_id, user_id).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn mutation_approve_tag(context: &(), group_ids: &Vec<uuid::Uuid>, tag_id: &uuid::Uuid) -> Result<Option<ApproveTagError>, String> {
    todo!()
}

fn mutation_approve_tag_wrapper<'args>(context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let group_ids = variables.get("groupIds").unwrap().downcast_ref::<Vec<uuid::Uuid>>().unwrap();
    let tag_id = variables.get("tagId").unwrap().downcast_ref::<uuid::Uuid>().unwrap();
    Box::pin(async move {
        mutation_approve_tag(context, group_ids, tag_id).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn mutation_change_password(context: &(), new_password: &String, old_password: &String) -> Result<Option<ErrorInvalidCredentials>, String> {
    todo!()
}

fn mutation_change_password_wrapper<'args>(context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let new_password = variables.get("newPassword").unwrap().downcast_ref::<String>().unwrap();
    let old_password = variables.get("oldPassword").unwrap().downcast_ref::<String>().unwrap();
    Box::pin(async move {
        mutation_change_password(context, new_password, old_password).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn mutation_commit_multipart_file_session(context: &(), session_id: &uuid::Uuid) -> Result<Option<CommitMultipartFileSessionResponse>, String> {
    todo!()
}

fn mutation_commit_multipart_file_session_wrapper<'args>(context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let session_id = variables.get("sessionId").unwrap().downcast_ref::<uuid::Uuid>().unwrap();
    Box::pin(async move {
        mutation_commit_multipart_file_session(context, session_id).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn mutation_commit_put_file_session(context: &(), session_id: &uuid::Uuid) -> Result<Option<CommitPutFileSessionResponse>, String> {
    todo!()
}

fn mutation_commit_put_file_session_wrapper<'args>(context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let session_id = variables.get("sessionId").unwrap().downcast_ref::<uuid::Uuid>().unwrap();
    Box::pin(async move {
        mutation_commit_put_file_session(context, session_id).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn mutation_confirm_otp_code(context: &(), code: &String, email: &String) -> Result<ConfirmOTPCodeResponse, String> {
    todo!()
}

fn mutation_confirm_otp_code_wrapper<'args>(context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let code = variables.get("code").unwrap().downcast_ref::<String>().unwrap();
    let email = variables.get("email").unwrap().downcast_ref::<String>().unwrap();
    Box::pin(async move {
        mutation_confirm_otp_code(context, code, email).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn mutation_confirm_user(context: &(), password: &String, token: &String) -> Result<Option<ConfirmUserError>, String> {
    todo!()
}

fn mutation_confirm_user_wrapper<'args>(context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let password = variables.get("password").unwrap().downcast_ref::<String>().unwrap();
    let token = variables.get("token").unwrap().downcast_ref::<String>().unwrap();
    Box::pin(async move {
        mutation_confirm_user(context, password, token).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn mutation_create_group(context: &(), group_in: &GroupIn, user_ids: &Vec<uuid::Uuid>) -> Result<Option<CreateGroupError>, String> {
    todo!()
}

fn mutation_create_group_wrapper<'args>(context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let group_in = variables.get("groupIn").unwrap().downcast_ref::<GroupIn>().unwrap();
    let user_ids = variables.get("userIds").unwrap().downcast_ref::<Vec<uuid::Uuid>>().unwrap();
    Box::pin(async move {
        mutation_create_group(context, group_in, user_ids).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn mutation_create_multipart_file_session(context: &(), file_in: &MultipartUploadFileIn) -> Result<CreateMultipartFileSessionResponse, String> {
    todo!()
}

fn mutation_create_multipart_file_session_wrapper<'args>(context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let file_in = variables.get("fileIn").unwrap().downcast_ref::<MultipartUploadFileIn>().unwrap();
    Box::pin(async move {
        mutation_create_multipart_file_session(context, file_in).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn mutation_create_put_file_session(context: &(), file_in: &PutUploadFileIn) -> Result<CreatePutFileSessionResponse, String> {
    todo!()
}

fn mutation_create_put_file_session_wrapper<'args>(context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let file_in = variables.get("fileIn").unwrap().downcast_ref::<PutUploadFileIn>().unwrap();
    Box::pin(async move {
        mutation_create_put_file_session(context, file_in).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn mutation_create_tag(context: &(), tag: &TagIn) -> Result<Option<CreateTagError>, String> {
    todo!()
}

fn mutation_create_tag_wrapper<'args>(context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let tag = variables.get("tag").unwrap().downcast_ref::<TagIn>().unwrap();
    Box::pin(async move {
        mutation_create_tag(context, tag).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn mutation_create_user(context: &(), user_in: &UserIn) -> Result<Option<CreateUserError>, String> {
    todo!()
}

fn mutation_create_user_wrapper<'args>(context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let user_in = variables.get("userIn").unwrap().downcast_ref::<UserIn>().unwrap();
    Box::pin(async move {
        mutation_create_user(context, user_in).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn mutation_decide_on_download_request(context: &(), allowed: &bool, file_id: &uuid::Uuid, user_id: &uuid::Uuid) -> Result<Option<DecideOnDownloadRequestError>, String> {
    todo!()
}

fn mutation_decide_on_download_request_wrapper<'args>(context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let allowed = variables.get("allowed").unwrap().downcast_ref::<bool>().unwrap();
    let file_id = variables.get("fileId").unwrap().downcast_ref::<uuid::Uuid>().unwrap();
    let user_id = variables.get("userId").unwrap().downcast_ref::<uuid::Uuid>().unwrap();
    Box::pin(async move {
        mutation_decide_on_download_request(context, allowed, file_id, user_id).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn mutation_delete_file(context: &(), id: &uuid::Uuid) -> Result<Option<DeleteFileError>, String> {
    todo!()
}

fn mutation_delete_file_wrapper<'args>(context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let id = variables.get("id").unwrap().downcast_ref::<uuid::Uuid>().unwrap();
    Box::pin(async move {
        mutation_delete_file(context, id).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn mutation_delete_files(context: &(), ids: &Vec<uuid::Uuid>) -> Result<Option<DeleteFilesError>, String> {
    todo!()
}

fn mutation_delete_files_wrapper<'args>(context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let ids = variables.get("ids").unwrap().downcast_ref::<Vec<uuid::Uuid>>().unwrap();
    Box::pin(async move {
        mutation_delete_files(context, ids).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn mutation_delete_group(context: &(), id: &uuid::Uuid) -> Result<Option<ErrorGroupNotFound>, String> {
    todo!()
}

fn mutation_delete_group_wrapper<'args>(context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let id = variables.get("id").unwrap().downcast_ref::<uuid::Uuid>().unwrap();
    Box::pin(async move {
        mutation_delete_group(context, id).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn mutation_delete_pending_user(context: &(), email: &String) -> Result<Option<ErrorNotFound>, String> {
    todo!()
}

fn mutation_delete_pending_user_wrapper<'args>(context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let email = variables.get("email").unwrap().downcast_ref::<String>().unwrap();
    Box::pin(async move {
        mutation_delete_pending_user(context, email).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn mutation_delete_tag(context: &(), id: &uuid::Uuid) -> Result<Option<ErrorNotFound>, String> {
    todo!()
}

fn mutation_delete_tag_wrapper<'args>(context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let id = variables.get("id").unwrap().downcast_ref::<uuid::Uuid>().unwrap();
    Box::pin(async move {
        mutation_delete_tag(context, id).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn mutation_delete_user(context: &(), id: &uuid::Uuid) -> Result<Option<ErrorNotFound>, String> {
    todo!()
}

fn mutation_delete_user_wrapper<'args>(context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let id = variables.get("id").unwrap().downcast_ref::<uuid::Uuid>().unwrap();
    Box::pin(async move {
        mutation_delete_user(context, id).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn mutation_edit_group(context: &(), group_in: &GroupIn, id: &uuid::Uuid) -> Result<Option<EditGroupError>, String> {
    todo!()
}

fn mutation_edit_group_wrapper<'args>(context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let group_in = variables.get("groupIn").unwrap().downcast_ref::<GroupIn>().unwrap();
    let id = variables.get("id").unwrap().downcast_ref::<uuid::Uuid>().unwrap();
    Box::pin(async move {
        mutation_edit_group(context, group_in, id).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn mutation_edit_tag(context: &(), id: &uuid::Uuid, tag: &TagIn) -> Result<Option<EditTagError>, String> {
    todo!()
}

fn mutation_edit_tag_wrapper<'args>(context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let id = variables.get("id").unwrap().downcast_ref::<uuid::Uuid>().unwrap();
    let tag = variables.get("tag").unwrap().downcast_ref::<TagIn>().unwrap();
    Box::pin(async move {
        mutation_edit_tag(context, id, tag).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn mutation_login(context: &(), email: &String, password: &String) -> Result<Option<ErrorInvalidCredentials>, String> {
    todo!()
}

fn mutation_login_wrapper<'args>(context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let email = variables.get("email").unwrap().downcast_ref::<String>().unwrap();
    let password = variables.get("password").unwrap().downcast_ref::<String>().unwrap();
    Box::pin(async move {
        mutation_login(context, email, password).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn mutation_logout(context: &()) -> Result<(), String> {
    todo!()
}

fn mutation_logout_wrapper<'args>(context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    Box::pin(async move {
        mutation_logout(context).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn mutation_remove_user_from_group(context: &(), group_id: &uuid::Uuid, user_id: &uuid::Uuid) -> Result<Option<ErrorGroupNotFoundOrErrorNotFound>, String> {
    todo!()
}

fn mutation_remove_user_from_group_wrapper<'args>(context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let group_id = variables.get("groupId").unwrap().downcast_ref::<uuid::Uuid>().unwrap();
    let user_id = variables.get("userId").unwrap().downcast_ref::<uuid::Uuid>().unwrap();
    Box::pin(async move {
        mutation_remove_user_from_group(context, group_id, user_id).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn mutation_reset_password(context: &(), new_password: &String, token: &String) -> Result<Option<ResetPasswordError>, String> {
    todo!()
}

fn mutation_reset_password_wrapper<'args>(context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let new_password = variables.get("newPassword").unwrap().downcast_ref::<String>().unwrap();
    let token = variables.get("token").unwrap().downcast_ref::<String>().unwrap();
    Box::pin(async move {
        mutation_reset_password(context, new_password, token).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn mutation_send_otp_code(context: &(), email: &String) -> Result<Option<ErrorInvalidCredentials>, String> {
    todo!()
}

fn mutation_send_otp_code_wrapper<'args>(context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let email = variables.get("email").unwrap().downcast_ref::<String>().unwrap();
    Box::pin(async move {
        mutation_send_otp_code(context, email).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn mutation_set_tag_is_favourite(context: &(), is_favourite: &bool, tag_id: &uuid::Uuid) -> Result<Option<ErrorAlreadyDoneOrUnknownTags>, String> {
    todo!()
}

fn mutation_set_tag_is_favourite_wrapper<'args>(context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let is_favourite = variables.get("isFavourite").unwrap().downcast_ref::<bool>().unwrap();
    let tag_id = variables.get("tagId").unwrap().downcast_ref::<uuid::Uuid>().unwrap();
    Box::pin(async move {
        mutation_set_tag_is_favourite(context, is_favourite, tag_id).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn mutation_update_file(context: &(), id: &uuid::Uuid, name: &String, tag_ids: &Vec<uuid::Uuid>) -> Result<Option<UpdateFileError>, String> {
    todo!()
}

fn mutation_update_file_wrapper<'args>(context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let id = variables.get("id").unwrap().downcast_ref::<uuid::Uuid>().unwrap();
    let name = variables.get("name").unwrap().downcast_ref::<String>().unwrap();
    let tag_ids = variables.get("tagIds").unwrap().downcast_ref::<Vec<uuid::Uuid>>().unwrap();
    Box::pin(async move {
        mutation_update_file(context, id, name, tag_ids).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn mutation_update_files_autotags(context: &(), autotag_ids: &Vec<uuid::Uuid>, file_ids: &Vec<uuid::Uuid>) -> Result<Option<ErrorCantAddAutotags>, String> {
    todo!()
}

fn mutation_update_files_autotags_wrapper<'args>(context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let autotag_ids = variables.get("autotagIds").unwrap().downcast_ref::<Vec<uuid::Uuid>>().unwrap();
    let file_ids = variables.get("fileIds").unwrap().downcast_ref::<Vec<uuid::Uuid>>().unwrap();
    Box::pin(async move {
        mutation_update_files_autotags(context, autotag_ids, file_ids).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

pub struct OTPToken {
    pub token: String,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for OTPToken {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for OTPToken {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("OTPToken".to_string(), libgql::executor::Values::from_iter([("token".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&self.token)?))),
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for PendingUser {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for PendingUser {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("PendingUser".to_string(), libgql::executor::Values::from_iter([("createdAt".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<chrono::DateTime<chrono::Utc> as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&self.created_at)?))),
        ("email".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&self.email)?))),
        ("groups".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Array(self.groups.into_iter().map(|element| Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(element)?.into())))).collect::<Result<Vec<_>, String>>()?))),
        ("name".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&self.name)?))),
        ("ttl".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<f32 as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&self.ttl)?))),
        ])))
    }
}

pub struct PutUploadSession {
    pub id: uuid::Uuid,
    pub upload_url: UploadUrl,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for PutUploadSession {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for PutUploadSession {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("PutUploadSession".to_string(), libgql::executor::Values::from_iter([("id".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<uuid::Uuid as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&self.id)?))),
        ("uploadURL".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(self.upload_url)?.into()))),
        ])))
    }
}

async fn query_get_deal_columns(context: &()) -> Result<Vec<DealColumn>, String> {
    todo!()
}

fn query_get_deal_columns_wrapper<'args>(root_any_ref: &'args libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>, context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    Box::pin(async move {
        query_get_deal_columns(context).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn query_get_deal_info(context: &(), deal_name: &String) -> Result<GetDealInfoResponse, String> {
    todo!()
}

fn query_get_deal_info_wrapper<'args>(root_any_ref: &'args libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>, context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let deal_name = variables.get("dealName").unwrap().downcast_ref::<String>().unwrap();
    Box::pin(async move {
        query_get_deal_info(context, deal_name).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn query_get_deals(context: &(), limit: &i32, query: Option<&String>, skip: &i32) -> Result<Vec<String>, String> {
    todo!()
}

fn query_get_deals_wrapper<'args>(root_any_ref: &'args libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>, context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let limit = variables.get("limit").unwrap().downcast_ref::<i32>().unwrap();
    let query = variables.get("query").map(|v| v.downcast_ref::<String>().unwrap());
    let skip = variables.get("skip").unwrap().downcast_ref::<i32>().unwrap();
    Box::pin(async move {
        query_get_deals(context, limit, query, skip).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn query_get_events(context: &(), date_range: &DateRange, filters: &EventFiltersIn, limit: &i32, query: Option<&String>, skip: &i32) -> Result<GetEventsResponse, String> {
    todo!()
}

fn query_get_events_wrapper<'args>(root_any_ref: &'args libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>, context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let date_range = variables.get("dateRange").unwrap().downcast_ref::<DateRange>().unwrap();
    let filters = variables.get("filters").unwrap().downcast_ref::<EventFiltersIn>().unwrap();
    let limit = variables.get("limit").unwrap().downcast_ref::<i32>().unwrap();
    let query = variables.get("query").map(|v| v.downcast_ref::<String>().unwrap());
    let skip = variables.get("skip").unwrap().downcast_ref::<i32>().unwrap();
    Box::pin(async move {
        query_get_events(context, date_range, filters, limit, query, skip).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn query_get_favourite_tags(context: &(), limit: &i32, skip: &i32) -> Result<Vec<Tag>, String> {
    todo!()
}

fn query_get_favourite_tags_wrapper<'args>(root_any_ref: &'args libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>, context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let limit = variables.get("limit").unwrap().downcast_ref::<i32>().unwrap();
    let skip = variables.get("skip").unwrap().downcast_ref::<i32>().unwrap();
    Box::pin(async move {
        query_get_favourite_tags(context, limit, skip).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn query_get_file_url(context: &(), id: &uuid::Uuid) -> Result<GetFileURLResponse, String> {
    todo!()
}

fn query_get_file_url_wrapper<'args>(root_any_ref: &'args libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>, context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let id = variables.get("id").unwrap().downcast_ref::<uuid::Uuid>().unwrap();
    Box::pin(async move {
        query_get_file_url(context, id).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn query_get_files(context: &(), filters: &Vec<Filter>, limit: &i32, skip: &i32, sort_by: &FileSortBy, tag_ids: &Vec<uuid::Uuid>) -> Result<GetFilesResponse, String> {
    todo!()
}

fn query_get_files_wrapper<'args>(root_any_ref: &'args libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>, context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let filters = variables.get("filters").unwrap().downcast_ref::<Vec<Filter>>().unwrap();
    let limit = variables.get("limit").unwrap().downcast_ref::<i32>().unwrap();
    let skip = variables.get("skip").unwrap().downcast_ref::<i32>().unwrap();
    let sort_by = variables.get("sortBy").unwrap().downcast_ref::<FileSortBy>().unwrap();
    let tag_ids = variables.get("tagIds").unwrap().downcast_ref::<Vec<uuid::Uuid>>().unwrap();
    Box::pin(async move {
        query_get_files(context, filters, limit, skip, sort_by, tag_ids).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn query_get_files_count(context: &(), filters: &Vec<Filter>, tag_ids: &Vec<uuid::Uuid>) -> Result<IntObjectOrErrorUnknownTags, String> {
    todo!()
}

fn query_get_files_count_wrapper<'args>(root_any_ref: &'args libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>, context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let filters = variables.get("filters").unwrap().downcast_ref::<Vec<Filter>>().unwrap();
    let tag_ids = variables.get("tagIds").unwrap().downcast_ref::<Vec<uuid::Uuid>>().unwrap();
    Box::pin(async move {
        query_get_files_count(context, filters, tag_ids).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn query_get_files_deal_info(context: &(), file_ids: &Vec<uuid::Uuid>) -> Result<FilesDealInfoOrError, String> {
    todo!()
}

fn query_get_files_deal_info_wrapper<'args>(root_any_ref: &'args libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>, context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let file_ids = variables.get("fileIds").unwrap().downcast_ref::<Vec<uuid::Uuid>>().unwrap();
    Box::pin(async move {
        query_get_files_deal_info(context, file_ids).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn query_get_group_tags(context: &(), id: &uuid::Uuid, limit: &i32, skip: &i32) -> Result<GetGroupTagsResponse, String> {
    todo!()
}

fn query_get_group_tags_wrapper<'args>(root_any_ref: &'args libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>, context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let id = variables.get("id").unwrap().downcast_ref::<uuid::Uuid>().unwrap();
    let limit = variables.get("limit").unwrap().downcast_ref::<i32>().unwrap();
    let skip = variables.get("skip").unwrap().downcast_ref::<i32>().unwrap();
    Box::pin(async move {
        query_get_group_tags(context, id, limit, skip).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn query_get_group_users(context: &(), group_id: &uuid::Uuid, limit: &i32, skip: &i32, sort_by: &GetGroupUsersSortBy) -> Result<GetGroupUsersResponse, String> {
    todo!()
}

fn query_get_group_users_wrapper<'args>(root_any_ref: &'args libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>, context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let group_id = variables.get("groupId").unwrap().downcast_ref::<uuid::Uuid>().unwrap();
    let limit = variables.get("limit").unwrap().downcast_ref::<i32>().unwrap();
    let skip = variables.get("skip").unwrap().downcast_ref::<i32>().unwrap();
    let sort_by = variables.get("sortBy").unwrap().downcast_ref::<GetGroupUsersSortBy>().unwrap();
    Box::pin(async move {
        query_get_group_users(context, group_id, limit, skip, sort_by).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn query_get_group_users_and_users(context: &(), group_id: &uuid::Uuid, limit: &i32, query: Option<&String>, skip: &i32, sort_by: &GetUsersSortBy) -> Result<GetGroupUsersAndUsersResponse, String> {
    todo!()
}

fn query_get_group_users_and_users_wrapper<'args>(root_any_ref: &'args libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>, context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let group_id = variables.get("groupId").unwrap().downcast_ref::<uuid::Uuid>().unwrap();
    let limit = variables.get("limit").unwrap().downcast_ref::<i32>().unwrap();
    let query = variables.get("query").map(|v| v.downcast_ref::<String>().unwrap());
    let skip = variables.get("skip").unwrap().downcast_ref::<i32>().unwrap();
    let sort_by = variables.get("sortBy").unwrap().downcast_ref::<GetUsersSortBy>().unwrap();
    Box::pin(async move {
        query_get_group_users_and_users(context, group_id, limit, query, skip, sort_by).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn query_get_group_users_total(context: &(), group_id: &uuid::Uuid) -> Result<GetGroupUsersTotalResponse, String> {
    todo!()
}

fn query_get_group_users_total_wrapper<'args>(root_any_ref: &'args libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>, context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let group_id = variables.get("groupId").unwrap().downcast_ref::<uuid::Uuid>().unwrap();
    Box::pin(async move {
        query_get_group_users_total(context, group_id).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn query_get_groups(context: &(), limit: &i32, skip: &i32, sort_by: &GetGroupsSortBy) -> Result<Vec<Group>, String> {
    todo!()
}

fn query_get_groups_wrapper<'args>(root_any_ref: &'args libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>, context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let limit = variables.get("limit").unwrap().downcast_ref::<i32>().unwrap();
    let skip = variables.get("skip").unwrap().downcast_ref::<i32>().unwrap();
    let sort_by = variables.get("sortBy").unwrap().downcast_ref::<GetGroupsSortBy>().unwrap();
    Box::pin(async move {
        query_get_groups(context, limit, skip, sort_by).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn query_get_groups_total(context: &()) -> Result<i32, String> {
    todo!()
}

fn query_get_groups_total_wrapper<'args>(root_any_ref: &'args libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>, context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    Box::pin(async move {
        query_get_groups_total(context).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn query_get_me(context: &()) -> Result<User, String> {
    todo!()
}

fn query_get_me_wrapper<'args>(root_any_ref: &'args libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>, context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    Box::pin(async move {
        query_get_me(context).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn query_get_my_tags(context: &(), limit: &i32, skip: &i32) -> Result<Vec<Tag>, String> {
    todo!()
}

fn query_get_my_tags_wrapper<'args>(root_any_ref: &'args libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>, context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let limit = variables.get("limit").unwrap().downcast_ref::<i32>().unwrap();
    let skip = variables.get("skip").unwrap().downcast_ref::<i32>().unwrap();
    Box::pin(async move {
        query_get_my_tags(context, limit, skip).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn query_get_my_tags_count(context: &()) -> Result<i32, String> {
    todo!()
}

fn query_get_my_tags_count_wrapper<'args>(root_any_ref: &'args libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>, context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    Box::pin(async move {
        query_get_my_tags_count(context).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn query_get_next_multipart_upload_urls(context: &(), last_part: &i32, limit: &i32, session_id: &uuid::Uuid) -> Result<GetNextMultipartUploadUrlsResponse, String> {
    todo!()
}

fn query_get_next_multipart_upload_urls_wrapper<'args>(root_any_ref: &'args libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>, context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let last_part = variables.get("lastPart").unwrap().downcast_ref::<i32>().unwrap();
    let limit = variables.get("limit").unwrap().downcast_ref::<i32>().unwrap();
    let session_id = variables.get("sessionId").unwrap().downcast_ref::<uuid::Uuid>().unwrap();
    Box::pin(async move {
        query_get_next_multipart_upload_urls(context, last_part, limit, session_id).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn query_get_path_to_tag(context: &(), tag_id: &uuid::Uuid) -> Result<GetPathToTagResponse, String> {
    todo!()
}

fn query_get_path_to_tag_wrapper<'args>(root_any_ref: &'args libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>, context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let tag_id = variables.get("tagId").unwrap().downcast_ref::<uuid::Uuid>().unwrap();
    Box::pin(async move {
        query_get_path_to_tag(context, tag_id).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn query_get_pending_users(context: &()) -> Result<Vec<PendingUser>, String> {
    todo!()
}

fn query_get_pending_users_wrapper<'args>(root_any_ref: &'args libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>, context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    Box::pin(async move {
        query_get_pending_users(context).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn query_get_popular_tags(context: &(), limit: &i32, skip: &i32) -> Result<Vec<Tag>, String> {
    todo!()
}

fn query_get_popular_tags_wrapper<'args>(root_any_ref: &'args libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>, context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let limit = variables.get("limit").unwrap().downcast_ref::<i32>().unwrap();
    let skip = variables.get("skip").unwrap().downcast_ref::<i32>().unwrap();
    Box::pin(async move {
        query_get_popular_tags(context, limit, skip).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn query_get_tag_children(context: &(), tag_id: &uuid::Uuid) -> Result<GetTagsResponse, String> {
    todo!()
}

fn query_get_tag_children_wrapper<'args>(root_any_ref: &'args libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>, context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let tag_id = variables.get("tagId").unwrap().downcast_ref::<uuid::Uuid>().unwrap();
    Box::pin(async move {
        query_get_tag_children(context, tag_id).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn query_get_tag_info(context: &(), tag_id: &uuid::Uuid) -> Result<GetTagInfoResponse, String> {
    todo!()
}

fn query_get_tag_info_wrapper<'args>(root_any_ref: &'args libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>, context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let tag_id = variables.get("tagId").unwrap().downcast_ref::<uuid::Uuid>().unwrap();
    Box::pin(async move {
        query_get_tag_info(context, tag_id).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn query_get_tags(context: &(), limit: &i32, parent_tag_id: Option<&uuid::Uuid>, query: Option<&String>, skip: &i32) -> Result<GetTagsResponse, String> {
    todo!()
}

fn query_get_tags_wrapper<'args>(root_any_ref: &'args libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>, context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let limit = variables.get("limit").unwrap().downcast_ref::<i32>().unwrap();
    let parent_tag_id = variables.get("parentTagId").map(|v| v.downcast_ref::<uuid::Uuid>().unwrap());
    let query = variables.get("query").map(|v| v.downcast_ref::<String>().unwrap());
    let skip = variables.get("skip").unwrap().downcast_ref::<i32>().unwrap();
    Box::pin(async move {
        query_get_tags(context, limit, parent_tag_id, query, skip).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn query_get_tags_count(context: &(), parent_tag_id: Option<&uuid::Uuid>, query: Option<&String>) -> Result<IntObjectOrErrorUnknownTags, String> {
    todo!()
}

fn query_get_tags_count_wrapper<'args>(root_any_ref: &'args libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>, context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let parent_tag_id = variables.get("parentTagId").map(|v| v.downcast_ref::<uuid::Uuid>().unwrap());
    let query = variables.get("query").map(|v| v.downcast_ref::<String>().unwrap());
    Box::pin(async move {
        query_get_tags_count(context, parent_tag_id, query).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn query_get_uploaded_files(context: &(), limit: &i32, skip: &i32, sort_by: &FileSortBy) -> Result<Vec<SearchFile>, String> {
    todo!()
}

fn query_get_uploaded_files_wrapper<'args>(root_any_ref: &'args libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>, context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let limit = variables.get("limit").unwrap().downcast_ref::<i32>().unwrap();
    let skip = variables.get("skip").unwrap().downcast_ref::<i32>().unwrap();
    let sort_by = variables.get("sortBy").unwrap().downcast_ref::<FileSortBy>().unwrap();
    Box::pin(async move {
        query_get_uploaded_files(context, limit, skip, sort_by).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn query_get_uploaded_files_count(context: &()) -> Result<i32, String> {
    todo!()
}

fn query_get_uploaded_files_count_wrapper<'args>(root_any_ref: &'args libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>, context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    Box::pin(async move {
        query_get_uploaded_files_count(context).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn query_get_users(context: &(), limit: &i32, query: Option<&String>, skip: &i32, sort_by: &GetUsersSortBy) -> Result<Vec<User>, String> {
    todo!()
}

fn query_get_users_wrapper<'args>(root_any_ref: &'args libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>, context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let limit = variables.get("limit").unwrap().downcast_ref::<i32>().unwrap();
    let query = variables.get("query").map(|v| v.downcast_ref::<String>().unwrap());
    let skip = variables.get("skip").unwrap().downcast_ref::<i32>().unwrap();
    let sort_by = variables.get("sortBy").unwrap().downcast_ref::<GetUsersSortBy>().unwrap();
    Box::pin(async move {
        query_get_users(context, limit, query, skip, sort_by).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn query_get_users_tags(context: &(), limit: &i32, query: Option<&String>, skip: &i32, sort_by: &UsersTagSortBy) -> Result<Vec<UsersTag>, String> {
    todo!()
}

fn query_get_users_tags_wrapper<'args>(root_any_ref: &'args libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>, context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let limit = variables.get("limit").unwrap().downcast_ref::<i32>().unwrap();
    let query = variables.get("query").map(|v| v.downcast_ref::<String>().unwrap());
    let skip = variables.get("skip").unwrap().downcast_ref::<i32>().unwrap();
    let sort_by = variables.get("sortBy").unwrap().downcast_ref::<UsersTagSortBy>().unwrap();
    Box::pin(async move {
        query_get_users_tags(context, limit, query, skip, sort_by).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn query_get_users_tags_count(context: &(), query: Option<&String>) -> Result<i32, String> {
    todo!()
}

fn query_get_users_tags_count_wrapper<'args>(root_any_ref: &'args libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>, context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let query = variables.get("query").map(|v| v.downcast_ref::<String>().unwrap());
    Box::pin(async move {
        query_get_users_tags_count(context, query).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn query_get_users_total(context: &(), query: Option<&String>) -> Result<i32, String> {
    todo!()
}

fn query_get_users_total_wrapper<'args>(root_any_ref: &'args libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>, context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let query = variables.get("query").map(|v| v.downcast_ref::<String>().unwrap());
    Box::pin(async move {
        query_get_users_total(context, query).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn query_is_allowed_to_download(context: &(), id: &uuid::Uuid) -> Result<IsAllowedToDownloadResponse, String> {
    todo!()
}

fn query_is_allowed_to_download_wrapper<'args>(root_any_ref: &'args libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>, context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let id = variables.get("id").unwrap().downcast_ref::<uuid::Uuid>().unwrap();
    Box::pin(async move {
        query_is_allowed_to_download(context, id).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn query_is_tag_exists(context: &(), tag: &String) -> Result<bool, String> {
    todo!()
}

fn query_is_tag_exists_wrapper<'args>(root_any_ref: &'args libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>, context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let tag = variables.get("tag").unwrap().downcast_ref::<String>().unwrap();
    Box::pin(async move {
        query_is_tag_exists(context, tag).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn query_retrieve_file(context: &(), id: &uuid::Uuid) -> Result<RetrieveFileResponse, String> {
    todo!()
}

fn query_retrieve_file_wrapper<'args>(root_any_ref: &'args libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>, context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let id = variables.get("id").unwrap().downcast_ref::<uuid::Uuid>().unwrap();
    Box::pin(async move {
        query_retrieve_file(context, id).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn query_retrieve_group(context: &(), id: &uuid::Uuid) -> Result<RetrieveGroupResponse, String> {
    todo!()
}

fn query_retrieve_group_wrapper<'args>(root_any_ref: &'args libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>, context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let id = variables.get("id").unwrap().downcast_ref::<uuid::Uuid>().unwrap();
    Box::pin(async move {
        query_retrieve_group(context, id).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

async fn query_search_tags(context: &(), query: &String) -> Result<Vec<Tag>, String> {
    todo!()
}

fn query_search_tags_wrapper<'args>(root_any_ref: &'args libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>, context: &'args (), variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::ExampleScalar> {
    let query = variables.get("query").unwrap().downcast_ref::<String>().unwrap();
    Box::pin(async move {
        query_search_tags(context, query).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>>)
    })
}

pub struct SearchFile {
    pub file: File,
    pub tags: Vec<Tag>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for SearchFile {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for SearchFileList {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for StageToWorktypesMapEntry {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for StringEntry {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for StringEntry {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("StringEntry".to_string(), libgql::executor::Values::from_iter([("key".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&self.key)?))),
        ("value".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&self.value)?))),
        ])))
    }
}

pub struct StringList {
    pub values: Vec<String>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for StringList {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for StringList {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("StringList".to_string(), libgql::executor::Values::from_iter([("values".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Array(self.values.into_iter().map(|element| Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&element)?)))).collect::<Result<Vec<_>, String>>()?))),
        ])))
    }
}

pub struct StringObject {
    pub svalue: String,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for StringObject {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for StringObject {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("StringObject".to_string(), libgql::executor::Values::from_iter([("svalue".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&self.svalue)?))),
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for Tag {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for Tag {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("Tag".to_string(), libgql::executor::Values::from_iter([("hasChildren".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&self.has_children)?))),
        ("id".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<uuid::Uuid as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&self.id)?))),
        ("isApproved".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&self.is_approved)?))),
        ("isFavourite".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<bool as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&self.is_favourite)?))),
        ("tag".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&self.tag)?))),
        ("value".to_string(), self.value.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(v)?.into())))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ])))
    }
}

pub struct TagInfo {
    pub parent_tag: Option<Tag>,
    pub tag: String,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for TagInfo {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for TagInfo {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("TagInfo".to_string(), libgql::executor::Values::from_iter([("parentTag".to_string(), self.parent_tag.map(|v| -> Result<libgql::executor::Value<super::scalar::ExampleScalar>, String> {Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(v)?.into())))}).transpose()?.unwrap_or(libgql::executor::Value::Null)),
        ("tag".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&self.tag)?))),
        ])))
    }
}

pub struct TagList {
    pub list: Vec<Tag>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for TagList {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for UploadUrl {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for UploadUrl {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("UploadUrl".to_string(), libgql::executor::Values::from_iter([("headers".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Array(self.headers.into_iter().map(|element| Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(element)?.into())))).collect::<Result<Vec<_>, String>>()?))),
        ("url".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<url::Url as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&self.url)?))),
        ])))
    }
}

pub struct UploadUrlList {
    pub urls: Vec<UploadUrl>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for UploadUrlList {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for UrlObject {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for UrlObject {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("UrlObject".to_string(), libgql::executor::Values::from_iter([("uvalue".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<url::Url as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&self.uvalue)?))),
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for User {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for User {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("User".to_string(), libgql::executor::Values::from_iter([("createdAt".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<chrono::DateTime<chrono::Utc> as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&self.created_at)?))),
        ("email".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&self.email)?))),
        ("id".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<uuid::Uuid as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&self.id)?))),
        ("name".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<String as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&self.name)?))),
        ("tenGroups".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Array(self.ten_groups.into_iter().map(|element| Ok(libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(element)?.into())))).collect::<Result<Vec<_>, String>>()?))),
        ])))
    }
}

pub struct UsersList {
    pub users: Vec<User>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for UsersList {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for UsersTag {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
}

impl TryInto<(String, libgql::executor::Values<super::scalar::ExampleScalar>)> for UsersTag {
    type Error = String;

    fn try_into(self) -> Result<(String, libgql::executor::Values<super::scalar::ExampleScalar>), Self::Error> {
        Ok(("UsersTag".to_string(), libgql::executor::Values::from_iter([("createdAt".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<chrono::DateTime<chrono::Utc> as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&self.created_at)?))),
        ("tag".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(TryInto::<(String, libgql::executor::Values::<super::scalar::ExampleScalar>)>::try_into(self.tag)?.into()))),
        ("usersCount".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(<i32 as libgql::executor::GQLScalar<super::scalar::ExampleScalar>>::to_literal_value(&self.users_count)?))),
        ])))
    }
}

pub enum AddTagsToFilesError {
    ErrorUnknownFiles(ErrorUnknownFiles),
    ErrorUnknownTags(ErrorUnknownTags),
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for AddTagsToFilesError {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ApproveTagError {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for CommitMultipartFileSessionResponse {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for CommitPutFileSessionResponse {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ConfirmOTPCodeResponse {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ConfirmUserError {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for CreateGroupError {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for CreateMultipartFileSessionResponse {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for CreatePutFileSessionResponse {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for CreateTagError {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for CreateUserError {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for DecideOnDownloadRequestError {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for DeleteFileError {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for DeleteFilesError {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for EditGroupError {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for EditTagError {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorAlreadyDoneOrUnknownTags {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorGroupNotFoundOrErrorNotFound {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for Event {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for FilesDealInfoOrError {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for GetDealInfoResponse {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for GetEventsResponse {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for GetFileURLResponse {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for GetFilesResponse {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for GetGroupTagsResponse {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for GetGroupUsersAndUsersResponse {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for GetGroupUsersResponse {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for GetGroupUsersTotalResponse {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for GetNextMultipartUploadUrlsResponse {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for GetPathToTagResponse {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for GetTagInfoResponse {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for GetTagsResponse {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for IntObjectOrErrorUnknownTags {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for IsAllowedToDownloadResponse {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ResetPasswordError {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for RetrieveFileResponse {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for RetrieveGroupResponse {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for TagValue {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for UpdateFileError {
    fn create_introspection_value<'a>(self: &'a Self) -> libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar> {
        todo!()
    }

    fn get_existing_fields(self: &Self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn to_value(&self, _callable_fields: Vec<(String, libgql::executor::ast::Value<super::scalar::ExampleScalar>)>) -> Result<libgql::executor::ast::Value<super::scalar::ExampleScalar>, String> {
        todo!()
    }
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