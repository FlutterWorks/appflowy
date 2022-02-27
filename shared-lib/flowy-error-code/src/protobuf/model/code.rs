// This file is generated by rust-protobuf 2.25.2. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `code.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_2;

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ErrorCode {
    Internal = 0,
    UserUnauthorized = 2,
    RecordNotFound = 3,
    WorkspaceNameInvalid = 100,
    WorkspaceIdInvalid = 101,
    AppColorStyleInvalid = 102,
    WorkspaceDescTooLong = 103,
    WorkspaceNameTooLong = 104,
    AppIdInvalid = 110,
    AppNameInvalid = 111,
    ViewNameInvalid = 120,
    ViewThumbnailInvalid = 121,
    ViewIdInvalid = 122,
    ViewDescTooLong = 123,
    ViewDataInvalid = 124,
    ViewNameTooLong = 125,
    ConnectError = 200,
    EmailIsEmpty = 300,
    EmailFormatInvalid = 301,
    EmailAlreadyExists = 302,
    PasswordIsEmpty = 303,
    PasswordTooLong = 304,
    PasswordContainsForbidCharacters = 305,
    PasswordFormatInvalid = 306,
    PasswordNotMatch = 307,
    UserNameTooLong = 308,
    UserNameContainForbiddenCharacters = 309,
    UserNameIsEmpty = 310,
    UserIdInvalid = 311,
    UserNotExist = 312,
}

impl ::protobuf::ProtobufEnum for ErrorCode {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ErrorCode> {
        match value {
            0 => ::std::option::Option::Some(ErrorCode::Internal),
            2 => ::std::option::Option::Some(ErrorCode::UserUnauthorized),
            3 => ::std::option::Option::Some(ErrorCode::RecordNotFound),
            100 => ::std::option::Option::Some(ErrorCode::WorkspaceNameInvalid),
            101 => ::std::option::Option::Some(ErrorCode::WorkspaceIdInvalid),
            102 => ::std::option::Option::Some(ErrorCode::AppColorStyleInvalid),
            103 => ::std::option::Option::Some(ErrorCode::WorkspaceDescTooLong),
            104 => ::std::option::Option::Some(ErrorCode::WorkspaceNameTooLong),
            110 => ::std::option::Option::Some(ErrorCode::AppIdInvalid),
            111 => ::std::option::Option::Some(ErrorCode::AppNameInvalid),
            120 => ::std::option::Option::Some(ErrorCode::ViewNameInvalid),
            121 => ::std::option::Option::Some(ErrorCode::ViewThumbnailInvalid),
            122 => ::std::option::Option::Some(ErrorCode::ViewIdInvalid),
            123 => ::std::option::Option::Some(ErrorCode::ViewDescTooLong),
            124 => ::std::option::Option::Some(ErrorCode::ViewDataInvalid),
            125 => ::std::option::Option::Some(ErrorCode::ViewNameTooLong),
            200 => ::std::option::Option::Some(ErrorCode::ConnectError),
            300 => ::std::option::Option::Some(ErrorCode::EmailIsEmpty),
            301 => ::std::option::Option::Some(ErrorCode::EmailFormatInvalid),
            302 => ::std::option::Option::Some(ErrorCode::EmailAlreadyExists),
            303 => ::std::option::Option::Some(ErrorCode::PasswordIsEmpty),
            304 => ::std::option::Option::Some(ErrorCode::PasswordTooLong),
            305 => ::std::option::Option::Some(ErrorCode::PasswordContainsForbidCharacters),
            306 => ::std::option::Option::Some(ErrorCode::PasswordFormatInvalid),
            307 => ::std::option::Option::Some(ErrorCode::PasswordNotMatch),
            308 => ::std::option::Option::Some(ErrorCode::UserNameTooLong),
            309 => ::std::option::Option::Some(ErrorCode::UserNameContainForbiddenCharacters),
            310 => ::std::option::Option::Some(ErrorCode::UserNameIsEmpty),
            311 => ::std::option::Option::Some(ErrorCode::UserIdInvalid),
            312 => ::std::option::Option::Some(ErrorCode::UserNotExist),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ErrorCode] = &[
            ErrorCode::Internal,
            ErrorCode::UserUnauthorized,
            ErrorCode::RecordNotFound,
            ErrorCode::WorkspaceNameInvalid,
            ErrorCode::WorkspaceIdInvalid,
            ErrorCode::AppColorStyleInvalid,
            ErrorCode::WorkspaceDescTooLong,
            ErrorCode::WorkspaceNameTooLong,
            ErrorCode::AppIdInvalid,
            ErrorCode::AppNameInvalid,
            ErrorCode::ViewNameInvalid,
            ErrorCode::ViewThumbnailInvalid,
            ErrorCode::ViewIdInvalid,
            ErrorCode::ViewDescTooLong,
            ErrorCode::ViewDataInvalid,
            ErrorCode::ViewNameTooLong,
            ErrorCode::ConnectError,
            ErrorCode::EmailIsEmpty,
            ErrorCode::EmailFormatInvalid,
            ErrorCode::EmailAlreadyExists,
            ErrorCode::PasswordIsEmpty,
            ErrorCode::PasswordTooLong,
            ErrorCode::PasswordContainsForbidCharacters,
            ErrorCode::PasswordFormatInvalid,
            ErrorCode::PasswordNotMatch,
            ErrorCode::UserNameTooLong,
            ErrorCode::UserNameContainForbiddenCharacters,
            ErrorCode::UserNameIsEmpty,
            ErrorCode::UserIdInvalid,
            ErrorCode::UserNotExist,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<ErrorCode>("ErrorCode", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for ErrorCode {
}

impl ::std::default::Default for ErrorCode {
    fn default() -> Self {
        ErrorCode::Internal
    }
}

impl ::protobuf::reflect::ProtobufValue for ErrorCode {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\ncode.proto*\xc4\x05\n\tErrorCode\x12\x0c\n\x08Internal\x10\0\x12\x14\
    \n\x10UserUnauthorized\x10\x02\x12\x12\n\x0eRecordNotFound\x10\x03\x12\
    \x18\n\x14WorkspaceNameInvalid\x10d\x12\x16\n\x12WorkspaceIdInvalid\x10e\
    \x12\x18\n\x14AppColorStyleInvalid\x10f\x12\x18\n\x14WorkspaceDescTooLon\
    g\x10g\x12\x18\n\x14WorkspaceNameTooLong\x10h\x12\x10\n\x0cAppIdInvalid\
    \x10n\x12\x12\n\x0eAppNameInvalid\x10o\x12\x13\n\x0fViewNameInvalid\x10x\
    \x12\x18\n\x14ViewThumbnailInvalid\x10y\x12\x11\n\rViewIdInvalid\x10z\
    \x12\x13\n\x0fViewDescTooLong\x10{\x12\x13\n\x0fViewDataInvalid\x10|\x12\
    \x13\n\x0fViewNameTooLong\x10}\x12\x11\n\x0cConnectError\x10\xc8\x01\x12\
    \x11\n\x0cEmailIsEmpty\x10\xac\x02\x12\x17\n\x12EmailFormatInvalid\x10\
    \xad\x02\x12\x17\n\x12EmailAlreadyExists\x10\xae\x02\x12\x14\n\x0fPasswo\
    rdIsEmpty\x10\xaf\x02\x12\x14\n\x0fPasswordTooLong\x10\xb0\x02\x12%\n\
    \x20PasswordContainsForbidCharacters\x10\xb1\x02\x12\x1a\n\x15PasswordFo\
    rmatInvalid\x10\xb2\x02\x12\x15\n\x10PasswordNotMatch\x10\xb3\x02\x12\
    \x14\n\x0fUserNameTooLong\x10\xb4\x02\x12'\n\"UserNameContainForbiddenCh\
    aracters\x10\xb5\x02\x12\x14\n\x0fUserNameIsEmpty\x10\xb6\x02\x12\x12\n\
    \rUserIdInvalid\x10\xb7\x02\x12\x11\n\x0cUserNotExist\x10\xb8\x02b\x06pr\
    oto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
