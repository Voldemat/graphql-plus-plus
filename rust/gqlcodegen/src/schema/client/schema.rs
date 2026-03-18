use super::{directive::Directive, fragment::Fragment, operation::Operation};

#[derive(Debug, serde::Deserialize)]
pub struct Schema {
    fragments: indexmap::IndexMap<String, Fragment>,
    operations: indexmap::IndexMap<String, Operation>,
    directives: indexmap::IndexMap<String, Directive>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema() {
        let _: Schema = serde_json_path_to_error::from_str(
            r##"
{
    "fragments": {
        "DealColumn": {
            "sourceText": "fragment DealColumn on DealColumn { id name type availableValues}",
            "spec": {
                "_type": "ObjectFragmentSpec",
                "name": "DealColumn",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "id",
                        "alias": "id",
                        "arguments": {},
                        "selection": null
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "name",
                        "alias": "name",
                        "arguments": {},
                        "selection": null
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "type",
                        "alias": "type",
                        "arguments": {},
                        "selection": null
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "availableValues",
                        "alias": "availableValues",
                        "arguments": {},
                        "selection": null
                    }
                ]
            }
        },
        "DealEntry": {
            "sourceText": "fragment DealEntry on DealEntry { columnName value { ...Tag }}",
            "spec": {
                "_type": "ObjectFragmentSpec",
                "name": "DealEntry",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "columnName",
                        "alias": "columnName",
                        "arguments": {},
                        "selection": null
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "value",
                        "alias": "value",
                        "arguments": {},
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "Tag",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "Tag"
                                }
                            ]
                        }
                    }
                ]
            }
        },
        "DealInfo": {
            "sourceText": "fragment DealInfo on DealInfo { values { ...DealEntry } stageToWorktypesMap { ...StageToWorktypesMapEntry }}",
            "spec": {
                "_type": "ObjectFragmentSpec",
                "name": "DealInfo",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "values",
                        "alias": "values",
                        "arguments": {},
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "DealEntry",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "DealEntry"
                                }
                            ]
                        }
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "stageToWorktypesMap",
                        "alias": "stageToWorktypesMap",
                        "arguments": {},
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "StageToWorktypesMapEntry",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "StageToWorktypesMapEntry"
                                }
                            ]
                        }
                    }
                ]
            }
        },
        "ErrorUnknownFiles": {
            "sourceText": "fragment ErrorUnknownFiles on ErrorUnknownFiles { ids}",
            "spec": {
                "_type": "ObjectFragmentSpec",
                "name": "ErrorUnknownFiles",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "ids",
                        "alias": "ids",
                        "arguments": {},
                        "selection": null
                    }
                ]
            }
        },
        "ErrorUnknownTags": {
            "sourceText": "fragment ErrorUnknownTags on ErrorUnknownTags { tagIds}",
            "spec": {
                "_type": "ObjectFragmentSpec",
                "name": "ErrorUnknownTags",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "tagIds",
                        "alias": "tagIds",
                        "arguments": {},
                        "selection": null
                    }
                ]
            }
        },
        "Event": {
            "sourceText": "fragment Event on Event { __typename\t... on EventFileDownloaded { ...EventFileDownloaded }\t... on EventFileUploaded { ...EventFileUploaded }\t... on EventFileDeleted { ...EventFileDeleted } ... on EventFileDownloadRequested { ...EventFileDownloadRequested }\t... on EventFileTagsEdited { ...EventFileTagsEdited }\t... on EventTagApprovalIsRequested { ...EventTagApprovalIsRequested }}",
            "spec": {
                "_type": "UnionFragmentSpec",
                "name": "Event",
                "selections": [
                    {
                        "_type": "TypenameField",
                        "alias": null
                    },
                    {
                        "_type": "ObjectConditionalSpreadSelection",
                        "object": "EventFileDownloaded",
                        "spec": {
                            "_type": "ObjectFragmentSpec",
                            "name": "EventFileDownloaded",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "EventFileDownloaded"
                                }
                            ]
                        }
                    },
                    {
                        "_type": "ObjectConditionalSpreadSelection",
                        "object": "EventFileUploaded",
                        "spec": {
                            "_type": "ObjectFragmentSpec",
                            "name": "EventFileUploaded",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "EventFileUploaded"
                                }
                            ]
                        }
                    },
                    {
                        "_type": "ObjectConditionalSpreadSelection",
                        "object": "EventFileDeleted",
                        "spec": {
                            "_type": "ObjectFragmentSpec",
                            "name": "EventFileDeleted",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "EventFileDeleted"
                                }
                            ]
                        }
                    },
                    {
                        "_type": "ObjectConditionalSpreadSelection",
                        "object": "EventFileDownloadRequested",
                        "spec": {
                            "_type": "ObjectFragmentSpec",
                            "name": "EventFileDownloadRequested",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "EventFileDownloadRequested"
                                }
                            ]
                        }
                    },
                    {
                        "_type": "ObjectConditionalSpreadSelection",
                        "object": "EventFileTagsEdited",
                        "spec": {
                            "_type": "ObjectFragmentSpec",
                            "name": "EventFileTagsEdited",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "EventFileTagsEdited"
                                }
                            ]
                        }
                    },
                    {
                        "_type": "ObjectConditionalSpreadSelection",
                        "object": "EventTagApprovalIsRequested",
                        "spec": {
                            "_type": "ObjectFragmentSpec",
                            "name": "EventTagApprovalIsRequested",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "EventTagApprovalIsRequested"
                                }
                            ]
                        }
                    }
                ]
            }
        },
        "EventFileDeleted": {
            "sourceText": "fragment EventFileDeleted on EventFileDeleted { file { ...File } createdAt}",
            "spec": {
                "_type": "ObjectFragmentSpec",
                "name": "EventFileDeleted",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "file",
                        "alias": "file",
                        "arguments": {},
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "File",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "File"
                                }
                            ]
                        }
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "createdAt",
                        "alias": "createdAt",
                        "arguments": {},
                        "selection": null
                    }
                ]
            }
        },
        "EventFileDownloadRequested": {
            "sourceText": "fragment EventFileDownloadRequested on EventFileDownloadRequested { file { ...File } user { ...User } decision createdAt}",
            "spec": {
                "_type": "ObjectFragmentSpec",
                "name": "EventFileDownloadRequested",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "file",
                        "alias": "file",
                        "arguments": {},
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "File",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "File"
                                }
                            ]
                        }
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "user",
                        "alias": "user",
                        "arguments": {},
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "User",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "User"
                                }
                            ]
                        }
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "decision",
                        "alias": "decision",
                        "arguments": {},
                        "selection": null
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "createdAt",
                        "alias": "createdAt",
                        "arguments": {},
                        "selection": null
                    }
                ]
            }
        },
        "EventFileDownloaded": {
            "sourceText": "fragment EventFileDownloaded on EventFileDownloaded { file { ...File } createdAt}",
            "spec": {
                "_type": "ObjectFragmentSpec",
                "name": "EventFileDownloaded",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "file",
                        "alias": "file",
                        "arguments": {},
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "File",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "File"
                                }
                            ]
                        }
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "createdAt",
                        "alias": "createdAt",
                        "arguments": {},
                        "selection": null
                    }
                ]
            }
        },
        "EventFileTagsEdited": {
            "sourceText": "fragment EventFileTagsEdited on EventFileTagsEdited { file { ...File } addedTags { ...Tag } removedTags { ...Tag } createdAt}",
            "spec": {
                "_type": "ObjectFragmentSpec",
                "name": "EventFileTagsEdited",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "file",
                        "alias": "file",
                        "arguments": {},
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "File",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "File"
                                }
                            ]
                        }
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "addedTags",
                        "alias": "addedTags",
                        "arguments": {},
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "Tag",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "Tag"
                                }
                            ]
                        }
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "removedTags",
                        "alias": "removedTags",
                        "arguments": {},
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "Tag",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "Tag"
                                }
                            ]
                        }
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "createdAt",
                        "alias": "createdAt",
                        "arguments": {},
                        "selection": null
                    }
                ]
            }
        },
        "EventFileUploaded": {
            "sourceText": "fragment EventFileUploaded on EventFileUploaded { file { ...File } createdAt}",
            "spec": {
                "_type": "ObjectFragmentSpec",
                "name": "EventFileUploaded",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "file",
                        "alias": "file",
                        "arguments": {},
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "File",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "File"
                                }
                            ]
                        }
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "createdAt",
                        "alias": "createdAt",
                        "arguments": {},
                        "selection": null
                    }
                ]
            }
        },
        "EventTagApprovalIsRequested": {
            "sourceText": "fragment EventTagApprovalIsRequested on EventTagApprovalIsRequested { tag { ...Tag } author { ...User } alreadyInCatalog createdAt}",
            "spec": {
                "_type": "ObjectFragmentSpec",
                "name": "EventTagApprovalIsRequested",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "tag",
                        "alias": "tag",
                        "arguments": {},
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "Tag",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "Tag"
                                }
                            ]
                        }
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "author",
                        "alias": "author",
                        "arguments": {},
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "User",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "User"
                                }
                            ]
                        }
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "alreadyInCatalog",
                        "alias": "alreadyInCatalog",
                        "arguments": {},
                        "selection": null
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "createdAt",
                        "alias": "createdAt",
                        "arguments": {},
                        "selection": null
                    }
                ]
            }
        },
        "File": {
            "sourceText": "fragment File on File { id filename mimeType previewUrl createdAt sizeInBytes user { id name }}",
            "spec": {
                "_type": "ObjectFragmentSpec",
                "name": "File",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "id",
                        "alias": "id",
                        "arguments": {},
                        "selection": null
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "filename",
                        "alias": "filename",
                        "arguments": {},
                        "selection": null
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "mimeType",
                        "alias": "mimeType",
                        "arguments": {},
                        "selection": null
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "previewUrl",
                        "alias": "previewUrl",
                        "arguments": {},
                        "selection": null
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "createdAt",
                        "alias": "createdAt",
                        "arguments": {},
                        "selection": null
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "sizeInBytes",
                        "alias": "sizeInBytes",
                        "arguments": {},
                        "selection": null
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "user",
                        "alias": "user",
                        "arguments": {},
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "User",
                            "selections": [
                                {
                                    "_type": "FieldSelection",
                                    "name": "id",
                                    "alias": "id",
                                    "arguments": {},
                                    "selection": null
                                },
                                {
                                    "_type": "FieldSelection",
                                    "name": "name",
                                    "alias": "name",
                                    "arguments": {},
                                    "selection": null
                                }
                            ]
                        }
                    }
                ]
            }
        },
        "FilesDealInfo": {
            "sourceText": "fragment FilesDealInfo on FilesDealInfo { dealName { ...Tag } dealInfo { ...DealInfo } unsetColumns { ...DealColumn }}",
            "spec": {
                "_type": "ObjectFragmentSpec",
                "name": "FilesDealInfo",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "dealName",
                        "alias": "dealName",
                        "arguments": {},
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "Tag",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "Tag"
                                }
                            ]
                        }
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "dealInfo",
                        "alias": "dealInfo",
                        "arguments": {},
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "DealInfo",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "DealInfo"
                                }
                            ]
                        }
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "unsetColumns",
                        "alias": "unsetColumns",
                        "arguments": {},
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "DealColumn",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "DealColumn"
                                }
                            ]
                        }
                    }
                ]
            }
        },
        "Group": {
            "sourceText": "fragment Group on Group { id name first10Tags { ...Tag } limitOfDownloadsPerDay}",
            "spec": {
                "_type": "ObjectFragmentSpec",
                "name": "Group",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "id",
                        "alias": "id",
                        "arguments": {},
                        "selection": null
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "name",
                        "alias": "name",
                        "arguments": {},
                        "selection": null
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "first10Tags",
                        "alias": "first10Tags",
                        "arguments": {},
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "Tag",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "Tag"
                                }
                            ]
                        }
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "limitOfDownloadsPerDay",
                        "alias": "limitOfDownloadsPerDay",
                        "arguments": {},
                        "selection": null
                    }
                ]
            }
        },
        "GroupUser": {
            "sourceText": "fragment GroupUser on GroupUser { inGroup user { ...User }}",
            "spec": {
                "_type": "ObjectFragmentSpec",
                "name": "GroupUser",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "inGroup",
                        "alias": "inGroup",
                        "arguments": {},
                        "selection": null
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "user",
                        "alias": "user",
                        "arguments": {},
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "User",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "User"
                                }
                            ]
                        }
                    }
                ]
            }
        },
        "GroupUserList": {
            "sourceText": "fragment GroupUserList on GroupUserList { users { ...GroupUser }}",
            "spec": {
                "_type": "ObjectFragmentSpec",
                "name": "GroupUserList",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "users",
                        "alias": "users",
                        "arguments": {},
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "GroupUser",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "GroupUser"
                                }
                            ]
                        }
                    }
                ]
            }
        },
        "IntObject": {
            "sourceText": "fragment IntObject on IntObject { ivalue}",
            "spec": {
                "_type": "ObjectFragmentSpec",
                "name": "IntObject",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "ivalue",
                        "alias": "ivalue",
                        "arguments": {},
                        "selection": null
                    }
                ]
            }
        },
        "MultipartUploadSession": {
            "sourceText": "fragment MultipartUploadSession on MultipartUploadSession { id initialUploadURLs { ...UploadUrl }}",
            "spec": {
                "_type": "ObjectFragmentSpec",
                "name": "MultipartUploadSession",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "id",
                        "alias": "id",
                        "arguments": {},
                        "selection": null
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "initialUploadURLs",
                        "alias": "initialUploadURLs",
                        "arguments": {},
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "UploadUrl",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "UploadUrl"
                                }
                            ]
                        }
                    }
                ]
            }
        },
        "PendingUser": {
            "sourceText": "fragment PendingUser on PendingUser { ttl name email groups { ...Group } createdAt}",
            "spec": {
                "_type": "ObjectFragmentSpec",
                "name": "PendingUser",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "ttl",
                        "alias": "ttl",
                        "arguments": {},
                        "selection": null
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "name",
                        "alias": "name",
                        "arguments": {},
                        "selection": null
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "email",
                        "alias": "email",
                        "arguments": {},
                        "selection": null
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "groups",
                        "alias": "groups",
                        "arguments": {},
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "Group",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "Group"
                                }
                            ]
                        }
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "createdAt",
                        "alias": "createdAt",
                        "arguments": {},
                        "selection": null
                    }
                ]
            }
        },
        "PutUploadSession": {
            "sourceText": "fragment PutUploadSession on PutUploadSession { id uploadURL { ...UploadUrl }}",
            "spec": {
                "_type": "ObjectFragmentSpec",
                "name": "PutUploadSession",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "id",
                        "alias": "id",
                        "arguments": {},
                        "selection": null
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "uploadURL",
                        "alias": "uploadURL",
                        "arguments": {},
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "UploadUrl",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "UploadUrl"
                                }
                            ]
                        }
                    }
                ]
            }
        },
        "SearchFile": {
            "sourceText": "fragment SearchFile on SearchFile { file { ...File } tags { ...SmallTag }}",
            "spec": {
                "_type": "ObjectFragmentSpec",
                "name": "SearchFile",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "file",
                        "alias": "file",
                        "arguments": {},
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "File",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "File"
                                }
                            ]
                        }
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "tags",
                        "alias": "tags",
                        "arguments": {},
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "Tag",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "SmallTag"
                                }
                            ]
                        }
                    }
                ]
            }
        },
        "SearchFileList": {
            "sourceText": "fragment SearchFileList on SearchFileList { files { ...SearchFile }}",
            "spec": {
                "_type": "ObjectFragmentSpec",
                "name": "SearchFileList",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "files",
                        "alias": "files",
                        "arguments": {},
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "SearchFile",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "SearchFile"
                                }
                            ]
                        }
                    }
                ]
            }
        },
        "SmallTag": {
            "sourceText": "fragment SmallTag on Tag { id tag value { ...TagValue }}",
            "spec": {
                "_type": "ObjectFragmentSpec",
                "name": "Tag",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "id",
                        "alias": "id",
                        "arguments": {},
                        "selection": null
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "tag",
                        "alias": "tag",
                        "arguments": {},
                        "selection": null
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "value",
                        "alias": "value",
                        "arguments": {},
                        "selection": {
                            "_type": "UnionFragmentSpec",
                            "name": "TagValue",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "TagValue"
                                }
                            ]
                        }
                    }
                ]
            }
        },
        "StageToWorktypesMapEntry": {
            "sourceText": "fragment StageToWorktypesMapEntry on StageToWorktypesMapEntry { stage { ...Tag } worktypes { ...Tag }}",
            "spec": {
                "_type": "ObjectFragmentSpec",
                "name": "StageToWorktypesMapEntry",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "stage",
                        "alias": "stage",
                        "arguments": {},
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "Tag",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "Tag"
                                }
                            ]
                        }
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "worktypes",
                        "alias": "worktypes",
                        "arguments": {},
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "Tag",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "Tag"
                                }
                            ]
                        }
                    }
                ]
            }
        },
        "Tag": {
            "sourceText": "fragment Tag on Tag { id tag isFavourite hasChildren isApproved value { ...TagValue }}",
            "spec": {
                "_type": "ObjectFragmentSpec",
                "name": "Tag",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "id",
                        "alias": "id",
                        "arguments": {},
                        "selection": null
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "tag",
                        "alias": "tag",
                        "arguments": {},
                        "selection": null
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "isFavourite",
                        "alias": "isFavourite",
                        "arguments": {},
                        "selection": null
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "hasChildren",
                        "alias": "hasChildren",
                        "arguments": {},
                        "selection": null
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "isApproved",
                        "alias": "isApproved",
                        "arguments": {},
                        "selection": null
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "value",
                        "alias": "value",
                        "arguments": {},
                        "selection": {
                            "_type": "UnionFragmentSpec",
                            "name": "TagValue",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "TagValue"
                                }
                            ]
                        }
                    }
                ]
            }
        },
        "TagList": {
            "sourceText": "fragment TagList on TagList { list { ...Tag }}",
            "spec": {
                "_type": "ObjectFragmentSpec",
                "name": "TagList",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "list",
                        "alias": "list",
                        "arguments": {},
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "Tag",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "Tag"
                                }
                            ]
                        }
                    }
                ]
            }
        },
        "TagValue": {
            "sourceText": "fragment TagValue on TagValue { __typename ... on StringObject { svalue } ... on FloatObject { fvalue } ... on DatetimeObject { dvalue }}",
            "spec": {
                "_type": "UnionFragmentSpec",
                "name": "TagValue",
                "selections": [
                    {
                        "_type": "TypenameField",
                        "alias": null
                    },
                    {
                        "_type": "ObjectConditionalSpreadSelection",
                        "object": "StringObject",
                        "spec": {
                            "_type": "ObjectFragmentSpec",
                            "name": "StringObject",
                            "selections": [
                                {
                                    "_type": "FieldSelection",
                                    "name": "svalue",
                                    "alias": "svalue",
                                    "arguments": {},
                                    "selection": null
                                }
                            ]
                        }
                    },
                    {
                        "_type": "ObjectConditionalSpreadSelection",
                        "object": "FloatObject",
                        "spec": {
                            "_type": "ObjectFragmentSpec",
                            "name": "FloatObject",
                            "selections": [
                                {
                                    "_type": "FieldSelection",
                                    "name": "fvalue",
                                    "alias": "fvalue",
                                    "arguments": {},
                                    "selection": null
                                }
                            ]
                        }
                    },
                    {
                        "_type": "ObjectConditionalSpreadSelection",
                        "object": "DatetimeObject",
                        "spec": {
                            "_type": "ObjectFragmentSpec",
                            "name": "DatetimeObject",
                            "selections": [
                                {
                                    "_type": "FieldSelection",
                                    "name": "dvalue",
                                    "alias": "dvalue",
                                    "arguments": {},
                                    "selection": null
                                }
                            ]
                        }
                    }
                ]
            }
        },
        "UploadUrl": {
            "sourceText": "fragment UploadUrl on UploadUrl { url headers { key value }}",
            "spec": {
                "_type": "ObjectFragmentSpec",
                "name": "UploadUrl",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "url",
                        "alias": "url",
                        "arguments": {},
                        "selection": null
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "headers",
                        "alias": "headers",
                        "arguments": {},
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "StringEntry",
                            "selections": [
                                {
                                    "_type": "FieldSelection",
                                    "name": "key",
                                    "alias": "key",
                                    "arguments": {},
                                    "selection": null
                                },
                                {
                                    "_type": "FieldSelection",
                                    "name": "value",
                                    "alias": "value",
                                    "arguments": {},
                                    "selection": null
                                }
                            ]
                        }
                    }
                ]
            }
        },
        "User": {
            "sourceText": "fragment User on User { id name email tenGroups { id name } createdAt}",
            "spec": {
                "_type": "ObjectFragmentSpec",
                "name": "User",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "id",
                        "alias": "id",
                        "arguments": {},
                        "selection": null
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "name",
                        "alias": "name",
                        "arguments": {},
                        "selection": null
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "email",
                        "alias": "email",
                        "arguments": {},
                        "selection": null
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "tenGroups",
                        "alias": "tenGroups",
                        "arguments": {},
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "Group",
                            "selections": [
                                {
                                    "_type": "FieldSelection",
                                    "name": "id",
                                    "alias": "id",
                                    "arguments": {},
                                    "selection": null
                                },
                                {
                                    "_type": "FieldSelection",
                                    "name": "name",
                                    "alias": "name",
                                    "arguments": {},
                                    "selection": null
                                }
                            ]
                        }
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "createdAt",
                        "alias": "createdAt",
                        "arguments": {},
                        "selection": null
                    }
                ]
            }
        },
        "UsersList": {
            "sourceText": "fragment UsersList on UsersList { users { ...User }}",
            "spec": {
                "_type": "ObjectFragmentSpec",
                "name": "UsersList",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "users",
                        "alias": "users",
                        "arguments": {},
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "User",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "User"
                                }
                            ]
                        }
                    }
                ]
            }
        },
        "UsersTag": {
            "sourceText": "fragment UsersTag on UsersTag { tag { id tag } usersCount createdAt}",
            "spec": {
                "_type": "ObjectFragmentSpec",
                "name": "UsersTag",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "tag",
                        "alias": "tag",
                        "arguments": {},
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "Tag",
                            "selections": [
                                {
                                    "_type": "FieldSelection",
                                    "name": "id",
                                    "alias": "id",
                                    "arguments": {},
                                    "selection": null
                                },
                                {
                                    "_type": "FieldSelection",
                                    "name": "tag",
                                    "alias": "tag",
                                    "arguments": {},
                                    "selection": null
                                }
                            ]
                        }
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "usersCount",
                        "alias": "usersCount",
                        "arguments": {},
                        "selection": null
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "createdAt",
                        "alias": "createdAt",
                        "arguments": {},
                        "selection": null
                    }
                ]
            }
        }
    },
    "operations": {
        "AddTagsToFiles": {
            "name": "AddTagsToFiles",
            "type": "MUTATION",
            "parameters": {
                "$fileIds": {
                    "nullable": false,
                    "spec": {
                        "_type": "array",
                        "nullable": false,
                        "type": {
                            "_type": "literal",
                            "type": {
                                "_type": "Scalar",
                                "name": "UUID"
                            },
                            "defaultValue": null
                        },
                        "defaultValue": null
                    }
                },
                "$tagIds": {
                    "nullable": false,
                    "spec": {
                        "_type": "array",
                        "nullable": false,
                        "type": {
                            "_type": "literal",
                            "type": {
                                "_type": "Scalar",
                                "name": "UUID"
                            },
                            "defaultValue": null
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Mutation",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "addTagsToFiles",
                        "alias": "error",
                        "arguments": {
                            "fileIds": {
                                "name": "fileIds",
                                "value": {
                                    "_type": "ref",
                                    "name": "$fileIds"
                                }
                            },
                            "tagIds": {
                                "name": "tagIds",
                                "value": {
                                    "_type": "ref",
                                    "name": "$tagIds"
                                }
                            }
                        },
                        "selection": {
                            "_type": "UnionFragmentSpec",
                            "name": "AddTagsToFilesError",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": null
                                },
                                {
                                    "_type": "ObjectConditionalSpreadSelection",
                                    "object": "ErrorUnknownTags",
                                    "spec": {
                                        "_type": "ObjectFragmentSpec",
                                        "name": "ErrorUnknownTags",
                                        "selections": [
                                            {
                                                "_type": "SpreadSelection",
                                                "fragment": "ErrorUnknownTags"
                                            }
                                        ]
                                    }
                                },
                                {
                                    "_type": "ObjectConditionalSpreadSelection",
                                    "object": "ErrorUnknownFiles",
                                    "spec": {
                                        "_type": "ObjectFragmentSpec",
                                        "name": "ErrorUnknownFiles",
                                        "selections": [
                                            {
                                                "_type": "SpreadSelection",
                                                "fragment": "ErrorUnknownFiles"
                                            }
                                        ]
                                    }
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "mutation AddTagsToFiles($fileIds: [UUID!]!, $tagIds: [UUID!]!) { error: addTagsToFiles(fileIds: $fileIds, tagIds: $tagIds) { __typename ... on ErrorUnknownTags { ...ErrorUnknownTags } ... on ErrorUnknownFiles { ...ErrorUnknownFiles } }}",
            "parametersHash": 2185702073865870716,
            "fragmentSpecHash": 1576722118365798537
        },
        "AddUserToGroup": {
            "name": "AddUserToGroup",
            "type": "MUTATION",
            "parameters": {
                "$groupId": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "UUID"
                        },
                        "defaultValue": null
                    }
                },
                "$userId": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "UUID"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Mutation",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "addUserToGroup",
                        "alias": "error",
                        "arguments": {
                            "groupId": {
                                "name": "groupId",
                                "value": {
                                    "_type": "ref",
                                    "name": "$groupId"
                                }
                            },
                            "userId": {
                                "name": "userId",
                                "value": {
                                    "_type": "ref",
                                    "name": "$userId"
                                }
                            }
                        },
                        "selection": {
                            "_type": "UnionFragmentSpec",
                            "name": "ErrorGroupNotFoundOrErrorNotFound",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": null
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "mutation AddUserToGroup($groupId: UUID!, $userId: UUID!) { error: addUserToGroup(groupId: $groupId, userId: $userId) { __typename }}",
            "parametersHash": 14153868134748337413,
            "fragmentSpecHash": 7888795992494618284
        },
        "ApproveTag": {
            "name": "ApproveTag",
            "type": "MUTATION",
            "parameters": {
                "$groupIds": {
                    "nullable": false,
                    "spec": {
                        "_type": "array",
                        "nullable": false,
                        "type": {
                            "_type": "literal",
                            "type": {
                                "_type": "Scalar",
                                "name": "UUID"
                            },
                            "defaultValue": null
                        },
                        "defaultValue": null
                    }
                },
                "$tagId": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "UUID"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Mutation",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "approveTag",
                        "alias": "error",
                        "arguments": {
                            "groupIds": {
                                "name": "groupIds",
                                "value": {
                                    "_type": "ref",
                                    "name": "$groupIds"
                                }
                            },
                            "tagId": {
                                "name": "tagId",
                                "value": {
                                    "_type": "ref",
                                    "name": "$tagId"
                                }
                            }
                        },
                        "selection": {
                            "_type": "UnionFragmentSpec",
                            "name": "ApproveTagError",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": null
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "mutation ApproveTag($tagId: UUID!, $groupIds: [UUID!]!) { error: approveTag(tagId: $tagId, groupIds: $groupIds) { __typename }}",
            "parametersHash": 11418067776560319514,
            "fragmentSpecHash": 9946562629747687387
        },
        "ChangePassword": {
            "name": "ChangePassword",
            "type": "MUTATION",
            "parameters": {
                "$newPassword": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "String"
                        },
                        "defaultValue": null
                    }
                },
                "$oldPassword": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "String"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Mutation",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "changePassword",
                        "alias": "error",
                        "arguments": {
                            "newPassword": {
                                "name": "newPassword",
                                "value": {
                                    "_type": "ref",
                                    "name": "$newPassword"
                                }
                            },
                            "oldPassword": {
                                "name": "oldPassword",
                                "value": {
                                    "_type": "ref",
                                    "name": "$oldPassword"
                                }
                            }
                        },
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "ErrorInvalidCredentials",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": null
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "mutation ChangePassword($oldPassword: String!, $newPassword: String!) { error: changePassword( oldPassword: $oldPassword, newPassword: $newPassword ) { __typename }}",
            "parametersHash": 7773917004686656983,
            "fragmentSpecHash": 4834969219030524016
        },
        "CommitMultipartFileSession": {
            "name": "CommitMultipartFileSession",
            "type": "MUTATION",
            "parameters": {
                "$sessionId": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "UUID"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Mutation",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "commitMultipartFileSession",
                        "alias": "commitMultipartFileSession",
                        "arguments": {
                            "sessionId": {
                                "name": "sessionId",
                                "value": {
                                    "_type": "ref",
                                    "name": "$sessionId"
                                }
                            }
                        },
                        "selection": {
                            "_type": "UnionFragmentSpec",
                            "name": "CommitMultipartFileSessionResponse",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": null
                                },
                                {
                                    "_type": "ObjectConditionalSpreadSelection",
                                    "object": "File",
                                    "spec": {
                                        "_type": "ObjectFragmentSpec",
                                        "name": "File",
                                        "selections": [
                                            {
                                                "_type": "FieldSelection",
                                                "name": "id",
                                                "alias": "id",
                                                "arguments": {},
                                                "selection": null
                                            }
                                        ]
                                    }
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "mutation CommitMultipartFileSession($sessionId: UUID!) { commitMultipartFileSession(sessionId: $sessionId) { __typename ... on File { id } }}",
            "parametersHash": 18025165575234809886,
            "fragmentSpecHash": 10666467772299217132
        },
        "CommitPutFileSession": {
            "name": "CommitPutFileSession",
            "type": "MUTATION",
            "parameters": {
                "$sessionId": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "UUID"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Mutation",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "commitPutFileSession",
                        "alias": "commitPutFileSession",
                        "arguments": {
                            "sessionId": {
                                "name": "sessionId",
                                "value": {
                                    "_type": "ref",
                                    "name": "$sessionId"
                                }
                            }
                        },
                        "selection": {
                            "_type": "UnionFragmentSpec",
                            "name": "CommitPutFileSessionResponse",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": null
                                },
                                {
                                    "_type": "ObjectConditionalSpreadSelection",
                                    "object": "File",
                                    "spec": {
                                        "_type": "ObjectFragmentSpec",
                                        "name": "File",
                                        "selections": [
                                            {
                                                "_type": "FieldSelection",
                                                "name": "id",
                                                "alias": "id",
                                                "arguments": {},
                                                "selection": null
                                            }
                                        ]
                                    }
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "mutation CommitPutFileSession($sessionId: UUID!) { commitPutFileSession(sessionId: $sessionId) { __typename ... on File { id } }}",
            "parametersHash": 18025165575234809886,
            "fragmentSpecHash": 2778925650749638357
        },
        "ConfirmOTPCode": {
            "name": "ConfirmOTPCode",
            "type": "MUTATION",
            "parameters": {
                "$code": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "String"
                        },
                        "defaultValue": null
                    }
                },
                "$email": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "String"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Mutation",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "confirmOTPCode",
                        "alias": "confirmOTPCode",
                        "arguments": {
                            "code": {
                                "name": "code",
                                "value": {
                                    "_type": "ref",
                                    "name": "$code"
                                }
                            },
                            "email": {
                                "name": "email",
                                "value": {
                                    "_type": "ref",
                                    "name": "$email"
                                }
                            }
                        },
                        "selection": {
                            "_type": "UnionFragmentSpec",
                            "name": "ConfirmOTPCodeResponse",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": null
                                },
                                {
                                    "_type": "ObjectConditionalSpreadSelection",
                                    "object": "OTPToken",
                                    "spec": {
                                        "_type": "ObjectFragmentSpec",
                                        "name": "OTPToken",
                                        "selections": [
                                            {
                                                "_type": "FieldSelection",
                                                "name": "token",
                                                "alias": "token",
                                                "arguments": {},
                                                "selection": null
                                            }
                                        ]
                                    }
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "mutation ConfirmOTPCode($email: String!, $code: String!) { confirmOTPCode(email: $email, code: $code) { __typename ... on OTPToken { token } }}",
            "parametersHash": 844345817621798774,
            "fragmentSpecHash": 7971179872168521914
        },
        "ConfirmUser": {
            "name": "ConfirmUser",
            "type": "MUTATION",
            "parameters": {
                "$password": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "String"
                        },
                        "defaultValue": null
                    }
                },
                "$token": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "String"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Mutation",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "confirmUser",
                        "alias": "confirmUser",
                        "arguments": {
                            "password": {
                                "name": "password",
                                "value": {
                                    "_type": "ref",
                                    "name": "$password"
                                }
                            },
                            "token": {
                                "name": "token",
                                "value": {
                                    "_type": "ref",
                                    "name": "$token"
                                }
                            }
                        },
                        "selection": {
                            "_type": "UnionFragmentSpec",
                            "name": "ConfirmUserError",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": "error"
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "mutation ConfirmUser($token: String!, $password: String!) { confirmUser(token: $token, password: $password) { error: __typename }}",
            "parametersHash": 16612246897318325471,
            "fragmentSpecHash": 11385119505126879251
        },
        "CreateGroup": {
            "name": "CreateGroup",
            "type": "MUTATION",
            "parameters": {
                "$groupIn": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "InputType",
                            "name": "GroupIn",
                            "$ref": "#/server/inputs/GroupIn"
                        },
                        "defaultValue": null
                    }
                },
                "$userIds": {
                    "nullable": false,
                    "spec": {
                        "_type": "array",
                        "nullable": false,
                        "type": {
                            "_type": "literal",
                            "type": {
                                "_type": "Scalar",
                                "name": "UUID"
                            },
                            "defaultValue": null
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Mutation",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "createGroup",
                        "alias": "error",
                        "arguments": {
                            "groupIn": {
                                "name": "groupIn",
                                "value": {
                                    "_type": "ref",
                                    "name": "$groupIn"
                                }
                            },
                            "userIds": {
                                "name": "userIds",
                                "value": {
                                    "_type": "ref",
                                    "name": "$userIds"
                                }
                            }
                        },
                        "selection": {
                            "_type": "UnionFragmentSpec",
                            "name": "CreateGroupError",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": null
                                },
                                {
                                    "_type": "ObjectConditionalSpreadSelection",
                                    "object": "ErrorUnknownTags",
                                    "spec": {
                                        "_type": "ObjectFragmentSpec",
                                        "name": "ErrorUnknownTags",
                                        "selections": [
                                            {
                                                "_type": "FieldSelection",
                                                "name": "tagIds",
                                                "alias": "tagIds",
                                                "arguments": {},
                                                "selection": null
                                            }
                                        ]
                                    }
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "mutation CreateGroup($groupIn: GroupIn!, $userIds: [UUID!]!) { error: createGroup(groupIn: $groupIn, userIds: $userIds) { __typename ... on ErrorUnknownTags { tagIds } }}",
            "parametersHash": 17939013936943830801,
            "fragmentSpecHash": 499813109395453037
        },
        "CreateMultipartFileSession": {
            "name": "CreateMultipartFileSession",
            "type": "MUTATION",
            "parameters": {
                "$fileIn": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "InputType",
                            "name": "MultipartUploadFileIn",
                            "$ref": "#/server/inputs/MultipartUploadFileIn"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Mutation",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "createMultipartFileSession",
                        "alias": "session",
                        "arguments": {
                            "fileIn": {
                                "name": "fileIn",
                                "value": {
                                    "_type": "ref",
                                    "name": "$fileIn"
                                }
                            }
                        },
                        "selection": {
                            "_type": "UnionFragmentSpec",
                            "name": "CreateMultipartFileSessionResponse",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": null
                                },
                                {
                                    "_type": "ObjectConditionalSpreadSelection",
                                    "object": "MultipartUploadSession",
                                    "spec": {
                                        "_type": "ObjectFragmentSpec",
                                        "name": "MultipartUploadSession",
                                        "selections": [
                                            {
                                                "_type": "SpreadSelection",
                                                "fragment": "MultipartUploadSession"
                                            }
                                        ]
                                    }
                                },
                                {
                                    "_type": "ObjectConditionalSpreadSelection",
                                    "object": "ErrorUnknownTags",
                                    "spec": {
                                        "_type": "ObjectFragmentSpec",
                                        "name": "ErrorUnknownTags",
                                        "selections": [
                                            {
                                                "_type": "SpreadSelection",
                                                "fragment": "ErrorUnknownTags"
                                            }
                                        ]
                                    }
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "mutation CreateMultipartFileSession($fileIn: MultipartUploadFileIn!) { session: createMultipartFileSession(fileIn: $fileIn) { __typename ... on MultipartUploadSession { ...MultipartUploadSession } ... on ErrorUnknownTags { ...ErrorUnknownTags } }}",
            "parametersHash": 335996733556532796,
            "fragmentSpecHash": 16395293096475142449
        },
        "CreatePutFileSession": {
            "name": "CreatePutFileSession",
            "type": "MUTATION",
            "parameters": {
                "$fileIn": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "InputType",
                            "name": "PutUploadFileIn",
                            "$ref": "#/server/inputs/PutUploadFileIn"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Mutation",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "createPutFileSession",
                        "alias": "session",
                        "arguments": {
                            "fileIn": {
                                "name": "fileIn",
                                "value": {
                                    "_type": "ref",
                                    "name": "$fileIn"
                                }
                            }
                        },
                        "selection": {
                            "_type": "UnionFragmentSpec",
                            "name": "CreatePutFileSessionResponse",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": null
                                },
                                {
                                    "_type": "ObjectConditionalSpreadSelection",
                                    "object": "PutUploadSession",
                                    "spec": {
                                        "_type": "ObjectFragmentSpec",
                                        "name": "PutUploadSession",
                                        "selections": [
                                            {
                                                "_type": "SpreadSelection",
                                                "fragment": "PutUploadSession"
                                            }
                                        ]
                                    }
                                },
                                {
                                    "_type": "ObjectConditionalSpreadSelection",
                                    "object": "ErrorUnknownTags",
                                    "spec": {
                                        "_type": "ObjectFragmentSpec",
                                        "name": "ErrorUnknownTags",
                                        "selections": [
                                            {
                                                "_type": "SpreadSelection",
                                                "fragment": "ErrorUnknownTags"
                                            }
                                        ]
                                    }
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "mutation CreatePutFileSession($fileIn: PutUploadFileIn!) { session: createPutFileSession(fileIn: $fileIn) { __typename ... on PutUploadSession { ...PutUploadSession } ... on ErrorUnknownTags { ...ErrorUnknownTags } }}",
            "parametersHash": 1456387490085069540,
            "fragmentSpecHash": 2314212201744212249
        },
        "CreateTag": {
            "name": "CreateTag",
            "type": "MUTATION",
            "parameters": {
                "$tag": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "InputType",
                            "name": "TagIn",
                            "$ref": "#/server/inputs/TagIn"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Mutation",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "createTag",
                        "alias": "error",
                        "arguments": {
                            "tag": {
                                "name": "tag",
                                "value": {
                                    "_type": "ref",
                                    "name": "$tag"
                                }
                            }
                        },
                        "selection": {
                            "_type": "UnionFragmentSpec",
                            "name": "CreateTagError",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": null
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "mutation CreateTag($tag: TagIn!) { error: createTag(tag: $tag) { __typename }}",
            "parametersHash": 7354411961644584575,
            "fragmentSpecHash": 15715392422010946559
        },
        "CreateUser": {
            "name": "CreateUser",
            "type": "MUTATION",
            "parameters": {
                "$userIn": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "InputType",
                            "name": "UserIn",
                            "$ref": "#/server/inputs/UserIn"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Mutation",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "createUser",
                        "alias": "createUser",
                        "arguments": {
                            "userIn": {
                                "name": "userIn",
                                "value": {
                                    "_type": "ref",
                                    "name": "$userIn"
                                }
                            }
                        },
                        "selection": {
                            "_type": "UnionFragmentSpec",
                            "name": "CreateUserError",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": "error"
                                },
                                {
                                    "_type": "ObjectConditionalSpreadSelection",
                                    "object": "ErrorUnknownGroups",
                                    "spec": {
                                        "_type": "ObjectFragmentSpec",
                                        "name": "ErrorUnknownGroups",
                                        "selections": [
                                            {
                                                "_type": "FieldSelection",
                                                "name": "groupIds",
                                                "alias": "groupIds",
                                                "arguments": {},
                                                "selection": null
                                            }
                                        ]
                                    }
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "mutation CreateUser($userIn: UserIn!) { createUser(userIn: $userIn) { error: __typename ... on ErrorUnknownGroups { groupIds } }}",
            "parametersHash": 3045789087024618670,
            "fragmentSpecHash": 7696275157430132236
        },
        "DecideOnDownloadRequest": {
            "name": "DecideOnDownloadRequest",
            "type": "MUTATION",
            "parameters": {
                "$allowed": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "Boolean"
                        },
                        "defaultValue": null
                    }
                },
                "$fileId": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "UUID"
                        },
                        "defaultValue": null
                    }
                },
                "$userId": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "UUID"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Mutation",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "decideOnDownloadRequest",
                        "alias": "error",
                        "arguments": {
                            "allowed": {
                                "name": "allowed",
                                "value": {
                                    "_type": "ref",
                                    "name": "$allowed"
                                }
                            },
                            "fileId": {
                                "name": "fileId",
                                "value": {
                                    "_type": "ref",
                                    "name": "$fileId"
                                }
                            },
                            "userId": {
                                "name": "userId",
                                "value": {
                                    "_type": "ref",
                                    "name": "$userId"
                                }
                            }
                        },
                        "selection": {
                            "_type": "UnionFragmentSpec",
                            "name": "DecideOnDownloadRequestError",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": null
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "mutation DecideOnDownloadRequest( $fileId: UUID!, $userId: UUID!, $allowed: Boolean!,) { error: decideOnDownloadRequest( fileId: $fileId, userId: $userId, allowed: $allowed, ) { __typename }}",
            "parametersHash": 17096297780655773730,
            "fragmentSpecHash": 6442321403991783826
        },
        "DeleteFile": {
            "name": "DeleteFile",
            "type": "MUTATION",
            "parameters": {
                "$id": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "UUID"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Mutation",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "deleteFile",
                        "alias": "error",
                        "arguments": {
                            "id": {
                                "name": "id",
                                "value": {
                                    "_type": "ref",
                                    "name": "$id"
                                }
                            }
                        },
                        "selection": {
                            "_type": "UnionFragmentSpec",
                            "name": "DeleteFileError",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": null
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "mutation DeleteFile($id: UUID!) { error: deleteFile(id: $id) { __typename }}",
            "parametersHash": 6071217685901131044,
            "fragmentSpecHash": 5908735227432555409
        },
        "DeleteFiles": {
            "name": "DeleteFiles",
            "type": "MUTATION",
            "parameters": {
                "$ids": {
                    "nullable": false,
                    "spec": {
                        "_type": "array",
                        "nullable": false,
                        "type": {
                            "_type": "literal",
                            "type": {
                                "_type": "Scalar",
                                "name": "UUID"
                            },
                            "defaultValue": null
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Mutation",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "deleteFiles",
                        "alias": "error",
                        "arguments": {
                            "ids": {
                                "name": "ids",
                                "value": {
                                    "_type": "ref",
                                    "name": "$ids"
                                }
                            }
                        },
                        "selection": {
                            "_type": "UnionFragmentSpec",
                            "name": "DeleteFilesError",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": null
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "mutation DeleteFiles($ids: [UUID!]!) { error: deleteFiles(ids: $ids) { __typename }}",
            "parametersHash": 7346005225015112690,
            "fragmentSpecHash": 12250631224202846524
        },
        "DeleteGroup": {
            "name": "DeleteGroup",
            "type": "MUTATION",
            "parameters": {
                "$id": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "UUID"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Mutation",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "deleteGroup",
                        "alias": "error",
                        "arguments": {
                            "id": {
                                "name": "id",
                                "value": {
                                    "_type": "ref",
                                    "name": "$id"
                                }
                            }
                        },
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "ErrorGroupNotFound",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": null
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "mutation DeleteGroup($id: UUID!) { error: deleteGroup(id: $id) { __typename }}",
            "parametersHash": 6071217685901131044,
            "fragmentSpecHash": 23967248466151464
        },
        "DeletePendingUser": {
            "name": "DeletePendingUser",
            "type": "MUTATION",
            "parameters": {
                "$email": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "String"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Mutation",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "deletePendingUser",
                        "alias": "error",
                        "arguments": {
                            "email": {
                                "name": "email",
                                "value": {
                                    "_type": "ref",
                                    "name": "$email"
                                }
                            }
                        },
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "ErrorNotFound",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": null
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "mutation DeletePendingUser($email: String!) { error: deletePendingUser(email: $email) { __typename }}",
            "parametersHash": 735258917282373466,
            "fragmentSpecHash": 6538470201039468940
        },
        "DeleteTag": {
            "name": "DeleteTag",
            "type": "MUTATION",
            "parameters": {
                "$id": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "UUID"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Mutation",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "deleteTag",
                        "alias": "error",
                        "arguments": {
                            "id": {
                                "name": "id",
                                "value": {
                                    "_type": "ref",
                                    "name": "$id"
                                }
                            }
                        },
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "ErrorNotFound",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": null
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "mutation DeleteTag($id: UUID!) { error: deleteTag(id: $id) { __typename }}",
            "parametersHash": 6071217685901131044,
            "fragmentSpecHash": 6800263551490809385
        },
        "DeleteUser": {
            "name": "DeleteUser",
            "type": "MUTATION",
            "parameters": {
                "$id": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "UUID"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Mutation",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "deleteUser",
                        "alias": "error",
                        "arguments": {
                            "id": {
                                "name": "id",
                                "value": {
                                    "_type": "ref",
                                    "name": "$id"
                                }
                            }
                        },
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "ErrorNotFound",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": null
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "mutation DeleteUser($id: UUID!) { error: deleteUser(id: $id) { __typename }}",
            "parametersHash": 6071217685901131044,
            "fragmentSpecHash": 1471820897431011025
        },
        "EditGroup": {
            "name": "EditGroup",
            "type": "MUTATION",
            "parameters": {
                "$groupIn": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "InputType",
                            "name": "GroupIn",
                            "$ref": "#/server/inputs/GroupIn"
                        },
                        "defaultValue": null
                    }
                },
                "$id": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "UUID"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Mutation",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "editGroup",
                        "alias": "error",
                        "arguments": {
                            "groupIn": {
                                "name": "groupIn",
                                "value": {
                                    "_type": "ref",
                                    "name": "$groupIn"
                                }
                            },
                            "id": {
                                "name": "id",
                                "value": {
                                    "_type": "ref",
                                    "name": "$id"
                                }
                            }
                        },
                        "selection": {
                            "_type": "UnionFragmentSpec",
                            "name": "EditGroupError",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": null
                                },
                                {
                                    "_type": "ObjectConditionalSpreadSelection",
                                    "object": "ErrorUnknownTags",
                                    "spec": {
                                        "_type": "ObjectFragmentSpec",
                                        "name": "ErrorUnknownTags",
                                        "selections": [
                                            {
                                                "_type": "FieldSelection",
                                                "name": "tagIds",
                                                "alias": "tagIds",
                                                "arguments": {},
                                                "selection": null
                                            }
                                        ]
                                    }
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "mutation EditGroup($id: UUID!, $groupIn: GroupIn!) { error: editGroup(id: $id, groupIn: $groupIn) { __typename ... on ErrorUnknownTags { tagIds } }}",
            "parametersHash": 11671618869723862284,
            "fragmentSpecHash": 7779389709270117563
        },
        "EditTag": {
            "name": "EditTag",
            "type": "MUTATION",
            "parameters": {
                "$id": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "UUID"
                        },
                        "defaultValue": null
                    }
                },
                "$tag": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "InputType",
                            "name": "TagIn",
                            "$ref": "#/server/inputs/TagIn"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Mutation",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "editTag",
                        "alias": "error",
                        "arguments": {
                            "id": {
                                "name": "id",
                                "value": {
                                    "_type": "ref",
                                    "name": "$id"
                                }
                            },
                            "tag": {
                                "name": "tag",
                                "value": {
                                    "_type": "ref",
                                    "name": "$tag"
                                }
                            }
                        },
                        "selection": {
                            "_type": "UnionFragmentSpec",
                            "name": "EditTagError",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": null
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "mutation EditTag($id: UUID!, $tag: TagIn!) { error: editTag(id: $id, tag: $tag) { __typename }}",
            "parametersHash": 4246196709300380361,
            "fragmentSpecHash": 14377991953512099708
        },
        "GetDealColumns": {
            "name": "GetDealColumns",
            "type": "QUERY",
            "parameters": {},
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Query",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "getDealColumns",
                        "alias": "getDealColumns",
                        "arguments": {},
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "DealColumn",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "DealColumn"
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "query GetDealColumns { getDealColumns { ...DealColumn }}",
            "parametersHash": 15130871412783076140,
            "fragmentSpecHash": 11959417340459385881
        },
        "GetDealInfo": {
            "name": "GetDealInfo",
            "type": "QUERY",
            "parameters": {
                "$dealName": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "String"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Query",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "getDealInfo",
                        "alias": "info",
                        "arguments": {
                            "dealName": {
                                "name": "dealName",
                                "value": {
                                    "_type": "ref",
                                    "name": "$dealName"
                                }
                            }
                        },
                        "selection": {
                            "_type": "UnionFragmentSpec",
                            "name": "GetDealInfoResponse",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": null
                                },
                                {
                                    "_type": "ObjectConditionalSpreadSelection",
                                    "object": "DealInfo",
                                    "spec": {
                                        "_type": "ObjectFragmentSpec",
                                        "name": "DealInfo",
                                        "selections": [
                                            {
                                                "_type": "SpreadSelection",
                                                "fragment": "DealInfo"
                                            }
                                        ]
                                    }
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "query GetDealInfo($dealName: String!){ info: getDealInfo(dealName: $dealName) { __typename ... on DealInfo { ...DealInfo } }}",
            "parametersHash": 13840027003433641761,
            "fragmentSpecHash": 15624099676455048432
        },
        "GetDeals": {
            "name": "GetDeals",
            "type": "QUERY",
            "parameters": {
                "$limit": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "Int"
                        },
                        "defaultValue": null
                    }
                },
                "$query": {
                    "nullable": true,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "String"
                        },
                        "defaultValue": null
                    }
                },
                "$skip": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "Int"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Query",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "getDeals",
                        "alias": "deals",
                        "arguments": {
                            "limit": {
                                "name": "limit",
                                "value": {
                                    "_type": "ref",
                                    "name": "$limit"
                                }
                            },
                            "query": {
                                "name": "query",
                                "value": {
                                    "_type": "ref",
                                    "name": "$query"
                                }
                            },
                            "skip": {
                                "name": "skip",
                                "value": {
                                    "_type": "ref",
                                    "name": "$skip"
                                }
                            }
                        },
                        "selection": null
                    }
                ]
            },
            "sourceText": "query GetDeals($skip: Int!, $limit: Int!, $query: String){ deals: getDeals(skip: $skip, limit: $limit, query: $query)}",
            "parametersHash": 4256777157633870176,
            "fragmentSpecHash": 7145625091949894964
        },
        "GetEvents": {
            "name": "GetEvents",
            "type": "QUERY",
            "parameters": {
                "$dateRange": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "InputType",
                            "name": "DateRange",
                            "$ref": "#/server/inputs/DateRange"
                        },
                        "defaultValue": null
                    }
                },
                "$filters": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "InputType",
                            "name": "EventFiltersIn",
                            "$ref": "#/server/inputs/EventFiltersIn"
                        },
                        "defaultValue": null
                    }
                },
                "$limit": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "Int"
                        },
                        "defaultValue": null
                    }
                },
                "$query": {
                    "nullable": true,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "String"
                        },
                        "defaultValue": null
                    }
                },
                "$skip": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "Int"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Query",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "getEvents",
                        "alias": "events",
                        "arguments": {
                            "dateRange": {
                                "name": "dateRange",
                                "value": {
                                    "_type": "ref",
                                    "name": "$dateRange"
                                }
                            },
                            "filters": {
                                "name": "filters",
                                "value": {
                                    "_type": "ref",
                                    "name": "$filters"
                                }
                            },
                            "limit": {
                                "name": "limit",
                                "value": {
                                    "_type": "ref",
                                    "name": "$limit"
                                }
                            },
                            "query": {
                                "name": "query",
                                "value": {
                                    "_type": "ref",
                                    "name": "$query"
                                }
                            },
                            "skip": {
                                "name": "skip",
                                "value": {
                                    "_type": "ref",
                                    "name": "$skip"
                                }
                            }
                        },
                        "selection": {
                            "_type": "UnionFragmentSpec",
                            "name": "GetEventsResponse",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": null
                                },
                                {
                                    "_type": "ObjectConditionalSpreadSelection",
                                    "object": "EventsList",
                                    "spec": {
                                        "_type": "ObjectFragmentSpec",
                                        "name": "EventsList",
                                        "selections": [
                                            {
                                                "_type": "FieldSelection",
                                                "name": "events",
                                                "alias": "events",
                                                "arguments": {},
                                                "selection": {
                                                    "_type": "UnionFragmentSpec",
                                                    "name": "Event",
                                                    "selections": [
                                                        {
                                                            "_type": "SpreadSelection",
                                                            "fragment": "Event"
                                                        }
                                                    ]
                                                }
                                            }
                                        ]
                                    }
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "query GetEvents( $skip: Int!, $limit: Int!, $dateRange: DateRange!, $filters: EventFiltersIn!, $query: String) { events: getEvents( skip: $skip, limit: $limit, dateRange: $dateRange, filters: $filters, query: $query, ) { __typename ... on EventsList { events { ...Event } } }}",
            "parametersHash": 2459577102124344271,
            "fragmentSpecHash": 13823890097572535246
        },
        "GetFavouriteTags": {
            "name": "GetFavouriteTags",
            "type": "QUERY",
            "parameters": {
                "$limit": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "Int"
                        },
                        "defaultValue": null
                    }
                },
                "$skip": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "Int"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Query",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "getFavouriteTags",
                        "alias": "data",
                        "arguments": {
                            "limit": {
                                "name": "limit",
                                "value": {
                                    "_type": "ref",
                                    "name": "$limit"
                                }
                            },
                            "skip": {
                                "name": "skip",
                                "value": {
                                    "_type": "ref",
                                    "name": "$skip"
                                }
                            }
                        },
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "Tag",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "Tag"
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "query GetFavouriteTags($skip: Int!, $limit: Int!) { data: getFavouriteTags(skip: $skip, limit: $limit) { ...Tag }}",
            "parametersHash": 11980981446252579325,
            "fragmentSpecHash": 6576658916172265759
        },
        "GetFileURL": {
            "name": "GetFileURL",
            "type": "QUERY",
            "parameters": {
                "$id": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "UUID"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Query",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "getFileURL",
                        "alias": "url",
                        "arguments": {
                            "id": {
                                "name": "id",
                                "value": {
                                    "_type": "ref",
                                    "name": "$id"
                                }
                            }
                        },
                        "selection": {
                            "_type": "UnionFragmentSpec",
                            "name": "GetFileURLResponse",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": null
                                },
                                {
                                    "_type": "ObjectConditionalSpreadSelection",
                                    "object": "UrlObject",
                                    "spec": {
                                        "_type": "ObjectFragmentSpec",
                                        "name": "UrlObject",
                                        "selections": [
                                            {
                                                "_type": "FieldSelection",
                                                "name": "uvalue",
                                                "alias": "uvalue",
                                                "arguments": {},
                                                "selection": null
                                            }
                                        ]
                                    }
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "query GetFileURL($id: UUID!) { url: getFileURL(id: $id) { __typename ... on UrlObject { uvalue } }}",
            "parametersHash": 6071217685901131044,
            "fragmentSpecHash": 12370122869700993613
        },
        "GetFiles": {
            "name": "GetFiles",
            "type": "QUERY",
            "parameters": {
                "$filters": {
                    "nullable": false,
                    "spec": {
                        "_type": "array",
                        "nullable": false,
                        "type": {
                            "_type": "literal",
                            "type": {
                                "_type": "InputType",
                                "name": "Filter",
                                "$ref": "#/server/inputs/Filter"
                            },
                            "defaultValue": null
                        },
                        "defaultValue": null
                    }
                },
                "$limit": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "Int"
                        },
                        "defaultValue": null
                    }
                },
                "$skip": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "Int"
                        },
                        "defaultValue": null
                    }
                },
                "$sortBy": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "InputType",
                            "name": "FileSortBy",
                            "$ref": "#/server/inputs/FileSortBy"
                        },
                        "defaultValue": null
                    }
                },
                "$tagIds": {
                    "nullable": false,
                    "spec": {
                        "_type": "array",
                        "nullable": false,
                        "type": {
                            "_type": "literal",
                            "type": {
                                "_type": "Scalar",
                                "name": "UUID"
                            },
                            "defaultValue": null
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Query",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "getFiles",
                        "alias": "data",
                        "arguments": {
                            "filters": {
                                "name": "filters",
                                "value": {
                                    "_type": "ref",
                                    "name": "$filters"
                                }
                            },
                            "limit": {
                                "name": "limit",
                                "value": {
                                    "_type": "ref",
                                    "name": "$limit"
                                }
                            },
                            "skip": {
                                "name": "skip",
                                "value": {
                                    "_type": "ref",
                                    "name": "$skip"
                                }
                            },
                            "sortBy": {
                                "name": "sortBy",
                                "value": {
                                    "_type": "ref",
                                    "name": "$sortBy"
                                }
                            },
                            "tagIds": {
                                "name": "tagIds",
                                "value": {
                                    "_type": "ref",
                                    "name": "$tagIds"
                                }
                            }
                        },
                        "selection": {
                            "_type": "UnionFragmentSpec",
                            "name": "GetFilesResponse",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": null
                                },
                                {
                                    "_type": "ObjectConditionalSpreadSelection",
                                    "object": "ErrorUnknownTags",
                                    "spec": {
                                        "_type": "ObjectFragmentSpec",
                                        "name": "ErrorUnknownTags",
                                        "selections": [
                                            {
                                                "_type": "SpreadSelection",
                                                "fragment": "ErrorUnknownTags"
                                            }
                                        ]
                                    }
                                },
                                {
                                    "_type": "ObjectConditionalSpreadSelection",
                                    "object": "SearchFileList",
                                    "spec": {
                                        "_type": "ObjectFragmentSpec",
                                        "name": "SearchFileList",
                                        "selections": [
                                            {
                                                "_type": "SpreadSelection",
                                                "fragment": "SearchFileList"
                                            }
                                        ]
                                    }
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "query GetFiles( $skip: Int!, $limit: Int!, $filters: [Filter!]!, $tagIds: [UUID!]!, $sortBy: FileSortBy!) { data: getFiles( skip: $skip, limit: $limit, tagIds: $tagIds, sortBy: $sortBy, filters: $filters, ) { __typename ... on ErrorUnknownTags { ...ErrorUnknownTags } ... on SearchFileList { ...SearchFileList } }}",
            "parametersHash": 542874794276788661,
            "fragmentSpecHash": 13844436972255274373
        },
        "GetFilesCount": {
            "name": "GetFilesCount",
            "type": "QUERY",
            "parameters": {
                "$filters": {
                    "nullable": false,
                    "spec": {
                        "_type": "array",
                        "nullable": false,
                        "type": {
                            "_type": "literal",
                            "type": {
                                "_type": "InputType",
                                "name": "Filter",
                                "$ref": "#/server/inputs/Filter"
                            },
                            "defaultValue": null
                        },
                        "defaultValue": null
                    }
                },
                "$tagIds": {
                    "nullable": false,
                    "spec": {
                        "_type": "array",
                        "nullable": false,
                        "type": {
                            "_type": "literal",
                            "type": {
                                "_type": "Scalar",
                                "name": "UUID"
                            },
                            "defaultValue": null
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Query",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "getFilesCount",
                        "alias": "count",
                        "arguments": {
                            "filters": {
                                "name": "filters",
                                "value": {
                                    "_type": "ref",
                                    "name": "$filters"
                                }
                            },
                            "tagIds": {
                                "name": "tagIds",
                                "value": {
                                    "_type": "ref",
                                    "name": "$tagIds"
                                }
                            }
                        },
                        "selection": {
                            "_type": "UnionFragmentSpec",
                            "name": "IntObjectOrErrorUnknownTags",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": null
                                },
                                {
                                    "_type": "ObjectConditionalSpreadSelection",
                                    "object": "ErrorUnknownTags",
                                    "spec": {
                                        "_type": "ObjectFragmentSpec",
                                        "name": "ErrorUnknownTags",
                                        "selections": [
                                            {
                                                "_type": "SpreadSelection",
                                                "fragment": "ErrorUnknownTags"
                                            }
                                        ]
                                    }
                                },
                                {
                                    "_type": "ObjectConditionalSpreadSelection",
                                    "object": "IntObject",
                                    "spec": {
                                        "_type": "ObjectFragmentSpec",
                                        "name": "IntObject",
                                        "selections": [
                                            {
                                                "_type": "SpreadSelection",
                                                "fragment": "IntObject"
                                            }
                                        ]
                                    }
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "query GetFilesCount($tagIds: [UUID!]!, $filters: [Filter!]!) { count: getFilesCount(tagIds: $tagIds, filters: $filters) { __typename ... on ErrorUnknownTags { ...ErrorUnknownTags } ... on IntObject { ...IntObject } }}",
            "parametersHash": 6419628783901775267,
            "fragmentSpecHash": 3200777033864579866
        },
        "GetFilesDealInfo": {
            "name": "GetFilesDealInfo",
            "type": "QUERY",
            "parameters": {
                "$fileIds": {
                    "nullable": false,
                    "spec": {
                        "_type": "array",
                        "nullable": false,
                        "type": {
                            "_type": "literal",
                            "type": {
                                "_type": "Scalar",
                                "name": "UUID"
                            },
                            "defaultValue": null
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Query",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "getFilesDealInfo",
                        "alias": "getFilesDealInfo",
                        "arguments": {
                            "fileIds": {
                                "name": "fileIds",
                                "value": {
                                    "_type": "ref",
                                    "name": "$fileIds"
                                }
                            }
                        },
                        "selection": {
                            "_type": "UnionFragmentSpec",
                            "name": "FilesDealInfoOrError",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": null
                                },
                                {
                                    "_type": "ObjectConditionalSpreadSelection",
                                    "object": "FilesDealInfo",
                                    "spec": {
                                        "_type": "ObjectFragmentSpec",
                                        "name": "FilesDealInfo",
                                        "selections": [
                                            {
                                                "_type": "SpreadSelection",
                                                "fragment": "FilesDealInfo"
                                            }
                                        ]
                                    }
                                },
                                {
                                    "_type": "ObjectConditionalSpreadSelection",
                                    "object": "ErrorCantAddAutotags",
                                    "spec": {
                                        "_type": "ObjectFragmentSpec",
                                        "name": "ErrorCantAddAutotags",
                                        "selections": [
                                            {
                                                "_type": "FieldSelection",
                                                "name": "a",
                                                "alias": "a",
                                                "arguments": {},
                                                "selection": null
                                            }
                                        ]
                                    }
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "query GetFilesDealInfo($fileIds: [UUID!]!) { getFilesDealInfo(fileIds: $fileIds) { __typename ... on FilesDealInfo { ...FilesDealInfo } ... on ErrorCantAddAutotags { a } }}",
            "parametersHash": 8625588151194238414,
            "fragmentSpecHash": 15949742031496296798
        },
        "GetGroupTags": {
            "name": "GetGroupTags",
            "type": "QUERY",
            "parameters": {
                "$id": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "UUID"
                        },
                        "defaultValue": null
                    }
                },
                "$limit": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "Int"
                        },
                        "defaultValue": null
                    }
                },
                "$skip": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "Int"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Query",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "getGroupTags",
                        "alias": "tags",
                        "arguments": {
                            "id": {
                                "name": "id",
                                "value": {
                                    "_type": "ref",
                                    "name": "$id"
                                }
                            },
                            "limit": {
                                "name": "limit",
                                "value": {
                                    "_type": "ref",
                                    "name": "$limit"
                                }
                            },
                            "skip": {
                                "name": "skip",
                                "value": {
                                    "_type": "ref",
                                    "name": "$skip"
                                }
                            }
                        },
                        "selection": {
                            "_type": "UnionFragmentSpec",
                            "name": "GetGroupTagsResponse",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": null
                                },
                                {
                                    "_type": "ObjectConditionalSpreadSelection",
                                    "object": "TagList",
                                    "spec": {
                                        "_type": "ObjectFragmentSpec",
                                        "name": "TagList",
                                        "selections": [
                                            {
                                                "_type": "FieldSelection",
                                                "name": "list",
                                                "alias": "list",
                                                "arguments": {},
                                                "selection": {
                                                    "_type": "ObjectFragmentSpec",
                                                    "name": "Tag",
                                                    "selections": [
                                                        {
                                                            "_type": "SpreadSelection",
                                                            "fragment": "Tag"
                                                        }
                                                    ]
                                                }
                                            }
                                        ]
                                    }
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "query GetGroupTags($id: UUID!, $skip: Int!, $limit: Int!) { tags: getGroupTags(id: $id, skip: $skip, limit: $limit) { __typename ... on TagList { list { ...Tag } } }}",
            "parametersHash": 6454552463448398754,
            "fragmentSpecHash": 16182391764401389224
        },
        "GetGroupUsers": {
            "name": "GetGroupUsers",
            "type": "QUERY",
            "parameters": {
                "$groupId": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "UUID"
                        },
                        "defaultValue": null
                    }
                },
                "$limit": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "Int"
                        },
                        "defaultValue": null
                    }
                },
                "$skip": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "Int"
                        },
                        "defaultValue": null
                    }
                },
                "$sortBy": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "InputType",
                            "name": "GetGroupUsersSortBy",
                            "$ref": "#/server/inputs/GetGroupUsersSortBy"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Query",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "getGroupUsers",
                        "alias": "users",
                        "arguments": {
                            "groupId": {
                                "name": "groupId",
                                "value": {
                                    "_type": "ref",
                                    "name": "$groupId"
                                }
                            },
                            "limit": {
                                "name": "limit",
                                "value": {
                                    "_type": "ref",
                                    "name": "$limit"
                                }
                            },
                            "skip": {
                                "name": "skip",
                                "value": {
                                    "_type": "ref",
                                    "name": "$skip"
                                }
                            },
                            "sortBy": {
                                "name": "sortBy",
                                "value": {
                                    "_type": "ref",
                                    "name": "$sortBy"
                                }
                            }
                        },
                        "selection": {
                            "_type": "UnionFragmentSpec",
                            "name": "GetGroupUsersResponse",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": null
                                },
                                {
                                    "_type": "ObjectConditionalSpreadSelection",
                                    "object": "UsersList",
                                    "spec": {
                                        "_type": "ObjectFragmentSpec",
                                        "name": "UsersList",
                                        "selections": [
                                            {
                                                "_type": "SpreadSelection",
                                                "fragment": "UsersList"
                                            }
                                        ]
                                    }
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "query GetGroupUsers( $groupId: UUID!, $skip: Int!, $limit: Int!, $sortBy: GetGroupUsersSortBy!) { users: getGroupUsers( groupId: $groupId, skip: $skip, limit: $limit, sortBy: $sortBy ) { __typename ... on UsersList { ...UsersList } }}",
            "parametersHash": 13540147684277004684,
            "fragmentSpecHash": 7956749287696150949
        },
        "GetGroupUsersAndTotal": {
            "name": "GetGroupUsersAndTotal",
            "type": "QUERY",
            "parameters": {
                "$groupId": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "UUID"
                        },
                        "defaultValue": null
                    }
                },
                "$limit": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "Int"
                        },
                        "defaultValue": null
                    }
                },
                "$skip": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "Int"
                        },
                        "defaultValue": null
                    }
                },
                "$sortBy": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "InputType",
                            "name": "GetGroupUsersSortBy",
                            "$ref": "#/server/inputs/GetGroupUsersSortBy"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Query",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "getGroupUsers",
                        "alias": "users",
                        "arguments": {
                            "groupId": {
                                "name": "groupId",
                                "value": {
                                    "_type": "ref",
                                    "name": "$groupId"
                                }
                            },
                            "limit": {
                                "name": "limit",
                                "value": {
                                    "_type": "ref",
                                    "name": "$limit"
                                }
                            },
                            "skip": {
                                "name": "skip",
                                "value": {
                                    "_type": "ref",
                                    "name": "$skip"
                                }
                            },
                            "sortBy": {
                                "name": "sortBy",
                                "value": {
                                    "_type": "ref",
                                    "name": "$sortBy"
                                }
                            }
                        },
                        "selection": {
                            "_type": "UnionFragmentSpec",
                            "name": "GetGroupUsersResponse",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": null
                                },
                                {
                                    "_type": "ObjectConditionalSpreadSelection",
                                    "object": "UsersList",
                                    "spec": {
                                        "_type": "ObjectFragmentSpec",
                                        "name": "UsersList",
                                        "selections": [
                                            {
                                                "_type": "SpreadSelection",
                                                "fragment": "UsersList"
                                            }
                                        ]
                                    }
                                }
                            ]
                        }
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "getGroupUsersTotal",
                        "alias": "total",
                        "arguments": {
                            "groupId": {
                                "name": "groupId",
                                "value": {
                                    "_type": "ref",
                                    "name": "$groupId"
                                }
                            }
                        },
                        "selection": {
                            "_type": "UnionFragmentSpec",
                            "name": "GetGroupUsersTotalResponse",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": null
                                },
                                {
                                    "_type": "ObjectConditionalSpreadSelection",
                                    "object": "IntObject",
                                    "spec": {
                                        "_type": "ObjectFragmentSpec",
                                        "name": "IntObject",
                                        "selections": [
                                            {
                                                "_type": "SpreadSelection",
                                                "fragment": "IntObject"
                                            }
                                        ]
                                    }
                                }
                            ]
                        }
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "retrieveGroup",
                        "alias": "group",
                        "arguments": {
                            "id": {
                                "name": "id",
                                "value": {
                                    "_type": "ref",
                                    "name": "$groupId"
                                }
                            }
                        },
                        "selection": {
                            "_type": "UnionFragmentSpec",
                            "name": "RetrieveGroupResponse",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": null
                                },
                                {
                                    "_type": "ObjectConditionalSpreadSelection",
                                    "object": "Group",
                                    "spec": {
                                        "_type": "ObjectFragmentSpec",
                                        "name": "Group",
                                        "selections": [
                                            {
                                                "_type": "FieldSelection",
                                                "name": "name",
                                                "alias": "name",
                                                "arguments": {},
                                                "selection": null
                                            }
                                        ]
                                    }
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "query GetGroupUsersAndTotal( $groupId: UUID!, $skip: Int!, $limit: Int!, $sortBy: GetGroupUsersSortBy!) { users: getGroupUsers( groupId: $groupId, skip: $skip, limit: $limit, sortBy: $sortBy ) { __typename ... on UsersList { ...UsersList } } total: getGroupUsersTotal(groupId: $groupId) { __typename ... on IntObject { ...IntObject } } group: retrieveGroup(id: $groupId) { __typename ... on Group { name } }}",
            "parametersHash": 13540147684277004684,
            "fragmentSpecHash": 5573978473793283339
        },
        "GetGroupUsersAndUsers": {
            "name": "GetGroupUsersAndUsers",
            "type": "QUERY",
            "parameters": {
                "$groupId": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "UUID"
                        },
                        "defaultValue": null
                    }
                },
                "$limit": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "Int"
                        },
                        "defaultValue": null
                    }
                },
                "$query": {
                    "nullable": true,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "String"
                        },
                        "defaultValue": null
                    }
                },
                "$skip": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "Int"
                        },
                        "defaultValue": null
                    }
                },
                "$sortBy": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "InputType",
                            "name": "GetUsersSortBy",
                            "$ref": "#/server/inputs/GetUsersSortBy"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Query",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "getGroupUsersAndUsers",
                        "alias": "users",
                        "arguments": {
                            "groupId": {
                                "name": "groupId",
                                "value": {
                                    "_type": "ref",
                                    "name": "$groupId"
                                }
                            },
                            "limit": {
                                "name": "limit",
                                "value": {
                                    "_type": "ref",
                                    "name": "$limit"
                                }
                            },
                            "query": {
                                "name": "query",
                                "value": {
                                    "_type": "ref",
                                    "name": "$query"
                                }
                            },
                            "skip": {
                                "name": "skip",
                                "value": {
                                    "_type": "ref",
                                    "name": "$skip"
                                }
                            },
                            "sortBy": {
                                "name": "sortBy",
                                "value": {
                                    "_type": "ref",
                                    "name": "$sortBy"
                                }
                            }
                        },
                        "selection": {
                            "_type": "UnionFragmentSpec",
                            "name": "GetGroupUsersAndUsersResponse",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": null
                                },
                                {
                                    "_type": "ObjectConditionalSpreadSelection",
                                    "object": "GroupUserList",
                                    "spec": {
                                        "_type": "ObjectFragmentSpec",
                                        "name": "GroupUserList",
                                        "selections": [
                                            {
                                                "_type": "SpreadSelection",
                                                "fragment": "GroupUserList"
                                            }
                                        ]
                                    }
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "query GetGroupUsersAndUsers( $skip: Int!, $limit: Int!, $groupId: UUID!, $sortBy: GetUsersSortBy!, $query: String) { users: getGroupUsersAndUsers( skip: $skip, limit: $limit, sortBy: $sortBy, groupId: $groupId, query: $query ) { __typename ... on GroupUserList { ...GroupUserList } }}",
            "parametersHash": 11176446127708286446,
            "fragmentSpecHash": 16881244345725584624
        },
        "GetGroupUsersAndUsersAndTotal": {
            "name": "GetGroupUsersAndUsersAndTotal",
            "type": "QUERY",
            "parameters": {
                "$groupId": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "UUID"
                        },
                        "defaultValue": null
                    }
                },
                "$limit": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "Int"
                        },
                        "defaultValue": null
                    }
                },
                "$query": {
                    "nullable": true,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "String"
                        },
                        "defaultValue": null
                    }
                },
                "$skip": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "Int"
                        },
                        "defaultValue": null
                    }
                },
                "$sortBy": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "InputType",
                            "name": "GetUsersSortBy",
                            "$ref": "#/server/inputs/GetUsersSortBy"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Query",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "getGroupUsersAndUsers",
                        "alias": "users",
                        "arguments": {
                            "groupId": {
                                "name": "groupId",
                                "value": {
                                    "_type": "ref",
                                    "name": "$groupId"
                                }
                            },
                            "limit": {
                                "name": "limit",
                                "value": {
                                    "_type": "ref",
                                    "name": "$limit"
                                }
                            },
                            "query": {
                                "name": "query",
                                "value": {
                                    "_type": "ref",
                                    "name": "$query"
                                }
                            },
                            "skip": {
                                "name": "skip",
                                "value": {
                                    "_type": "ref",
                                    "name": "$skip"
                                }
                            },
                            "sortBy": {
                                "name": "sortBy",
                                "value": {
                                    "_type": "ref",
                                    "name": "$sortBy"
                                }
                            }
                        },
                        "selection": {
                            "_type": "UnionFragmentSpec",
                            "name": "GetGroupUsersAndUsersResponse",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": null
                                },
                                {
                                    "_type": "ObjectConditionalSpreadSelection",
                                    "object": "GroupUserList",
                                    "spec": {
                                        "_type": "ObjectFragmentSpec",
                                        "name": "GroupUserList",
                                        "selections": [
                                            {
                                                "_type": "SpreadSelection",
                                                "fragment": "GroupUserList"
                                            }
                                        ]
                                    }
                                }
                            ]
                        }
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "getUsersTotal",
                        "alias": "total",
                        "arguments": {
                            "query": {
                                "name": "query",
                                "value": {
                                    "_type": "ref",
                                    "name": "$query"
                                }
                            }
                        },
                        "selection": null
                    }
                ]
            },
            "sourceText": "query GetGroupUsersAndUsersAndTotal( $skip: Int!, $limit: Int!, $groupId: UUID!, $sortBy: GetUsersSortBy!, $query: String) { users: getGroupUsersAndUsers( skip: $skip, limit: $limit, sortBy: $sortBy, groupId: $groupId, query: $query ) { __typename ... on GroupUserList { ...GroupUserList } } total: getUsersTotal(query: $query)}",
            "parametersHash": 11176446127708286446,
            "fragmentSpecHash": 8739136137047510803
        },
        "GetGroupUsersTotal": {
            "name": "GetGroupUsersTotal",
            "type": "QUERY",
            "parameters": {
                "$groupId": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "UUID"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Query",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "getGroupUsersTotal",
                        "alias": "total",
                        "arguments": {
                            "groupId": {
                                "name": "groupId",
                                "value": {
                                    "_type": "ref",
                                    "name": "$groupId"
                                }
                            }
                        },
                        "selection": {
                            "_type": "UnionFragmentSpec",
                            "name": "GetGroupUsersTotalResponse",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": null
                                },
                                {
                                    "_type": "ObjectConditionalSpreadSelection",
                                    "object": "IntObject",
                                    "spec": {
                                        "_type": "ObjectFragmentSpec",
                                        "name": "IntObject",
                                        "selections": [
                                            {
                                                "_type": "SpreadSelection",
                                                "fragment": "IntObject"
                                            }
                                        ]
                                    }
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "query GetGroupUsersTotal($groupId: UUID!) { total: getGroupUsersTotal(groupId: $groupId) { __typename ... on IntObject { ...IntObject } }}",
            "parametersHash": 2898351860018605515,
            "fragmentSpecHash": 961238175908812160
        },
        "GetGroups": {
            "name": "GetGroups",
            "type": "QUERY",
            "parameters": {
                "$limit": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "Int"
                        },
                        "defaultValue": null
                    }
                },
                "$skip": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "Int"
                        },
                        "defaultValue": null
                    }
                },
                "$sortBy": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "InputType",
                            "name": "GetGroupsSortBy",
                            "$ref": "#/server/inputs/GetGroupsSortBy"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Query",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "getGroups",
                        "alias": "groups",
                        "arguments": {
                            "limit": {
                                "name": "limit",
                                "value": {
                                    "_type": "ref",
                                    "name": "$limit"
                                }
                            },
                            "skip": {
                                "name": "skip",
                                "value": {
                                    "_type": "ref",
                                    "name": "$skip"
                                }
                            },
                            "sortBy": {
                                "name": "sortBy",
                                "value": {
                                    "_type": "ref",
                                    "name": "$sortBy"
                                }
                            }
                        },
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "Group",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "Group"
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "query GetGroups($skip: Int!, $limit: Int!, $sortBy: GetGroupsSortBy!) { groups: getGroups(skip: $skip, limit: $limit, sortBy: $sortBy) { ...Group }}",
            "parametersHash": 3398569938723492769,
            "fragmentSpecHash": 8776225336384766921
        },
        "GetGroupsAndTotal": {
            "name": "GetGroupsAndTotal",
            "type": "QUERY",
            "parameters": {
                "$limit": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "Int"
                        },
                        "defaultValue": null
                    }
                },
                "$skip": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "Int"
                        },
                        "defaultValue": null
                    }
                },
                "$sortBy": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "InputType",
                            "name": "GetGroupsSortBy",
                            "$ref": "#/server/inputs/GetGroupsSortBy"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Query",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "getGroups",
                        "alias": "groups",
                        "arguments": {
                            "limit": {
                                "name": "limit",
                                "value": {
                                    "_type": "ref",
                                    "name": "$limit"
                                }
                            },
                            "skip": {
                                "name": "skip",
                                "value": {
                                    "_type": "ref",
                                    "name": "$skip"
                                }
                            },
                            "sortBy": {
                                "name": "sortBy",
                                "value": {
                                    "_type": "ref",
                                    "name": "$sortBy"
                                }
                            }
                        },
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "Group",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "Group"
                                }
                            ]
                        }
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "getGroupsTotal",
                        "alias": "total",
                        "arguments": {},
                        "selection": null
                    }
                ]
            },
            "sourceText": "query GetGroupsAndTotal($skip: Int!, $limit: Int!, $sortBy: GetGroupsSortBy!) { groups: getGroups(skip: $skip, limit: $limit, sortBy: $sortBy) { ...Group } total: getGroupsTotal}",
            "parametersHash": 3398569938723492769,
            "fragmentSpecHash": 13638029066960679651
        },
        "GetGroupsTotal": {
            "name": "GetGroupsTotal",
            "type": "QUERY",
            "parameters": {},
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Query",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "getGroupsTotal",
                        "alias": "total",
                        "arguments": {},
                        "selection": null
                    }
                ]
            },
            "sourceText": "query GetGroupsTotal { total: getGroupsTotal}",
            "parametersHash": 15130871412783076140,
            "fragmentSpecHash": 15447805653861973871
        },
        "GetMe": {
            "name": "GetMe",
            "type": "QUERY",
            "parameters": {},
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Query",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "getMe",
                        "alias": "getMe",
                        "arguments": {},
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "User",
                            "selections": [
                                {
                                    "_type": "FieldSelection",
                                    "name": "name",
                                    "alias": "name",
                                    "arguments": {},
                                    "selection": null
                                },
                                {
                                    "_type": "FieldSelection",
                                    "name": "email",
                                    "alias": "email",
                                    "arguments": {},
                                    "selection": null
                                },
                                {
                                    "_type": "FieldSelection",
                                    "name": "tenGroups",
                                    "alias": "tenGroups",
                                    "arguments": {},
                                    "selection": {
                                        "_type": "ObjectFragmentSpec",
                                        "name": "Group",
                                        "selections": [
                                            {
                                                "_type": "FieldSelection",
                                                "name": "name",
                                                "alias": "name",
                                                "arguments": {},
                                                "selection": null
                                            }
                                        ]
                                    }
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "query GetMe { getMe { name email tenGroups { name } }}",
            "parametersHash": 15130871412783076140,
            "fragmentSpecHash": 15568987991688220693
        },
        "GetMyTags": {
            "name": "GetMyTags",
            "type": "QUERY",
            "parameters": {
                "$limit": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "Int"
                        },
                        "defaultValue": null
                    }
                },
                "$skip": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "Int"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Query",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "getMyTags",
                        "alias": "tags",
                        "arguments": {
                            "limit": {
                                "name": "limit",
                                "value": {
                                    "_type": "ref",
                                    "name": "$limit"
                                }
                            },
                            "skip": {
                                "name": "skip",
                                "value": {
                                    "_type": "ref",
                                    "name": "$skip"
                                }
                            }
                        },
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "Tag",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "Tag"
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "query GetMyTags($skip: Int!, $limit: Int!) { tags: getMyTags( skip: $skip, limit: $limit ) { ...Tag }}",
            "parametersHash": 11980981446252579325,
            "fragmentSpecHash": 3050849416166469243
        },
        "GetMyTagsCount": {
            "name": "GetMyTagsCount",
            "type": "QUERY",
            "parameters": {},
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Query",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "getMyTagsCount",
                        "alias": "count",
                        "arguments": {},
                        "selection": null
                    }
                ]
            },
            "sourceText": "query GetMyTagsCount { count: getMyTagsCount}",
            "parametersHash": 15130871412783076140,
            "fragmentSpecHash": 13297633992362554292
        },
        "GetNextMultipartUploadUrls": {
            "name": "GetNextMultipartUploadUrls",
            "type": "QUERY",
            "parameters": {
                "$lastPart": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "Int"
                        },
                        "defaultValue": null
                    }
                },
                "$limit": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "Int"
                        },
                        "defaultValue": null
                    }
                },
                "$sessionId": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "UUID"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Query",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "getNextMultipartUploadUrls",
                        "alias": "getNextMultipartUploadUrls",
                        "arguments": {
                            "lastPart": {
                                "name": "lastPart",
                                "value": {
                                    "_type": "ref",
                                    "name": "$lastPart"
                                }
                            },
                            "limit": {
                                "name": "limit",
                                "value": {
                                    "_type": "ref",
                                    "name": "$limit"
                                }
                            },
                            "sessionId": {
                                "name": "sessionId",
                                "value": {
                                    "_type": "ref",
                                    "name": "$sessionId"
                                }
                            }
                        },
                        "selection": {
                            "_type": "UnionFragmentSpec",
                            "name": "GetNextMultipartUploadUrlsResponse",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": null
                                },
                                {
                                    "_type": "ObjectConditionalSpreadSelection",
                                    "object": "UploadUrlList",
                                    "spec": {
                                        "_type": "ObjectFragmentSpec",
                                        "name": "UploadUrlList",
                                        "selections": [
                                            {
                                                "_type": "FieldSelection",
                                                "name": "urls",
                                                "alias": "urls",
                                                "arguments": {},
                                                "selection": {
                                                    "_type": "ObjectFragmentSpec",
                                                    "name": "UploadUrl",
                                                    "selections": [
                                                        {
                                                            "_type": "SpreadSelection",
                                                            "fragment": "UploadUrl"
                                                        }
                                                    ]
                                                }
                                            }
                                        ]
                                    }
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "query GetNextMultipartUploadUrls( $sessionId: UUID!, $lastPart: Int!, $limit: Int!) { getNextMultipartUploadUrls( sessionId: $sessionId, lastPart: $lastPart, limit: $limit ) { __typename ... on UploadUrlList { urls { ...UploadUrl } } }}",
            "parametersHash": 17313230528166658526,
            "fragmentSpecHash": 6439276821278957187
        },
        "GetPathToTag": {
            "name": "GetPathToTag",
            "type": "QUERY",
            "parameters": {
                "$tagId": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "UUID"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Query",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "getPathToTag",
                        "alias": "getPathToTag",
                        "arguments": {
                            "tagId": {
                                "name": "tagId",
                                "value": {
                                    "_type": "ref",
                                    "name": "$tagId"
                                }
                            }
                        },
                        "selection": {
                            "_type": "UnionFragmentSpec",
                            "name": "GetPathToTagResponse",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": null
                                },
                                {
                                    "_type": "ObjectConditionalSpreadSelection",
                                    "object": "StringList",
                                    "spec": {
                                        "_type": "ObjectFragmentSpec",
                                        "name": "StringList",
                                        "selections": [
                                            {
                                                "_type": "FieldSelection",
                                                "name": "values",
                                                "alias": "values",
                                                "arguments": {},
                                                "selection": null
                                            }
                                        ]
                                    }
                                },
                                {
                                    "_type": "ObjectConditionalSpreadSelection",
                                    "object": "ErrorUnknownTags",
                                    "spec": {
                                        "_type": "ObjectFragmentSpec",
                                        "name": "ErrorUnknownTags",
                                        "selections": [
                                            {
                                                "_type": "SpreadSelection",
                                                "fragment": "ErrorUnknownTags"
                                            }
                                        ]
                                    }
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "query GetPathToTag($tagId: UUID!) { getPathToTag(tagId: $tagId) { __typename ... on StringList { values } ... on ErrorUnknownTags { ...ErrorUnknownTags } }}",
            "parametersHash": 4198591995056155396,
            "fragmentSpecHash": 4197321539444128132
        },
        "GetPendingUsers": {
            "name": "GetPendingUsers",
            "type": "QUERY",
            "parameters": {},
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Query",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "getPendingUsers",
                        "alias": "getPendingUsers",
                        "arguments": {},
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "PendingUser",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "PendingUser"
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "query GetPendingUsers { getPendingUsers { ...PendingUser }}",
            "parametersHash": 15130871412783076140,
            "fragmentSpecHash": 8158582487935591494
        },
        "GetPopularTags": {
            "name": "GetPopularTags",
            "type": "QUERY",
            "parameters": {
                "$limit": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "Int"
                        },
                        "defaultValue": null
                    }
                },
                "$skip": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "Int"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Query",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "getPopularTags",
                        "alias": "data",
                        "arguments": {
                            "limit": {
                                "name": "limit",
                                "value": {
                                    "_type": "ref",
                                    "name": "$limit"
                                }
                            },
                            "skip": {
                                "name": "skip",
                                "value": {
                                    "_type": "ref",
                                    "name": "$skip"
                                }
                            }
                        },
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "Tag",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "Tag"
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "query GetPopularTags($skip: Int!, $limit: Int!) { data: getPopularTags( skip: $skip, limit: $limit ) { ...Tag }}",
            "parametersHash": 11980981446252579325,
            "fragmentSpecHash": 3976343828445744987
        },
        "GetTagChildren": {
            "name": "GetTagChildren",
            "type": "QUERY",
            "parameters": {
                "$tagId": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "UUID"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Query",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "getTagChildren",
                        "alias": "tags",
                        "arguments": {
                            "tagId": {
                                "name": "tagId",
                                "value": {
                                    "_type": "ref",
                                    "name": "$tagId"
                                }
                            }
                        },
                        "selection": {
                            "_type": "UnionFragmentSpec",
                            "name": "GetTagsResponse",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": null
                                },
                                {
                                    "_type": "ObjectConditionalSpreadSelection",
                                    "object": "TagList",
                                    "spec": {
                                        "_type": "ObjectFragmentSpec",
                                        "name": "TagList",
                                        "selections": [
                                            {
                                                "_type": "FieldSelection",
                                                "name": "list",
                                                "alias": "list",
                                                "arguments": {},
                                                "selection": {
                                                    "_type": "ObjectFragmentSpec",
                                                    "name": "Tag",
                                                    "selections": [
                                                        {
                                                            "_type": "SpreadSelection",
                                                            "fragment": "Tag"
                                                        }
                                                    ]
                                                }
                                            }
                                        ]
                                    }
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "query GetTagChildren($tagId: UUID!) { tags: getTagChildren(tagId: $tagId) { __typename ... on TagList { list { ...Tag } } }}",
            "parametersHash": 4198591995056155396,
            "fragmentSpecHash": 5075763947238411264
        },
        "GetTagInfo": {
            "name": "GetTagInfo",
            "type": "QUERY",
            "parameters": {
                "$tagId": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "UUID"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Query",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "getTagInfo",
                        "alias": "info",
                        "arguments": {
                            "tagId": {
                                "name": "tagId",
                                "value": {
                                    "_type": "ref",
                                    "name": "$tagId"
                                }
                            }
                        },
                        "selection": {
                            "_type": "UnionFragmentSpec",
                            "name": "GetTagInfoResponse",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": null
                                },
                                {
                                    "_type": "ObjectConditionalSpreadSelection",
                                    "object": "TagInfo",
                                    "spec": {
                                        "_type": "ObjectFragmentSpec",
                                        "name": "TagInfo",
                                        "selections": [
                                            {
                                                "_type": "FieldSelection",
                                                "name": "tag",
                                                "alias": "tag",
                                                "arguments": {},
                                                "selection": null
                                            },
                                            {
                                                "_type": "FieldSelection",
                                                "name": "parentTag",
                                                "alias": "parentTag",
                                                "arguments": {},
                                                "selection": {
                                                    "_type": "ObjectFragmentSpec",
                                                    "name": "Tag",
                                                    "selections": [
                                                        {
                                                            "_type": "SpreadSelection",
                                                            "fragment": "Tag"
                                                        }
                                                    ]
                                                }
                                            }
                                        ]
                                    }
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "query GetTagInfo($tagId: UUID!) { info: getTagInfo(tagId: $tagId) { __typename ... on TagInfo { tag parentTag { ...Tag } } }}",
            "parametersHash": 4198591995056155396,
            "fragmentSpecHash": 12356813680311363084
        },
        "GetTags": {
            "name": "GetTags",
            "type": "QUERY",
            "parameters": {
                "$limit": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "Int"
                        },
                        "defaultValue": null
                    }
                },
                "$parentTagId": {
                    "nullable": true,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "UUID"
                        },
                        "defaultValue": null
                    }
                },
                "$query": {
                    "nullable": true,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "String"
                        },
                        "defaultValue": null
                    }
                },
                "$skip": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "Int"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Query",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "getTags",
                        "alias": "data",
                        "arguments": {
                            "limit": {
                                "name": "limit",
                                "value": {
                                    "_type": "ref",
                                    "name": "$limit"
                                }
                            },
                            "parentTagId": {
                                "name": "parentTagId",
                                "value": {
                                    "_type": "ref",
                                    "name": "$parentTagId"
                                }
                            },
                            "query": {
                                "name": "query",
                                "value": {
                                    "_type": "ref",
                                    "name": "$query"
                                }
                            },
                            "skip": {
                                "name": "skip",
                                "value": {
                                    "_type": "ref",
                                    "name": "$skip"
                                }
                            }
                        },
                        "selection": {
                            "_type": "UnionFragmentSpec",
                            "name": "GetTagsResponse",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": null
                                },
                                {
                                    "_type": "ObjectConditionalSpreadSelection",
                                    "object": "TagList",
                                    "spec": {
                                        "_type": "ObjectFragmentSpec",
                                        "name": "TagList",
                                        "selections": [
                                            {
                                                "_type": "SpreadSelection",
                                                "fragment": "TagList"
                                            }
                                        ]
                                    }
                                },
                                {
                                    "_type": "ObjectConditionalSpreadSelection",
                                    "object": "ErrorUnknownTags",
                                    "spec": {
                                        "_type": "ObjectFragmentSpec",
                                        "name": "ErrorUnknownTags",
                                        "selections": [
                                            {
                                                "_type": "SpreadSelection",
                                                "fragment": "ErrorUnknownTags"
                                            }
                                        ]
                                    }
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "query GetTags($skip: Int!, $limit: Int!, $parentTagId: UUID, $query: String) { data: getTags( skip: $skip, limit: $limit, parentTagId: $parentTagId, query: $query ) { __typename ... on TagList { ...TagList } ... on ErrorUnknownTags { ...ErrorUnknownTags } }}",
            "parametersHash": 8795226622109273856,
            "fragmentSpecHash": 17641646650088890267
        },
        "GetTagsCount": {
            "name": "GetTagsCount",
            "type": "QUERY",
            "parameters": {
                "$parentTagId": {
                    "nullable": true,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "UUID"
                        },
                        "defaultValue": null
                    }
                },
                "$query": {
                    "nullable": true,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "String"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Query",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "getTagsCount",
                        "alias": "data",
                        "arguments": {
                            "parentTagId": {
                                "name": "parentTagId",
                                "value": {
                                    "_type": "ref",
                                    "name": "$parentTagId"
                                }
                            },
                            "query": {
                                "name": "query",
                                "value": {
                                    "_type": "ref",
                                    "name": "$query"
                                }
                            }
                        },
                        "selection": {
                            "_type": "UnionFragmentSpec",
                            "name": "IntObjectOrErrorUnknownTags",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": null
                                },
                                {
                                    "_type": "ObjectConditionalSpreadSelection",
                                    "object": "IntObject",
                                    "spec": {
                                        "_type": "ObjectFragmentSpec",
                                        "name": "IntObject",
                                        "selections": [
                                            {
                                                "_type": "SpreadSelection",
                                                "fragment": "IntObject"
                                            }
                                        ]
                                    }
                                },
                                {
                                    "_type": "ObjectConditionalSpreadSelection",
                                    "object": "ErrorUnknownTags",
                                    "spec": {
                                        "_type": "ObjectFragmentSpec",
                                        "name": "ErrorUnknownTags",
                                        "selections": [
                                            {
                                                "_type": "SpreadSelection",
                                                "fragment": "ErrorUnknownTags"
                                            }
                                        ]
                                    }
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "query GetTagsCount($parentTagId: UUID, $query: String) { data: getTagsCount( parentTagId: $parentTagId, query: $query ) { __typename ... on IntObject { ...IntObject } ... on ErrorUnknownTags { ...ErrorUnknownTags } }}",
            "parametersHash": 8454563612122981265,
            "fragmentSpecHash": 15553601645142723910
        },
        "GetUploadedFiles": {
            "name": "GetUploadedFiles",
            "type": "QUERY",
            "parameters": {
                "$limit": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "Int"
                        },
                        "defaultValue": null
                    }
                },
                "$skip": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "Int"
                        },
                        "defaultValue": null
                    }
                },
                "$sortBy": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "InputType",
                            "name": "FileSortBy",
                            "$ref": "#/server/inputs/FileSortBy"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Query",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "getUploadedFiles",
                        "alias": "data",
                        "arguments": {
                            "limit": {
                                "name": "limit",
                                "value": {
                                    "_type": "ref",
                                    "name": "$limit"
                                }
                            },
                            "skip": {
                                "name": "skip",
                                "value": {
                                    "_type": "ref",
                                    "name": "$skip"
                                }
                            },
                            "sortBy": {
                                "name": "sortBy",
                                "value": {
                                    "_type": "ref",
                                    "name": "$sortBy"
                                }
                            }
                        },
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "SearchFile",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "SearchFile"
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "query GetUploadedFiles( $skip: Int!, $limit: Int!, $sortBy: FileSortBy!) { data: getUploadedFiles( skip: $skip, limit: $limit, sortBy: $sortBy ) { ...SearchFile }}",
            "parametersHash": 16801246020131192780,
            "fragmentSpecHash": 3410642900396115328
        },
        "GetUploadedFilesCount": {
            "name": "GetUploadedFilesCount",
            "type": "QUERY",
            "parameters": {},
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Query",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "getUploadedFilesCount",
                        "alias": "count",
                        "arguments": {},
                        "selection": null
                    }
                ]
            },
            "sourceText": "query GetUploadedFilesCount { count: getUploadedFilesCount}",
            "parametersHash": 15130871412783076140,
            "fragmentSpecHash": 10074292085583712928
        },
        "GetUsers": {
            "name": "GetUsers",
            "type": "QUERY",
            "parameters": {
                "$limit": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "Int"
                        },
                        "defaultValue": null
                    }
                },
                "$query": {
                    "nullable": true,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "String"
                        },
                        "defaultValue": null
                    }
                },
                "$skip": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "Int"
                        },
                        "defaultValue": null
                    }
                },
                "$sortBy": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "InputType",
                            "name": "GetUsersSortBy",
                            "$ref": "#/server/inputs/GetUsersSortBy"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Query",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "getUsers",
                        "alias": "users",
                        "arguments": {
                            "limit": {
                                "name": "limit",
                                "value": {
                                    "_type": "ref",
                                    "name": "$limit"
                                }
                            },
                            "query": {
                                "name": "query",
                                "value": {
                                    "_type": "ref",
                                    "name": "$query"
                                }
                            },
                            "skip": {
                                "name": "skip",
                                "value": {
                                    "_type": "ref",
                                    "name": "$skip"
                                }
                            },
                            "sortBy": {
                                "name": "sortBy",
                                "value": {
                                    "_type": "ref",
                                    "name": "$sortBy"
                                }
                            }
                        },
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "User",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "User"
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "query GetUsers( $skip: Int!, $limit: Int!, $sortBy: GetUsersSortBy!, $query: String) { users: getUsers( skip: $skip, limit: $limit, sortBy: $sortBy, query: $query ) { ...User }}",
            "parametersHash": 6506087562793145020,
            "fragmentSpecHash": 2894464083211936894
        },
        "GetUsersTags": {
            "name": "GetUsersTags",
            "type": "QUERY",
            "parameters": {
                "$limit": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "Int"
                        },
                        "defaultValue": null
                    }
                },
                "$query": {
                    "nullable": true,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "String"
                        },
                        "defaultValue": null
                    }
                },
                "$skip": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "Int"
                        },
                        "defaultValue": null
                    }
                },
                "$sortBy": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "InputType",
                            "name": "UsersTagSortBy",
                            "$ref": "#/server/inputs/UsersTagSortBy"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Query",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "getUsersTags",
                        "alias": "tags",
                        "arguments": {
                            "limit": {
                                "name": "limit",
                                "value": {
                                    "_type": "ref",
                                    "name": "$limit"
                                }
                            },
                            "query": {
                                "name": "query",
                                "value": {
                                    "_type": "ref",
                                    "name": "$query"
                                }
                            },
                            "skip": {
                                "name": "skip",
                                "value": {
                                    "_type": "ref",
                                    "name": "$skip"
                                }
                            },
                            "sortBy": {
                                "name": "sortBy",
                                "value": {
                                    "_type": "ref",
                                    "name": "$sortBy"
                                }
                            }
                        },
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "UsersTag",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "UsersTag"
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "query GetUsersTags( $skip: Int!, $limit: Int!, $query: String, $sortBy: UsersTagSortBy!) { tags: getUsersTags( skip: $skip, limit: $limit, query: $query, sortBy: $sortBy ) { ...UsersTag }}",
            "parametersHash": 12826146951960952326,
            "fragmentSpecHash": 10663144811797573323
        },
        "GetUsersTagsAndCount": {
            "name": "GetUsersTagsAndCount",
            "type": "QUERY",
            "parameters": {
                "$limit": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "Int"
                        },
                        "defaultValue": null
                    }
                },
                "$query": {
                    "nullable": true,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "String"
                        },
                        "defaultValue": null
                    }
                },
                "$skip": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "Int"
                        },
                        "defaultValue": null
                    }
                },
                "$sortBy": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "InputType",
                            "name": "UsersTagSortBy",
                            "$ref": "#/server/inputs/UsersTagSortBy"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Query",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "getUsersTags",
                        "alias": "tags",
                        "arguments": {
                            "limit": {
                                "name": "limit",
                                "value": {
                                    "_type": "ref",
                                    "name": "$limit"
                                }
                            },
                            "query": {
                                "name": "query",
                                "value": {
                                    "_type": "ref",
                                    "name": "$query"
                                }
                            },
                            "skip": {
                                "name": "skip",
                                "value": {
                                    "_type": "ref",
                                    "name": "$skip"
                                }
                            },
                            "sortBy": {
                                "name": "sortBy",
                                "value": {
                                    "_type": "ref",
                                    "name": "$sortBy"
                                }
                            }
                        },
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "UsersTag",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "UsersTag"
                                }
                            ]
                        }
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "getUsersTagsCount",
                        "alias": "count",
                        "arguments": {
                            "query": {
                                "name": "query",
                                "value": {
                                    "_type": "ref",
                                    "name": "$query"
                                }
                            }
                        },
                        "selection": null
                    }
                ]
            },
            "sourceText": "query GetUsersTagsAndCount( $skip: Int!, $limit: Int!, $query: String, $sortBy: UsersTagSortBy!) { tags: getUsersTags( skip: $skip, limit: $limit, query: $query, sortBy: $sortBy ) { ...UsersTag } count: getUsersTagsCount(query: $query)}",
            "parametersHash": 12826146951960952326,
            "fragmentSpecHash": 7045030505040730231
        },
        "GetUsersTagsCount": {
            "name": "GetUsersTagsCount",
            "type": "QUERY",
            "parameters": {
                "$query": {
                    "nullable": true,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "String"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Query",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "getUsersTagsCount",
                        "alias": "count",
                        "arguments": {
                            "query": {
                                "name": "query",
                                "value": {
                                    "_type": "ref",
                                    "name": "$query"
                                }
                            }
                        },
                        "selection": null
                    }
                ]
            },
            "sourceText": "query GetUsersTagsCount($query: String) { count: getUsersTagsCount(query: $query)}",
            "parametersHash": 4809337597314547940,
            "fragmentSpecHash": 12749526881273360571
        },
        "GetUsersTotal": {
            "name": "GetUsersTotal",
            "type": "QUERY",
            "parameters": {
                "$query": {
                    "nullable": true,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "String"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Query",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "getUsersTotal",
                        "alias": "getUsersTotal",
                        "arguments": {
                            "query": {
                                "name": "query",
                                "value": {
                                    "_type": "ref",
                                    "name": "$query"
                                }
                            }
                        },
                        "selection": null
                    }
                ]
            },
            "sourceText": "query GetUsersTotal($query: String) { getUsersTotal(query: $query)}",
            "parametersHash": 4809337597314547940,
            "fragmentSpecHash": 5137619920672669758
        },
        "GetUsersTotalAndUsers": {
            "name": "GetUsersTotalAndUsers",
            "type": "QUERY",
            "parameters": {
                "$limit": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "Int"
                        },
                        "defaultValue": null
                    }
                },
                "$query": {
                    "nullable": true,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "String"
                        },
                        "defaultValue": null
                    }
                },
                "$skip": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "Int"
                        },
                        "defaultValue": null
                    }
                },
                "$sortBy": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "InputType",
                            "name": "GetUsersSortBy",
                            "$ref": "#/server/inputs/GetUsersSortBy"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Query",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "getUsersTotal",
                        "alias": "total",
                        "arguments": {
                            "query": {
                                "name": "query",
                                "value": {
                                    "_type": "ref",
                                    "name": "$query"
                                }
                            }
                        },
                        "selection": null
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "getUsers",
                        "alias": "users",
                        "arguments": {
                            "limit": {
                                "name": "limit",
                                "value": {
                                    "_type": "ref",
                                    "name": "$limit"
                                }
                            },
                            "query": {
                                "name": "query",
                                "value": {
                                    "_type": "ref",
                                    "name": "$query"
                                }
                            },
                            "skip": {
                                "name": "skip",
                                "value": {
                                    "_type": "ref",
                                    "name": "$skip"
                                }
                            },
                            "sortBy": {
                                "name": "sortBy",
                                "value": {
                                    "_type": "ref",
                                    "name": "$sortBy"
                                }
                            }
                        },
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "User",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "User"
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "query GetUsersTotalAndUsers( $skip: Int!, $limit: Int!, $sortBy: GetUsersSortBy!, $query: String) { total: getUsersTotal(query: $query) users: getUsers( skip: $skip, limit: $limit, sortBy: $sortBy, query: $query ) { ...User }}",
            "parametersHash": 6506087562793145020,
            "fragmentSpecHash": 16182086905575915424
        },
        "IsAllowedToDownload": {
            "name": "IsAllowedToDownload",
            "type": "QUERY",
            "parameters": {
                "$id": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "UUID"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Query",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "isAllowedToDownload",
                        "alias": "allowed",
                        "arguments": {
                            "id": {
                                "name": "id",
                                "value": {
                                    "_type": "ref",
                                    "name": "$id"
                                }
                            }
                        },
                        "selection": {
                            "_type": "UnionFragmentSpec",
                            "name": "IsAllowedToDownloadResponse",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": null
                                },
                                {
                                    "_type": "ObjectConditionalSpreadSelection",
                                    "object": "BooleanObject",
                                    "spec": {
                                        "_type": "ObjectFragmentSpec",
                                        "name": "BooleanObject",
                                        "selections": [
                                            {
                                                "_type": "FieldSelection",
                                                "name": "bvalue",
                                                "alias": "bvalue",
                                                "arguments": {},
                                                "selection": null
                                            }
                                        ]
                                    }
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "query IsAllowedToDownload($id: UUID!) { allowed: isAllowedToDownload(id: $id) { __typename ... on BooleanObject { bvalue } }}",
            "parametersHash": 6071217685901131044,
            "fragmentSpecHash": 17761166072619972125
        },
        "IsTagExists": {
            "name": "IsTagExists",
            "type": "QUERY",
            "parameters": {
                "$tag": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "String"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Query",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "isTagExists",
                        "alias": "isTagExists",
                        "arguments": {
                            "tag": {
                                "name": "tag",
                                "value": {
                                    "_type": "ref",
                                    "name": "$tag"
                                }
                            }
                        },
                        "selection": null
                    }
                ]
            },
            "sourceText": "query IsTagExists($tag: String!) { isTagExists(tag: $tag)}",
            "parametersHash": 6325144460434547702,
            "fragmentSpecHash": 9975639271701492416
        },
        "Login": {
            "name": "Login",
            "type": "MUTATION",
            "parameters": {
                "$email": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "String"
                        },
                        "defaultValue": null
                    }
                },
                "$password": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "String"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Mutation",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "login",
                        "alias": "login",
                        "arguments": {
                            "email": {
                                "name": "email",
                                "value": {
                                    "_type": "ref",
                                    "name": "$email"
                                }
                            },
                            "password": {
                                "name": "password",
                                "value": {
                                    "_type": "ref",
                                    "name": "$password"
                                }
                            }
                        },
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "ErrorInvalidCredentials",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": "error"
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "mutation Login($email: String!, $password: String!) { login(email: $email, password: $password) { error: __typename }}",
            "parametersHash": 17521459447584200257,
            "fragmentSpecHash": 11041489417827420873
        },
        "Logout": {
            "name": "Logout",
            "type": "MUTATION",
            "parameters": {},
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Mutation",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "logout",
                        "alias": "logout",
                        "arguments": {},
                        "selection": null
                    }
                ]
            },
            "sourceText": "mutation Logout { logout}",
            "parametersHash": 15130871412783076140,
            "fragmentSpecHash": 509582491249423728
        },
        "RemoveUserFromGroup": {
            "name": "RemoveUserFromGroup",
            "type": "MUTATION",
            "parameters": {
                "$groupId": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "UUID"
                        },
                        "defaultValue": null
                    }
                },
                "$userId": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "UUID"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Mutation",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "removeUserFromGroup",
                        "alias": "error",
                        "arguments": {
                            "groupId": {
                                "name": "groupId",
                                "value": {
                                    "_type": "ref",
                                    "name": "$groupId"
                                }
                            },
                            "userId": {
                                "name": "userId",
                                "value": {
                                    "_type": "ref",
                                    "name": "$userId"
                                }
                            }
                        },
                        "selection": {
                            "_type": "UnionFragmentSpec",
                            "name": "ErrorGroupNotFoundOrErrorNotFound",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": null
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "mutation RemoveUserFromGroup($groupId: UUID!, $userId: UUID!) { error: removeUserFromGroup(groupId: $groupId, userId: $userId) { __typename }}",
            "parametersHash": 14153868134748337413,
            "fragmentSpecHash": 18084683163488280623
        },
        "ResetPassword": {
            "name": "ResetPassword",
            "type": "MUTATION",
            "parameters": {
                "$newPassword": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "String"
                        },
                        "defaultValue": null
                    }
                },
                "$token": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "String"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Mutation",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "resetPassword",
                        "alias": "error",
                        "arguments": {
                            "newPassword": {
                                "name": "newPassword",
                                "value": {
                                    "_type": "ref",
                                    "name": "$newPassword"
                                }
                            },
                            "token": {
                                "name": "token",
                                "value": {
                                    "_type": "ref",
                                    "name": "$token"
                                }
                            }
                        },
                        "selection": {
                            "_type": "UnionFragmentSpec",
                            "name": "ResetPasswordError",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": null
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "mutation ResetPassword($token: String!, $newPassword: String!) { error: resetPassword(token: $token, newPassword: $newPassword) { __typename }}",
            "parametersHash": 16584749347970824821,
            "fragmentSpecHash": 7411701364167059351
        },
        "RetrieveFile": {
            "name": "RetrieveFile",
            "type": "QUERY",
            "parameters": {
                "$id": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "UUID"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Query",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "retrieveFile",
                        "alias": "file",
                        "arguments": {
                            "id": {
                                "name": "id",
                                "value": {
                                    "_type": "ref",
                                    "name": "$id"
                                }
                            }
                        },
                        "selection": {
                            "_type": "UnionFragmentSpec",
                            "name": "RetrieveFileResponse",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": null
                                },
                                {
                                    "_type": "ObjectConditionalSpreadSelection",
                                    "object": "SearchFile",
                                    "spec": {
                                        "_type": "ObjectFragmentSpec",
                                        "name": "SearchFile",
                                        "selections": [
                                            {
                                                "_type": "FieldSelection",
                                                "name": "tags",
                                                "alias": "tags",
                                                "arguments": {},
                                                "selection": {
                                                    "_type": "ObjectFragmentSpec",
                                                    "name": "Tag",
                                                    "selections": [
                                                        {
                                                            "_type": "SpreadSelection",
                                                            "fragment": "SmallTag"
                                                        }
                                                    ]
                                                }
                                            },
                                            {
                                                "_type": "FieldSelection",
                                                "name": "file",
                                                "alias": "file",
                                                "arguments": {},
                                                "selection": {
                                                    "_type": "ObjectFragmentSpec",
                                                    "name": "File",
                                                    "selections": [
                                                        {
                                                            "_type": "FieldSelection",
                                                            "name": "filename",
                                                            "alias": "filename",
                                                            "arguments": {},
                                                            "selection": null
                                                        }
                                                    ]
                                                }
                                            }
                                        ]
                                    }
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "query RetrieveFile($id: UUID!) { file: retrieveFile(id: $id) { __typename ... on SearchFile { tags { ...SmallTag } file { filename } } }}",
            "parametersHash": 6071217685901131044,
            "fragmentSpecHash": 1565753371691928007
        },
        "RetrieveGroupAndTags": {
            "name": "RetrieveGroupAndTags",
            "type": "QUERY",
            "parameters": {
                "$id": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "UUID"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Query",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "retrieveGroup",
                        "alias": "group",
                        "arguments": {
                            "id": {
                                "name": "id",
                                "value": {
                                    "_type": "ref",
                                    "name": "$id"
                                }
                            }
                        },
                        "selection": {
                            "_type": "UnionFragmentSpec",
                            "name": "RetrieveGroupResponse",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": null
                                },
                                {
                                    "_type": "ObjectConditionalSpreadSelection",
                                    "object": "Group",
                                    "spec": {
                                        "_type": "ObjectFragmentSpec",
                                        "name": "Group",
                                        "selections": [
                                            {
                                                "_type": "SpreadSelection",
                                                "fragment": "Group"
                                            }
                                        ]
                                    }
                                }
                            ]
                        }
                    },
                    {
                        "_type": "FieldSelection",
                        "name": "getGroupTags",
                        "alias": "tags",
                        "arguments": {
                            "id": {
                                "name": "id",
                                "value": {
                                    "_type": "ref",
                                    "name": "$id"
                                }
                            },
                            "limit": {
                                "name": "limit",
                                "value": {
                                    "_type": "literal",
                                    "value": 10000000
                                }
                            },
                            "skip": {
                                "name": "skip",
                                "value": {
                                    "_type": "literal",
                                    "value": 0
                                }
                            }
                        },
                        "selection": {
                            "_type": "UnionFragmentSpec",
                            "name": "GetGroupTagsResponse",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": null
                                },
                                {
                                    "_type": "ObjectConditionalSpreadSelection",
                                    "object": "TagList",
                                    "spec": {
                                        "_type": "ObjectFragmentSpec",
                                        "name": "TagList",
                                        "selections": [
                                            {
                                                "_type": "FieldSelection",
                                                "name": "list",
                                                "alias": "list",
                                                "arguments": {},
                                                "selection": {
                                                    "_type": "ObjectFragmentSpec",
                                                    "name": "Tag",
                                                    "selections": [
                                                        {
                                                            "_type": "SpreadSelection",
                                                            "fragment": "Tag"
                                                        }
                                                    ]
                                                }
                                            }
                                        ]
                                    }
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "query RetrieveGroupAndTags($id: UUID!) { group: retrieveGroup(id: $id) { __typename ... on Group { ...Group } } tags: getGroupTags(id: $id, skip: 0, limit: 10000000) { __typename ... on TagList { list { ...Tag } } }}",
            "parametersHash": 6071217685901131044,
            "fragmentSpecHash": 14758244729364364478
        },
        "SearchTags": {
            "name": "SearchTags",
            "type": "QUERY",
            "parameters": {
                "$query": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "String"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Query",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "searchTags",
                        "alias": "tags",
                        "arguments": {
                            "query": {
                                "name": "query",
                                "value": {
                                    "_type": "ref",
                                    "name": "$query"
                                }
                            }
                        },
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "Tag",
                            "selections": [
                                {
                                    "_type": "SpreadSelection",
                                    "fragment": "Tag"
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "query SearchTags($query: String!){ tags: searchTags(query: $query) { ...Tag }}",
            "parametersHash": 1935036102675183799,
            "fragmentSpecHash": 14386611121265448952
        },
        "SendOTPCode": {
            "name": "SendOTPCode",
            "type": "MUTATION",
            "parameters": {
                "$email": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "String"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Mutation",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "sendOTPCode",
                        "alias": "error",
                        "arguments": {
                            "email": {
                                "name": "email",
                                "value": {
                                    "_type": "ref",
                                    "name": "$email"
                                }
                            }
                        },
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "ErrorInvalidCredentials",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": null
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "mutation SendOTPCode($email: String!) { error: sendOTPCode(email: $email) { __typename }}",
            "parametersHash": 735258917282373466,
            "fragmentSpecHash": 15491116146514641123
        },
        "SetTagIsFavourite": {
            "name": "SetTagIsFavourite",
            "type": "MUTATION",
            "parameters": {
                "$isFavourite": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "Boolean"
                        },
                        "defaultValue": null
                    }
                },
                "$tagId": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "UUID"
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Mutation",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "setTagIsFavourite",
                        "alias": "setTagIsFavourite",
                        "arguments": {
                            "isFavourite": {
                                "name": "isFavourite",
                                "value": {
                                    "_type": "ref",
                                    "name": "$isFavourite"
                                }
                            },
                            "tagId": {
                                "name": "tagId",
                                "value": {
                                    "_type": "ref",
                                    "name": "$tagId"
                                }
                            }
                        },
                        "selection": {
                            "_type": "UnionFragmentSpec",
                            "name": "ErrorAlreadyDoneOrUnknownTags",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": "error"
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "mutation SetTagIsFavourite($tagId: UUID!, $isFavourite: Boolean!) { setTagIsFavourite(tagId: $tagId, isFavourite: $isFavourite) { error: __typename }}",
            "parametersHash": 15942773466879643624,
            "fragmentSpecHash": 2284851869925856194
        },
        "UpdateFile": {
            "name": "UpdateFile",
            "type": "MUTATION",
            "parameters": {
                "$id": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "UUID"
                        },
                        "defaultValue": null
                    }
                },
                "$name": {
                    "nullable": false,
                    "spec": {
                        "_type": "literal",
                        "type": {
                            "_type": "Scalar",
                            "name": "String"
                        },
                        "defaultValue": null
                    }
                },
                "$tagIds": {
                    "nullable": false,
                    "spec": {
                        "_type": "array",
                        "nullable": false,
                        "type": {
                            "_type": "literal",
                            "type": {
                                "_type": "Scalar",
                                "name": "UUID"
                            },
                            "defaultValue": null
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Mutation",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "updateFile",
                        "alias": "error",
                        "arguments": {
                            "id": {
                                "name": "id",
                                "value": {
                                    "_type": "ref",
                                    "name": "$id"
                                }
                            },
                            "name": {
                                "name": "name",
                                "value": {
                                    "_type": "ref",
                                    "name": "$name"
                                }
                            },
                            "tagIds": {
                                "name": "tagIds",
                                "value": {
                                    "_type": "ref",
                                    "name": "$tagIds"
                                }
                            }
                        },
                        "selection": {
                            "_type": "UnionFragmentSpec",
                            "name": "UpdateFileError",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": null
                                },
                                {
                                    "_type": "ObjectConditionalSpreadSelection",
                                    "object": "ErrorUnknownTags",
                                    "spec": {
                                        "_type": "ObjectFragmentSpec",
                                        "name": "ErrorUnknownTags",
                                        "selections": [
                                            {
                                                "_type": "SpreadSelection",
                                                "fragment": "ErrorUnknownTags"
                                            }
                                        ]
                                    }
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "mutation UpdateFile($id: UUID!, $name: String!, $tagIds: [UUID!]!) { error: updateFile(id: $id, name: $name, tagIds: $tagIds) { __typename ... on ErrorUnknownTags { ...ErrorUnknownTags } }}",
            "parametersHash": 5995529342963911082,
            "fragmentSpecHash": 5843161243328139378
        },
        "UpdateFilesAutotags": {
            "name": "UpdateFilesAutotags",
            "type": "MUTATION",
            "parameters": {
                "$autotagIds": {
                    "nullable": false,
                    "spec": {
                        "_type": "array",
                        "nullable": false,
                        "type": {
                            "_type": "literal",
                            "type": {
                                "_type": "Scalar",
                                "name": "UUID"
                            },
                            "defaultValue": null
                        },
                        "defaultValue": null
                    }
                },
                "$fileIds": {
                    "nullable": false,
                    "spec": {
                        "_type": "array",
                        "nullable": false,
                        "type": {
                            "_type": "literal",
                            "type": {
                                "_type": "Scalar",
                                "name": "UUID"
                            },
                            "defaultValue": null
                        },
                        "defaultValue": null
                    }
                }
            },
            "fragmentSpec": {
                "_type": "ObjectFragmentSpec",
                "name": "Mutation",
                "selections": [
                    {
                        "_type": "FieldSelection",
                        "name": "updateFilesAutotags",
                        "alias": "updateFilesAutotags",
                        "arguments": {
                            "autotagIds": {
                                "name": "autotagIds",
                                "value": {
                                    "_type": "ref",
                                    "name": "$autotagIds"
                                }
                            },
                            "fileIds": {
                                "name": "fileIds",
                                "value": {
                                    "_type": "ref",
                                    "name": "$fileIds"
                                }
                            }
                        },
                        "selection": {
                            "_type": "ObjectFragmentSpec",
                            "name": "ErrorCantAddAutotags",
                            "selections": [
                                {
                                    "_type": "TypenameField",
                                    "alias": null
                                }
                            ]
                        }
                    }
                ]
            },
            "sourceText": "mutation UpdateFilesAutotags($fileIds: [UUID!]!, $autotagIds: [UUID!]!) { updateFilesAutotags(fileIds: $fileIds, autotagIds: $autotagIds) { __typename }}",
            "parametersHash": 17938087220498764406,
            "fragmentSpecHash": 10604776142287213235
        }
    },
    "directives": {}
}
        "##,
        )
        .unwrap();
    }
}
