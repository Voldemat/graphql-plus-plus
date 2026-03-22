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

    fn to_str(self: &Self) -> Result<&str, String> {
        match self {
        Self::List => Ok("LIST"),
        Self::Number => Ok("NUMBER"),
        Self::Date => Ok("DATE"),
        }
    }
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for EDealColumnType {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        libgql::executor::GQLEnum::<super::scalar::ExampleScalar>::to_scalar(self).map(|s| Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Scalar(s)))
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

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for EFileField {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        libgql::executor::GQLEnum::<super::scalar::ExampleScalar>::to_scalar(self).map(|s| Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Scalar(s)))
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

    fn to_str(self: &Self) -> Result<&str, String> {
        match self {
        Self::Name => Ok("NAME"),
        Self::CreatedAt => Ok("CREATED_AT"),
        Self::LimitOfDownloadsPerDay => Ok("LIMIT_OF_DOWNLOADS_PER_DAY"),
        }
    }
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for EGroupField {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        libgql::executor::GQLEnum::<super::scalar::ExampleScalar>::to_scalar(self).map(|s| Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Scalar(s)))
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

    fn to_str(self: &Self) -> Result<&str, String> {
        match self {
        Self::Name => Ok("NAME"),
        Self::Email => Ok("EMAIL"),
        }
    }
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for EGroupUsersField {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        libgql::executor::GQLEnum::<super::scalar::ExampleScalar>::to_scalar(self).map(|s| Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Scalar(s)))
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

    fn to_str(self: &Self) -> Result<&str, String> {
        match self {
        Self::Asc => Ok("ASC"),
        Self::Dsc => Ok("DSC"),
        }
    }
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ESortDirection {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        libgql::executor::GQLEnum::<super::scalar::ExampleScalar>::to_scalar(self).map(|s| Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Scalar(s)))
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

    fn to_str(self: &Self) -> Result<&str, String> {
        match self {
        Self::Name => Ok("NAME"),
        Self::Email => Ok("EMAIL"),
        Self::CreatedAt => Ok("CREATED_AT"),
        }
    }
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for EUserField {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        libgql::executor::GQLEnum::<super::scalar::ExampleScalar>::to_scalar(self).map(|s| Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Scalar(s)))
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

    fn to_str(self: &Self) -> Result<&str, String> {
        match self {
        Self::Tag => Ok("TAG"),
        Self::UsersCount => Ok("USERS_COUNT"),
        Self::CreatedAt => Ok("CREATED_AT"),
        }
    }
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for EUsersTagField {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        libgql::executor::GQLEnum::<super::scalar::ExampleScalar>::to_scalar(self).map(|s| Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Scalar(s)))
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
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "BooleanObject", std::collections::HashMap::from_iter([("bvalue", &self.bvalue as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct DatetimeObject {
    pub dvalue: chrono::DateTime<chrono::Utc>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for DatetimeObject {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "DatetimeObject", std::collections::HashMap::from_iter([("dvalue", &self.dvalue as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct DealColumn {
    pub available_values: Vec<String>,
    pub id: uuid::Uuid,
    pub name: String,
    pub r#type: EDealColumnType,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for DealColumn {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "DealColumn", std::collections::HashMap::from_iter([("availableValues", &self.available_values as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("id", &self.id as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("name", &self.name as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("type", &self.r#type as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct DealEntry {
    pub column_name: String,
    pub value: Tag,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for DealEntry {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "DealEntry", std::collections::HashMap::from_iter([("columnName", &self.column_name as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("value", &self.value as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct DealInfo {
    pub stage_to_worktypes_map: Vec<StageToWorktypesMapEntry>,
    pub values: Vec<DealEntry>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for DealInfo {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "DealInfo", std::collections::HashMap::from_iter([("stageToWorktypesMap", &self.stage_to_worktypes_map as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("values", &self.values as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct ErrorAlreadyApprovedByAdmin {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorAlreadyApprovedByAdmin {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "ErrorAlreadyApprovedByAdmin", std::collections::HashMap::from_iter([("a", &self.a as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct ErrorAlreadyDone {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorAlreadyDone {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "ErrorAlreadyDone", std::collections::HashMap::from_iter([("a", &self.a as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct ErrorAlreadyExists {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorAlreadyExists {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "ErrorAlreadyExists", std::collections::HashMap::from_iter([("a", &self.a as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct ErrorAlreadyPending {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorAlreadyPending {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "ErrorAlreadyPending", std::collections::HashMap::from_iter([("a", &self.a as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct ErrorCantAddAutotags {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorCantAddAutotags {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "ErrorCantAddAutotags", std::collections::HashMap::from_iter([("a", &self.a as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct ErrorChangeForbidden {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorChangeForbidden {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "ErrorChangeForbidden", std::collections::HashMap::from_iter([("a", &self.a as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct ErrorDateRangeIsInvalid {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorDateRangeIsInvalid {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "ErrorDateRangeIsInvalid", std::collections::HashMap::from_iter([("a", &self.a as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct ErrorEmailCollision {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorEmailCollision {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "ErrorEmailCollision", std::collections::HashMap::from_iter([("a", &self.a as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct ErrorFileNotUploaded {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorFileNotUploaded {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "ErrorFileNotUploaded", std::collections::HashMap::from_iter([("a", &self.a as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct ErrorFilesChangeForbidden {
    pub ids: Vec<uuid::Uuid>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorFilesChangeForbidden {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "ErrorFilesChangeForbidden", std::collections::HashMap::from_iter([("ids", &self.ids as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct ErrorGroupNotFound {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorGroupNotFound {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "ErrorGroupNotFound", std::collections::HashMap::from_iter([("a", &self.a as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct ErrorInvalidCredentials {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorInvalidCredentials {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "ErrorInvalidCredentials", std::collections::HashMap::from_iter([("a", &self.a as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct ErrorInvalidEmail {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorInvalidEmail {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "ErrorInvalidEmail", std::collections::HashMap::from_iter([("a", &self.a as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct ErrorInvalidGroupName {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorInvalidGroupName {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "ErrorInvalidGroupName", std::collections::HashMap::from_iter([("a", &self.a as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct ErrorInvalidLimitOfDownloadsPerDay {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorInvalidLimitOfDownloadsPerDay {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "ErrorInvalidLimitOfDownloadsPerDay", std::collections::HashMap::from_iter([("a", &self.a as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct ErrorInvalidOTPCode {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorInvalidOTPCode {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "ErrorInvalidOTPCode", std::collections::HashMap::from_iter([("a", &self.a as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct ErrorInvalidPassword {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorInvalidPassword {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "ErrorInvalidPassword", std::collections::HashMap::from_iter([("a", &self.a as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct ErrorInvalidToken {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorInvalidToken {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "ErrorInvalidToken", std::collections::HashMap::from_iter([("a", &self.a as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct ErrorInvalidUserName {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorInvalidUserName {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "ErrorInvalidUserName", std::collections::HashMap::from_iter([("a", &self.a as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct ErrorMultipartUploadFileIsTooBig {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorMultipartUploadFileIsTooBig {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "ErrorMultipartUploadFileIsTooBig", std::collections::HashMap::from_iter([("a", &self.a as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct ErrorMultipartUploadFileIsTooLight {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorMultipartUploadFileIsTooLight {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "ErrorMultipartUploadFileIsTooLight", std::collections::HashMap::from_iter([("a", &self.a as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct ErrorMultipartUploadFilePartSizeIsTooBig {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorMultipartUploadFilePartSizeIsTooBig {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "ErrorMultipartUploadFilePartSizeIsTooBig", std::collections::HashMap::from_iter([("a", &self.a as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct ErrorMultipartUploadFilePartSizeIsTooSmall {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorMultipartUploadFilePartSizeIsTooSmall {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "ErrorMultipartUploadFilePartSizeIsTooSmall", std::collections::HashMap::from_iter([("a", &self.a as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct ErrorNoDealTag {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorNoDealTag {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "ErrorNoDealTag", std::collections::HashMap::from_iter([("a", &self.a as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct ErrorNotFound {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorNotFound {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "ErrorNotFound", std::collections::HashMap::from_iter([("a", &self.a as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct ErrorOTPCodeExpired {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorOTPCodeExpired {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "ErrorOTPCodeExpired", std::collections::HashMap::from_iter([("a", &self.a as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct ErrorOTPTokenExpired {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorOTPTokenExpired {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "ErrorOTPTokenExpired", std::collections::HashMap::from_iter([("a", &self.a as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct ErrorPutUploadFileIsTooBig {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorPutUploadFileIsTooBig {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "ErrorPutUploadFileIsTooBig", std::collections::HashMap::from_iter([("a", &self.a as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct ErrorUnknownFile {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorUnknownFile {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "ErrorUnknownFile", std::collections::HashMap::from_iter([("a", &self.a as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct ErrorUnknownFiles {
    pub ids: Vec<uuid::Uuid>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorUnknownFiles {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "ErrorUnknownFiles", std::collections::HashMap::from_iter([("ids", &self.ids as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct ErrorUnknownGroupIds {
    pub group_ids: Vec<uuid::Uuid>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorUnknownGroupIds {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "ErrorUnknownGroupIds", std::collections::HashMap::from_iter([("groupIds", &self.group_ids as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct ErrorUnknownGroups {
    pub group_ids: Vec<uuid::Uuid>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorUnknownGroups {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "ErrorUnknownGroups", std::collections::HashMap::from_iter([("groupIds", &self.group_ids as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct ErrorUnknownParentId {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorUnknownParentId {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "ErrorUnknownParentId", std::collections::HashMap::from_iter([("a", &self.a as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct ErrorUnknownSessionId {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorUnknownSessionId {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "ErrorUnknownSessionId", std::collections::HashMap::from_iter([("a", &self.a as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct ErrorUnknownTags {
    pub tag_ids: Vec<uuid::Uuid>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorUnknownTags {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "ErrorUnknownTags", std::collections::HashMap::from_iter([("tagIds", &self.tag_ids as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct ErrorUnknownUser {
    pub a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorUnknownUser {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "ErrorUnknownUser", std::collections::HashMap::from_iter([("a", &self.a as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct ErrorUnknownUsers {
    pub user_ids: Vec<uuid::Uuid>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorUnknownUsers {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "ErrorUnknownUsers", std::collections::HashMap::from_iter([("userIds", &self.user_ids as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct EventFileDeleted {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub file: File,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for EventFileDeleted {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "EventFileDeleted", std::collections::HashMap::from_iter([("createdAt", &self.created_at as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("file", &self.file as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct EventFileDownloadRequested {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub decision: Option<bool>,
    pub file: File,
    pub user: User,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for EventFileDownloadRequested {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "EventFileDownloadRequested", std::collections::HashMap::from_iter([("createdAt", &self.created_at as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("decision", &self.decision as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("file", &self.file as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("user", &self.user as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct EventFileDownloaded {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub file: File,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for EventFileDownloaded {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "EventFileDownloaded", std::collections::HashMap::from_iter([("createdAt", &self.created_at as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("file", &self.file as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct EventFileTagsEdited {
    pub added_tags: Vec<Tag>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub file: File,
    pub removed_tags: Vec<Tag>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for EventFileTagsEdited {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "EventFileTagsEdited", std::collections::HashMap::from_iter([("addedTags", &self.added_tags as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("createdAt", &self.created_at as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("file", &self.file as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("removedTags", &self.removed_tags as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct EventFileUploaded {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub file: File,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for EventFileUploaded {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "EventFileUploaded", std::collections::HashMap::from_iter([("createdAt", &self.created_at as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("file", &self.file as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct EventTagApprovalIsRequested {
    pub already_in_catalog: bool,
    pub author: User,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub tag: Tag,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for EventTagApprovalIsRequested {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "EventTagApprovalIsRequested", std::collections::HashMap::from_iter([("alreadyInCatalog", &self.already_in_catalog as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("author", &self.author as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("createdAt", &self.created_at as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("tag", &self.tag as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct EventsList {
    pub events: Vec<Event>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for EventsList {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "EventsList", std::collections::HashMap::from_iter([("events", &self.events as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
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
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "File", std::collections::HashMap::from_iter([("createdAt", &self.created_at as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("filename", &self.filename as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("id", &self.id as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("mimeType", &self.mime_type as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("previewUrl", &self.preview_url as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("sizeInBytes", &self.size_in_bytes as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("user", &self.user as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct FilesDealInfo {
    pub deal_info: DealInfo,
    pub deal_name: Tag,
    pub unset_columns: Vec<DealColumn>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for FilesDealInfo {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "FilesDealInfo", std::collections::HashMap::from_iter([("dealInfo", &self.deal_info as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("dealName", &self.deal_name as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("unsetColumns", &self.unset_columns as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct FloatObject {
    pub fvalue: f32,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for FloatObject {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "FloatObject", std::collections::HashMap::from_iter([("fvalue", &self.fvalue as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct Group {
    pub first_10_tags: Vec<Tag>,
    pub id: uuid::Uuid,
    pub limit_of_downloads_per_day: i32,
    pub name: String,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for Group {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "Group", std::collections::HashMap::from_iter([("first10Tags", &self.first_10_tags as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("id", &self.id as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("limitOfDownloadsPerDay", &self.limit_of_downloads_per_day as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("name", &self.name as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct GroupUser {
    pub in_group: bool,
    pub user: User,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for GroupUser {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "GroupUser", std::collections::HashMap::from_iter([("inGroup", &self.in_group as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("user", &self.user as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct GroupUserList {
    pub users: Vec<GroupUser>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for GroupUserList {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "GroupUserList", std::collections::HashMap::from_iter([("users", &self.users as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct IntObject {
    pub ivalue: i32,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for IntObject {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "IntObject", std::collections::HashMap::from_iter([("ivalue", &self.ivalue as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct MultipartUploadSession {
    pub id: uuid::Uuid,
    pub initial_upload_ur_ls: Vec<UploadUrl>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for MultipartUploadSession {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "MultipartUploadSession", std::collections::HashMap::from_iter([("id", &self.id as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("initialUploadURLs", &self.initial_upload_ur_ls as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
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
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "OTPToken", std::collections::HashMap::from_iter([("token", &self.token as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
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
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "PendingUser", std::collections::HashMap::from_iter([("createdAt", &self.created_at as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("email", &self.email as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("groups", &self.groups as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("name", &self.name as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("ttl", &self.ttl as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct PutUploadSession {
    pub id: uuid::Uuid,
    pub upload_url: UploadUrl,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for PutUploadSession {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "PutUploadSession", std::collections::HashMap::from_iter([("id", &self.id as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("uploadURL", &self.upload_url as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
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
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "SearchFile", std::collections::HashMap::from_iter([("file", &self.file as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("tags", &self.tags as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct SearchFileList {
    pub files: Vec<SearchFile>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for SearchFileList {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "SearchFileList", std::collections::HashMap::from_iter([("files", &self.files as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct StageToWorktypesMapEntry {
    pub stage: Tag,
    pub worktypes: Vec<Tag>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for StageToWorktypesMapEntry {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "StageToWorktypesMapEntry", std::collections::HashMap::from_iter([("stage", &self.stage as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("worktypes", &self.worktypes as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct StringEntry {
    pub key: String,
    pub value: String,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for StringEntry {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "StringEntry", std::collections::HashMap::from_iter([("key", &self.key as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("value", &self.value as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct StringList {
    pub values: Vec<String>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for StringList {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "StringList", std::collections::HashMap::from_iter([("values", &self.values as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct StringObject {
    pub svalue: String,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for StringObject {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "StringObject", std::collections::HashMap::from_iter([("svalue", &self.svalue as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
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
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "Tag", std::collections::HashMap::from_iter([("hasChildren", &self.has_children as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("id", &self.id as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("isApproved", &self.is_approved as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("isFavourite", &self.is_favourite as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("tag", &self.tag as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("value", &self.value as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct TagInfo {
    pub parent_tag: Option<Tag>,
    pub tag: String,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for TagInfo {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "TagInfo", std::collections::HashMap::from_iter([("parentTag", &self.parent_tag as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("tag", &self.tag as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct TagList {
    pub list: Vec<Tag>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for TagList {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "TagList", std::collections::HashMap::from_iter([("list", &self.list as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct UploadUrl {
    pub headers: Vec<StringEntry>,
    pub url: url::Url,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for UploadUrl {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "UploadUrl", std::collections::HashMap::from_iter([("headers", &self.headers as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("url", &self.url as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct UploadUrlList {
    pub urls: Vec<UploadUrl>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for UploadUrlList {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "UploadUrlList", std::collections::HashMap::from_iter([("urls", &self.urls as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct UrlObject {
    pub uvalue: url::Url,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for UrlObject {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "UrlObject", std::collections::HashMap::from_iter([("uvalue", &self.uvalue as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
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
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "User", std::collections::HashMap::from_iter([("createdAt", &self.created_at as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("email", &self.email as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("id", &self.id as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("name", &self.name as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("tenGroups", &self.ten_groups as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct UsersList {
    pub users: Vec<User>,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for UsersList {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "UsersList", std::collections::HashMap::from_iter([("users", &self.users as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub struct UsersTag {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub tag: Tag,
    pub users_count: i32,
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for UsersTag {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(self, "UsersTag", std::collections::HashMap::from_iter([("createdAt", &self.created_at as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("tag", &self.tag as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>),
        ("usersCount", &self.users_count as &libgql::executor::ast::ResolverRoot<super::scalar::ExampleScalar>)]))))
    }
}

pub enum AddTagsToFilesError {
    ErrorUnknownFiles(ErrorUnknownFiles),
    ErrorUnknownTags(ErrorUnknownTags),
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for AddTagsToFilesError {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        match self {
        Self::ErrorUnknownFiles(item) => item.to_value(),
        Self::ErrorUnknownTags(item) => item.to_value(),
        }
    }
}

pub enum ApproveTagError {
    ErrorAlreadyExists(ErrorAlreadyExists),
    ErrorNotFound(ErrorNotFound),
    ErrorUnknownGroupIds(ErrorUnknownGroupIds),
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ApproveTagError {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        match self {
        Self::ErrorAlreadyExists(item) => item.to_value(),
        Self::ErrorNotFound(item) => item.to_value(),
        Self::ErrorUnknownGroupIds(item) => item.to_value(),
        }
    }
}

pub enum CommitMultipartFileSessionResponse {
    ErrorFileNotUploaded(ErrorFileNotUploaded),
    ErrorUnknownSessionId(ErrorUnknownSessionId),
    File(File),
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for CommitMultipartFileSessionResponse {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        match self {
        Self::ErrorFileNotUploaded(item) => item.to_value(),
        Self::ErrorUnknownSessionId(item) => item.to_value(),
        Self::File(item) => item.to_value(),
        }
    }
}

pub enum CommitPutFileSessionResponse {
    ErrorFileNotUploaded(ErrorFileNotUploaded),
    ErrorUnknownSessionId(ErrorUnknownSessionId),
    File(File),
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for CommitPutFileSessionResponse {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        match self {
        Self::ErrorFileNotUploaded(item) => item.to_value(),
        Self::ErrorUnknownSessionId(item) => item.to_value(),
        Self::File(item) => item.to_value(),
        }
    }
}

pub enum ConfirmOTPCodeResponse {
    ErrorInvalidOTPCode(ErrorInvalidOTPCode),
    ErrorOTPCodeExpired(ErrorOTPCodeExpired),
    OTPToken(OTPToken),
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ConfirmOTPCodeResponse {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        match self {
        Self::ErrorInvalidOTPCode(item) => item.to_value(),
        Self::ErrorOTPCodeExpired(item) => item.to_value(),
        Self::OTPToken(item) => item.to_value(),
        }
    }
}

pub enum ConfirmUserError {
    ErrorInvalidPassword(ErrorInvalidPassword),
    ErrorInvalidToken(ErrorInvalidToken),
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ConfirmUserError {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        match self {
        Self::ErrorInvalidPassword(item) => item.to_value(),
        Self::ErrorInvalidToken(item) => item.to_value(),
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
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        match self {
        Self::ErrorAlreadyExists(item) => item.to_value(),
        Self::ErrorInvalidGroupName(item) => item.to_value(),
        Self::ErrorInvalidLimitOfDownloadsPerDay(item) => item.to_value(),
        Self::ErrorUnknownTags(item) => item.to_value(),
        Self::ErrorUnknownUsers(item) => item.to_value(),
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
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        match self {
        Self::ErrorMultipartUploadFileIsTooBig(item) => item.to_value(),
        Self::ErrorMultipartUploadFileIsTooLight(item) => item.to_value(),
        Self::ErrorMultipartUploadFilePartSizeIsTooBig(item) => item.to_value(),
        Self::ErrorMultipartUploadFilePartSizeIsTooSmall(item) => item.to_value(),
        Self::ErrorNoDealTag(item) => item.to_value(),
        Self::ErrorUnknownTags(item) => item.to_value(),
        Self::MultipartUploadSession(item) => item.to_value(),
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
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        match self {
        Self::ErrorNoDealTag(item) => item.to_value(),
        Self::ErrorPutUploadFileIsTooBig(item) => item.to_value(),
        Self::ErrorUnknownTags(item) => item.to_value(),
        Self::PutUploadSession(item) => item.to_value(),
        }
    }
}

pub enum CreateTagError {
    ErrorAlreadyExists(ErrorAlreadyExists),
    ErrorUnknownParentId(ErrorUnknownParentId),
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for CreateTagError {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        match self {
        Self::ErrorAlreadyExists(item) => item.to_value(),
        Self::ErrorUnknownParentId(item) => item.to_value(),
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
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        match self {
        Self::ErrorAlreadyPending(item) => item.to_value(),
        Self::ErrorEmailCollision(item) => item.to_value(),
        Self::ErrorInvalidEmail(item) => item.to_value(),
        Self::ErrorInvalidUserName(item) => item.to_value(),
        Self::ErrorUnknownGroups(item) => item.to_value(),
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
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        match self {
        Self::ErrorAlreadyDone(item) => item.to_value(),
        Self::ErrorNotFound(item) => item.to_value(),
        Self::ErrorUnknownFile(item) => item.to_value(),
        Self::ErrorUnknownUser(item) => item.to_value(),
        }
    }
}

pub enum DeleteFileError {
    ErrorChangeForbidden(ErrorChangeForbidden),
    ErrorUnknownFile(ErrorUnknownFile),
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for DeleteFileError {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        match self {
        Self::ErrorChangeForbidden(item) => item.to_value(),
        Self::ErrorUnknownFile(item) => item.to_value(),
        }
    }
}

pub enum DeleteFilesError {
    ErrorFilesChangeForbidden(ErrorFilesChangeForbidden),
    ErrorUnknownFiles(ErrorUnknownFiles),
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for DeleteFilesError {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        match self {
        Self::ErrorFilesChangeForbidden(item) => item.to_value(),
        Self::ErrorUnknownFiles(item) => item.to_value(),
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
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        match self {
        Self::ErrorAlreadyExists(item) => item.to_value(),
        Self::ErrorGroupNotFound(item) => item.to_value(),
        Self::ErrorInvalidGroupName(item) => item.to_value(),
        Self::ErrorInvalidLimitOfDownloadsPerDay(item) => item.to_value(),
        Self::ErrorUnknownTags(item) => item.to_value(),
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
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        match self {
        Self::ErrorAlreadyApprovedByAdmin(item) => item.to_value(),
        Self::ErrorAlreadyExists(item) => item.to_value(),
        Self::ErrorNotFound(item) => item.to_value(),
        Self::ErrorUnknownParentId(item) => item.to_value(),
        }
    }
}

pub enum ErrorAlreadyDoneOrUnknownTags {
    ErrorAlreadyDone(ErrorAlreadyDone),
    ErrorUnknownTags(ErrorUnknownTags),
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorAlreadyDoneOrUnknownTags {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        match self {
        Self::ErrorAlreadyDone(item) => item.to_value(),
        Self::ErrorUnknownTags(item) => item.to_value(),
        }
    }
}

pub enum ErrorGroupNotFoundOrErrorNotFound {
    ErrorGroupNotFound(ErrorGroupNotFound),
    ErrorNotFound(ErrorNotFound),
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ErrorGroupNotFoundOrErrorNotFound {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        match self {
        Self::ErrorGroupNotFound(item) => item.to_value(),
        Self::ErrorNotFound(item) => item.to_value(),
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
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        match self {
        Self::EventFileDeleted(item) => item.to_value(),
        Self::EventFileDownloadRequested(item) => item.to_value(),
        Self::EventFileDownloaded(item) => item.to_value(),
        Self::EventFileTagsEdited(item) => item.to_value(),
        Self::EventFileUploaded(item) => item.to_value(),
        Self::EventTagApprovalIsRequested(item) => item.to_value(),
        }
    }
}

pub enum FilesDealInfoOrError {
    ErrorCantAddAutotags(ErrorCantAddAutotags),
    FilesDealInfo(FilesDealInfo),
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for FilesDealInfoOrError {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        match self {
        Self::ErrorCantAddAutotags(item) => item.to_value(),
        Self::FilesDealInfo(item) => item.to_value(),
        }
    }
}

pub enum GetDealInfoResponse {
    DealInfo(DealInfo),
    ErrorNotFound(ErrorNotFound),
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for GetDealInfoResponse {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        match self {
        Self::DealInfo(item) => item.to_value(),
        Self::ErrorNotFound(item) => item.to_value(),
        }
    }
}

pub enum GetEventsResponse {
    ErrorDateRangeIsInvalid(ErrorDateRangeIsInvalid),
    EventsList(EventsList),
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for GetEventsResponse {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        match self {
        Self::ErrorDateRangeIsInvalid(item) => item.to_value(),
        Self::EventsList(item) => item.to_value(),
        }
    }
}

pub enum GetFileURLResponse {
    ErrorUnknownFile(ErrorUnknownFile),
    UrlObject(UrlObject),
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for GetFileURLResponse {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        match self {
        Self::ErrorUnknownFile(item) => item.to_value(),
        Self::UrlObject(item) => item.to_value(),
        }
    }
}

pub enum GetFilesResponse {
    ErrorUnknownTags(ErrorUnknownTags),
    SearchFileList(SearchFileList),
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for GetFilesResponse {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        match self {
        Self::ErrorUnknownTags(item) => item.to_value(),
        Self::SearchFileList(item) => item.to_value(),
        }
    }
}

pub enum GetGroupTagsResponse {
    ErrorNotFound(ErrorNotFound),
    TagList(TagList),
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for GetGroupTagsResponse {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        match self {
        Self::ErrorNotFound(item) => item.to_value(),
        Self::TagList(item) => item.to_value(),
        }
    }
}

pub enum GetGroupUsersAndUsersResponse {
    ErrorGroupNotFound(ErrorGroupNotFound),
    GroupUserList(GroupUserList),
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for GetGroupUsersAndUsersResponse {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        match self {
        Self::ErrorGroupNotFound(item) => item.to_value(),
        Self::GroupUserList(item) => item.to_value(),
        }
    }
}

pub enum GetGroupUsersResponse {
    ErrorNotFound(ErrorNotFound),
    UsersList(UsersList),
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for GetGroupUsersResponse {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        match self {
        Self::ErrorNotFound(item) => item.to_value(),
        Self::UsersList(item) => item.to_value(),
        }
    }
}

pub enum GetGroupUsersTotalResponse {
    ErrorNotFound(ErrorNotFound),
    IntObject(IntObject),
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for GetGroupUsersTotalResponse {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        match self {
        Self::ErrorNotFound(item) => item.to_value(),
        Self::IntObject(item) => item.to_value(),
        }
    }
}

pub enum GetNextMultipartUploadUrlsResponse {
    ErrorUnknownSessionId(ErrorUnknownSessionId),
    UploadUrlList(UploadUrlList),
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for GetNextMultipartUploadUrlsResponse {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        match self {
        Self::ErrorUnknownSessionId(item) => item.to_value(),
        Self::UploadUrlList(item) => item.to_value(),
        }
    }
}

pub enum GetPathToTagResponse {
    ErrorUnknownTags(ErrorUnknownTags),
    StringList(StringList),
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for GetPathToTagResponse {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        match self {
        Self::ErrorUnknownTags(item) => item.to_value(),
        Self::StringList(item) => item.to_value(),
        }
    }
}

pub enum GetTagInfoResponse {
    ErrorNotFound(ErrorNotFound),
    TagInfo(TagInfo),
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for GetTagInfoResponse {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        match self {
        Self::ErrorNotFound(item) => item.to_value(),
        Self::TagInfo(item) => item.to_value(),
        }
    }
}

pub enum GetTagsResponse {
    ErrorUnknownTags(ErrorUnknownTags),
    TagList(TagList),
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for GetTagsResponse {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        match self {
        Self::ErrorUnknownTags(item) => item.to_value(),
        Self::TagList(item) => item.to_value(),
        }
    }
}

pub enum IntObjectOrErrorUnknownTags {
    ErrorUnknownTags(ErrorUnknownTags),
    IntObject(IntObject),
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for IntObjectOrErrorUnknownTags {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        match self {
        Self::ErrorUnknownTags(item) => item.to_value(),
        Self::IntObject(item) => item.to_value(),
        }
    }
}

pub enum IsAllowedToDownloadResponse {
    BooleanObject(BooleanObject),
    ErrorUnknownFile(ErrorUnknownFile),
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for IsAllowedToDownloadResponse {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        match self {
        Self::BooleanObject(item) => item.to_value(),
        Self::ErrorUnknownFile(item) => item.to_value(),
        }
    }
}

pub enum ResetPasswordError {
    ErrorInvalidPassword(ErrorInvalidPassword),
    ErrorOTPTokenExpired(ErrorOTPTokenExpired),
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for ResetPasswordError {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        match self {
        Self::ErrorInvalidPassword(item) => item.to_value(),
        Self::ErrorOTPTokenExpired(item) => item.to_value(),
        }
    }
}

pub enum RetrieveFileResponse {
    ErrorUnknownFile(ErrorUnknownFile),
    SearchFile(SearchFile),
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for RetrieveFileResponse {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        match self {
        Self::ErrorUnknownFile(item) => item.to_value(),
        Self::SearchFile(item) => item.to_value(),
        }
    }
}

pub enum RetrieveGroupResponse {
    ErrorNotFound(ErrorNotFound),
    Group(Group),
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for RetrieveGroupResponse {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        match self {
        Self::ErrorNotFound(item) => item.to_value(),
        Self::Group(item) => item.to_value(),
        }
    }
}

pub enum TagValue {
    DatetimeObject(DatetimeObject),
    FloatObject(FloatObject),
    StringObject(StringObject),
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for TagValue {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        match self {
        Self::DatetimeObject(item) => item.to_value(),
        Self::FloatObject(item) => item.to_value(),
        Self::StringObject(item) => item.to_value(),
        }
    }
}

pub enum UpdateFileError {
    ErrorChangeForbidden(ErrorChangeForbidden),
    ErrorUnknownFile(ErrorUnknownFile),
    ErrorUnknownTags(ErrorUnknownTags),
}

impl libgql::executor::ast::ResolverValue<super::scalar::ExampleScalar> for UpdateFileError {
    fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, super::scalar::ExampleScalar>, String> {
        match self {
        Self::ErrorChangeForbidden(item) => item.to_value(),
        Self::ErrorUnknownFile(item) => item.to_value(),
        Self::ErrorUnknownTags(item) => item.to_value(),
        }
    }
}
