#[derive(Debug, juniper::GraphQLEnum)]
#[graphql(scalar = super::scalar::MyScalarValue)]
pub enum EGroupField {
    NAME,
    CREATED_AT,
    LIMIT_OF_DOWNLOADS_PER_DAY,
}

#[derive(Debug, juniper::GraphQLEnum)]
#[graphql(scalar = super::scalar::MyScalarValue)]
pub enum ESortDirection {
    ASC,
    DSC,
}

#[derive(Debug, juniper::GraphQLEnum)]
#[graphql(scalar = super::scalar::MyScalarValue)]
pub enum EUserField {
    NAME,
    EMAIL,
    CREATED_AT,
}

#[derive(Debug, juniper::GraphQLEnum)]
#[graphql(scalar = super::scalar::MyScalarValue)]
pub enum EGroupUsers {
    NAME,
    EMAIL,
}

#[derive(Debug, juniper::GraphQLInputObject)]
#[graphql(scalar = super::scalar::MyScalarValue)]
pub struct GetGroupsSortBy {
    pub direction: ESortDirection,
    pub field: EGroupField,
}

#[derive(Debug, juniper::GraphQLInputObject)]
#[graphql(scalar = super::scalar::MyScalarValue)]
pub struct GroupIn {
    pub name: String,
    #[graphql(name = "tagIds")]
    pub tag_ids: Vec<uuid::Uuid>,
    #[graphql(name = "limitOfDownloadsPerDay")]
    pub limit_of_downloads_per_day: i32,
}

#[derive(Debug, juniper::GraphQLInputObject)]
#[graphql(scalar = super::scalar::MyScalarValue)]
pub struct DateRange {
    #[graphql(name = "endAt")]
    pub end_at: chrono::DateTime<chrono::Utc>,
    #[graphql(name = "startAt")]
    pub start_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, juniper::GraphQLInputObject)]
#[graphql(scalar = super::scalar::MyScalarValue)]
pub struct GetUsersSortBy {
    pub direction: ESortDirection,
    pub field: EUserField,
}

#[derive(Debug, juniper::GraphQLInputObject)]
#[graphql(scalar = super::scalar::MyScalarValue)]
pub struct GetGroupUsersSortBy {
    pub direction: ESortDirection,
    pub field: EGroupUsers,
}

#[derive(Debug, juniper::GraphQLInputObject)]
#[graphql(scalar = super::scalar::MyScalarValue)]
pub struct EventFiltersIn {
    #[graphql(name = "eventFileDeleted")]
    pub event_file_deleted: bool,
    #[graphql(name = "eventFileDownloaded")]
    pub event_file_downloaded: bool,
    #[graphql(name = "eventFileTagsEdited")]
    pub event_file_tags_edited: bool,
    #[graphql(name = "eventFileUploaded")]
    pub event_file_uploaded: bool,
    #[graphql(name = "eventFileDownloadRequested")]
    pub event_file_download_requested: bool,
    #[graphql(name = "eventTagApprovalIsRequested")]
    pub event_tag_approval_is_requested: bool,
}

#[derive(Debug, juniper::GraphQLObject)]
#[graphql(scalar = super::scalar::MyScalarValue)]
pub struct Tag {
    pub tag: String,
    #[graphql(name = "isApproved")]
    pub is_approved: bool,
    #[graphql(name = "isFavourite")]
    pub is_favourite: bool,
    pub id: uuid::Uuid,
    #[graphql(name = "hasChildren")]
    pub has_children: bool,
}

#[derive(Debug, juniper::GraphQLObject)]
#[graphql(scalar = super::scalar::MyScalarValue)]
pub struct EventFileUploaded {
    pub file: File,
    #[graphql(name = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, juniper::GraphQLObject)]
#[graphql(scalar = super::scalar::MyScalarValue)]
pub struct EventFileDownloadRequested {
    #[graphql(name = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub user: User,
    pub file: File,
    pub decision: Option<bool>,
}

#[derive(Debug, juniper::GraphQLObject)]
#[graphql(scalar = super::scalar::MyScalarValue)]
pub struct GroupUserList {
    pub users: Vec<GroupUser>,
}

#[derive(Debug, juniper::GraphQLObject)]
#[graphql(scalar = super::scalar::MyScalarValue)]
pub struct EventTagApprovalIsRequested {
    #[graphql(name = "alreadyInCatalog")]
    pub already_in_catalog: bool,
    #[graphql(name = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub tag: Tag,
    pub author: User,
}

#[derive(Debug, juniper::GraphQLObject)]
#[graphql(scalar = super::scalar::MyScalarValue)]
pub struct GroupUser {
    #[graphql(name = "inGroup")]
    pub in_group: bool,
    pub user: User,
}

#[derive(Debug, juniper::GraphQLObject)]
#[graphql(scalar = super::scalar::MyScalarValue)]
pub struct EventFileDeleted {
    #[graphql(name = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub file: File,
}

#[derive(Debug, juniper::GraphQLObject)]
#[graphql(scalar = super::scalar::MyScalarValue)]
pub struct EventFileDownloaded {
    pub file: File,
    #[graphql(name = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, juniper::GraphQLObject)]
#[graphql(scalar = super::scalar::MyScalarValue)]
pub struct ErrorGroupNotFound {
    pub a: Option<bool>,
}

#[derive(Debug, juniper::GraphQLObject)]
#[graphql(scalar = super::scalar::MyScalarValue)]
pub struct EventsList {
    pub events: Vec<Event>,
}

#[derive(Debug, juniper::GraphQLObject)]
#[graphql(scalar = super::scalar::MyScalarValue)]
pub struct File {
    #[graphql(name = "sizeInBytes")]
    pub size_in_bytes: i64,
    pub user: User,
    #[graphql(name = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub id: uuid::Uuid,
    #[graphql(name = "mimeType")]
    pub mime_type: Option<String>,
    #[graphql(name = "previewUrl")]
    pub preview_url: Option<url::Url>,
    pub filename: String,
}

#[derive(Debug, juniper::GraphQLObject)]
#[graphql(scalar = super::scalar::MyScalarValue)]
pub struct ErrorInvalidGroupName {
    pub a: Option<bool>,
}

#[derive(Debug, juniper::GraphQLObject)]
#[graphql(scalar = super::scalar::MyScalarValue)]
pub struct ErrorNotFound {
    pub a: Option<bool>,
}

#[derive(Debug, juniper::GraphQLObject)]
#[graphql(scalar = super::scalar::MyScalarValue)]
pub struct EventFileTagsEdited {
    #[graphql(name = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub file: File,
    #[graphql(name = "addedTags")]
    pub added_tags: Vec<Tag>,
    #[graphql(name = "removedTags")]
    pub removed_tags: Vec<Tag>,
}

#[derive(Debug, juniper::GraphQLObject)]
#[graphql(scalar = super::scalar::MyScalarValue)]
pub struct ErrorInvalidLimitOfDownloadsPerDay {
    pub a: Option<bool>,
}

#[derive(Debug, juniper::GraphQLObject)]
#[graphql(scalar = super::scalar::MyScalarValue)]
pub struct Group {
    #[graphql(name = "first10Tags")]
    pub first_10_tags: Vec<Tag>,
    #[graphql(name = "limitOfDownloadsPerDay")]
    pub limit_of_downloads_per_day: i32,
    pub id: uuid::Uuid,
    pub name: String,
}

#[derive(Debug, juniper::GraphQLObject)]
#[graphql(scalar = super::scalar::MyScalarValue)]
pub struct Query {
    #[graphql(name = "getUsers")]
    pub get_users: UsersList,
    #[graphql(name = "getUsersTotal")]
    pub get_users_total: i32,
    #[graphql(name = "retrieveGroup")]
    pub retrieve_group: RetrieveGroupResponse,
    #[graphql(name = "getEvents")]
    pub get_events: GetEventsResponse,
    #[graphql(name = "getGroupUsersAndUsers")]
    pub get_group_users_and_users: GetGroupUsersAndUsersResponse,
    #[graphql(name = "getGroupsTotal")]
    pub get_groups_total: i32,
    #[graphql(name = "getGroupTags")]
    pub get_group_tags: GetGroupTagsResponse,
    #[graphql(name = "getGroupUsersTotal")]
    pub get_group_users_total: GetGroupUsersTotalResponse,
    #[graphql(name = "getGroups")]
    pub get_groups: Vec<Group>,
    #[graphql(name = "getMe")]
    pub get_me: User,
    #[graphql(name = "getGroupUsers")]
    pub get_group_users: GetGroupUsersResponse,
}

#[derive(Debug, juniper::GraphQLObject)]
#[graphql(scalar = super::scalar::MyScalarValue)]
pub struct TagList {
    pub list: Vec<Tag>,
}

#[derive(Debug, juniper::GraphQLObject)]
#[graphql(scalar = super::scalar::MyScalarValue)]
pub struct ErrorUnknownTags {
    #[graphql(name = "tagIds")]
    pub tag_ids: Vec<uuid::Uuid>,
}

#[derive(Debug, juniper::GraphQLObject)]
#[graphql(scalar = super::scalar::MyScalarValue)]
pub struct ErrorDateRangeIsInvalid {
    pub a: Option<bool>,
}

#[derive(Debug, juniper::GraphQLObject)]
#[graphql(scalar = super::scalar::MyScalarValue)]
pub struct User {
    pub name: String,
    #[graphql(name = "tenGroups")]
    pub ten_groups: Vec<Group>,
    pub email: String,
    #[graphql(name = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub id: uuid::Uuid,
}

#[derive(Debug, juniper::GraphQLObject)]
#[graphql(scalar = super::scalar::MyScalarValue)]
pub struct UsersList {
    pub users: Vec<User>,
}

#[derive(Debug, juniper::GraphQLObject)]
#[graphql(scalar = super::scalar::MyScalarValue)]
pub struct ErrorAlreadyExists {
    pub a: Option<bool>,
}

#[derive(Debug, juniper::GraphQLObject)]
#[graphql(scalar = super::scalar::MyScalarValue)]
pub struct ErrorUnknownUsers {
    #[graphql(name = "userIds")]
    pub user_ids: Vec<uuid::Uuid>,
}

#[derive(Debug, juniper::GraphQLObject)]
#[graphql(scalar = super::scalar::MyScalarValue)]
pub struct IntObject {
    pub value: i32,
}

#[derive(Debug, juniper::GraphQLObject)]
#[graphql(scalar = super::scalar::MyScalarValue)]
pub struct Mutation {
    #[graphql(name = "deleteGroup")]
    pub delete_group: Option<ErrorGroupNotFound>,
    #[graphql(name = "editGroup")]
    pub edit_group: Option<EditGroupError>,
    #[graphql(name = "addUserToGroup")]
    pub add_user_to_group: Option<ErrorGroupNotFoundOrErrorNotFound>,
    #[graphql(name = "removeUserFromGroup")]
    pub remove_user_from_group: Option<ErrorGroupNotFoundOrErrorNotFound>,
    #[graphql(name = "createGroup")]
    pub create_group: Option<CreateGroupError>,
}

#[derive(Debug, juniper::GraphQLUnion)]
#[graphql(scalar = super::scalar::MyScalarValue)]
pub enum ErrorGroupNotFoundOrErrorNotFound {
    ErrorGroupNotFound(ErrorGroupNotFound),
    ErrorNotFound(ErrorNotFound),
}

#[derive(Debug, juniper::GraphQLUnion)]
#[graphql(scalar = super::scalar::MyScalarValue)]
pub enum GetGroupUsersResponse {
    ErrorNotFound(ErrorNotFound),
    UsersList(UsersList),
}

#[derive(Debug, juniper::GraphQLUnion)]
#[graphql(scalar = super::scalar::MyScalarValue)]
pub enum GetEventsResponse {
    EventsList(EventsList),
    ErrorDateRangeIsInvalid(ErrorDateRangeIsInvalid),
}

#[derive(Debug, juniper::GraphQLUnion)]
#[graphql(scalar = super::scalar::MyScalarValue)]
pub enum GetGroupTagsResponse {
    ErrorNotFound(ErrorNotFound),
    TagList(TagList),
}

#[derive(Debug, juniper::GraphQLUnion)]
#[graphql(scalar = super::scalar::MyScalarValue)]
pub enum Event {
    EventFileDeleted(EventFileDeleted),
    EventFileDownloaded(EventFileDownloaded),
    EventFileUploaded(EventFileUploaded),
    EventTagApprovalIsRequested(EventTagApprovalIsRequested),
    EventFileTagsEdited(EventFileTagsEdited),
    EventFileDownloadRequested(EventFileDownloadRequested),
}

#[derive(Debug, juniper::GraphQLUnion)]
#[graphql(scalar = super::scalar::MyScalarValue)]
pub enum RetrieveGroupResponse {
    Group(Group),
    ErrorNotFound(ErrorNotFound),
}

#[derive(Debug, juniper::GraphQLUnion)]
#[graphql(scalar = super::scalar::MyScalarValue)]
pub enum CreateGroupError {
    ErrorInvalidLimitOfDownloadsPerDay(ErrorInvalidLimitOfDownloadsPerDay),
    ErrorInvalidGroupName(ErrorInvalidGroupName),
    ErrorAlreadyExists(ErrorAlreadyExists),
    ErrorUnknownUsers(ErrorUnknownUsers),
    ErrorUnknownTags(ErrorUnknownTags),
}

#[derive(Debug, juniper::GraphQLUnion)]
#[graphql(scalar = super::scalar::MyScalarValue)]
pub enum EditGroupError {
    ErrorInvalidLimitOfDownloadsPerDay(ErrorInvalidLimitOfDownloadsPerDay),
    ErrorInvalidGroupName(ErrorInvalidGroupName),
    ErrorGroupNotFound(ErrorGroupNotFound),
    ErrorAlreadyExists(ErrorAlreadyExists),
    ErrorUnknownTags(ErrorUnknownTags),
}

#[derive(Debug, juniper::GraphQLUnion)]
#[graphql(scalar = super::scalar::MyScalarValue)]
pub enum GetGroupUsersTotalResponse {
    IntObject(IntObject),
    ErrorNotFound(ErrorNotFound),
}

#[derive(Debug, juniper::GraphQLUnion)]
#[graphql(scalar = super::scalar::MyScalarValue)]
pub enum GetGroupUsersAndUsersResponse {
    ErrorGroupNotFound(ErrorGroupNotFound),
    GroupUserList(GroupUserList),
}