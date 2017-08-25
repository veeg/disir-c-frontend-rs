// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct SemanticVersion {
    // message fields
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SemanticVersion {}

impl SemanticVersion {
    pub fn new() -> SemanticVersion {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SemanticVersion {
        static mut instance: ::protobuf::lazy::Lazy<SemanticVersion> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SemanticVersion,
        };
        unsafe {
            instance.get(SemanticVersion::new)
        }
    }

    // uint32 major = 1;

    pub fn clear_major(&mut self) {
        self.major = 0;
    }

    // Param is passed by value, moved
    pub fn set_major(&mut self, v: u32) {
        self.major = v;
    }

    pub fn get_major(&self) -> u32 {
        self.major
    }

    fn get_major_for_reflect(&self) -> &u32 {
        &self.major
    }

    fn mut_major_for_reflect(&mut self) -> &mut u32 {
        &mut self.major
    }

    // uint32 minor = 2;

    pub fn clear_minor(&mut self) {
        self.minor = 0;
    }

    // Param is passed by value, moved
    pub fn set_minor(&mut self, v: u32) {
        self.minor = v;
    }

    pub fn get_minor(&self) -> u32 {
        self.minor
    }

    fn get_minor_for_reflect(&self) -> &u32 {
        &self.minor
    }

    fn mut_minor_for_reflect(&mut self) -> &mut u32 {
        &mut self.minor
    }

    // uint32 patch = 3;

    pub fn clear_patch(&mut self) {
        self.patch = 0;
    }

    // Param is passed by value, moved
    pub fn set_patch(&mut self, v: u32) {
        self.patch = v;
    }

    pub fn get_patch(&self) -> u32 {
        self.patch
    }

    fn get_patch_for_reflect(&self) -> &u32 {
        &self.patch
    }

    fn mut_patch_for_reflect(&mut self) -> &mut u32 {
        &mut self.patch
    }
}

impl ::protobuf::Message for SemanticVersion {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.major = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.minor = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.patch = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.major != 0 {
            my_size += ::protobuf::rt::value_size(1, self.major, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.minor != 0 {
            my_size += ::protobuf::rt::value_size(2, self.minor, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.patch != 0 {
            my_size += ::protobuf::rt::value_size(3, self.patch, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.major != 0 {
            os.write_uint32(1, self.major)?;
        };
        if self.minor != 0 {
            os.write_uint32(2, self.minor)?;
        };
        if self.patch != 0 {
            os.write_uint32(3, self.patch)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SemanticVersion {
    fn new() -> SemanticVersion {
        SemanticVersion::new()
    }

    fn descriptor_static(_: ::std::option::Option<SemanticVersion>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "major",
                    SemanticVersion::get_major_for_reflect,
                    SemanticVersion::mut_major_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "minor",
                    SemanticVersion::get_minor_for_reflect,
                    SemanticVersion::mut_minor_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "patch",
                    SemanticVersion::get_patch_for_reflect,
                    SemanticVersion::mut_patch_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SemanticVersion>(
                    "SemanticVersion",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SemanticVersion {
    fn clear(&mut self) {
        self.clear_major();
        self.clear_minor();
        self.clear_patch();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SemanticVersion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SemanticVersion {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UserInfo {
    // message fields
    version: ::protobuf::SingularPtrField<SemanticVersion>,
    pub user_agent: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UserInfo {}

impl UserInfo {
    pub fn new() -> UserInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UserInfo {
        static mut instance: ::protobuf::lazy::Lazy<UserInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UserInfo,
        };
        unsafe {
            instance.get(UserInfo::new)
        }
    }

    // .disirpb.SemanticVersion version = 1;

    pub fn clear_version(&mut self) {
        self.version.clear();
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: SemanticVersion) {
        self.version = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_version(&mut self) -> &mut SemanticVersion {
        if self.version.is_none() {
            self.version.set_default();
        };
        self.version.as_mut().unwrap()
    }

    // Take field
    pub fn take_version(&mut self) -> SemanticVersion {
        self.version.take().unwrap_or_else(|| SemanticVersion::new())
    }

    pub fn get_version(&self) -> &SemanticVersion {
        self.version.as_ref().unwrap_or_else(|| SemanticVersion::default_instance())
    }

    fn get_version_for_reflect(&self) -> &::protobuf::SingularPtrField<SemanticVersion> {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SemanticVersion> {
        &mut self.version
    }

    // string user_agent = 2;

    pub fn clear_user_agent(&mut self) {
        self.user_agent.clear();
    }

    // Param is passed by value, moved
    pub fn set_user_agent(&mut self, v: ::std::string::String) {
        self.user_agent = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_user_agent(&mut self) -> &mut ::std::string::String {
        &mut self.user_agent
    }

    // Take field
    pub fn take_user_agent(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.user_agent, ::std::string::String::new())
    }

    pub fn get_user_agent(&self) -> &str {
        &self.user_agent
    }

    fn get_user_agent_for_reflect(&self) -> &::std::string::String {
        &self.user_agent
    }

    fn mut_user_agent_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.user_agent
    }
}

impl ::protobuf::Message for UserInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.version)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.user_agent)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.version.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if !self.user_agent.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.user_agent);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.version.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if !self.user_agent.is_empty() {
            os.write_string(2, &self.user_agent)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UserInfo {
    fn new() -> UserInfo {
        UserInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<UserInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SemanticVersion>>(
                    "version",
                    UserInfo::get_version_for_reflect,
                    UserInfo::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "user_agent",
                    UserInfo::get_user_agent_for_reflect,
                    UserInfo::mut_user_agent_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UserInfo>(
                    "UserInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UserInfo {
    fn clear(&mut self) {
        self.clear_version();
        self.clear_user_agent();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UserInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UserInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SystemInfo {
    // message fields
    disirpb: ::protobuf::SingularPtrField<SemanticVersion>,
    libdisir: ::protobuf::SingularPtrField<SemanticVersion>,
    pub bind_accepted: bool,
    pub reason: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SystemInfo {}

impl SystemInfo {
    pub fn new() -> SystemInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SystemInfo {
        static mut instance: ::protobuf::lazy::Lazy<SystemInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SystemInfo,
        };
        unsafe {
            instance.get(SystemInfo::new)
        }
    }

    // .disirpb.SemanticVersion disirpb = 1;

    pub fn clear_disirpb(&mut self) {
        self.disirpb.clear();
    }

    pub fn has_disirpb(&self) -> bool {
        self.disirpb.is_some()
    }

    // Param is passed by value, moved
    pub fn set_disirpb(&mut self, v: SemanticVersion) {
        self.disirpb = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_disirpb(&mut self) -> &mut SemanticVersion {
        if self.disirpb.is_none() {
            self.disirpb.set_default();
        };
        self.disirpb.as_mut().unwrap()
    }

    // Take field
    pub fn take_disirpb(&mut self) -> SemanticVersion {
        self.disirpb.take().unwrap_or_else(|| SemanticVersion::new())
    }

    pub fn get_disirpb(&self) -> &SemanticVersion {
        self.disirpb.as_ref().unwrap_or_else(|| SemanticVersion::default_instance())
    }

    fn get_disirpb_for_reflect(&self) -> &::protobuf::SingularPtrField<SemanticVersion> {
        &self.disirpb
    }

    fn mut_disirpb_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SemanticVersion> {
        &mut self.disirpb
    }

    // .disirpb.SemanticVersion libdisir = 2;

    pub fn clear_libdisir(&mut self) {
        self.libdisir.clear();
    }

    pub fn has_libdisir(&self) -> bool {
        self.libdisir.is_some()
    }

    // Param is passed by value, moved
    pub fn set_libdisir(&mut self, v: SemanticVersion) {
        self.libdisir = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_libdisir(&mut self) -> &mut SemanticVersion {
        if self.libdisir.is_none() {
            self.libdisir.set_default();
        };
        self.libdisir.as_mut().unwrap()
    }

    // Take field
    pub fn take_libdisir(&mut self) -> SemanticVersion {
        self.libdisir.take().unwrap_or_else(|| SemanticVersion::new())
    }

    pub fn get_libdisir(&self) -> &SemanticVersion {
        self.libdisir.as_ref().unwrap_or_else(|| SemanticVersion::default_instance())
    }

    fn get_libdisir_for_reflect(&self) -> &::protobuf::SingularPtrField<SemanticVersion> {
        &self.libdisir
    }

    fn mut_libdisir_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SemanticVersion> {
        &mut self.libdisir
    }

    // bool bind_accepted = 3;

    pub fn clear_bind_accepted(&mut self) {
        self.bind_accepted = false;
    }

    // Param is passed by value, moved
    pub fn set_bind_accepted(&mut self, v: bool) {
        self.bind_accepted = v;
    }

    pub fn get_bind_accepted(&self) -> bool {
        self.bind_accepted
    }

    fn get_bind_accepted_for_reflect(&self) -> &bool {
        &self.bind_accepted
    }

    fn mut_bind_accepted_for_reflect(&mut self) -> &mut bool {
        &mut self.bind_accepted
    }

    // string reason = 4;

    pub fn clear_reason(&mut self) {
        self.reason.clear();
    }

    // Param is passed by value, moved
    pub fn set_reason(&mut self, v: ::std::string::String) {
        self.reason = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_reason(&mut self) -> &mut ::std::string::String {
        &mut self.reason
    }

    // Take field
    pub fn take_reason(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.reason, ::std::string::String::new())
    }

    pub fn get_reason(&self) -> &str {
        &self.reason
    }

    fn get_reason_for_reflect(&self) -> &::std::string::String {
        &self.reason
    }

    fn mut_reason_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.reason
    }
}

impl ::protobuf::Message for SystemInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.disirpb)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.libdisir)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.bind_accepted = tmp;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.reason)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.disirpb.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.libdisir.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.bind_accepted != false {
            my_size += 2;
        };
        if !self.reason.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.reason);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.disirpb.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.libdisir.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.bind_accepted != false {
            os.write_bool(3, self.bind_accepted)?;
        };
        if !self.reason.is_empty() {
            os.write_string(4, &self.reason)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SystemInfo {
    fn new() -> SystemInfo {
        SystemInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<SystemInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SemanticVersion>>(
                    "disirpb",
                    SystemInfo::get_disirpb_for_reflect,
                    SystemInfo::mut_disirpb_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SemanticVersion>>(
                    "libdisir",
                    SystemInfo::get_libdisir_for_reflect,
                    SystemInfo::mut_libdisir_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "bind_accepted",
                    SystemInfo::get_bind_accepted_for_reflect,
                    SystemInfo::mut_bind_accepted_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "reason",
                    SystemInfo::get_reason_for_reflect,
                    SystemInfo::mut_reason_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SystemInfo>(
                    "SystemInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SystemInfo {
    fn clear(&mut self) {
        self.clear_disirpb();
        self.clear_libdisir();
        self.clear_bind_accepted();
        self.clear_reason();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SystemInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SystemInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DisirValueType {
    // message oneof groups
    value: ::std::option::Option<DisirValueType_oneof_value>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DisirValueType {}

#[derive(Clone,PartialEq)]
pub enum DisirValueType_oneof_value {
    string_type(bool),
    integer_type(bool),
    float_type(bool),
    boolean_type(bool),
    enum_type(bool),
}

impl DisirValueType {
    pub fn new() -> DisirValueType {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DisirValueType {
        static mut instance: ::protobuf::lazy::Lazy<DisirValueType> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DisirValueType,
        };
        unsafe {
            instance.get(DisirValueType::new)
        }
    }

    // bool string_type = 1;

    pub fn clear_string_type(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_string_type(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(DisirValueType_oneof_value::string_type(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_string_type(&mut self, v: bool) {
        self.value = ::std::option::Option::Some(DisirValueType_oneof_value::string_type(v))
    }

    pub fn get_string_type(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(DisirValueType_oneof_value::string_type(v)) => v,
            _ => false,
        }
    }

    // bool integer_type = 2;

    pub fn clear_integer_type(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_integer_type(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(DisirValueType_oneof_value::integer_type(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_integer_type(&mut self, v: bool) {
        self.value = ::std::option::Option::Some(DisirValueType_oneof_value::integer_type(v))
    }

    pub fn get_integer_type(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(DisirValueType_oneof_value::integer_type(v)) => v,
            _ => false,
        }
    }

    // bool float_type = 3;

    pub fn clear_float_type(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_float_type(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(DisirValueType_oneof_value::float_type(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_float_type(&mut self, v: bool) {
        self.value = ::std::option::Option::Some(DisirValueType_oneof_value::float_type(v))
    }

    pub fn get_float_type(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(DisirValueType_oneof_value::float_type(v)) => v,
            _ => false,
        }
    }

    // bool boolean_type = 4;

    pub fn clear_boolean_type(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_boolean_type(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(DisirValueType_oneof_value::boolean_type(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_boolean_type(&mut self, v: bool) {
        self.value = ::std::option::Option::Some(DisirValueType_oneof_value::boolean_type(v))
    }

    pub fn get_boolean_type(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(DisirValueType_oneof_value::boolean_type(v)) => v,
            _ => false,
        }
    }

    // bool enum_type = 5;

    pub fn clear_enum_type(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_enum_type(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(DisirValueType_oneof_value::enum_type(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_enum_type(&mut self, v: bool) {
        self.value = ::std::option::Option::Some(DisirValueType_oneof_value::enum_type(v))
    }

    pub fn get_enum_type(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(DisirValueType_oneof_value::enum_type(v)) => v,
            _ => false,
        }
    }
}

impl ::protobuf::Message for DisirValueType {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(DisirValueType_oneof_value::string_type(is.read_bool()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(DisirValueType_oneof_value::integer_type(is.read_bool()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(DisirValueType_oneof_value::float_type(is.read_bool()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(DisirValueType_oneof_value::boolean_type(is.read_bool()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(DisirValueType_oneof_value::enum_type(is.read_bool()?));
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let ::std::option::Option::Some(ref v) = self.value {
            match v {
                &DisirValueType_oneof_value::string_type(v) => {
                    my_size += 2;
                },
                &DisirValueType_oneof_value::integer_type(v) => {
                    my_size += 2;
                },
                &DisirValueType_oneof_value::float_type(v) => {
                    my_size += 2;
                },
                &DisirValueType_oneof_value::boolean_type(v) => {
                    my_size += 2;
                },
                &DisirValueType_oneof_value::enum_type(v) => {
                    my_size += 2;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.value {
            match v {
                &DisirValueType_oneof_value::string_type(v) => {
                    os.write_bool(1, v)?;
                },
                &DisirValueType_oneof_value::integer_type(v) => {
                    os.write_bool(2, v)?;
                },
                &DisirValueType_oneof_value::float_type(v) => {
                    os.write_bool(3, v)?;
                },
                &DisirValueType_oneof_value::boolean_type(v) => {
                    os.write_bool(4, v)?;
                },
                &DisirValueType_oneof_value::enum_type(v) => {
                    os.write_bool(5, v)?;
                },
            };
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DisirValueType {
    fn new() -> DisirValueType {
        DisirValueType::new()
    }

    fn descriptor_static(_: ::std::option::Option<DisirValueType>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor::<_>(
                    "string_type",
                    DisirValueType::has_string_type,
                    DisirValueType::get_string_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor::<_>(
                    "integer_type",
                    DisirValueType::has_integer_type,
                    DisirValueType::get_integer_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor::<_>(
                    "float_type",
                    DisirValueType::has_float_type,
                    DisirValueType::get_float_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor::<_>(
                    "boolean_type",
                    DisirValueType::has_boolean_type,
                    DisirValueType::get_boolean_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor::<_>(
                    "enum_type",
                    DisirValueType::has_enum_type,
                    DisirValueType::get_enum_type,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DisirValueType>(
                    "DisirValueType",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DisirValueType {
    fn clear(&mut self) {
        self.clear_string_type();
        self.clear_integer_type();
        self.clear_float_type();
        self.clear_boolean_type();
        self.clear_enum_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DisirValueType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DisirValueType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DisirMoldInput {
    // message fields
    pub field_type: ::std::string::String,
    pub id: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DisirMoldInput {}

impl DisirMoldInput {
    pub fn new() -> DisirMoldInput {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DisirMoldInput {
        static mut instance: ::protobuf::lazy::Lazy<DisirMoldInput> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DisirMoldInput,
        };
        unsafe {
            instance.get(DisirMoldInput::new)
        }
    }

    // string type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ::std::string::String) {
        self.field_type = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_type(&mut self) -> &mut ::std::string::String {
        &mut self.field_type
    }

    // Take field
    pub fn take_field_type(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.field_type, ::std::string::String::new())
    }

    pub fn get_field_type(&self) -> &str {
        &self.field_type
    }

    fn get_field_type_for_reflect(&self) -> &::std::string::String {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.field_type
    }

    // string id = 2;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.id, ::std::string::String::new())
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    fn get_id_for_reflect(&self) -> &::std::string::String {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }
}

impl ::protobuf::Message for DisirMoldInput {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.field_type)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.id)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.field_type.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.field_type);
        };
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.id);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.field_type.is_empty() {
            os.write_string(1, &self.field_type)?;
        };
        if !self.id.is_empty() {
            os.write_string(2, &self.id)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DisirMoldInput {
    fn new() -> DisirMoldInput {
        DisirMoldInput::new()
    }

    fn descriptor_static(_: ::std::option::Option<DisirMoldInput>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "type",
                    DisirMoldInput::get_field_type_for_reflect,
                    DisirMoldInput::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "id",
                    DisirMoldInput::get_id_for_reflect,
                    DisirMoldInput::mut_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DisirMoldInput>(
                    "DisirMoldInput",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DisirMoldInput {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DisirMoldInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DisirMoldInput {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DisirConfigInput {
    // message fields
    pub field_type: ::std::string::String,
    pub id: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DisirConfigInput {}

impl DisirConfigInput {
    pub fn new() -> DisirConfigInput {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DisirConfigInput {
        static mut instance: ::protobuf::lazy::Lazy<DisirConfigInput> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DisirConfigInput,
        };
        unsafe {
            instance.get(DisirConfigInput::new)
        }
    }

    // string type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ::std::string::String) {
        self.field_type = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_type(&mut self) -> &mut ::std::string::String {
        &mut self.field_type
    }

    // Take field
    pub fn take_field_type(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.field_type, ::std::string::String::new())
    }

    pub fn get_field_type(&self) -> &str {
        &self.field_type
    }

    fn get_field_type_for_reflect(&self) -> &::std::string::String {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.field_type
    }

    // string id = 2;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.id, ::std::string::String::new())
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    fn get_id_for_reflect(&self) -> &::std::string::String {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }
}

impl ::protobuf::Message for DisirConfigInput {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.field_type)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.id)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.field_type.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.field_type);
        };
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.id);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.field_type.is_empty() {
            os.write_string(1, &self.field_type)?;
        };
        if !self.id.is_empty() {
            os.write_string(2, &self.id)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DisirConfigInput {
    fn new() -> DisirConfigInput {
        DisirConfigInput::new()
    }

    fn descriptor_static(_: ::std::option::Option<DisirConfigInput>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "type",
                    DisirConfigInput::get_field_type_for_reflect,
                    DisirConfigInput::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "id",
                    DisirConfigInput::get_id_for_reflect,
                    DisirConfigInput::mut_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DisirConfigInput>(
                    "DisirConfigInput",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DisirConfigInput {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DisirConfigInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DisirConfigInput {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DisirMold {
    // message fields
    pub mold_id: u64,
    version: ::protobuf::SingularPtrField<SemanticVersion>,
    pub documentation: ::std::string::String,
    element_ids: ::std::vec::Vec<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DisirMold {}

impl DisirMold {
    pub fn new() -> DisirMold {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DisirMold {
        static mut instance: ::protobuf::lazy::Lazy<DisirMold> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DisirMold,
        };
        unsafe {
            instance.get(DisirMold::new)
        }
    }

    // uint64 mold_id = 1;

    pub fn clear_mold_id(&mut self) {
        self.mold_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_mold_id(&mut self, v: u64) {
        self.mold_id = v;
    }

    pub fn get_mold_id(&self) -> u64 {
        self.mold_id
    }

    fn get_mold_id_for_reflect(&self) -> &u64 {
        &self.mold_id
    }

    fn mut_mold_id_for_reflect(&mut self) -> &mut u64 {
        &mut self.mold_id
    }

    // .disirpb.SemanticVersion version = 2;

    pub fn clear_version(&mut self) {
        self.version.clear();
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: SemanticVersion) {
        self.version = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_version(&mut self) -> &mut SemanticVersion {
        if self.version.is_none() {
            self.version.set_default();
        };
        self.version.as_mut().unwrap()
    }

    // Take field
    pub fn take_version(&mut self) -> SemanticVersion {
        self.version.take().unwrap_or_else(|| SemanticVersion::new())
    }

    pub fn get_version(&self) -> &SemanticVersion {
        self.version.as_ref().unwrap_or_else(|| SemanticVersion::default_instance())
    }

    fn get_version_for_reflect(&self) -> &::protobuf::SingularPtrField<SemanticVersion> {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SemanticVersion> {
        &mut self.version
    }

    // string documentation = 3;

    pub fn clear_documentation(&mut self) {
        self.documentation.clear();
    }

    // Param is passed by value, moved
    pub fn set_documentation(&mut self, v: ::std::string::String) {
        self.documentation = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_documentation(&mut self) -> &mut ::std::string::String {
        &mut self.documentation
    }

    // Take field
    pub fn take_documentation(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.documentation, ::std::string::String::new())
    }

    pub fn get_documentation(&self) -> &str {
        &self.documentation
    }

    fn get_documentation_for_reflect(&self) -> &::std::string::String {
        &self.documentation
    }

    fn mut_documentation_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.documentation
    }

    // repeated uint64 element_ids = 4;

    pub fn clear_element_ids(&mut self) {
        self.element_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_element_ids(&mut self, v: ::std::vec::Vec<u64>) {
        self.element_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_element_ids(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.element_ids
    }

    // Take field
    pub fn take_element_ids(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.element_ids, ::std::vec::Vec::new())
    }

    pub fn get_element_ids(&self) -> &[u64] {
        &self.element_ids
    }

    fn get_element_ids_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.element_ids
    }

    fn mut_element_ids_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.element_ids
    }
}

impl ::protobuf::Message for DisirMold {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.mold_id = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.version)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.documentation)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.element_ids)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.mold_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.mold_id, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.version.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if !self.documentation.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.documentation);
        };
        for value in &self.element_ids {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.mold_id != 0 {
            os.write_uint64(1, self.mold_id)?;
        };
        if let Some(v) = self.version.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if !self.documentation.is_empty() {
            os.write_string(3, &self.documentation)?;
        };
        for v in &self.element_ids {
            os.write_uint64(4, *v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DisirMold {
    fn new() -> DisirMold {
        DisirMold::new()
    }

    fn descriptor_static(_: ::std::option::Option<DisirMold>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "mold_id",
                    DisirMold::get_mold_id_for_reflect,
                    DisirMold::mut_mold_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SemanticVersion>>(
                    "version",
                    DisirMold::get_version_for_reflect,
                    DisirMold::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "documentation",
                    DisirMold::get_documentation_for_reflect,
                    DisirMold::mut_documentation_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "element_ids",
                    DisirMold::get_element_ids_for_reflect,
                    DisirMold::mut_element_ids_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DisirMold>(
                    "DisirMold",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DisirMold {
    fn clear(&mut self) {
        self.clear_mold_id();
        self.clear_version();
        self.clear_documentation();
        self.clear_element_ids();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DisirMold {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DisirMold {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DisirConfig {
    // message fields
    pub config_id: u64,
    version: ::protobuf::SingularPtrField<SemanticVersion>,
    element_ids: ::std::vec::Vec<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DisirConfig {}

impl DisirConfig {
    pub fn new() -> DisirConfig {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DisirConfig {
        static mut instance: ::protobuf::lazy::Lazy<DisirConfig> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DisirConfig,
        };
        unsafe {
            instance.get(DisirConfig::new)
        }
    }

    // uint64 config_id = 1;

    pub fn clear_config_id(&mut self) {
        self.config_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_config_id(&mut self, v: u64) {
        self.config_id = v;
    }

    pub fn get_config_id(&self) -> u64 {
        self.config_id
    }

    fn get_config_id_for_reflect(&self) -> &u64 {
        &self.config_id
    }

    fn mut_config_id_for_reflect(&mut self) -> &mut u64 {
        &mut self.config_id
    }

    // .disirpb.SemanticVersion version = 2;

    pub fn clear_version(&mut self) {
        self.version.clear();
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: SemanticVersion) {
        self.version = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_version(&mut self) -> &mut SemanticVersion {
        if self.version.is_none() {
            self.version.set_default();
        };
        self.version.as_mut().unwrap()
    }

    // Take field
    pub fn take_version(&mut self) -> SemanticVersion {
        self.version.take().unwrap_or_else(|| SemanticVersion::new())
    }

    pub fn get_version(&self) -> &SemanticVersion {
        self.version.as_ref().unwrap_or_else(|| SemanticVersion::default_instance())
    }

    fn get_version_for_reflect(&self) -> &::protobuf::SingularPtrField<SemanticVersion> {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SemanticVersion> {
        &mut self.version
    }

    // repeated uint64 element_ids = 3;

    pub fn clear_element_ids(&mut self) {
        self.element_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_element_ids(&mut self, v: ::std::vec::Vec<u64>) {
        self.element_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_element_ids(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.element_ids
    }

    // Take field
    pub fn take_element_ids(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.element_ids, ::std::vec::Vec::new())
    }

    pub fn get_element_ids(&self) -> &[u64] {
        &self.element_ids
    }

    fn get_element_ids_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.element_ids
    }

    fn mut_element_ids_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.element_ids
    }
}

impl ::protobuf::Message for DisirConfig {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.config_id = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.version)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.element_ids)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.config_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.config_id, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.version.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.element_ids {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.config_id != 0 {
            os.write_uint64(1, self.config_id)?;
        };
        if let Some(v) = self.version.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.element_ids {
            os.write_uint64(3, *v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DisirConfig {
    fn new() -> DisirConfig {
        DisirConfig::new()
    }

    fn descriptor_static(_: ::std::option::Option<DisirConfig>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "config_id",
                    DisirConfig::get_config_id_for_reflect,
                    DisirConfig::mut_config_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SemanticVersion>>(
                    "version",
                    DisirConfig::get_version_for_reflect,
                    DisirConfig::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "element_ids",
                    DisirConfig::get_element_ids_for_reflect,
                    DisirConfig::mut_element_ids_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DisirConfig>(
                    "DisirConfig",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DisirConfig {
    fn clear(&mut self) {
        self.clear_config_id();
        self.clear_version();
        self.clear_element_ids();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DisirConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DisirConfig {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MoldKeyval {
    // message fields
    pub keyval_id: u64,
    pub name: ::std::string::String,
    field_type: ::protobuf::SingularPtrField<DisirValueType>,
    introduced: ::protobuf::SingularPtrField<SemanticVersion>,
    deprecated: ::protobuf::SingularPtrField<SemanticVersion>,
    pub documentation: ::std::string::String,
    defaults: ::protobuf::RepeatedField<DisirDefault>,
    pub invalid: bool,
    // message oneof groups
    parent: ::std::option::Option<MoldKeyval_oneof_parent>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MoldKeyval {}

#[derive(Clone,PartialEq)]
pub enum MoldKeyval_oneof_parent {
    parent_section_id(u64),
    parent_mold_id(u64),
}

impl MoldKeyval {
    pub fn new() -> MoldKeyval {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MoldKeyval {
        static mut instance: ::protobuf::lazy::Lazy<MoldKeyval> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MoldKeyval,
        };
        unsafe {
            instance.get(MoldKeyval::new)
        }
    }

    // uint64 keyval_id = 1;

    pub fn clear_keyval_id(&mut self) {
        self.keyval_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_keyval_id(&mut self, v: u64) {
        self.keyval_id = v;
    }

    pub fn get_keyval_id(&self) -> u64 {
        self.keyval_id
    }

    fn get_keyval_id_for_reflect(&self) -> &u64 {
        &self.keyval_id
    }

    fn mut_keyval_id_for_reflect(&mut self) -> &mut u64 {
        &mut self.keyval_id
    }

    // uint64 parent_section_id = 20;

    pub fn clear_parent_section_id(&mut self) {
        self.parent = ::std::option::Option::None;
    }

    pub fn has_parent_section_id(&self) -> bool {
        match self.parent {
            ::std::option::Option::Some(MoldKeyval_oneof_parent::parent_section_id(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_parent_section_id(&mut self, v: u64) {
        self.parent = ::std::option::Option::Some(MoldKeyval_oneof_parent::parent_section_id(v))
    }

    pub fn get_parent_section_id(&self) -> u64 {
        match self.parent {
            ::std::option::Option::Some(MoldKeyval_oneof_parent::parent_section_id(v)) => v,
            _ => 0,
        }
    }

    // uint64 parent_mold_id = 21;

    pub fn clear_parent_mold_id(&mut self) {
        self.parent = ::std::option::Option::None;
    }

    pub fn has_parent_mold_id(&self) -> bool {
        match self.parent {
            ::std::option::Option::Some(MoldKeyval_oneof_parent::parent_mold_id(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_parent_mold_id(&mut self, v: u64) {
        self.parent = ::std::option::Option::Some(MoldKeyval_oneof_parent::parent_mold_id(v))
    }

    pub fn get_parent_mold_id(&self) -> u64 {
        match self.parent {
            ::std::option::Option::Some(MoldKeyval_oneof_parent::parent_mold_id(v)) => v,
            _ => 0,
        }
    }

    // string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // .disirpb.DisirValueType type = 3;

    pub fn clear_field_type(&mut self) {
        self.field_type.clear();
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: DisirValueType) {
        self.field_type = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_type(&mut self) -> &mut DisirValueType {
        if self.field_type.is_none() {
            self.field_type.set_default();
        };
        self.field_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_field_type(&mut self) -> DisirValueType {
        self.field_type.take().unwrap_or_else(|| DisirValueType::new())
    }

    pub fn get_field_type(&self) -> &DisirValueType {
        self.field_type.as_ref().unwrap_or_else(|| DisirValueType::default_instance())
    }

    fn get_field_type_for_reflect(&self) -> &::protobuf::SingularPtrField<DisirValueType> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DisirValueType> {
        &mut self.field_type
    }

    // .disirpb.SemanticVersion introduced = 4;

    pub fn clear_introduced(&mut self) {
        self.introduced.clear();
    }

    pub fn has_introduced(&self) -> bool {
        self.introduced.is_some()
    }

    // Param is passed by value, moved
    pub fn set_introduced(&mut self, v: SemanticVersion) {
        self.introduced = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_introduced(&mut self) -> &mut SemanticVersion {
        if self.introduced.is_none() {
            self.introduced.set_default();
        };
        self.introduced.as_mut().unwrap()
    }

    // Take field
    pub fn take_introduced(&mut self) -> SemanticVersion {
        self.introduced.take().unwrap_or_else(|| SemanticVersion::new())
    }

    pub fn get_introduced(&self) -> &SemanticVersion {
        self.introduced.as_ref().unwrap_or_else(|| SemanticVersion::default_instance())
    }

    fn get_introduced_for_reflect(&self) -> &::protobuf::SingularPtrField<SemanticVersion> {
        &self.introduced
    }

    fn mut_introduced_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SemanticVersion> {
        &mut self.introduced
    }

    // .disirpb.SemanticVersion deprecated = 5;

    pub fn clear_deprecated(&mut self) {
        self.deprecated.clear();
    }

    pub fn has_deprecated(&self) -> bool {
        self.deprecated.is_some()
    }

    // Param is passed by value, moved
    pub fn set_deprecated(&mut self, v: SemanticVersion) {
        self.deprecated = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_deprecated(&mut self) -> &mut SemanticVersion {
        if self.deprecated.is_none() {
            self.deprecated.set_default();
        };
        self.deprecated.as_mut().unwrap()
    }

    // Take field
    pub fn take_deprecated(&mut self) -> SemanticVersion {
        self.deprecated.take().unwrap_or_else(|| SemanticVersion::new())
    }

    pub fn get_deprecated(&self) -> &SemanticVersion {
        self.deprecated.as_ref().unwrap_or_else(|| SemanticVersion::default_instance())
    }

    fn get_deprecated_for_reflect(&self) -> &::protobuf::SingularPtrField<SemanticVersion> {
        &self.deprecated
    }

    fn mut_deprecated_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SemanticVersion> {
        &mut self.deprecated
    }

    // string documentation = 6;

    pub fn clear_documentation(&mut self) {
        self.documentation.clear();
    }

    // Param is passed by value, moved
    pub fn set_documentation(&mut self, v: ::std::string::String) {
        self.documentation = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_documentation(&mut self) -> &mut ::std::string::String {
        &mut self.documentation
    }

    // Take field
    pub fn take_documentation(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.documentation, ::std::string::String::new())
    }

    pub fn get_documentation(&self) -> &str {
        &self.documentation
    }

    fn get_documentation_for_reflect(&self) -> &::std::string::String {
        &self.documentation
    }

    fn mut_documentation_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.documentation
    }

    // repeated .disirpb.DisirDefault defaults = 7;

    pub fn clear_defaults(&mut self) {
        self.defaults.clear();
    }

    // Param is passed by value, moved
    pub fn set_defaults(&mut self, v: ::protobuf::RepeatedField<DisirDefault>) {
        self.defaults = v;
    }

    // Mutable pointer to the field.
    pub fn mut_defaults(&mut self) -> &mut ::protobuf::RepeatedField<DisirDefault> {
        &mut self.defaults
    }

    // Take field
    pub fn take_defaults(&mut self) -> ::protobuf::RepeatedField<DisirDefault> {
        ::std::mem::replace(&mut self.defaults, ::protobuf::RepeatedField::new())
    }

    pub fn get_defaults(&self) -> &[DisirDefault] {
        &self.defaults
    }

    fn get_defaults_for_reflect(&self) -> &::protobuf::RepeatedField<DisirDefault> {
        &self.defaults
    }

    fn mut_defaults_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<DisirDefault> {
        &mut self.defaults
    }

    // bool invalid = 10;

    pub fn clear_invalid(&mut self) {
        self.invalid = false;
    }

    // Param is passed by value, moved
    pub fn set_invalid(&mut self, v: bool) {
        self.invalid = v;
    }

    pub fn get_invalid(&self) -> bool {
        self.invalid
    }

    fn get_invalid_for_reflect(&self) -> &bool {
        &self.invalid
    }

    fn mut_invalid_for_reflect(&mut self) -> &mut bool {
        &mut self.invalid
    }
}

impl ::protobuf::Message for MoldKeyval {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.keyval_id = tmp;
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.parent = ::std::option::Option::Some(MoldKeyval_oneof_parent::parent_section_id(is.read_uint64()?));
                },
                21 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.parent = ::std::option::Option::Some(MoldKeyval_oneof_parent::parent_mold_id(is.read_uint64()?));
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.field_type)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.introduced)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.deprecated)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.documentation)?;
                },
                7 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.defaults)?;
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.invalid = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.keyval_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.keyval_id, ::protobuf::wire_format::WireTypeVarint);
        };
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.name);
        };
        if let Some(v) = self.field_type.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.introduced.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.deprecated.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if !self.documentation.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.documentation);
        };
        for value in &self.defaults {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.invalid != false {
            my_size += 2;
        };
        if let ::std::option::Option::Some(ref v) = self.parent {
            match v {
                &MoldKeyval_oneof_parent::parent_section_id(v) => {
                    my_size += ::protobuf::rt::value_size(20, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &MoldKeyval_oneof_parent::parent_mold_id(v) => {
                    my_size += ::protobuf::rt::value_size(21, v, ::protobuf::wire_format::WireTypeVarint);
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.keyval_id != 0 {
            os.write_uint64(1, self.keyval_id)?;
        };
        if !self.name.is_empty() {
            os.write_string(2, &self.name)?;
        };
        if let Some(v) = self.field_type.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.introduced.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.deprecated.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if !self.documentation.is_empty() {
            os.write_string(6, &self.documentation)?;
        };
        for v in &self.defaults {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.invalid != false {
            os.write_bool(10, self.invalid)?;
        };
        if let ::std::option::Option::Some(ref v) = self.parent {
            match v {
                &MoldKeyval_oneof_parent::parent_section_id(v) => {
                    os.write_uint64(20, v)?;
                },
                &MoldKeyval_oneof_parent::parent_mold_id(v) => {
                    os.write_uint64(21, v)?;
                },
            };
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for MoldKeyval {
    fn new() -> MoldKeyval {
        MoldKeyval::new()
    }

    fn descriptor_static(_: ::std::option::Option<MoldKeyval>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "keyval_id",
                    MoldKeyval::get_keyval_id_for_reflect,
                    MoldKeyval::mut_keyval_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor::<_>(
                    "parent_section_id",
                    MoldKeyval::has_parent_section_id,
                    MoldKeyval::get_parent_section_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor::<_>(
                    "parent_mold_id",
                    MoldKeyval::has_parent_mold_id,
                    MoldKeyval::get_parent_mold_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    MoldKeyval::get_name_for_reflect,
                    MoldKeyval::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DisirValueType>>(
                    "type",
                    MoldKeyval::get_field_type_for_reflect,
                    MoldKeyval::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SemanticVersion>>(
                    "introduced",
                    MoldKeyval::get_introduced_for_reflect,
                    MoldKeyval::mut_introduced_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SemanticVersion>>(
                    "deprecated",
                    MoldKeyval::get_deprecated_for_reflect,
                    MoldKeyval::mut_deprecated_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "documentation",
                    MoldKeyval::get_documentation_for_reflect,
                    MoldKeyval::mut_documentation_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DisirDefault>>(
                    "defaults",
                    MoldKeyval::get_defaults_for_reflect,
                    MoldKeyval::mut_defaults_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "invalid",
                    MoldKeyval::get_invalid_for_reflect,
                    MoldKeyval::mut_invalid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MoldKeyval>(
                    "MoldKeyval",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MoldKeyval {
    fn clear(&mut self) {
        self.clear_keyval_id();
        self.clear_parent_section_id();
        self.clear_parent_mold_id();
        self.clear_name();
        self.clear_field_type();
        self.clear_introduced();
        self.clear_deprecated();
        self.clear_documentation();
        self.clear_defaults();
        self.clear_invalid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MoldKeyval {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MoldKeyval {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ConfigKeyval {
    // message fields
    pub keyval_id: u64,
    pub name: ::std::string::String,
    value: ::protobuf::SingularPtrField<DisirValue>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ConfigKeyval {}

impl ConfigKeyval {
    pub fn new() -> ConfigKeyval {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ConfigKeyval {
        static mut instance: ::protobuf::lazy::Lazy<ConfigKeyval> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ConfigKeyval,
        };
        unsafe {
            instance.get(ConfigKeyval::new)
        }
    }

    // uint64 keyval_id = 1;

    pub fn clear_keyval_id(&mut self) {
        self.keyval_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_keyval_id(&mut self, v: u64) {
        self.keyval_id = v;
    }

    pub fn get_keyval_id(&self) -> u64 {
        self.keyval_id
    }

    fn get_keyval_id_for_reflect(&self) -> &u64 {
        &self.keyval_id
    }

    fn mut_keyval_id_for_reflect(&mut self) -> &mut u64 {
        &mut self.keyval_id
    }

    // string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // .disirpb.DisirValue value = 3;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: DisirValue) {
        self.value = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut DisirValue {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> DisirValue {
        self.value.take().unwrap_or_else(|| DisirValue::new())
    }

    pub fn get_value(&self) -> &DisirValue {
        self.value.as_ref().unwrap_or_else(|| DisirValue::default_instance())
    }

    fn get_value_for_reflect(&self) -> &::protobuf::SingularPtrField<DisirValue> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DisirValue> {
        &mut self.value
    }
}

impl ::protobuf::Message for ConfigKeyval {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.keyval_id = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.value)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.keyval_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.keyval_id, ::protobuf::wire_format::WireTypeVarint);
        };
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.name);
        };
        if let Some(v) = self.value.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.keyval_id != 0 {
            os.write_uint64(1, self.keyval_id)?;
        };
        if !self.name.is_empty() {
            os.write_string(2, &self.name)?;
        };
        if let Some(v) = self.value.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ConfigKeyval {
    fn new() -> ConfigKeyval {
        ConfigKeyval::new()
    }

    fn descriptor_static(_: ::std::option::Option<ConfigKeyval>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "keyval_id",
                    ConfigKeyval::get_keyval_id_for_reflect,
                    ConfigKeyval::mut_keyval_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    ConfigKeyval::get_name_for_reflect,
                    ConfigKeyval::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DisirValue>>(
                    "value",
                    ConfigKeyval::get_value_for_reflect,
                    ConfigKeyval::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ConfigKeyval>(
                    "ConfigKeyval",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ConfigKeyval {
    fn clear(&mut self) {
        self.clear_keyval_id();
        self.clear_name();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ConfigKeyval {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ConfigKeyval {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MoldSection {
    // message fields
    pub section_id: u64,
    pub name: ::std::string::String,
    introduced: ::protobuf::SingularPtrField<SemanticVersion>,
    deprecated: ::protobuf::SingularPtrField<SemanticVersion>,
    pub documentation: ::std::string::String,
    element_ids: ::std::vec::Vec<u64>,
    // message oneof groups
    parent: ::std::option::Option<MoldSection_oneof_parent>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MoldSection {}

#[derive(Clone,PartialEq)]
pub enum MoldSection_oneof_parent {
    parent_section_id(u64),
    parent_mold_id(u64),
}

impl MoldSection {
    pub fn new() -> MoldSection {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MoldSection {
        static mut instance: ::protobuf::lazy::Lazy<MoldSection> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MoldSection,
        };
        unsafe {
            instance.get(MoldSection::new)
        }
    }

    // uint64 section_id = 1;

    pub fn clear_section_id(&mut self) {
        self.section_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_section_id(&mut self, v: u64) {
        self.section_id = v;
    }

    pub fn get_section_id(&self) -> u64 {
        self.section_id
    }

    fn get_section_id_for_reflect(&self) -> &u64 {
        &self.section_id
    }

    fn mut_section_id_for_reflect(&mut self) -> &mut u64 {
        &mut self.section_id
    }

    // uint64 parent_section_id = 10;

    pub fn clear_parent_section_id(&mut self) {
        self.parent = ::std::option::Option::None;
    }

    pub fn has_parent_section_id(&self) -> bool {
        match self.parent {
            ::std::option::Option::Some(MoldSection_oneof_parent::parent_section_id(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_parent_section_id(&mut self, v: u64) {
        self.parent = ::std::option::Option::Some(MoldSection_oneof_parent::parent_section_id(v))
    }

    pub fn get_parent_section_id(&self) -> u64 {
        match self.parent {
            ::std::option::Option::Some(MoldSection_oneof_parent::parent_section_id(v)) => v,
            _ => 0,
        }
    }

    // uint64 parent_mold_id = 11;

    pub fn clear_parent_mold_id(&mut self) {
        self.parent = ::std::option::Option::None;
    }

    pub fn has_parent_mold_id(&self) -> bool {
        match self.parent {
            ::std::option::Option::Some(MoldSection_oneof_parent::parent_mold_id(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_parent_mold_id(&mut self, v: u64) {
        self.parent = ::std::option::Option::Some(MoldSection_oneof_parent::parent_mold_id(v))
    }

    pub fn get_parent_mold_id(&self) -> u64 {
        match self.parent {
            ::std::option::Option::Some(MoldSection_oneof_parent::parent_mold_id(v)) => v,
            _ => 0,
        }
    }

    // string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // .disirpb.SemanticVersion introduced = 3;

    pub fn clear_introduced(&mut self) {
        self.introduced.clear();
    }

    pub fn has_introduced(&self) -> bool {
        self.introduced.is_some()
    }

    // Param is passed by value, moved
    pub fn set_introduced(&mut self, v: SemanticVersion) {
        self.introduced = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_introduced(&mut self) -> &mut SemanticVersion {
        if self.introduced.is_none() {
            self.introduced.set_default();
        };
        self.introduced.as_mut().unwrap()
    }

    // Take field
    pub fn take_introduced(&mut self) -> SemanticVersion {
        self.introduced.take().unwrap_or_else(|| SemanticVersion::new())
    }

    pub fn get_introduced(&self) -> &SemanticVersion {
        self.introduced.as_ref().unwrap_or_else(|| SemanticVersion::default_instance())
    }

    fn get_introduced_for_reflect(&self) -> &::protobuf::SingularPtrField<SemanticVersion> {
        &self.introduced
    }

    fn mut_introduced_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SemanticVersion> {
        &mut self.introduced
    }

    // .disirpb.SemanticVersion deprecated = 4;

    pub fn clear_deprecated(&mut self) {
        self.deprecated.clear();
    }

    pub fn has_deprecated(&self) -> bool {
        self.deprecated.is_some()
    }

    // Param is passed by value, moved
    pub fn set_deprecated(&mut self, v: SemanticVersion) {
        self.deprecated = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_deprecated(&mut self) -> &mut SemanticVersion {
        if self.deprecated.is_none() {
            self.deprecated.set_default();
        };
        self.deprecated.as_mut().unwrap()
    }

    // Take field
    pub fn take_deprecated(&mut self) -> SemanticVersion {
        self.deprecated.take().unwrap_or_else(|| SemanticVersion::new())
    }

    pub fn get_deprecated(&self) -> &SemanticVersion {
        self.deprecated.as_ref().unwrap_or_else(|| SemanticVersion::default_instance())
    }

    fn get_deprecated_for_reflect(&self) -> &::protobuf::SingularPtrField<SemanticVersion> {
        &self.deprecated
    }

    fn mut_deprecated_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SemanticVersion> {
        &mut self.deprecated
    }

    // string documentation = 5;

    pub fn clear_documentation(&mut self) {
        self.documentation.clear();
    }

    // Param is passed by value, moved
    pub fn set_documentation(&mut self, v: ::std::string::String) {
        self.documentation = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_documentation(&mut self) -> &mut ::std::string::String {
        &mut self.documentation
    }

    // Take field
    pub fn take_documentation(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.documentation, ::std::string::String::new())
    }

    pub fn get_documentation(&self) -> &str {
        &self.documentation
    }

    fn get_documentation_for_reflect(&self) -> &::std::string::String {
        &self.documentation
    }

    fn mut_documentation_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.documentation
    }

    // repeated uint64 element_ids = 6;

    pub fn clear_element_ids(&mut self) {
        self.element_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_element_ids(&mut self, v: ::std::vec::Vec<u64>) {
        self.element_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_element_ids(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.element_ids
    }

    // Take field
    pub fn take_element_ids(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.element_ids, ::std::vec::Vec::new())
    }

    pub fn get_element_ids(&self) -> &[u64] {
        &self.element_ids
    }

    fn get_element_ids_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.element_ids
    }

    fn mut_element_ids_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.element_ids
    }
}

impl ::protobuf::Message for MoldSection {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.section_id = tmp;
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.parent = ::std::option::Option::Some(MoldSection_oneof_parent::parent_section_id(is.read_uint64()?));
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.parent = ::std::option::Option::Some(MoldSection_oneof_parent::parent_mold_id(is.read_uint64()?));
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.introduced)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.deprecated)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.documentation)?;
                },
                6 => {
                    ::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.element_ids)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.section_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.section_id, ::protobuf::wire_format::WireTypeVarint);
        };
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.name);
        };
        if let Some(v) = self.introduced.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.deprecated.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if !self.documentation.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.documentation);
        };
        for value in &self.element_ids {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if let ::std::option::Option::Some(ref v) = self.parent {
            match v {
                &MoldSection_oneof_parent::parent_section_id(v) => {
                    my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &MoldSection_oneof_parent::parent_mold_id(v) => {
                    my_size += ::protobuf::rt::value_size(11, v, ::protobuf::wire_format::WireTypeVarint);
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.section_id != 0 {
            os.write_uint64(1, self.section_id)?;
        };
        if !self.name.is_empty() {
            os.write_string(2, &self.name)?;
        };
        if let Some(v) = self.introduced.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.deprecated.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if !self.documentation.is_empty() {
            os.write_string(5, &self.documentation)?;
        };
        for v in &self.element_ids {
            os.write_uint64(6, *v)?;
        };
        if let ::std::option::Option::Some(ref v) = self.parent {
            match v {
                &MoldSection_oneof_parent::parent_section_id(v) => {
                    os.write_uint64(10, v)?;
                },
                &MoldSection_oneof_parent::parent_mold_id(v) => {
                    os.write_uint64(11, v)?;
                },
            };
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for MoldSection {
    fn new() -> MoldSection {
        MoldSection::new()
    }

    fn descriptor_static(_: ::std::option::Option<MoldSection>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "section_id",
                    MoldSection::get_section_id_for_reflect,
                    MoldSection::mut_section_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor::<_>(
                    "parent_section_id",
                    MoldSection::has_parent_section_id,
                    MoldSection::get_parent_section_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor::<_>(
                    "parent_mold_id",
                    MoldSection::has_parent_mold_id,
                    MoldSection::get_parent_mold_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    MoldSection::get_name_for_reflect,
                    MoldSection::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SemanticVersion>>(
                    "introduced",
                    MoldSection::get_introduced_for_reflect,
                    MoldSection::mut_introduced_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SemanticVersion>>(
                    "deprecated",
                    MoldSection::get_deprecated_for_reflect,
                    MoldSection::mut_deprecated_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "documentation",
                    MoldSection::get_documentation_for_reflect,
                    MoldSection::mut_documentation_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "element_ids",
                    MoldSection::get_element_ids_for_reflect,
                    MoldSection::mut_element_ids_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MoldSection>(
                    "MoldSection",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MoldSection {
    fn clear(&mut self) {
        self.clear_section_id();
        self.clear_parent_section_id();
        self.clear_parent_mold_id();
        self.clear_name();
        self.clear_introduced();
        self.clear_deprecated();
        self.clear_documentation();
        self.clear_element_ids();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MoldSection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MoldSection {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ConfigSection {
    // message fields
    pub section_id: u64,
    pub name: ::std::string::String,
    element_ids: ::std::vec::Vec<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ConfigSection {}

impl ConfigSection {
    pub fn new() -> ConfigSection {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ConfigSection {
        static mut instance: ::protobuf::lazy::Lazy<ConfigSection> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ConfigSection,
        };
        unsafe {
            instance.get(ConfigSection::new)
        }
    }

    // uint64 section_id = 1;

    pub fn clear_section_id(&mut self) {
        self.section_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_section_id(&mut self, v: u64) {
        self.section_id = v;
    }

    pub fn get_section_id(&self) -> u64 {
        self.section_id
    }

    fn get_section_id_for_reflect(&self) -> &u64 {
        &self.section_id
    }

    fn mut_section_id_for_reflect(&mut self) -> &mut u64 {
        &mut self.section_id
    }

    // string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // repeated uint64 element_ids = 3;

    pub fn clear_element_ids(&mut self) {
        self.element_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_element_ids(&mut self, v: ::std::vec::Vec<u64>) {
        self.element_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_element_ids(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.element_ids
    }

    // Take field
    pub fn take_element_ids(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.element_ids, ::std::vec::Vec::new())
    }

    pub fn get_element_ids(&self) -> &[u64] {
        &self.element_ids
    }

    fn get_element_ids_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.element_ids
    }

    fn mut_element_ids_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.element_ids
    }
}

impl ::protobuf::Message for ConfigSection {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.section_id = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.element_ids)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.section_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.section_id, ::protobuf::wire_format::WireTypeVarint);
        };
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.name);
        };
        for value in &self.element_ids {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.section_id != 0 {
            os.write_uint64(1, self.section_id)?;
        };
        if !self.name.is_empty() {
            os.write_string(2, &self.name)?;
        };
        for v in &self.element_ids {
            os.write_uint64(3, *v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ConfigSection {
    fn new() -> ConfigSection {
        ConfigSection::new()
    }

    fn descriptor_static(_: ::std::option::Option<ConfigSection>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "section_id",
                    ConfigSection::get_section_id_for_reflect,
                    ConfigSection::mut_section_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    ConfigSection::get_name_for_reflect,
                    ConfigSection::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "element_ids",
                    ConfigSection::get_element_ids_for_reflect,
                    ConfigSection::mut_element_ids_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ConfigSection>(
                    "ConfigSection",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ConfigSection {
    fn clear(&mut self) {
        self.clear_section_id();
        self.clear_name();
        self.clear_element_ids();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ConfigSection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ConfigSection {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Element {
    // message oneof groups
    msg: ::std::option::Option<Element_oneof_msg>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Element {}

#[derive(Clone,PartialEq)]
pub enum Element_oneof_msg {
    mold_keyval(MoldKeyval),
    mold_section(MoldSection),
    config_keyval(ConfigKeyval),
    config_section(ConfigSection),
}

impl Element {
    pub fn new() -> Element {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Element {
        static mut instance: ::protobuf::lazy::Lazy<Element> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Element,
        };
        unsafe {
            instance.get(Element::new)
        }
    }

    // .disirpb.MoldKeyval mold_keyval = 1;

    pub fn clear_mold_keyval(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_mold_keyval(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(Element_oneof_msg::mold_keyval(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_mold_keyval(&mut self, v: MoldKeyval) {
        self.msg = ::std::option::Option::Some(Element_oneof_msg::mold_keyval(v))
    }

    // Mutable pointer to the field.
    pub fn mut_mold_keyval(&mut self) -> &mut MoldKeyval {
        if let ::std::option::Option::Some(Element_oneof_msg::mold_keyval(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(Element_oneof_msg::mold_keyval(MoldKeyval::new()));
        }
        match self.msg {
            ::std::option::Option::Some(Element_oneof_msg::mold_keyval(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_mold_keyval(&mut self) -> MoldKeyval {
        if self.has_mold_keyval() {
            match self.msg.take() {
                ::std::option::Option::Some(Element_oneof_msg::mold_keyval(v)) => v,
                _ => panic!(),
            }
        } else {
            MoldKeyval::new()
        }
    }

    pub fn get_mold_keyval(&self) -> &MoldKeyval {
        match self.msg {
            ::std::option::Option::Some(Element_oneof_msg::mold_keyval(ref v)) => v,
            _ => MoldKeyval::default_instance(),
        }
    }

    // .disirpb.MoldSection mold_section = 2;

    pub fn clear_mold_section(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_mold_section(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(Element_oneof_msg::mold_section(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_mold_section(&mut self, v: MoldSection) {
        self.msg = ::std::option::Option::Some(Element_oneof_msg::mold_section(v))
    }

    // Mutable pointer to the field.
    pub fn mut_mold_section(&mut self) -> &mut MoldSection {
        if let ::std::option::Option::Some(Element_oneof_msg::mold_section(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(Element_oneof_msg::mold_section(MoldSection::new()));
        }
        match self.msg {
            ::std::option::Option::Some(Element_oneof_msg::mold_section(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_mold_section(&mut self) -> MoldSection {
        if self.has_mold_section() {
            match self.msg.take() {
                ::std::option::Option::Some(Element_oneof_msg::mold_section(v)) => v,
                _ => panic!(),
            }
        } else {
            MoldSection::new()
        }
    }

    pub fn get_mold_section(&self) -> &MoldSection {
        match self.msg {
            ::std::option::Option::Some(Element_oneof_msg::mold_section(ref v)) => v,
            _ => MoldSection::default_instance(),
        }
    }

    // .disirpb.ConfigKeyval config_keyval = 3;

    pub fn clear_config_keyval(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_config_keyval(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(Element_oneof_msg::config_keyval(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_config_keyval(&mut self, v: ConfigKeyval) {
        self.msg = ::std::option::Option::Some(Element_oneof_msg::config_keyval(v))
    }

    // Mutable pointer to the field.
    pub fn mut_config_keyval(&mut self) -> &mut ConfigKeyval {
        if let ::std::option::Option::Some(Element_oneof_msg::config_keyval(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(Element_oneof_msg::config_keyval(ConfigKeyval::new()));
        }
        match self.msg {
            ::std::option::Option::Some(Element_oneof_msg::config_keyval(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_config_keyval(&mut self) -> ConfigKeyval {
        if self.has_config_keyval() {
            match self.msg.take() {
                ::std::option::Option::Some(Element_oneof_msg::config_keyval(v)) => v,
                _ => panic!(),
            }
        } else {
            ConfigKeyval::new()
        }
    }

    pub fn get_config_keyval(&self) -> &ConfigKeyval {
        match self.msg {
            ::std::option::Option::Some(Element_oneof_msg::config_keyval(ref v)) => v,
            _ => ConfigKeyval::default_instance(),
        }
    }

    // .disirpb.ConfigSection config_section = 4;

    pub fn clear_config_section(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_config_section(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(Element_oneof_msg::config_section(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_config_section(&mut self, v: ConfigSection) {
        self.msg = ::std::option::Option::Some(Element_oneof_msg::config_section(v))
    }

    // Mutable pointer to the field.
    pub fn mut_config_section(&mut self) -> &mut ConfigSection {
        if let ::std::option::Option::Some(Element_oneof_msg::config_section(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(Element_oneof_msg::config_section(ConfigSection::new()));
        }
        match self.msg {
            ::std::option::Option::Some(Element_oneof_msg::config_section(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_config_section(&mut self) -> ConfigSection {
        if self.has_config_section() {
            match self.msg.take() {
                ::std::option::Option::Some(Element_oneof_msg::config_section(v)) => v,
                _ => panic!(),
            }
        } else {
            ConfigSection::new()
        }
    }

    pub fn get_config_section(&self) -> &ConfigSection {
        match self.msg {
            ::std::option::Option::Some(Element_oneof_msg::config_section(ref v)) => v,
            _ => ConfigSection::default_instance(),
        }
    }
}

impl ::protobuf::Message for Element {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(Element_oneof_msg::mold_keyval(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(Element_oneof_msg::mold_section(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(Element_oneof_msg::config_keyval(is.read_message()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(Element_oneof_msg::config_section(is.read_message()?));
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let ::std::option::Option::Some(ref v) = self.msg {
            match v {
                &Element_oneof_msg::mold_keyval(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Element_oneof_msg::mold_section(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Element_oneof_msg::config_keyval(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Element_oneof_msg::config_section(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.msg {
            match v {
                &Element_oneof_msg::mold_keyval(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Element_oneof_msg::mold_section(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Element_oneof_msg::config_keyval(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Element_oneof_msg::config_section(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Element {
    fn new() -> Element {
        Element::new()
    }

    fn descriptor_static(_: ::std::option::Option<Element>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, MoldKeyval>(
                    "mold_keyval",
                    Element::has_mold_keyval,
                    Element::get_mold_keyval,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, MoldSection>(
                    "mold_section",
                    Element::has_mold_section,
                    Element::get_mold_section,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ConfigKeyval>(
                    "config_keyval",
                    Element::has_config_keyval,
                    Element::get_config_keyval,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ConfigSection>(
                    "config_section",
                    Element::has_config_section,
                    Element::get_config_section,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Element>(
                    "Element",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Element {
    fn clear(&mut self) {
        self.clear_mold_keyval();
        self.clear_mold_section();
        self.clear_config_keyval();
        self.clear_config_section();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Element {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Element {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DisirDefault {
    // message fields
    pub default_id: u64,
    introduced: ::protobuf::SingularPtrField<SemanticVersion>,
    value: ::protobuf::SingularPtrField<DisirValue>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DisirDefault {}

impl DisirDefault {
    pub fn new() -> DisirDefault {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DisirDefault {
        static mut instance: ::protobuf::lazy::Lazy<DisirDefault> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DisirDefault,
        };
        unsafe {
            instance.get(DisirDefault::new)
        }
    }

    // uint64 default_id = 1;

    pub fn clear_default_id(&mut self) {
        self.default_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_default_id(&mut self, v: u64) {
        self.default_id = v;
    }

    pub fn get_default_id(&self) -> u64 {
        self.default_id
    }

    fn get_default_id_for_reflect(&self) -> &u64 {
        &self.default_id
    }

    fn mut_default_id_for_reflect(&mut self) -> &mut u64 {
        &mut self.default_id
    }

    // .disirpb.SemanticVersion introduced = 2;

    pub fn clear_introduced(&mut self) {
        self.introduced.clear();
    }

    pub fn has_introduced(&self) -> bool {
        self.introduced.is_some()
    }

    // Param is passed by value, moved
    pub fn set_introduced(&mut self, v: SemanticVersion) {
        self.introduced = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_introduced(&mut self) -> &mut SemanticVersion {
        if self.introduced.is_none() {
            self.introduced.set_default();
        };
        self.introduced.as_mut().unwrap()
    }

    // Take field
    pub fn take_introduced(&mut self) -> SemanticVersion {
        self.introduced.take().unwrap_or_else(|| SemanticVersion::new())
    }

    pub fn get_introduced(&self) -> &SemanticVersion {
        self.introduced.as_ref().unwrap_or_else(|| SemanticVersion::default_instance())
    }

    fn get_introduced_for_reflect(&self) -> &::protobuf::SingularPtrField<SemanticVersion> {
        &self.introduced
    }

    fn mut_introduced_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SemanticVersion> {
        &mut self.introduced
    }

    // .disirpb.DisirValue value = 3;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: DisirValue) {
        self.value = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut DisirValue {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> DisirValue {
        self.value.take().unwrap_or_else(|| DisirValue::new())
    }

    pub fn get_value(&self) -> &DisirValue {
        self.value.as_ref().unwrap_or_else(|| DisirValue::default_instance())
    }

    fn get_value_for_reflect(&self) -> &::protobuf::SingularPtrField<DisirValue> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DisirValue> {
        &mut self.value
    }
}

impl ::protobuf::Message for DisirDefault {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.default_id = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.introduced)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.value)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.default_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.default_id, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.introduced.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.value.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.default_id != 0 {
            os.write_uint64(1, self.default_id)?;
        };
        if let Some(v) = self.introduced.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.value.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DisirDefault {
    fn new() -> DisirDefault {
        DisirDefault::new()
    }

    fn descriptor_static(_: ::std::option::Option<DisirDefault>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "default_id",
                    DisirDefault::get_default_id_for_reflect,
                    DisirDefault::mut_default_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SemanticVersion>>(
                    "introduced",
                    DisirDefault::get_introduced_for_reflect,
                    DisirDefault::mut_introduced_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DisirValue>>(
                    "value",
                    DisirDefault::get_value_for_reflect,
                    DisirDefault::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DisirDefault>(
                    "DisirDefault",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DisirDefault {
    fn clear(&mut self) {
        self.clear_default_id();
        self.clear_introduced();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DisirDefault {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DisirDefault {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DisirValue {
    // message oneof groups
    value: ::std::option::Option<DisirValue_oneof_value>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DisirValue {}

#[derive(Clone,PartialEq)]
pub enum DisirValue_oneof_value {
    string_value(::std::string::String),
    boolean_value(bool),
    integer_value(i64),
    float_value(f64),
}

impl DisirValue {
    pub fn new() -> DisirValue {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DisirValue {
        static mut instance: ::protobuf::lazy::Lazy<DisirValue> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DisirValue,
        };
        unsafe {
            instance.get(DisirValue::new)
        }
    }

    // string string_value = 1;

    pub fn clear_string_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_string_value(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(DisirValue_oneof_value::string_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_string_value(&mut self, v: ::std::string::String) {
        self.value = ::std::option::Option::Some(DisirValue_oneof_value::string_value(v))
    }

    // Mutable pointer to the field.
    pub fn mut_string_value(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(DisirValue_oneof_value::string_value(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(DisirValue_oneof_value::string_value(::std::string::String::new()));
        }
        match self.value {
            ::std::option::Option::Some(DisirValue_oneof_value::string_value(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_string_value(&mut self) -> ::std::string::String {
        if self.has_string_value() {
            match self.value.take() {
                ::std::option::Option::Some(DisirValue_oneof_value::string_value(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_string_value(&self) -> &str {
        match self.value {
            ::std::option::Option::Some(DisirValue_oneof_value::string_value(ref v)) => v,
            _ => "",
        }
    }

    // bool boolean_value = 2;

    pub fn clear_boolean_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_boolean_value(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(DisirValue_oneof_value::boolean_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_boolean_value(&mut self, v: bool) {
        self.value = ::std::option::Option::Some(DisirValue_oneof_value::boolean_value(v))
    }

    pub fn get_boolean_value(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(DisirValue_oneof_value::boolean_value(v)) => v,
            _ => false,
        }
    }

    // int64 integer_value = 3;

    pub fn clear_integer_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_integer_value(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(DisirValue_oneof_value::integer_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_integer_value(&mut self, v: i64) {
        self.value = ::std::option::Option::Some(DisirValue_oneof_value::integer_value(v))
    }

    pub fn get_integer_value(&self) -> i64 {
        match self.value {
            ::std::option::Option::Some(DisirValue_oneof_value::integer_value(v)) => v,
            _ => 0,
        }
    }

    // double float_value = 4;

    pub fn clear_float_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_float_value(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(DisirValue_oneof_value::float_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_float_value(&mut self, v: f64) {
        self.value = ::std::option::Option::Some(DisirValue_oneof_value::float_value(v))
    }

    pub fn get_float_value(&self) -> f64 {
        match self.value {
            ::std::option::Option::Some(DisirValue_oneof_value::float_value(v)) => v,
            _ => 0.,
        }
    }
}

impl ::protobuf::Message for DisirValue {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(DisirValue_oneof_value::string_value(is.read_string()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(DisirValue_oneof_value::boolean_value(is.read_bool()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(DisirValue_oneof_value::integer_value(is.read_int64()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(DisirValue_oneof_value::float_value(is.read_double()?));
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let ::std::option::Option::Some(ref v) = self.value {
            match v {
                &DisirValue_oneof_value::string_value(ref v) => {
                    my_size += ::protobuf::rt::string_size(1, &v);
                },
                &DisirValue_oneof_value::boolean_value(v) => {
                    my_size += 2;
                },
                &DisirValue_oneof_value::integer_value(v) => {
                    my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &DisirValue_oneof_value::float_value(v) => {
                    my_size += 9;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.value {
            match v {
                &DisirValue_oneof_value::string_value(ref v) => {
                    os.write_string(1, v)?;
                },
                &DisirValue_oneof_value::boolean_value(v) => {
                    os.write_bool(2, v)?;
                },
                &DisirValue_oneof_value::integer_value(v) => {
                    os.write_int64(3, v)?;
                },
                &DisirValue_oneof_value::float_value(v) => {
                    os.write_double(4, v)?;
                },
            };
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DisirValue {
    fn new() -> DisirValue {
        DisirValue::new()
    }

    fn descriptor_static(_: ::std::option::Option<DisirValue>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "string_value",
                    DisirValue::has_string_value,
                    DisirValue::get_string_value,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor::<_>(
                    "boolean_value",
                    DisirValue::has_boolean_value,
                    DisirValue::get_boolean_value,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor::<_>(
                    "integer_value",
                    DisirValue::has_integer_value,
                    DisirValue::get_integer_value,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor::<_>(
                    "float_value",
                    DisirValue::has_float_value,
                    DisirValue::get_float_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DisirValue>(
                    "DisirValue",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DisirValue {
    fn clear(&mut self) {
        self.clear_string_value();
        self.clear_boolean_value();
        self.clear_integer_value();
        self.clear_float_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DisirValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DisirValue {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct QueryMoldEntry {
    // message fields
    pub field_type: ::std::string::String,
    pub id: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for QueryMoldEntry {}

impl QueryMoldEntry {
    pub fn new() -> QueryMoldEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static QueryMoldEntry {
        static mut instance: ::protobuf::lazy::Lazy<QueryMoldEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const QueryMoldEntry,
        };
        unsafe {
            instance.get(QueryMoldEntry::new)
        }
    }

    // string type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ::std::string::String) {
        self.field_type = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_type(&mut self) -> &mut ::std::string::String {
        &mut self.field_type
    }

    // Take field
    pub fn take_field_type(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.field_type, ::std::string::String::new())
    }

    pub fn get_field_type(&self) -> &str {
        &self.field_type
    }

    fn get_field_type_for_reflect(&self) -> &::std::string::String {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.field_type
    }

    // string id = 2;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.id, ::std::string::String::new())
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    fn get_id_for_reflect(&self) -> &::std::string::String {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }
}

impl ::protobuf::Message for QueryMoldEntry {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.field_type)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.id)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.field_type.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.field_type);
        };
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.id);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.field_type.is_empty() {
            os.write_string(1, &self.field_type)?;
        };
        if !self.id.is_empty() {
            os.write_string(2, &self.id)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for QueryMoldEntry {
    fn new() -> QueryMoldEntry {
        QueryMoldEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<QueryMoldEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "type",
                    QueryMoldEntry::get_field_type_for_reflect,
                    QueryMoldEntry::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "id",
                    QueryMoldEntry::get_id_for_reflect,
                    QueryMoldEntry::mut_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<QueryMoldEntry>(
                    "QueryMoldEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for QueryMoldEntry {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for QueryMoldEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for QueryMoldEntry {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct QueryConfigEntry {
    // message fields
    pub field_type: ::std::string::String,
    pub id: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for QueryConfigEntry {}

impl QueryConfigEntry {
    pub fn new() -> QueryConfigEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static QueryConfigEntry {
        static mut instance: ::protobuf::lazy::Lazy<QueryConfigEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const QueryConfigEntry,
        };
        unsafe {
            instance.get(QueryConfigEntry::new)
        }
    }

    // string type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ::std::string::String) {
        self.field_type = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_type(&mut self) -> &mut ::std::string::String {
        &mut self.field_type
    }

    // Take field
    pub fn take_field_type(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.field_type, ::std::string::String::new())
    }

    pub fn get_field_type(&self) -> &str {
        &self.field_type
    }

    fn get_field_type_for_reflect(&self) -> &::std::string::String {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.field_type
    }

    // string id = 2;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.id, ::std::string::String::new())
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    fn get_id_for_reflect(&self) -> &::std::string::String {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }
}

impl ::protobuf::Message for QueryConfigEntry {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.field_type)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.id)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.field_type.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.field_type);
        };
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.id);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.field_type.is_empty() {
            os.write_string(1, &self.field_type)?;
        };
        if !self.id.is_empty() {
            os.write_string(2, &self.id)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for QueryConfigEntry {
    fn new() -> QueryConfigEntry {
        QueryConfigEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<QueryConfigEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "type",
                    QueryConfigEntry::get_field_type_for_reflect,
                    QueryConfigEntry::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "id",
                    QueryConfigEntry::get_id_for_reflect,
                    QueryConfigEntry::mut_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<QueryConfigEntry>(
                    "QueryConfigEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for QueryConfigEntry {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for QueryConfigEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for QueryConfigEntry {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AddConfigEntry {
    // message fields
    pub config_id: u64,
    pub parent_id: u64,
    // message oneof groups
    msg: ::std::option::Option<AddConfigEntry_oneof_msg>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AddConfigEntry {}

#[derive(Clone,PartialEq)]
pub enum AddConfigEntry_oneof_msg {
    keyval(ConfigKeyval),
}

impl AddConfigEntry {
    pub fn new() -> AddConfigEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AddConfigEntry {
        static mut instance: ::protobuf::lazy::Lazy<AddConfigEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AddConfigEntry,
        };
        unsafe {
            instance.get(AddConfigEntry::new)
        }
    }

    // .disirpb.ConfigKeyval keyval = 1;

    pub fn clear_keyval(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_keyval(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(AddConfigEntry_oneof_msg::keyval(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_keyval(&mut self, v: ConfigKeyval) {
        self.msg = ::std::option::Option::Some(AddConfigEntry_oneof_msg::keyval(v))
    }

    // Mutable pointer to the field.
    pub fn mut_keyval(&mut self) -> &mut ConfigKeyval {
        if let ::std::option::Option::Some(AddConfigEntry_oneof_msg::keyval(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(AddConfigEntry_oneof_msg::keyval(ConfigKeyval::new()));
        }
        match self.msg {
            ::std::option::Option::Some(AddConfigEntry_oneof_msg::keyval(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_keyval(&mut self) -> ConfigKeyval {
        if self.has_keyval() {
            match self.msg.take() {
                ::std::option::Option::Some(AddConfigEntry_oneof_msg::keyval(v)) => v,
                _ => panic!(),
            }
        } else {
            ConfigKeyval::new()
        }
    }

    pub fn get_keyval(&self) -> &ConfigKeyval {
        match self.msg {
            ::std::option::Option::Some(AddConfigEntry_oneof_msg::keyval(ref v)) => v,
            _ => ConfigKeyval::default_instance(),
        }
    }

    // uint64 config_id = 2;

    pub fn clear_config_id(&mut self) {
        self.config_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_config_id(&mut self, v: u64) {
        self.config_id = v;
    }

    pub fn get_config_id(&self) -> u64 {
        self.config_id
    }

    fn get_config_id_for_reflect(&self) -> &u64 {
        &self.config_id
    }

    fn mut_config_id_for_reflect(&mut self) -> &mut u64 {
        &mut self.config_id
    }

    // uint64 parent_id = 3;

    pub fn clear_parent_id(&mut self) {
        self.parent_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_parent_id(&mut self, v: u64) {
        self.parent_id = v;
    }

    pub fn get_parent_id(&self) -> u64 {
        self.parent_id
    }

    fn get_parent_id_for_reflect(&self) -> &u64 {
        &self.parent_id
    }

    fn mut_parent_id_for_reflect(&mut self) -> &mut u64 {
        &mut self.parent_id
    }
}

impl ::protobuf::Message for AddConfigEntry {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(AddConfigEntry_oneof_msg::keyval(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.config_id = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.parent_id = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.config_id != 0 {
            my_size += ::protobuf::rt::value_size(2, self.config_id, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.parent_id != 0 {
            my_size += ::protobuf::rt::value_size(3, self.parent_id, ::protobuf::wire_format::WireTypeVarint);
        };
        if let ::std::option::Option::Some(ref v) = self.msg {
            match v {
                &AddConfigEntry_oneof_msg::keyval(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.config_id != 0 {
            os.write_uint64(2, self.config_id)?;
        };
        if self.parent_id != 0 {
            os.write_uint64(3, self.parent_id)?;
        };
        if let ::std::option::Option::Some(ref v) = self.msg {
            match v {
                &AddConfigEntry_oneof_msg::keyval(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AddConfigEntry {
    fn new() -> AddConfigEntry {
        AddConfigEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<AddConfigEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ConfigKeyval>(
                    "keyval",
                    AddConfigEntry::has_keyval,
                    AddConfigEntry::get_keyval,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "config_id",
                    AddConfigEntry::get_config_id_for_reflect,
                    AddConfigEntry::mut_config_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "parent_id",
                    AddConfigEntry::get_parent_id_for_reflect,
                    AddConfigEntry::mut_parent_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AddConfigEntry>(
                    "AddConfigEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AddConfigEntry {
    fn clear(&mut self) {
        self.clear_keyval();
        self.clear_config_id();
        self.clear_parent_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AddConfigEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AddConfigEntry {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct QueryElement {
    // message fields
    pub element_id: u64,
    // message oneof groups
    msg: ::std::option::Option<QueryElement_oneof_msg>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for QueryElement {}

#[derive(Clone,PartialEq)]
pub enum QueryElement_oneof_msg {
    mold_id(u64),
    config_id(u64),
}

impl QueryElement {
    pub fn new() -> QueryElement {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static QueryElement {
        static mut instance: ::protobuf::lazy::Lazy<QueryElement> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const QueryElement,
        };
        unsafe {
            instance.get(QueryElement::new)
        }
    }

    // uint64 mold_id = 1;

    pub fn clear_mold_id(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_mold_id(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(QueryElement_oneof_msg::mold_id(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_mold_id(&mut self, v: u64) {
        self.msg = ::std::option::Option::Some(QueryElement_oneof_msg::mold_id(v))
    }

    pub fn get_mold_id(&self) -> u64 {
        match self.msg {
            ::std::option::Option::Some(QueryElement_oneof_msg::mold_id(v)) => v,
            _ => 0,
        }
    }

    // uint64 config_id = 2;

    pub fn clear_config_id(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_config_id(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(QueryElement_oneof_msg::config_id(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_config_id(&mut self, v: u64) {
        self.msg = ::std::option::Option::Some(QueryElement_oneof_msg::config_id(v))
    }

    pub fn get_config_id(&self) -> u64 {
        match self.msg {
            ::std::option::Option::Some(QueryElement_oneof_msg::config_id(v)) => v,
            _ => 0,
        }
    }

    // uint64 element_id = 3;

    pub fn clear_element_id(&mut self) {
        self.element_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_element_id(&mut self, v: u64) {
        self.element_id = v;
    }

    pub fn get_element_id(&self) -> u64 {
        self.element_id
    }

    fn get_element_id_for_reflect(&self) -> &u64 {
        &self.element_id
    }

    fn mut_element_id_for_reflect(&mut self) -> &mut u64 {
        &mut self.element_id
    }
}

impl ::protobuf::Message for QueryElement {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(QueryElement_oneof_msg::mold_id(is.read_uint64()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(QueryElement_oneof_msg::config_id(is.read_uint64()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.element_id = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.element_id != 0 {
            my_size += ::protobuf::rt::value_size(3, self.element_id, ::protobuf::wire_format::WireTypeVarint);
        };
        if let ::std::option::Option::Some(ref v) = self.msg {
            match v {
                &QueryElement_oneof_msg::mold_id(v) => {
                    my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &QueryElement_oneof_msg::config_id(v) => {
                    my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.element_id != 0 {
            os.write_uint64(3, self.element_id)?;
        };
        if let ::std::option::Option::Some(ref v) = self.msg {
            match v {
                &QueryElement_oneof_msg::mold_id(v) => {
                    os.write_uint64(1, v)?;
                },
                &QueryElement_oneof_msg::config_id(v) => {
                    os.write_uint64(2, v)?;
                },
            };
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for QueryElement {
    fn new() -> QueryElement {
        QueryElement::new()
    }

    fn descriptor_static(_: ::std::option::Option<QueryElement>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor::<_>(
                    "mold_id",
                    QueryElement::has_mold_id,
                    QueryElement::get_mold_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor::<_>(
                    "config_id",
                    QueryElement::has_config_id,
                    QueryElement::get_config_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "element_id",
                    QueryElement::get_element_id_for_reflect,
                    QueryElement::mut_element_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<QueryElement>(
                    "QueryElement",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for QueryElement {
    fn clear(&mut self) {
        self.clear_mold_id();
        self.clear_config_id();
        self.clear_element_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for QueryElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for QueryElement {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LibDisirRequest {
    // message oneof groups
    msg: ::std::option::Option<LibDisirRequest_oneof_msg>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LibDisirRequest {}

#[derive(Clone,PartialEq)]
pub enum LibDisirRequest_oneof_msg {
    query_groups(bool),
    query_mold_entries(::std::string::String),
    query_mold_entry(QueryMoldEntry),
    close_mold_entry(u64),
    query_element(QueryElement),
    query_config_entry(QueryConfigEntry),
    add_config_entry(AddConfigEntry),
}

impl LibDisirRequest {
    pub fn new() -> LibDisirRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LibDisirRequest {
        static mut instance: ::protobuf::lazy::Lazy<LibDisirRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LibDisirRequest,
        };
        unsafe {
            instance.get(LibDisirRequest::new)
        }
    }

    // bool query_groups = 2;

    pub fn clear_query_groups(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_query_groups(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(LibDisirRequest_oneof_msg::query_groups(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_query_groups(&mut self, v: bool) {
        self.msg = ::std::option::Option::Some(LibDisirRequest_oneof_msg::query_groups(v))
    }

    pub fn get_query_groups(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(LibDisirRequest_oneof_msg::query_groups(v)) => v,
            _ => false,
        }
    }

    // string query_mold_entries = 3;

    pub fn clear_query_mold_entries(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_query_mold_entries(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(LibDisirRequest_oneof_msg::query_mold_entries(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_query_mold_entries(&mut self, v: ::std::string::String) {
        self.msg = ::std::option::Option::Some(LibDisirRequest_oneof_msg::query_mold_entries(v))
    }

    // Mutable pointer to the field.
    pub fn mut_query_mold_entries(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(LibDisirRequest_oneof_msg::query_mold_entries(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(LibDisirRequest_oneof_msg::query_mold_entries(::std::string::String::new()));
        }
        match self.msg {
            ::std::option::Option::Some(LibDisirRequest_oneof_msg::query_mold_entries(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_query_mold_entries(&mut self) -> ::std::string::String {
        if self.has_query_mold_entries() {
            match self.msg.take() {
                ::std::option::Option::Some(LibDisirRequest_oneof_msg::query_mold_entries(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_query_mold_entries(&self) -> &str {
        match self.msg {
            ::std::option::Option::Some(LibDisirRequest_oneof_msg::query_mold_entries(ref v)) => v,
            _ => "",
        }
    }

    // .disirpb.QueryMoldEntry query_mold_entry = 4;

    pub fn clear_query_mold_entry(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_query_mold_entry(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(LibDisirRequest_oneof_msg::query_mold_entry(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_query_mold_entry(&mut self, v: QueryMoldEntry) {
        self.msg = ::std::option::Option::Some(LibDisirRequest_oneof_msg::query_mold_entry(v))
    }

    // Mutable pointer to the field.
    pub fn mut_query_mold_entry(&mut self) -> &mut QueryMoldEntry {
        if let ::std::option::Option::Some(LibDisirRequest_oneof_msg::query_mold_entry(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(LibDisirRequest_oneof_msg::query_mold_entry(QueryMoldEntry::new()));
        }
        match self.msg {
            ::std::option::Option::Some(LibDisirRequest_oneof_msg::query_mold_entry(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_query_mold_entry(&mut self) -> QueryMoldEntry {
        if self.has_query_mold_entry() {
            match self.msg.take() {
                ::std::option::Option::Some(LibDisirRequest_oneof_msg::query_mold_entry(v)) => v,
                _ => panic!(),
            }
        } else {
            QueryMoldEntry::new()
        }
    }

    pub fn get_query_mold_entry(&self) -> &QueryMoldEntry {
        match self.msg {
            ::std::option::Option::Some(LibDisirRequest_oneof_msg::query_mold_entry(ref v)) => v,
            _ => QueryMoldEntry::default_instance(),
        }
    }

    // uint64 close_mold_entry = 5;

    pub fn clear_close_mold_entry(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_close_mold_entry(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(LibDisirRequest_oneof_msg::close_mold_entry(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_close_mold_entry(&mut self, v: u64) {
        self.msg = ::std::option::Option::Some(LibDisirRequest_oneof_msg::close_mold_entry(v))
    }

    pub fn get_close_mold_entry(&self) -> u64 {
        match self.msg {
            ::std::option::Option::Some(LibDisirRequest_oneof_msg::close_mold_entry(v)) => v,
            _ => 0,
        }
    }

    // .disirpb.QueryElement query_element = 6;

    pub fn clear_query_element(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_query_element(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(LibDisirRequest_oneof_msg::query_element(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_query_element(&mut self, v: QueryElement) {
        self.msg = ::std::option::Option::Some(LibDisirRequest_oneof_msg::query_element(v))
    }

    // Mutable pointer to the field.
    pub fn mut_query_element(&mut self) -> &mut QueryElement {
        if let ::std::option::Option::Some(LibDisirRequest_oneof_msg::query_element(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(LibDisirRequest_oneof_msg::query_element(QueryElement::new()));
        }
        match self.msg {
            ::std::option::Option::Some(LibDisirRequest_oneof_msg::query_element(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_query_element(&mut self) -> QueryElement {
        if self.has_query_element() {
            match self.msg.take() {
                ::std::option::Option::Some(LibDisirRequest_oneof_msg::query_element(v)) => v,
                _ => panic!(),
            }
        } else {
            QueryElement::new()
        }
    }

    pub fn get_query_element(&self) -> &QueryElement {
        match self.msg {
            ::std::option::Option::Some(LibDisirRequest_oneof_msg::query_element(ref v)) => v,
            _ => QueryElement::default_instance(),
        }
    }

    // .disirpb.QueryConfigEntry query_config_entry = 7;

    pub fn clear_query_config_entry(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_query_config_entry(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(LibDisirRequest_oneof_msg::query_config_entry(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_query_config_entry(&mut self, v: QueryConfigEntry) {
        self.msg = ::std::option::Option::Some(LibDisirRequest_oneof_msg::query_config_entry(v))
    }

    // Mutable pointer to the field.
    pub fn mut_query_config_entry(&mut self) -> &mut QueryConfigEntry {
        if let ::std::option::Option::Some(LibDisirRequest_oneof_msg::query_config_entry(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(LibDisirRequest_oneof_msg::query_config_entry(QueryConfigEntry::new()));
        }
        match self.msg {
            ::std::option::Option::Some(LibDisirRequest_oneof_msg::query_config_entry(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_query_config_entry(&mut self) -> QueryConfigEntry {
        if self.has_query_config_entry() {
            match self.msg.take() {
                ::std::option::Option::Some(LibDisirRequest_oneof_msg::query_config_entry(v)) => v,
                _ => panic!(),
            }
        } else {
            QueryConfigEntry::new()
        }
    }

    pub fn get_query_config_entry(&self) -> &QueryConfigEntry {
        match self.msg {
            ::std::option::Option::Some(LibDisirRequest_oneof_msg::query_config_entry(ref v)) => v,
            _ => QueryConfigEntry::default_instance(),
        }
    }

    // .disirpb.AddConfigEntry add_config_entry = 8;

    pub fn clear_add_config_entry(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_add_config_entry(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(LibDisirRequest_oneof_msg::add_config_entry(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_add_config_entry(&mut self, v: AddConfigEntry) {
        self.msg = ::std::option::Option::Some(LibDisirRequest_oneof_msg::add_config_entry(v))
    }

    // Mutable pointer to the field.
    pub fn mut_add_config_entry(&mut self) -> &mut AddConfigEntry {
        if let ::std::option::Option::Some(LibDisirRequest_oneof_msg::add_config_entry(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(LibDisirRequest_oneof_msg::add_config_entry(AddConfigEntry::new()));
        }
        match self.msg {
            ::std::option::Option::Some(LibDisirRequest_oneof_msg::add_config_entry(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_add_config_entry(&mut self) -> AddConfigEntry {
        if self.has_add_config_entry() {
            match self.msg.take() {
                ::std::option::Option::Some(LibDisirRequest_oneof_msg::add_config_entry(v)) => v,
                _ => panic!(),
            }
        } else {
            AddConfigEntry::new()
        }
    }

    pub fn get_add_config_entry(&self) -> &AddConfigEntry {
        match self.msg {
            ::std::option::Option::Some(LibDisirRequest_oneof_msg::add_config_entry(ref v)) => v,
            _ => AddConfigEntry::default_instance(),
        }
    }
}

impl ::protobuf::Message for LibDisirRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(LibDisirRequest_oneof_msg::query_groups(is.read_bool()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(LibDisirRequest_oneof_msg::query_mold_entries(is.read_string()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(LibDisirRequest_oneof_msg::query_mold_entry(is.read_message()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(LibDisirRequest_oneof_msg::close_mold_entry(is.read_uint64()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(LibDisirRequest_oneof_msg::query_element(is.read_message()?));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(LibDisirRequest_oneof_msg::query_config_entry(is.read_message()?));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(LibDisirRequest_oneof_msg::add_config_entry(is.read_message()?));
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let ::std::option::Option::Some(ref v) = self.msg {
            match v {
                &LibDisirRequest_oneof_msg::query_groups(v) => {
                    my_size += 2;
                },
                &LibDisirRequest_oneof_msg::query_mold_entries(ref v) => {
                    my_size += ::protobuf::rt::string_size(3, &v);
                },
                &LibDisirRequest_oneof_msg::query_mold_entry(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &LibDisirRequest_oneof_msg::close_mold_entry(v) => {
                    my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &LibDisirRequest_oneof_msg::query_element(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &LibDisirRequest_oneof_msg::query_config_entry(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &LibDisirRequest_oneof_msg::add_config_entry(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.msg {
            match v {
                &LibDisirRequest_oneof_msg::query_groups(v) => {
                    os.write_bool(2, v)?;
                },
                &LibDisirRequest_oneof_msg::query_mold_entries(ref v) => {
                    os.write_string(3, v)?;
                },
                &LibDisirRequest_oneof_msg::query_mold_entry(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &LibDisirRequest_oneof_msg::close_mold_entry(v) => {
                    os.write_uint64(5, v)?;
                },
                &LibDisirRequest_oneof_msg::query_element(ref v) => {
                    os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &LibDisirRequest_oneof_msg::query_config_entry(ref v) => {
                    os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &LibDisirRequest_oneof_msg::add_config_entry(ref v) => {
                    os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for LibDisirRequest {
    fn new() -> LibDisirRequest {
        LibDisirRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<LibDisirRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor::<_>(
                    "query_groups",
                    LibDisirRequest::has_query_groups,
                    LibDisirRequest::get_query_groups,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "query_mold_entries",
                    LibDisirRequest::has_query_mold_entries,
                    LibDisirRequest::get_query_mold_entries,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, QueryMoldEntry>(
                    "query_mold_entry",
                    LibDisirRequest::has_query_mold_entry,
                    LibDisirRequest::get_query_mold_entry,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor::<_>(
                    "close_mold_entry",
                    LibDisirRequest::has_close_mold_entry,
                    LibDisirRequest::get_close_mold_entry,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, QueryElement>(
                    "query_element",
                    LibDisirRequest::has_query_element,
                    LibDisirRequest::get_query_element,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, QueryConfigEntry>(
                    "query_config_entry",
                    LibDisirRequest::has_query_config_entry,
                    LibDisirRequest::get_query_config_entry,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, AddConfigEntry>(
                    "add_config_entry",
                    LibDisirRequest::has_add_config_entry,
                    LibDisirRequest::get_add_config_entry,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LibDisirRequest>(
                    "LibDisirRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LibDisirRequest {
    fn clear(&mut self) {
        self.clear_query_groups();
        self.clear_query_mold_entries();
        self.clear_query_mold_entry();
        self.clear_close_mold_entry();
        self.clear_query_element();
        self.clear_query_config_entry();
        self.clear_add_config_entry();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LibDisirRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LibDisirRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LibDisirResponse {
    // message fields
    pub status: Status,
    pub error_string: ::std::string::String,
    group_ids: ::protobuf::RepeatedField<::std::string::String>,
    mold_entry_ids: ::protobuf::RepeatedField<::std::string::String>,
    mold: ::protobuf::SingularPtrField<DisirMold>,
    element: ::protobuf::SingularPtrField<Element>,
    config: ::protobuf::SingularPtrField<DisirConfig>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LibDisirResponse {}

impl LibDisirResponse {
    pub fn new() -> LibDisirResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LibDisirResponse {
        static mut instance: ::protobuf::lazy::Lazy<LibDisirResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LibDisirResponse,
        };
        unsafe {
            instance.get(LibDisirResponse::new)
        }
    }

    // .disirpb.Status status = 1;

    pub fn clear_status(&mut self) {
        self.status = Status::UNKNOWN;
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: Status) {
        self.status = v;
    }

    pub fn get_status(&self) -> Status {
        self.status
    }

    fn get_status_for_reflect(&self) -> &Status {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut Status {
        &mut self.status
    }

    // string error_string = 2;

    pub fn clear_error_string(&mut self) {
        self.error_string.clear();
    }

    // Param is passed by value, moved
    pub fn set_error_string(&mut self, v: ::std::string::String) {
        self.error_string = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error_string(&mut self) -> &mut ::std::string::String {
        &mut self.error_string
    }

    // Take field
    pub fn take_error_string(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.error_string, ::std::string::String::new())
    }

    pub fn get_error_string(&self) -> &str {
        &self.error_string
    }

    fn get_error_string_for_reflect(&self) -> &::std::string::String {
        &self.error_string
    }

    fn mut_error_string_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.error_string
    }

    // repeated string group_ids = 10;

    pub fn clear_group_ids(&mut self) {
        self.group_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_group_ids(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.group_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_group_ids(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.group_ids
    }

    // Take field
    pub fn take_group_ids(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.group_ids, ::protobuf::RepeatedField::new())
    }

    pub fn get_group_ids(&self) -> &[::std::string::String] {
        &self.group_ids
    }

    fn get_group_ids_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.group_ids
    }

    fn mut_group_ids_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.group_ids
    }

    // repeated string mold_entry_ids = 12;

    pub fn clear_mold_entry_ids(&mut self) {
        self.mold_entry_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_mold_entry_ids(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.mold_entry_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_mold_entry_ids(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.mold_entry_ids
    }

    // Take field
    pub fn take_mold_entry_ids(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.mold_entry_ids, ::protobuf::RepeatedField::new())
    }

    pub fn get_mold_entry_ids(&self) -> &[::std::string::String] {
        &self.mold_entry_ids
    }

    fn get_mold_entry_ids_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.mold_entry_ids
    }

    fn mut_mold_entry_ids_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.mold_entry_ids
    }

    // .disirpb.DisirMold mold = 13;

    pub fn clear_mold(&mut self) {
        self.mold.clear();
    }

    pub fn has_mold(&self) -> bool {
        self.mold.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mold(&mut self, v: DisirMold) {
        self.mold = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_mold(&mut self) -> &mut DisirMold {
        if self.mold.is_none() {
            self.mold.set_default();
        };
        self.mold.as_mut().unwrap()
    }

    // Take field
    pub fn take_mold(&mut self) -> DisirMold {
        self.mold.take().unwrap_or_else(|| DisirMold::new())
    }

    pub fn get_mold(&self) -> &DisirMold {
        self.mold.as_ref().unwrap_or_else(|| DisirMold::default_instance())
    }

    fn get_mold_for_reflect(&self) -> &::protobuf::SingularPtrField<DisirMold> {
        &self.mold
    }

    fn mut_mold_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DisirMold> {
        &mut self.mold
    }

    // .disirpb.Element element = 14;

    pub fn clear_element(&mut self) {
        self.element.clear();
    }

    pub fn has_element(&self) -> bool {
        self.element.is_some()
    }

    // Param is passed by value, moved
    pub fn set_element(&mut self, v: Element) {
        self.element = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_element(&mut self) -> &mut Element {
        if self.element.is_none() {
            self.element.set_default();
        };
        self.element.as_mut().unwrap()
    }

    // Take field
    pub fn take_element(&mut self) -> Element {
        self.element.take().unwrap_or_else(|| Element::new())
    }

    pub fn get_element(&self) -> &Element {
        self.element.as_ref().unwrap_or_else(|| Element::default_instance())
    }

    fn get_element_for_reflect(&self) -> &::protobuf::SingularPtrField<Element> {
        &self.element
    }

    fn mut_element_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Element> {
        &mut self.element
    }

    // .disirpb.DisirConfig config = 15;

    pub fn clear_config(&mut self) {
        self.config.clear();
    }

    pub fn has_config(&self) -> bool {
        self.config.is_some()
    }

    // Param is passed by value, moved
    pub fn set_config(&mut self, v: DisirConfig) {
        self.config = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_config(&mut self) -> &mut DisirConfig {
        if self.config.is_none() {
            self.config.set_default();
        };
        self.config.as_mut().unwrap()
    }

    // Take field
    pub fn take_config(&mut self) -> DisirConfig {
        self.config.take().unwrap_or_else(|| DisirConfig::new())
    }

    pub fn get_config(&self) -> &DisirConfig {
        self.config.as_ref().unwrap_or_else(|| DisirConfig::default_instance())
    }

    fn get_config_for_reflect(&self) -> &::protobuf::SingularPtrField<DisirConfig> {
        &self.config
    }

    fn mut_config_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DisirConfig> {
        &mut self.config
    }
}

impl ::protobuf::Message for LibDisirResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.status = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.error_string)?;
                },
                10 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.group_ids)?;
                },
                12 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.mold_entry_ids)?;
                },
                13 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.mold)?;
                },
                14 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.element)?;
                },
                15 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.config)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.status != Status::UNKNOWN {
            my_size += ::protobuf::rt::enum_size(1, self.status);
        };
        if !self.error_string.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.error_string);
        };
        for value in &self.group_ids {
            my_size += ::protobuf::rt::string_size(10, &value);
        };
        for value in &self.mold_entry_ids {
            my_size += ::protobuf::rt::string_size(12, &value);
        };
        if let Some(v) = self.mold.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.element.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.config.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.status != Status::UNKNOWN {
            os.write_enum(1, self.status.value())?;
        };
        if !self.error_string.is_empty() {
            os.write_string(2, &self.error_string)?;
        };
        for v in &self.group_ids {
            os.write_string(10, &v)?;
        };
        for v in &self.mold_entry_ids {
            os.write_string(12, &v)?;
        };
        if let Some(v) = self.mold.as_ref() {
            os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.element.as_ref() {
            os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.config.as_ref() {
            os.write_tag(15, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for LibDisirResponse {
    fn new() -> LibDisirResponse {
        LibDisirResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<LibDisirResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Status>>(
                    "status",
                    LibDisirResponse::get_status_for_reflect,
                    LibDisirResponse::mut_status_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error_string",
                    LibDisirResponse::get_error_string_for_reflect,
                    LibDisirResponse::mut_error_string_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "group_ids",
                    LibDisirResponse::get_group_ids_for_reflect,
                    LibDisirResponse::mut_group_ids_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "mold_entry_ids",
                    LibDisirResponse::get_mold_entry_ids_for_reflect,
                    LibDisirResponse::mut_mold_entry_ids_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DisirMold>>(
                    "mold",
                    LibDisirResponse::get_mold_for_reflect,
                    LibDisirResponse::mut_mold_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Element>>(
                    "element",
                    LibDisirResponse::get_element_for_reflect,
                    LibDisirResponse::mut_element_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DisirConfig>>(
                    "config",
                    LibDisirResponse::get_config_for_reflect,
                    LibDisirResponse::mut_config_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LibDisirResponse>(
                    "LibDisirResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LibDisirResponse {
    fn clear(&mut self) {
        self.clear_status();
        self.clear_error_string();
        self.clear_group_ids();
        self.clear_mold_entry_ids();
        self.clear_mold();
        self.clear_element();
        self.clear_config();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LibDisirResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LibDisirResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ToClient {
    // message fields
    pub command_identifier: u64,
    // message oneof groups
    msg: ::std::option::Option<ToClient_oneof_msg>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ToClient {}

#[derive(Clone,PartialEq)]
pub enum ToClient_oneof_msg {
    ping(bool),
    pong(bool),
    system_info(SystemInfo),
    response(LibDisirResponse),
}

impl ToClient {
    pub fn new() -> ToClient {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ToClient {
        static mut instance: ::protobuf::lazy::Lazy<ToClient> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ToClient,
        };
        unsafe {
            instance.get(ToClient::new)
        }
    }

    // uint64 command_identifier = 1;

    pub fn clear_command_identifier(&mut self) {
        self.command_identifier = 0;
    }

    // Param is passed by value, moved
    pub fn set_command_identifier(&mut self, v: u64) {
        self.command_identifier = v;
    }

    pub fn get_command_identifier(&self) -> u64 {
        self.command_identifier
    }

    fn get_command_identifier_for_reflect(&self) -> &u64 {
        &self.command_identifier
    }

    fn mut_command_identifier_for_reflect(&mut self) -> &mut u64 {
        &mut self.command_identifier
    }

    // bool ping = 2;

    pub fn clear_ping(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_ping(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(ToClient_oneof_msg::ping(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_ping(&mut self, v: bool) {
        self.msg = ::std::option::Option::Some(ToClient_oneof_msg::ping(v))
    }

    pub fn get_ping(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(ToClient_oneof_msg::ping(v)) => v,
            _ => false,
        }
    }

    // bool pong = 3;

    pub fn clear_pong(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_pong(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(ToClient_oneof_msg::pong(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_pong(&mut self, v: bool) {
        self.msg = ::std::option::Option::Some(ToClient_oneof_msg::pong(v))
    }

    pub fn get_pong(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(ToClient_oneof_msg::pong(v)) => v,
            _ => false,
        }
    }

    // .disirpb.SystemInfo system_info = 4;

    pub fn clear_system_info(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_system_info(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(ToClient_oneof_msg::system_info(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_system_info(&mut self, v: SystemInfo) {
        self.msg = ::std::option::Option::Some(ToClient_oneof_msg::system_info(v))
    }

    // Mutable pointer to the field.
    pub fn mut_system_info(&mut self) -> &mut SystemInfo {
        if let ::std::option::Option::Some(ToClient_oneof_msg::system_info(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(ToClient_oneof_msg::system_info(SystemInfo::new()));
        }
        match self.msg {
            ::std::option::Option::Some(ToClient_oneof_msg::system_info(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_system_info(&mut self) -> SystemInfo {
        if self.has_system_info() {
            match self.msg.take() {
                ::std::option::Option::Some(ToClient_oneof_msg::system_info(v)) => v,
                _ => panic!(),
            }
        } else {
            SystemInfo::new()
        }
    }

    pub fn get_system_info(&self) -> &SystemInfo {
        match self.msg {
            ::std::option::Option::Some(ToClient_oneof_msg::system_info(ref v)) => v,
            _ => SystemInfo::default_instance(),
        }
    }

    // .disirpb.LibDisirResponse response = 5;

    pub fn clear_response(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_response(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(ToClient_oneof_msg::response(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_response(&mut self, v: LibDisirResponse) {
        self.msg = ::std::option::Option::Some(ToClient_oneof_msg::response(v))
    }

    // Mutable pointer to the field.
    pub fn mut_response(&mut self) -> &mut LibDisirResponse {
        if let ::std::option::Option::Some(ToClient_oneof_msg::response(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(ToClient_oneof_msg::response(LibDisirResponse::new()));
        }
        match self.msg {
            ::std::option::Option::Some(ToClient_oneof_msg::response(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_response(&mut self) -> LibDisirResponse {
        if self.has_response() {
            match self.msg.take() {
                ::std::option::Option::Some(ToClient_oneof_msg::response(v)) => v,
                _ => panic!(),
            }
        } else {
            LibDisirResponse::new()
        }
    }

    pub fn get_response(&self) -> &LibDisirResponse {
        match self.msg {
            ::std::option::Option::Some(ToClient_oneof_msg::response(ref v)) => v,
            _ => LibDisirResponse::default_instance(),
        }
    }
}

impl ::protobuf::Message for ToClient {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.command_identifier = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(ToClient_oneof_msg::ping(is.read_bool()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(ToClient_oneof_msg::pong(is.read_bool()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(ToClient_oneof_msg::system_info(is.read_message()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(ToClient_oneof_msg::response(is.read_message()?));
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.command_identifier != 0 {
            my_size += ::protobuf::rt::value_size(1, self.command_identifier, ::protobuf::wire_format::WireTypeVarint);
        };
        if let ::std::option::Option::Some(ref v) = self.msg {
            match v {
                &ToClient_oneof_msg::ping(v) => {
                    my_size += 2;
                },
                &ToClient_oneof_msg::pong(v) => {
                    my_size += 2;
                },
                &ToClient_oneof_msg::system_info(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ToClient_oneof_msg::response(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.command_identifier != 0 {
            os.write_uint64(1, self.command_identifier)?;
        };
        if let ::std::option::Option::Some(ref v) = self.msg {
            match v {
                &ToClient_oneof_msg::ping(v) => {
                    os.write_bool(2, v)?;
                },
                &ToClient_oneof_msg::pong(v) => {
                    os.write_bool(3, v)?;
                },
                &ToClient_oneof_msg::system_info(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ToClient_oneof_msg::response(ref v) => {
                    os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ToClient {
    fn new() -> ToClient {
        ToClient::new()
    }

    fn descriptor_static(_: ::std::option::Option<ToClient>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "command_identifier",
                    ToClient::get_command_identifier_for_reflect,
                    ToClient::mut_command_identifier_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor::<_>(
                    "ping",
                    ToClient::has_ping,
                    ToClient::get_ping,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor::<_>(
                    "pong",
                    ToClient::has_pong,
                    ToClient::get_pong,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, SystemInfo>(
                    "system_info",
                    ToClient::has_system_info,
                    ToClient::get_system_info,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, LibDisirResponse>(
                    "response",
                    ToClient::has_response,
                    ToClient::get_response,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ToClient>(
                    "ToClient",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ToClient {
    fn clear(&mut self) {
        self.clear_command_identifier();
        self.clear_ping();
        self.clear_pong();
        self.clear_system_info();
        self.clear_response();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ToClient {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ToClient {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FromClient {
    // message fields
    pub command_identifier: u64,
    // message oneof groups
    msg: ::std::option::Option<FromClient_oneof_msg>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FromClient {}

#[derive(Clone,PartialEq)]
pub enum FromClient_oneof_msg {
    ping(bool),
    pong(bool),
    user_info(UserInfo),
    request(LibDisirRequest),
}

impl FromClient {
    pub fn new() -> FromClient {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FromClient {
        static mut instance: ::protobuf::lazy::Lazy<FromClient> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FromClient,
        };
        unsafe {
            instance.get(FromClient::new)
        }
    }

    // uint64 command_identifier = 1;

    pub fn clear_command_identifier(&mut self) {
        self.command_identifier = 0;
    }

    // Param is passed by value, moved
    pub fn set_command_identifier(&mut self, v: u64) {
        self.command_identifier = v;
    }

    pub fn get_command_identifier(&self) -> u64 {
        self.command_identifier
    }

    fn get_command_identifier_for_reflect(&self) -> &u64 {
        &self.command_identifier
    }

    fn mut_command_identifier_for_reflect(&mut self) -> &mut u64 {
        &mut self.command_identifier
    }

    // bool ping = 2;

    pub fn clear_ping(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_ping(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(FromClient_oneof_msg::ping(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_ping(&mut self, v: bool) {
        self.msg = ::std::option::Option::Some(FromClient_oneof_msg::ping(v))
    }

    pub fn get_ping(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(FromClient_oneof_msg::ping(v)) => v,
            _ => false,
        }
    }

    // bool pong = 3;

    pub fn clear_pong(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_pong(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(FromClient_oneof_msg::pong(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_pong(&mut self, v: bool) {
        self.msg = ::std::option::Option::Some(FromClient_oneof_msg::pong(v))
    }

    pub fn get_pong(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(FromClient_oneof_msg::pong(v)) => v,
            _ => false,
        }
    }

    // .disirpb.UserInfo user_info = 4;

    pub fn clear_user_info(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_user_info(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(FromClient_oneof_msg::user_info(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_user_info(&mut self, v: UserInfo) {
        self.msg = ::std::option::Option::Some(FromClient_oneof_msg::user_info(v))
    }

    // Mutable pointer to the field.
    pub fn mut_user_info(&mut self) -> &mut UserInfo {
        if let ::std::option::Option::Some(FromClient_oneof_msg::user_info(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(FromClient_oneof_msg::user_info(UserInfo::new()));
        }
        match self.msg {
            ::std::option::Option::Some(FromClient_oneof_msg::user_info(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_user_info(&mut self) -> UserInfo {
        if self.has_user_info() {
            match self.msg.take() {
                ::std::option::Option::Some(FromClient_oneof_msg::user_info(v)) => v,
                _ => panic!(),
            }
        } else {
            UserInfo::new()
        }
    }

    pub fn get_user_info(&self) -> &UserInfo {
        match self.msg {
            ::std::option::Option::Some(FromClient_oneof_msg::user_info(ref v)) => v,
            _ => UserInfo::default_instance(),
        }
    }

    // .disirpb.LibDisirRequest request = 5;

    pub fn clear_request(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_request(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(FromClient_oneof_msg::request(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_request(&mut self, v: LibDisirRequest) {
        self.msg = ::std::option::Option::Some(FromClient_oneof_msg::request(v))
    }

    // Mutable pointer to the field.
    pub fn mut_request(&mut self) -> &mut LibDisirRequest {
        if let ::std::option::Option::Some(FromClient_oneof_msg::request(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(FromClient_oneof_msg::request(LibDisirRequest::new()));
        }
        match self.msg {
            ::std::option::Option::Some(FromClient_oneof_msg::request(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_request(&mut self) -> LibDisirRequest {
        if self.has_request() {
            match self.msg.take() {
                ::std::option::Option::Some(FromClient_oneof_msg::request(v)) => v,
                _ => panic!(),
            }
        } else {
            LibDisirRequest::new()
        }
    }

    pub fn get_request(&self) -> &LibDisirRequest {
        match self.msg {
            ::std::option::Option::Some(FromClient_oneof_msg::request(ref v)) => v,
            _ => LibDisirRequest::default_instance(),
        }
    }
}

impl ::protobuf::Message for FromClient {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.command_identifier = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(FromClient_oneof_msg::ping(is.read_bool()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(FromClient_oneof_msg::pong(is.read_bool()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(FromClient_oneof_msg::user_info(is.read_message()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(FromClient_oneof_msg::request(is.read_message()?));
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.command_identifier != 0 {
            my_size += ::protobuf::rt::value_size(1, self.command_identifier, ::protobuf::wire_format::WireTypeVarint);
        };
        if let ::std::option::Option::Some(ref v) = self.msg {
            match v {
                &FromClient_oneof_msg::ping(v) => {
                    my_size += 2;
                },
                &FromClient_oneof_msg::pong(v) => {
                    my_size += 2;
                },
                &FromClient_oneof_msg::user_info(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &FromClient_oneof_msg::request(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.command_identifier != 0 {
            os.write_uint64(1, self.command_identifier)?;
        };
        if let ::std::option::Option::Some(ref v) = self.msg {
            match v {
                &FromClient_oneof_msg::ping(v) => {
                    os.write_bool(2, v)?;
                },
                &FromClient_oneof_msg::pong(v) => {
                    os.write_bool(3, v)?;
                },
                &FromClient_oneof_msg::user_info(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &FromClient_oneof_msg::request(ref v) => {
                    os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FromClient {
    fn new() -> FromClient {
        FromClient::new()
    }

    fn descriptor_static(_: ::std::option::Option<FromClient>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "command_identifier",
                    FromClient::get_command_identifier_for_reflect,
                    FromClient::mut_command_identifier_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor::<_>(
                    "ping",
                    FromClient::has_ping,
                    FromClient::get_ping,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor::<_>(
                    "pong",
                    FromClient::has_pong,
                    FromClient::get_pong,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, UserInfo>(
                    "user_info",
                    FromClient::has_user_info,
                    FromClient::get_user_info,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, LibDisirRequest>(
                    "request",
                    FromClient::has_request,
                    FromClient::get_request,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FromClient>(
                    "FromClient",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FromClient {
    fn clear(&mut self) {
        self.clear_command_identifier();
        self.clear_ping();
        self.clear_pong();
        self.clear_user_info();
        self.clear_request();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FromClient {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FromClient {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Status {
    UNKNOWN = 0,
    OK = 1,
    INTERNAL_ERROR = 2,
}

impl ::protobuf::ProtobufEnum for Status {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Status> {
        match value {
            0 => ::std::option::Option::Some(Status::UNKNOWN),
            1 => ::std::option::Option::Some(Status::OK),
            2 => ::std::option::Option::Some(Status::INTERNAL_ERROR),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Status] = &[
            Status::UNKNOWN,
            Status::OK,
            Status::INTERNAL_ERROR,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Status>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Status", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Status {
}

impl ::std::default::Default for Status {
    fn default() -> Self {
        Status::UNKNOWN
    }
}

impl ::protobuf::reflect::ProtobufValue for Status {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0b, 0x64, 0x69, 0x73, 0x69, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x07, 0x64,
    0x69, 0x73, 0x69, 0x72, 0x70, 0x62, 0x22, 0x53, 0x0a, 0x0f, 0x53, 0x65, 0x6d, 0x61, 0x6e, 0x74,
    0x69, 0x63, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x14, 0x0a, 0x05, 0x6d, 0x61, 0x6a,
    0x6f, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x05, 0x6d, 0x61, 0x6a, 0x6f, 0x72, 0x12,
    0x14, 0x0a, 0x05, 0x6d, 0x69, 0x6e, 0x6f, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x05,
    0x6d, 0x69, 0x6e, 0x6f, 0x72, 0x12, 0x14, 0x0a, 0x05, 0x70, 0x61, 0x74, 0x63, 0x68, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x0d, 0x52, 0x05, 0x70, 0x61, 0x74, 0x63, 0x68, 0x22, 0x5d, 0x0a, 0x08, 0x55,
    0x73, 0x65, 0x72, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x32, 0x0a, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69,
    0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x64, 0x69, 0x73, 0x69, 0x72,
    0x70, 0x62, 0x2e, 0x53, 0x65, 0x6d, 0x61, 0x6e, 0x74, 0x69, 0x63, 0x56, 0x65, 0x72, 0x73, 0x69,
    0x6f, 0x6e, 0x52, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x1d, 0x0a, 0x0a, 0x75,
    0x73, 0x65, 0x72, 0x5f, 0x61, 0x67, 0x65, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x09, 0x75, 0x73, 0x65, 0x72, 0x41, 0x67, 0x65, 0x6e, 0x74, 0x22, 0xb3, 0x01, 0x0a, 0x0a, 0x53,
    0x79, 0x73, 0x74, 0x65, 0x6d, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x32, 0x0a, 0x07, 0x64, 0x69, 0x73,
    0x69, 0x72, 0x70, 0x62, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x64, 0x69, 0x73,
    0x69, 0x72, 0x70, 0x62, 0x2e, 0x53, 0x65, 0x6d, 0x61, 0x6e, 0x74, 0x69, 0x63, 0x56, 0x65, 0x72,
    0x73, 0x69, 0x6f, 0x6e, 0x52, 0x07, 0x64, 0x69, 0x73, 0x69, 0x72, 0x70, 0x62, 0x12, 0x34, 0x0a,
    0x08, 0x6c, 0x69, 0x62, 0x64, 0x69, 0x73, 0x69, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x18, 0x2e, 0x64, 0x69, 0x73, 0x69, 0x72, 0x70, 0x62, 0x2e, 0x53, 0x65, 0x6d, 0x61, 0x6e, 0x74,
    0x69, 0x63, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x52, 0x08, 0x6c, 0x69, 0x62, 0x64, 0x69,
    0x73, 0x69, 0x72, 0x12, 0x23, 0x0a, 0x0d, 0x62, 0x69, 0x6e, 0x64, 0x5f, 0x61, 0x63, 0x63, 0x65,
    0x70, 0x74, 0x65, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0c, 0x62, 0x69, 0x6e, 0x64,
    0x41, 0x63, 0x63, 0x65, 0x70, 0x74, 0x65, 0x64, 0x12, 0x16, 0x0a, 0x06, 0x72, 0x65, 0x61, 0x73,
    0x6f, 0x6e, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x72, 0x65, 0x61, 0x73, 0x6f, 0x6e,
    0x22, 0xc6, 0x01, 0x0a, 0x0e, 0x44, 0x69, 0x73, 0x69, 0x72, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x54,
    0x79, 0x70, 0x65, 0x12, 0x21, 0x0a, 0x0b, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x5f, 0x74, 0x79,
    0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x48, 0x00, 0x52, 0x0a, 0x73, 0x74, 0x72, 0x69,
    0x6e, 0x67, 0x54, 0x79, 0x70, 0x65, 0x12, 0x23, 0x0a, 0x0c, 0x69, 0x6e, 0x74, 0x65, 0x67, 0x65,
    0x72, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x48, 0x00, 0x52, 0x0b,
    0x69, 0x6e, 0x74, 0x65, 0x67, 0x65, 0x72, 0x54, 0x79, 0x70, 0x65, 0x12, 0x1f, 0x0a, 0x0a, 0x66,
    0x6c, 0x6f, 0x61, 0x74, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x48,
    0x00, 0x52, 0x09, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x54, 0x79, 0x70, 0x65, 0x12, 0x23, 0x0a, 0x0c,
    0x62, 0x6f, 0x6f, 0x6c, 0x65, 0x61, 0x6e, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x08, 0x48, 0x00, 0x52, 0x0b, 0x62, 0x6f, 0x6f, 0x6c, 0x65, 0x61, 0x6e, 0x54, 0x79, 0x70,
    0x65, 0x12, 0x1d, 0x0a, 0x09, 0x65, 0x6e, 0x75, 0x6d, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x05,
    0x20, 0x01, 0x28, 0x08, 0x48, 0x00, 0x52, 0x08, 0x65, 0x6e, 0x75, 0x6d, 0x54, 0x79, 0x70, 0x65,
    0x42, 0x07, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x22, 0x34, 0x0a, 0x0e, 0x44, 0x69, 0x73,
    0x69, 0x72, 0x4d, 0x6f, 0x6c, 0x64, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x74,
    0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65, 0x12,
    0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x02, 0x69, 0x64, 0x22,
    0x36, 0x0a, 0x10, 0x44, 0x69, 0x73, 0x69, 0x72, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x49, 0x6e,
    0x70, 0x75, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x02, 0x69, 0x64, 0x22, 0x9f, 0x01, 0x0a, 0x09, 0x44, 0x69, 0x73, 0x69,
    0x72, 0x4d, 0x6f, 0x6c, 0x64, 0x12, 0x17, 0x0a, 0x07, 0x6d, 0x6f, 0x6c, 0x64, 0x5f, 0x69, 0x64,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x06, 0x6d, 0x6f, 0x6c, 0x64, 0x49, 0x64, 0x12, 0x32,
    0x0a, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x18, 0x2e, 0x64, 0x69, 0x73, 0x69, 0x72, 0x70, 0x62, 0x2e, 0x53, 0x65, 0x6d, 0x61, 0x6e, 0x74,
    0x69, 0x63, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x52, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69,
    0x6f, 0x6e, 0x12, 0x24, 0x0a, 0x0d, 0x64, 0x6f, 0x63, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0d, 0x64, 0x6f, 0x63, 0x75, 0x6d,
    0x65, 0x6e, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x1f, 0x0a, 0x0b, 0x65, 0x6c, 0x65, 0x6d,
    0x65, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x73, 0x18, 0x04, 0x20, 0x03, 0x28, 0x04, 0x52, 0x0a, 0x65,
    0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x49, 0x64, 0x73, 0x22, 0x7f, 0x0a, 0x0b, 0x44, 0x69, 0x73,
    0x69, 0x72, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x12, 0x1b, 0x0a, 0x09, 0x63, 0x6f, 0x6e, 0x66,
    0x69, 0x67, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x08, 0x63, 0x6f, 0x6e,
    0x66, 0x69, 0x67, 0x49, 0x64, 0x12, 0x32, 0x0a, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x64, 0x69, 0x73, 0x69, 0x72, 0x70, 0x62,
    0x2e, 0x53, 0x65, 0x6d, 0x61, 0x6e, 0x74, 0x69, 0x63, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e,
    0x52, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x1f, 0x0a, 0x0b, 0x65, 0x6c, 0x65,
    0x6d, 0x65, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x04, 0x52, 0x0a,
    0x65, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x49, 0x64, 0x73, 0x22, 0xb1, 0x03, 0x0a, 0x0a, 0x4d,
    0x6f, 0x6c, 0x64, 0x4b, 0x65, 0x79, 0x76, 0x61, 0x6c, 0x12, 0x1b, 0x0a, 0x09, 0x6b, 0x65, 0x79,
    0x76, 0x61, 0x6c, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x08, 0x6b, 0x65,
    0x79, 0x76, 0x61, 0x6c, 0x49, 0x64, 0x12, 0x2c, 0x0a, 0x11, 0x70, 0x61, 0x72, 0x65, 0x6e, 0x74,
    0x5f, 0x73, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x14, 0x20, 0x01, 0x28,
    0x04, 0x48, 0x00, 0x52, 0x0f, 0x70, 0x61, 0x72, 0x65, 0x6e, 0x74, 0x53, 0x65, 0x63, 0x74, 0x69,
    0x6f, 0x6e, 0x49, 0x64, 0x12, 0x26, 0x0a, 0x0e, 0x70, 0x61, 0x72, 0x65, 0x6e, 0x74, 0x5f, 0x6d,
    0x6f, 0x6c, 0x64, 0x5f, 0x69, 0x64, 0x18, 0x15, 0x20, 0x01, 0x28, 0x04, 0x48, 0x00, 0x52, 0x0c,
    0x70, 0x61, 0x72, 0x65, 0x6e, 0x74, 0x4d, 0x6f, 0x6c, 0x64, 0x49, 0x64, 0x12, 0x12, 0x0a, 0x04,
    0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65,
    0x12, 0x2b, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17,
    0x2e, 0x64, 0x69, 0x73, 0x69, 0x72, 0x70, 0x62, 0x2e, 0x44, 0x69, 0x73, 0x69, 0x72, 0x56, 0x61,
    0x6c, 0x75, 0x65, 0x54, 0x79, 0x70, 0x65, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65, 0x12, 0x38, 0x0a,
    0x0a, 0x69, 0x6e, 0x74, 0x72, 0x6f, 0x64, 0x75, 0x63, 0x65, 0x64, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x18, 0x2e, 0x64, 0x69, 0x73, 0x69, 0x72, 0x70, 0x62, 0x2e, 0x53, 0x65, 0x6d, 0x61,
    0x6e, 0x74, 0x69, 0x63, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x52, 0x0a, 0x69, 0x6e, 0x74,
    0x72, 0x6f, 0x64, 0x75, 0x63, 0x65, 0x64, 0x12, 0x38, 0x0a, 0x0a, 0x64, 0x65, 0x70, 0x72, 0x65,
    0x63, 0x61, 0x74, 0x65, 0x64, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x64, 0x69,
    0x73, 0x69, 0x72, 0x70, 0x62, 0x2e, 0x53, 0x65, 0x6d, 0x61, 0x6e, 0x74, 0x69, 0x63, 0x56, 0x65,
    0x72, 0x73, 0x69, 0x6f, 0x6e, 0x52, 0x0a, 0x64, 0x65, 0x70, 0x72, 0x65, 0x63, 0x61, 0x74, 0x65,
    0x64, 0x12, 0x24, 0x0a, 0x0d, 0x64, 0x6f, 0x63, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x18, 0x06, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0d, 0x64, 0x6f, 0x63, 0x75, 0x6d, 0x65,
    0x6e, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x31, 0x0a, 0x08, 0x64, 0x65, 0x66, 0x61, 0x75,
    0x6c, 0x74, 0x73, 0x18, 0x07, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x64, 0x69, 0x73, 0x69,
    0x72, 0x70, 0x62, 0x2e, 0x44, 0x69, 0x73, 0x69, 0x72, 0x44, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74,
    0x52, 0x08, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x73, 0x12, 0x18, 0x0a, 0x07, 0x69, 0x6e,
    0x76, 0x61, 0x6c, 0x69, 0x64, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x08, 0x52, 0x07, 0x69, 0x6e, 0x76,
    0x61, 0x6c, 0x69, 0x64, 0x42, 0x08, 0x0a, 0x06, 0x70, 0x61, 0x72, 0x65, 0x6e, 0x74, 0x22, 0x6a,
    0x0a, 0x0c, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x4b, 0x65, 0x79, 0x76, 0x61, 0x6c, 0x12, 0x1b,
    0x0a, 0x09, 0x6b, 0x65, 0x79, 0x76, 0x61, 0x6c, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x04, 0x52, 0x08, 0x6b, 0x65, 0x79, 0x76, 0x61, 0x6c, 0x49, 0x64, 0x12, 0x12, 0x0a, 0x04, 0x6e,
    0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12,
    0x29, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x13,
    0x2e, 0x64, 0x69, 0x73, 0x69, 0x72, 0x70, 0x62, 0x2e, 0x44, 0x69, 0x73, 0x69, 0x72, 0x56, 0x61,
    0x6c, 0x75, 0x65, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x22, 0xdb, 0x02, 0x0a, 0x0b, 0x4d,
    0x6f, 0x6c, 0x64, 0x53, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x1d, 0x0a, 0x0a, 0x73, 0x65,
    0x63, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x09,
    0x73, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x64, 0x12, 0x2c, 0x0a, 0x11, 0x70, 0x61, 0x72,
    0x65, 0x6e, 0x74, 0x5f, 0x73, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x0a,
    0x20, 0x01, 0x28, 0x04, 0x48, 0x00, 0x52, 0x0f, 0x70, 0x61, 0x72, 0x65, 0x6e, 0x74, 0x53, 0x65,
    0x63, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x64, 0x12, 0x26, 0x0a, 0x0e, 0x70, 0x61, 0x72, 0x65, 0x6e,
    0x74, 0x5f, 0x6d, 0x6f, 0x6c, 0x64, 0x5f, 0x69, 0x64, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x04, 0x48,
    0x00, 0x52, 0x0c, 0x70, 0x61, 0x72, 0x65, 0x6e, 0x74, 0x4d, 0x6f, 0x6c, 0x64, 0x49, 0x64, 0x12,
    0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e,
    0x61, 0x6d, 0x65, 0x12, 0x38, 0x0a, 0x0a, 0x69, 0x6e, 0x74, 0x72, 0x6f, 0x64, 0x75, 0x63, 0x65,
    0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x64, 0x69, 0x73, 0x69, 0x72, 0x70,
    0x62, 0x2e, 0x53, 0x65, 0x6d, 0x61, 0x6e, 0x74, 0x69, 0x63, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f,
    0x6e, 0x52, 0x0a, 0x69, 0x6e, 0x74, 0x72, 0x6f, 0x64, 0x75, 0x63, 0x65, 0x64, 0x12, 0x38, 0x0a,
    0x0a, 0x64, 0x65, 0x70, 0x72, 0x65, 0x63, 0x61, 0x74, 0x65, 0x64, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x18, 0x2e, 0x64, 0x69, 0x73, 0x69, 0x72, 0x70, 0x62, 0x2e, 0x53, 0x65, 0x6d, 0x61,
    0x6e, 0x74, 0x69, 0x63, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x52, 0x0a, 0x64, 0x65, 0x70,
    0x72, 0x65, 0x63, 0x61, 0x74, 0x65, 0x64, 0x12, 0x24, 0x0a, 0x0d, 0x64, 0x6f, 0x63, 0x75, 0x6d,
    0x65, 0x6e, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0d,
    0x64, 0x6f, 0x63, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x1f, 0x0a,
    0x0b, 0x65, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x73, 0x18, 0x06, 0x20, 0x03,
    0x28, 0x04, 0x52, 0x0a, 0x65, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x49, 0x64, 0x73, 0x42, 0x08,
    0x0a, 0x06, 0x70, 0x61, 0x72, 0x65, 0x6e, 0x74, 0x22, 0x63, 0x0a, 0x0d, 0x43, 0x6f, 0x6e, 0x66,
    0x69, 0x67, 0x53, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x1d, 0x0a, 0x0a, 0x73, 0x65, 0x63,
    0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x09, 0x73,
    0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x64, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x1f, 0x0a, 0x0b,
    0x65, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28,
    0x04, 0x52, 0x0a, 0x65, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x49, 0x64, 0x73, 0x22, 0x82, 0x02,
    0x0a, 0x07, 0x45, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x12, 0x36, 0x0a, 0x0b, 0x6d, 0x6f, 0x6c,
    0x64, 0x5f, 0x6b, 0x65, 0x79, 0x76, 0x61, 0x6c, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x13,
    0x2e, 0x64, 0x69, 0x73, 0x69, 0x72, 0x70, 0x62, 0x2e, 0x4d, 0x6f, 0x6c, 0x64, 0x4b, 0x65, 0x79,
    0x76, 0x61, 0x6c, 0x48, 0x00, 0x52, 0x0a, 0x6d, 0x6f, 0x6c, 0x64, 0x4b, 0x65, 0x79, 0x76, 0x61,
    0x6c, 0x12, 0x39, 0x0a, 0x0c, 0x6d, 0x6f, 0x6c, 0x64, 0x5f, 0x73, 0x65, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x64, 0x69, 0x73, 0x69, 0x72, 0x70,
    0x62, 0x2e, 0x4d, 0x6f, 0x6c, 0x64, 0x53, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x48, 0x00, 0x52,
    0x0b, 0x6d, 0x6f, 0x6c, 0x64, 0x53, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x3c, 0x0a, 0x0d,
    0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x5f, 0x6b, 0x65, 0x79, 0x76, 0x61, 0x6c, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x64, 0x69, 0x73, 0x69, 0x72, 0x70, 0x62, 0x2e, 0x43, 0x6f,
    0x6e, 0x66, 0x69, 0x67, 0x4b, 0x65, 0x79, 0x76, 0x61, 0x6c, 0x48, 0x00, 0x52, 0x0c, 0x63, 0x6f,
    0x6e, 0x66, 0x69, 0x67, 0x4b, 0x65, 0x79, 0x76, 0x61, 0x6c, 0x12, 0x3f, 0x0a, 0x0e, 0x63, 0x6f,
    0x6e, 0x66, 0x69, 0x67, 0x5f, 0x73, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x16, 0x2e, 0x64, 0x69, 0x73, 0x69, 0x72, 0x70, 0x62, 0x2e, 0x43, 0x6f, 0x6e,
    0x66, 0x69, 0x67, 0x53, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x48, 0x00, 0x52, 0x0d, 0x63, 0x6f,
    0x6e, 0x66, 0x69, 0x67, 0x53, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x42, 0x05, 0x0a, 0x03, 0x6d,
    0x73, 0x67, 0x22, 0x92, 0x01, 0x0a, 0x0c, 0x44, 0x69, 0x73, 0x69, 0x72, 0x44, 0x65, 0x66, 0x61,
    0x75, 0x6c, 0x74, 0x12, 0x1d, 0x0a, 0x0a, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x5f, 0x69,
    0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x09, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74,
    0x49, 0x64, 0x12, 0x38, 0x0a, 0x0a, 0x69, 0x6e, 0x74, 0x72, 0x6f, 0x64, 0x75, 0x63, 0x65, 0x64,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x64, 0x69, 0x73, 0x69, 0x72, 0x70, 0x62,
    0x2e, 0x53, 0x65, 0x6d, 0x61, 0x6e, 0x74, 0x69, 0x63, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e,
    0x52, 0x0a, 0x69, 0x6e, 0x74, 0x72, 0x6f, 0x64, 0x75, 0x63, 0x65, 0x64, 0x12, 0x29, 0x0a, 0x05,
    0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x64, 0x69,
    0x73, 0x69, 0x72, 0x70, 0x62, 0x2e, 0x44, 0x69, 0x73, 0x69, 0x72, 0x56, 0x61, 0x6c, 0x75, 0x65,
    0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x22, 0xab, 0x01, 0x0a, 0x0a, 0x44, 0x69, 0x73, 0x69,
    0x72, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x23, 0x0a, 0x0c, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67,
    0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x0b,
    0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x25, 0x0a, 0x0d, 0x62,
    0x6f, 0x6f, 0x6c, 0x65, 0x61, 0x6e, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x08, 0x48, 0x00, 0x52, 0x0c, 0x62, 0x6f, 0x6f, 0x6c, 0x65, 0x61, 0x6e, 0x56, 0x61, 0x6c,
    0x75, 0x65, 0x12, 0x25, 0x0a, 0x0d, 0x69, 0x6e, 0x74, 0x65, 0x67, 0x65, 0x72, 0x5f, 0x76, 0x61,
    0x6c, 0x75, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x03, 0x48, 0x00, 0x52, 0x0c, 0x69, 0x6e, 0x74,
    0x65, 0x67, 0x65, 0x72, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x21, 0x0a, 0x0b, 0x66, 0x6c, 0x6f,
    0x61, 0x74, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x01, 0x48, 0x00,
    0x52, 0x0a, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x42, 0x07, 0x0a, 0x05,
    0x76, 0x61, 0x6c, 0x75, 0x65, 0x22, 0x34, 0x0a, 0x0e, 0x51, 0x75, 0x65, 0x72, 0x79, 0x4d, 0x6f,
    0x6c, 0x64, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12, 0x12, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65, 0x12, 0x0e, 0x0a, 0x02, 0x69,
    0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x02, 0x69, 0x64, 0x22, 0x36, 0x0a, 0x10, 0x51,
    0x75, 0x65, 0x72, 0x79, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12,
    0x12, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x74,
    0x79, 0x70, 0x65, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x02, 0x69, 0x64, 0x22, 0x82, 0x01, 0x0a, 0x0e, 0x41, 0x64, 0x64, 0x43, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12, 0x2f, 0x0a, 0x06, 0x6b, 0x65, 0x79, 0x76, 0x61, 0x6c,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x64, 0x69, 0x73, 0x69, 0x72, 0x70, 0x62,
    0x2e, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x4b, 0x65, 0x79, 0x76, 0x61, 0x6c, 0x48, 0x00, 0x52,
    0x06, 0x6b, 0x65, 0x79, 0x76, 0x61, 0x6c, 0x12, 0x1b, 0x0a, 0x09, 0x63, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x08, 0x63, 0x6f, 0x6e, 0x66,
    0x69, 0x67, 0x49, 0x64, 0x12, 0x1b, 0x0a, 0x09, 0x70, 0x61, 0x72, 0x65, 0x6e, 0x74, 0x5f, 0x69,
    0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x52, 0x08, 0x70, 0x61, 0x72, 0x65, 0x6e, 0x74, 0x49,
    0x64, 0x42, 0x05, 0x0a, 0x03, 0x6d, 0x73, 0x67, 0x22, 0x6e, 0x0a, 0x0c, 0x51, 0x75, 0x65, 0x72,
    0x79, 0x45, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x12, 0x19, 0x0a, 0x07, 0x6d, 0x6f, 0x6c, 0x64,
    0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x48, 0x00, 0x52, 0x06, 0x6d, 0x6f, 0x6c,
    0x64, 0x49, 0x64, 0x12, 0x1d, 0x0a, 0x09, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x5f, 0x69, 0x64,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x48, 0x00, 0x52, 0x08, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67,
    0x49, 0x64, 0x12, 0x1d, 0x0a, 0x0a, 0x65, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x5f, 0x69, 0x64,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x52, 0x09, 0x65, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x49,
    0x64, 0x42, 0x05, 0x0a, 0x03, 0x6d, 0x73, 0x67, 0x22, 0xac, 0x03, 0x0a, 0x0f, 0x4c, 0x69, 0x62,
    0x44, 0x69, 0x73, 0x69, 0x72, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x23, 0x0a, 0x0c,
    0x71, 0x75, 0x65, 0x72, 0x79, 0x5f, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x73, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x08, 0x48, 0x00, 0x52, 0x0b, 0x71, 0x75, 0x65, 0x72, 0x79, 0x47, 0x72, 0x6f, 0x75, 0x70,
    0x73, 0x12, 0x2e, 0x0a, 0x12, 0x71, 0x75, 0x65, 0x72, 0x79, 0x5f, 0x6d, 0x6f, 0x6c, 0x64, 0x5f,
    0x65, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52,
    0x10, 0x71, 0x75, 0x65, 0x72, 0x79, 0x4d, 0x6f, 0x6c, 0x64, 0x45, 0x6e, 0x74, 0x72, 0x69, 0x65,
    0x73, 0x12, 0x43, 0x0a, 0x10, 0x71, 0x75, 0x65, 0x72, 0x79, 0x5f, 0x6d, 0x6f, 0x6c, 0x64, 0x5f,
    0x65, 0x6e, 0x74, 0x72, 0x79, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x64, 0x69,
    0x73, 0x69, 0x72, 0x70, 0x62, 0x2e, 0x51, 0x75, 0x65, 0x72, 0x79, 0x4d, 0x6f, 0x6c, 0x64, 0x45,
    0x6e, 0x74, 0x72, 0x79, 0x48, 0x00, 0x52, 0x0e, 0x71, 0x75, 0x65, 0x72, 0x79, 0x4d, 0x6f, 0x6c,
    0x64, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12, 0x2a, 0x0a, 0x10, 0x63, 0x6c, 0x6f, 0x73, 0x65, 0x5f,
    0x6d, 0x6f, 0x6c, 0x64, 0x5f, 0x65, 0x6e, 0x74, 0x72, 0x79, 0x18, 0x05, 0x20, 0x01, 0x28, 0x04,
    0x48, 0x00, 0x52, 0x0e, 0x63, 0x6c, 0x6f, 0x73, 0x65, 0x4d, 0x6f, 0x6c, 0x64, 0x45, 0x6e, 0x74,
    0x72, 0x79, 0x12, 0x3c, 0x0a, 0x0d, 0x71, 0x75, 0x65, 0x72, 0x79, 0x5f, 0x65, 0x6c, 0x65, 0x6d,
    0x65, 0x6e, 0x74, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x64, 0x69, 0x73, 0x69,
    0x72, 0x70, 0x62, 0x2e, 0x51, 0x75, 0x65, 0x72, 0x79, 0x45, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74,
    0x48, 0x00, 0x52, 0x0c, 0x71, 0x75, 0x65, 0x72, 0x79, 0x45, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74,
    0x12, 0x49, 0x0a, 0x12, 0x71, 0x75, 0x65, 0x72, 0x79, 0x5f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67,
    0x5f, 0x65, 0x6e, 0x74, 0x72, 0x79, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x64,
    0x69, 0x73, 0x69, 0x72, 0x70, 0x62, 0x2e, 0x51, 0x75, 0x65, 0x72, 0x79, 0x43, 0x6f, 0x6e, 0x66,
    0x69, 0x67, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x48, 0x00, 0x52, 0x10, 0x71, 0x75, 0x65, 0x72, 0x79,
    0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12, 0x43, 0x0a, 0x10, 0x61,
    0x64, 0x64, 0x5f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x5f, 0x65, 0x6e, 0x74, 0x72, 0x79, 0x18,
    0x08, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x64, 0x69, 0x73, 0x69, 0x72, 0x70, 0x62, 0x2e,
    0x41, 0x64, 0x64, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x48, 0x00,
    0x52, 0x0e, 0x61, 0x64, 0x64, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x45, 0x6e, 0x74, 0x72, 0x79,
    0x42, 0x05, 0x0a, 0x03, 0x6d, 0x73, 0x67, 0x22, 0xa3, 0x02, 0x0a, 0x10, 0x4c, 0x69, 0x62, 0x44,
    0x69, 0x73, 0x69, 0x72, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x27, 0x0a, 0x06,
    0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x0f, 0x2e, 0x64,
    0x69, 0x73, 0x69, 0x72, 0x70, 0x62, 0x2e, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x52, 0x06, 0x73,
    0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x21, 0x0a, 0x0c, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x5f, 0x73,
    0x74, 0x72, 0x69, 0x6e, 0x67, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x65, 0x72, 0x72,
    0x6f, 0x72, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x12, 0x1b, 0x0a, 0x09, 0x67, 0x72, 0x6f, 0x75,
    0x70, 0x5f, 0x69, 0x64, 0x73, 0x18, 0x0a, 0x20, 0x03, 0x28, 0x09, 0x52, 0x08, 0x67, 0x72, 0x6f,
    0x75, 0x70, 0x49, 0x64, 0x73, 0x12, 0x24, 0x0a, 0x0e, 0x6d, 0x6f, 0x6c, 0x64, 0x5f, 0x65, 0x6e,
    0x74, 0x72, 0x79, 0x5f, 0x69, 0x64, 0x73, 0x18, 0x0c, 0x20, 0x03, 0x28, 0x09, 0x52, 0x0c, 0x6d,
    0x6f, 0x6c, 0x64, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x49, 0x64, 0x73, 0x12, 0x26, 0x0a, 0x04, 0x6d,
    0x6f, 0x6c, 0x64, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x64, 0x69, 0x73, 0x69,
    0x72, 0x70, 0x62, 0x2e, 0x44, 0x69, 0x73, 0x69, 0x72, 0x4d, 0x6f, 0x6c, 0x64, 0x52, 0x04, 0x6d,
    0x6f, 0x6c, 0x64, 0x12, 0x2a, 0x0a, 0x07, 0x65, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x18, 0x0e,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64, 0x69, 0x73, 0x69, 0x72, 0x70, 0x62, 0x2e, 0x45,
    0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x52, 0x07, 0x65, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x12,
    0x2c, 0x0a, 0x06, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x18, 0x0f, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x14, 0x2e, 0x64, 0x69, 0x73, 0x69, 0x72, 0x70, 0x62, 0x2e, 0x44, 0x69, 0x73, 0x69, 0x72, 0x43,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x52, 0x06, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x22, 0xdd, 0x01,
    0x0a, 0x08, 0x54, 0x6f, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x12, 0x2d, 0x0a, 0x12, 0x63, 0x6f,
    0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x5f, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x72,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x11, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x49,
    0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x72, 0x12, 0x14, 0x0a, 0x04, 0x70, 0x69, 0x6e,
    0x67, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x48, 0x00, 0x52, 0x04, 0x70, 0x69, 0x6e, 0x67, 0x12,
    0x14, 0x0a, 0x04, 0x70, 0x6f, 0x6e, 0x67, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x48, 0x00, 0x52,
    0x04, 0x70, 0x6f, 0x6e, 0x67, 0x12, 0x36, 0x0a, 0x0b, 0x73, 0x79, 0x73, 0x74, 0x65, 0x6d, 0x5f,
    0x69, 0x6e, 0x66, 0x6f, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x64, 0x69, 0x73,
    0x69, 0x72, 0x70, 0x62, 0x2e, 0x53, 0x79, 0x73, 0x74, 0x65, 0x6d, 0x49, 0x6e, 0x66, 0x6f, 0x48,
    0x00, 0x52, 0x0a, 0x73, 0x79, 0x73, 0x74, 0x65, 0x6d, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x37, 0x0a,
    0x08, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x19, 0x2e, 0x64, 0x69, 0x73, 0x69, 0x72, 0x70, 0x62, 0x2e, 0x4c, 0x69, 0x62, 0x44, 0x69, 0x73,
    0x69, 0x72, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x00, 0x52, 0x08, 0x72, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x42, 0x05, 0x0a, 0x03, 0x6d, 0x73, 0x67, 0x22, 0xd6, 0x01,
    0x0a, 0x0a, 0x46, 0x72, 0x6f, 0x6d, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x12, 0x2d, 0x0a, 0x12,
    0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x5f, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69,
    0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x11, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e,
    0x64, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x72, 0x12, 0x14, 0x0a, 0x04, 0x70,
    0x69, 0x6e, 0x67, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x48, 0x00, 0x52, 0x04, 0x70, 0x69, 0x6e,
    0x67, 0x12, 0x14, 0x0a, 0x04, 0x70, 0x6f, 0x6e, 0x67, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x48,
    0x00, 0x52, 0x04, 0x70, 0x6f, 0x6e, 0x67, 0x12, 0x30, 0x0a, 0x09, 0x75, 0x73, 0x65, 0x72, 0x5f,
    0x69, 0x6e, 0x66, 0x6f, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x64, 0x69, 0x73,
    0x69, 0x72, 0x70, 0x62, 0x2e, 0x55, 0x73, 0x65, 0x72, 0x49, 0x6e, 0x66, 0x6f, 0x48, 0x00, 0x52,
    0x08, 0x75, 0x73, 0x65, 0x72, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x34, 0x0a, 0x07, 0x72, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x64, 0x69, 0x73,
    0x69, 0x72, 0x70, 0x62, 0x2e, 0x4c, 0x69, 0x62, 0x44, 0x69, 0x73, 0x69, 0x72, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x48, 0x00, 0x52, 0x07, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x42,
    0x05, 0x0a, 0x03, 0x6d, 0x73, 0x67, 0x2a, 0x31, 0x0a, 0x06, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73,
    0x12, 0x0b, 0x0a, 0x07, 0x55, 0x4e, 0x4b, 0x4e, 0x4f, 0x57, 0x4e, 0x10, 0x00, 0x12, 0x06, 0x0a,
    0x02, 0x4f, 0x4b, 0x10, 0x01, 0x12, 0x12, 0x0a, 0x0e, 0x49, 0x4e, 0x54, 0x45, 0x52, 0x4e, 0x41,
    0x4c, 0x5f, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x10, 0x02, 0x4a, 0xce, 0x40, 0x0a, 0x07, 0x12, 0x05,
    0x00, 0x00, 0xdc, 0x01, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a,
    0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x08, 0x0f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12,
    0x04, 0x05, 0x00, 0x09, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x05, 0x08,
    0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x06, 0x04, 0x15, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x06, 0x04, 0x05, 0x19, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x06, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x06, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x06, 0x13, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12,
    0x03, 0x07, 0x04, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0x07,
    0x04, 0x06, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x07, 0x04,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x07, 0x0b, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x07, 0x13, 0x14, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x08, 0x04, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x04, 0x12, 0x04, 0x08, 0x04, 0x07, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x08, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x08, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x08, 0x13, 0x14, 0x0a, 0x5b, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x0c, 0x00, 0x0f, 0x01, 0x1a,
    0x4f, 0x20, 0x55, 0x73, 0x65, 0x72, 0x20, 0x73, 0x65, 0x6e, 0x64, 0x73, 0x20, 0x74, 0x68, 0x69,
    0x73, 0x20, 0x55, 0x73, 0x65, 0x72, 0x49, 0x6e, 0x66, 0x6f, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x66, 0x69, 0x72, 0x73, 0x74, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20,
    0x61, 0x66, 0x74, 0x65, 0x72, 0x20, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e,
    0x20, 0x69, 0x73, 0x20, 0x65, 0x73, 0x74, 0x61, 0x62, 0x6c, 0x69, 0x73, 0x68, 0x65, 0x64, 0x0a,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x0c, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0d, 0x04, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x04, 0x12, 0x04, 0x0d, 0x04, 0x0c, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x06, 0x12, 0x03, 0x0d, 0x04, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x0d, 0x14, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0d,
    0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0e, 0x04, 0x1a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x04, 0x0e, 0x04, 0x0d, 0x20, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0e, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0e, 0x0b, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x0e, 0x18, 0x19, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04,
    0x11, 0x00, 0x15, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x11, 0x05, 0x0b,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x12, 0x04, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x12, 0x04, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x12, 0x0e, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02,
    0x01, 0x12, 0x03, 0x13, 0x04, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x13, 0x04, 0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x13,
    0x09, 0x0a, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x14, 0x04, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x14, 0x04, 0x12, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x14, 0x15, 0x16, 0x0a, 0x45, 0x0a, 0x02, 0x04,
    0x02, 0x12, 0x04, 0x18, 0x00, 0x1d, 0x01, 0x1a, 0x39, 0x20, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72,
    0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x20, 0x55, 0x73,
    0x65, 0x72, 0x49, 0x6e, 0x66, 0x6f, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x77,
    0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x18, 0x08, 0x12, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x19, 0x04, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x00, 0x04, 0x12, 0x04, 0x19, 0x04, 0x18, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x06, 0x12, 0x03, 0x19, 0x04, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x19, 0x14, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x19, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1a, 0x04,
    0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x04, 0x1a, 0x04, 0x19, 0x20,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x06, 0x12, 0x03, 0x1a, 0x04, 0x13, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1a, 0x14, 0x1c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1a, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x02, 0x12, 0x03, 0x1b, 0x04, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04,
    0x12, 0x04, 0x1b, 0x04, 0x1a, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x05, 0x12,
    0x03, 0x1b, 0x04, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1b,
    0x09, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x1b, 0x19, 0x1a,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x1c, 0x04, 0x16, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x03, 0x04, 0x12, 0x04, 0x1c, 0x04, 0x1b, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x03, 0x05, 0x12, 0x03, 0x1c, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x1c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x1c, 0x14, 0x15, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x1f, 0x00,
    0x27, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x1f, 0x08, 0x16, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x03, 0x08, 0x00, 0x12, 0x04, 0x20, 0x04, 0x26, 0x05, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x08, 0x00, 0x01, 0x12, 0x03, 0x20, 0x0a, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03,
    0x02, 0x00, 0x12, 0x03, 0x21, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x21, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x21, 0x0d, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x21, 0x1b,
    0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x22, 0x08, 0x1e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x22, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x22, 0x0d, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x22, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02,
    0x12, 0x03, 0x23, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x05, 0x12, 0x03,
    0x23, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x23, 0x0d,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x23, 0x1a, 0x1b, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x03, 0x12, 0x03, 0x24, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x03, 0x05, 0x12, 0x03, 0x24, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x24, 0x0d, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x24, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x04, 0x12, 0x03,
    0x25, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x05, 0x12, 0x03, 0x25, 0x08,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x01, 0x12, 0x03, 0x25, 0x0d, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x03, 0x12, 0x03, 0x25, 0x19, 0x1a, 0x0a, 0x29, 0x0a,
    0x02, 0x04, 0x04, 0x12, 0x04, 0x2a, 0x00, 0x2d, 0x01, 0x1a, 0x1d, 0x20, 0x66, 0x75, 0x6e, 0x63,
    0x74, 0x69, 0x6f, 0x6e, 0x20, 0x64, 0x69, 0x73, 0x69, 0x72, 0x5f, 0x6d, 0x6f, 0x6c, 0x64, 0x5f,
    0x69, 0x6e, 0x70, 0x75, 0x74, 0x28, 0x29, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12,
    0x03, 0x2a, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x2b, 0x04,
    0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x04, 0x2b, 0x04, 0x2a, 0x18,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2b, 0x04, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2b, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2b, 0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04,
    0x02, 0x01, 0x12, 0x03, 0x2c, 0x04, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04,
    0x12, 0x04, 0x2c, 0x04, 0x2b, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x2c, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2c,
    0x0b, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2c, 0x10, 0x11,
    0x0a, 0x2b, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x30, 0x00, 0x33, 0x01, 0x1a, 0x1f, 0x20, 0x66,
    0x75, 0x6e, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x64, 0x69, 0x73, 0x69, 0x72, 0x5f, 0x63, 0x6f,
    0x6e, 0x66, 0x69, 0x67, 0x5f, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x28, 0x29, 0x0a, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x30, 0x08, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02,
    0x00, 0x12, 0x03, 0x31, 0x04, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12,
    0x04, 0x31, 0x04, 0x30, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x31, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x31, 0x0b,
    0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x31, 0x12, 0x13, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x32, 0x04, 0x12, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x01, 0x04, 0x12, 0x04, 0x32, 0x04, 0x31, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x01, 0x05, 0x12, 0x03, 0x32, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x32, 0x0b, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x32, 0x10, 0x11, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x35, 0x00, 0x3a,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x35, 0x08, 0x11, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x36, 0x04, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x36, 0x04, 0x35, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x36, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x36, 0x0b, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x36, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01, 0x12, 0x03, 0x37, 0x04, 0x20,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x04, 0x12, 0x04, 0x37, 0x04, 0x36, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x06, 0x12, 0x03, 0x37, 0x04, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x03, 0x37, 0x14, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x01, 0x03, 0x12, 0x03, 0x37, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02,
    0x02, 0x12, 0x03, 0x38, 0x04, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x04, 0x12,
    0x04, 0x38, 0x04, 0x37, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x05, 0x12, 0x03,
    0x38, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x01, 0x12, 0x03, 0x38, 0x0b,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x03, 0x12, 0x03, 0x38, 0x1b, 0x1c, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x03, 0x12, 0x03, 0x39, 0x04, 0x24, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x03, 0x04, 0x12, 0x03, 0x39, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x03, 0x05, 0x12, 0x03, 0x39, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x39, 0x14, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x03, 0x12,
    0x03, 0x39, 0x22, 0x23, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x3c, 0x00, 0x40, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x3c, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x3d, 0x04, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x00, 0x04, 0x12, 0x04, 0x3d, 0x04, 0x3c, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x3d, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x3d, 0x0b, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3d,
    0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x01, 0x12, 0x03, 0x3e, 0x04, 0x20, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x04, 0x12, 0x04, 0x3e, 0x04, 0x3d, 0x19, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x06, 0x12, 0x03, 0x3e, 0x04, 0x13, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3e, 0x14, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x3e, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x02,
    0x12, 0x03, 0x3f, 0x04, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x04, 0x12, 0x03,
    0x3f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x05, 0x12, 0x03, 0x3f, 0x0d,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x01, 0x12, 0x03, 0x3f, 0x14, 0x1f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x03, 0x12, 0x03, 0x3f, 0x22, 0x23, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x08, 0x12, 0x04, 0x42, 0x00, 0x50, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01,
    0x12, 0x03, 0x42, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03, 0x43,
    0x04, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x04, 0x43, 0x04, 0x42,
    0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x05, 0x12, 0x03, 0x43, 0x04, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x43, 0x0b, 0x14, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x43, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x08, 0x08, 0x00, 0x12, 0x04, 0x44, 0x04, 0x47, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x08,
    0x00, 0x01, 0x12, 0x03, 0x44, 0x0a, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x01, 0x12,
    0x03, 0x45, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x05, 0x12, 0x03, 0x45,
    0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x01, 0x12, 0x03, 0x45, 0x0f, 0x20,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x03, 0x12, 0x03, 0x45, 0x23, 0x25, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x08, 0x02, 0x02, 0x12, 0x03, 0x46, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x02, 0x05, 0x12, 0x03, 0x46, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x46, 0x0f, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x46, 0x20, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x03, 0x12, 0x03, 0x48,
    0x04, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x04, 0x12, 0x04, 0x48, 0x04, 0x47,
    0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x05, 0x12, 0x03, 0x48, 0x04, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x01, 0x12, 0x03, 0x48, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x03, 0x03, 0x12, 0x03, 0x48, 0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x08, 0x02, 0x04, 0x12, 0x03, 0x49, 0x04, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04,
    0x04, 0x12, 0x04, 0x49, 0x04, 0x48, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x06,
    0x12, 0x03, 0x49, 0x04, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x01, 0x12, 0x03,
    0x49, 0x13, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x03, 0x12, 0x03, 0x49, 0x1a,
    0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x05, 0x12, 0x03, 0x4a, 0x04, 0x23, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x05, 0x04, 0x12, 0x04, 0x4a, 0x04, 0x49, 0x1c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x05, 0x06, 0x12, 0x03, 0x4a, 0x04, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x05, 0x01, 0x12, 0x03, 0x4a, 0x14, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x05, 0x03, 0x12, 0x03, 0x4a, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x06, 0x12,
    0x03, 0x4b, 0x04, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x06, 0x04, 0x12, 0x04, 0x4b,
    0x04, 0x4a, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x06, 0x06, 0x12, 0x03, 0x4b, 0x04,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x06, 0x01, 0x12, 0x03, 0x4b, 0x14, 0x1e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x06, 0x03, 0x12, 0x03, 0x4b, 0x21, 0x22, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x08, 0x02, 0x07, 0x12, 0x03, 0x4c, 0x04, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x07, 0x04, 0x12, 0x04, 0x4c, 0x04, 0x4b, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x07, 0x05, 0x12, 0x03, 0x4c, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x07, 0x01,
    0x12, 0x03, 0x4c, 0x0b, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x07, 0x03, 0x12, 0x03,
    0x4c, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x08, 0x12, 0x03, 0x4d, 0x04, 0x27,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x08, 0x04, 0x12, 0x03, 0x4d, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x08, 0x06, 0x12, 0x03, 0x4d, 0x0d, 0x19, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x08, 0x01, 0x12, 0x03, 0x4d, 0x1a, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x08, 0x03, 0x12, 0x03, 0x4d, 0x25, 0x26, 0x0a, 0x27, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x09,
    0x12, 0x03, 0x4e, 0x04, 0x16, 0x22, 0x1a, 0x20, 0x58, 0x58, 0x58, 0x20, 0x4d, 0x69, 0x73, 0x73,
    0x69, 0x6e, 0x67, 0x20, 0x72, 0x65, 0x73, 0x74, 0x72, 0x69, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x09, 0x04, 0x12, 0x04, 0x4e, 0x04, 0x4d, 0x27,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x09, 0x05, 0x12, 0x03, 0x4e, 0x04, 0x08, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x09, 0x01, 0x12, 0x03, 0x4e, 0x09, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x09, 0x03, 0x12, 0x03, 0x4e, 0x13, 0x15, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x09,
    0x12, 0x04, 0x52, 0x00, 0x56, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x03, 0x52,
    0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00, 0x12, 0x03, 0x53, 0x04, 0x19, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x04, 0x53, 0x04, 0x52, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x05, 0x12, 0x03, 0x53, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x03, 0x53, 0x0b, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x53, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x01,
    0x12, 0x03, 0x54, 0x04, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x04, 0x12, 0x04,
    0x54, 0x04, 0x53, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x05, 0x12, 0x03, 0x54,
    0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x01, 0x12, 0x03, 0x54, 0x0b, 0x0f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x03, 0x12, 0x03, 0x54, 0x12, 0x13, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x09, 0x02, 0x02, 0x12, 0x03, 0x55, 0x04, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x02, 0x04, 0x12, 0x04, 0x55, 0x04, 0x54, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x02, 0x06, 0x12, 0x03, 0x55, 0x04, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x55, 0x0f, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x55, 0x17, 0x18, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0a, 0x12, 0x04, 0x58, 0x00, 0x63, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12, 0x03, 0x58, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x0a, 0x02, 0x00, 0x12, 0x03, 0x59, 0x04, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02,
    0x00, 0x04, 0x12, 0x04, 0x59, 0x04, 0x58, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x59, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x59, 0x0b, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x03, 0x59,
    0x18, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0a, 0x08, 0x00, 0x12, 0x04, 0x5a, 0x04, 0x5d, 0x05,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x08, 0x00, 0x01, 0x12, 0x03, 0x5a, 0x0a, 0x10, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x0a, 0x02, 0x01, 0x12, 0x03, 0x5b, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0a, 0x02, 0x01, 0x05, 0x12, 0x03, 0x5b, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x5b, 0x0f, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x5b, 0x23, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x02, 0x12, 0x03, 0x5c,
    0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x02, 0x05, 0x12, 0x03, 0x5c, 0x08, 0x0e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x02, 0x01, 0x12, 0x03, 0x5c, 0x0f, 0x1d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0a, 0x02, 0x02, 0x03, 0x12, 0x03, 0x5c, 0x20, 0x22, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x0a, 0x02, 0x03, 0x12, 0x03, 0x5e, 0x04, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02,
    0x03, 0x04, 0x12, 0x04, 0x5e, 0x04, 0x5d, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x03,
    0x05, 0x12, 0x03, 0x5e, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x5e, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x03, 0x03, 0x12, 0x03, 0x5e,
    0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x04, 0x12, 0x03, 0x5f, 0x04, 0x23, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x5f, 0x04, 0x5e, 0x14, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x03, 0x5f, 0x04, 0x13, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x03, 0x5f, 0x14, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a,
    0x02, 0x04, 0x03, 0x12, 0x03, 0x5f, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x05,
    0x12, 0x03, 0x60, 0x04, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x05, 0x04, 0x12, 0x04,
    0x60, 0x04, 0x5f, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x05, 0x06, 0x12, 0x03, 0x60,
    0x04, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x05, 0x01, 0x12, 0x03, 0x60, 0x14, 0x1e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x05, 0x03, 0x12, 0x03, 0x60, 0x21, 0x22, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x0a, 0x02, 0x06, 0x12, 0x03, 0x61, 0x04, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0a, 0x02, 0x06, 0x04, 0x12, 0x04, 0x61, 0x04, 0x60, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a,
    0x02, 0x06, 0x05, 0x12, 0x03, 0x61, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x06,
    0x01, 0x12, 0x03, 0x61, 0x0b, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x06, 0x03, 0x12,
    0x03, 0x61, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x07, 0x12, 0x03, 0x62, 0x04,
    0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x07, 0x04, 0x12, 0x03, 0x62, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x07, 0x05, 0x12, 0x03, 0x62, 0x0d, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0a, 0x02, 0x07, 0x01, 0x12, 0x03, 0x62, 0x14, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0a, 0x02, 0x07, 0x03, 0x12, 0x03, 0x62, 0x22, 0x23, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0b, 0x12,
    0x04, 0x65, 0x00, 0x69, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x03, 0x65, 0x08,
    0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x00, 0x12, 0x03, 0x66, 0x04, 0x1a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x04, 0x12, 0x04, 0x66, 0x04, 0x65, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x00, 0x05, 0x12, 0x03, 0x66, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x00, 0x01, 0x12, 0x03, 0x66, 0x0b, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x66, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x01, 0x12,
    0x03, 0x67, 0x04, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x04, 0x12, 0x04, 0x67,
    0x04, 0x66, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x05, 0x12, 0x03, 0x67, 0x04,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x01, 0x12, 0x03, 0x67, 0x0b, 0x0f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x03, 0x12, 0x03, 0x67, 0x12, 0x13, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x0b, 0x02, 0x02, 0x12, 0x03, 0x68, 0x04, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x02, 0x04, 0x12, 0x03, 0x68, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02,
    0x05, 0x12, 0x03, 0x68, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x68, 0x14, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x03, 0x12, 0x03, 0x68,
    0x22, 0x23, 0x0a, 0x1d, 0x0a, 0x02, 0x04, 0x0c, 0x12, 0x04, 0x6c, 0x00, 0x73, 0x01, 0x1a, 0x11,
    0x21, 0x20, 0x42, 0x75, 0x69, 0x6c, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b,
    0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0c, 0x01, 0x12, 0x03, 0x6c, 0x08, 0x0f, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x0c, 0x08, 0x00, 0x12, 0x04, 0x6d, 0x04, 0x72, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0c, 0x08, 0x00, 0x01, 0x12, 0x03, 0x6d, 0x0a, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02,
    0x00, 0x12, 0x03, 0x6e, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x6e, 0x08, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x01, 0x12, 0x03, 0x6e,
    0x13, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x03, 0x12, 0x03, 0x6e, 0x21, 0x22,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x01, 0x12, 0x03, 0x6f, 0x08, 0x25, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x01, 0x06, 0x12, 0x03, 0x6f, 0x08, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x01, 0x01, 0x12, 0x03, 0x6f, 0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x6f, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x02, 0x12,
    0x03, 0x70, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x06, 0x12, 0x03, 0x70,
    0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x01, 0x12, 0x03, 0x70, 0x15, 0x22,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x03, 0x12, 0x03, 0x70, 0x25, 0x26, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x0c, 0x02, 0x03, 0x12, 0x03, 0x71, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x03, 0x06, 0x12, 0x03, 0x71, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x71, 0x16, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x71, 0x27, 0x28, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0d, 0x12, 0x04, 0x75, 0x00, 0x79,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0d, 0x01, 0x12, 0x03, 0x75, 0x08, 0x14, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x0d, 0x02, 0x00, 0x12, 0x03, 0x76, 0x04, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x76, 0x04, 0x75, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x76, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x76, 0x0b, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x76, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x01, 0x12, 0x03, 0x77, 0x04, 0x23,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x04, 0x12, 0x04, 0x77, 0x04, 0x76, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x06, 0x12, 0x03, 0x77, 0x04, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x01, 0x01, 0x12, 0x03, 0x77, 0x14, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x01, 0x03, 0x12, 0x03, 0x77, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02,
    0x02, 0x12, 0x03, 0x78, 0x04, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02, 0x04, 0x12,
    0x04, 0x78, 0x04, 0x77, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02, 0x06, 0x12, 0x03,
    0x78, 0x04, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02, 0x01, 0x12, 0x03, 0x78, 0x0f,
    0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02, 0x03, 0x12, 0x03, 0x78, 0x17, 0x18, 0x0a,
    0x0b, 0x0a, 0x02, 0x04, 0x0e, 0x12, 0x05, 0x7b, 0x00, 0x83, 0x01, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x0e, 0x01, 0x12, 0x03, 0x7b, 0x08, 0x12, 0x0a, 0x0d, 0x0a, 0x04, 0x04, 0x0e, 0x08, 0x00,
    0x12, 0x05, 0x7c, 0x04, 0x82, 0x01, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x08, 0x00, 0x01,
    0x12, 0x03, 0x7c, 0x0a, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x00, 0x12, 0x03, 0x7d,
    0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x05, 0x12, 0x03, 0x7d, 0x08, 0x0e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x01, 0x12, 0x03, 0x7d, 0x0f, 0x1b, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x03, 0x12, 0x03, 0x7d, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x0e, 0x02, 0x01, 0x12, 0x03, 0x7e, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x7e, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x7e, 0x0d, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x7e, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x02, 0x12, 0x03, 0x7f, 0x08, 0x20,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x05, 0x12, 0x03, 0x7f, 0x08, 0x0d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x01, 0x12, 0x03, 0x7f, 0x0e, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x02, 0x03, 0x12, 0x03, 0x7f, 0x1e, 0x1f, 0x0a, 0x1c, 0x0a, 0x04, 0x04, 0x0e,
    0x02, 0x03, 0x12, 0x04, 0x80, 0x01, 0x08, 0x1f, 0x22, 0x0e, 0x20, 0x4d, 0x69, 0x73, 0x73, 0x69,
    0x6e, 0x67, 0x20, 0x45, 0x4e, 0x55, 0x4d, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x03,
    0x05, 0x12, 0x04, 0x80, 0x01, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x03, 0x01,
    0x12, 0x04, 0x80, 0x01, 0x0f, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x03, 0x03, 0x12,
    0x04, 0x80, 0x01, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0f, 0x12, 0x06, 0x85, 0x01, 0x00,
    0x88, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0f, 0x01, 0x12, 0x04, 0x85, 0x01, 0x08, 0x16,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x00, 0x12, 0x04, 0x86, 0x01, 0x04, 0x14, 0x0a, 0x0f,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x04, 0x12, 0x06, 0x86, 0x01, 0x04, 0x85, 0x01, 0x18, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x05, 0x12, 0x04, 0x86, 0x01, 0x04, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x01, 0x12, 0x04, 0x86, 0x01, 0x0b, 0x0f, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x00, 0x03, 0x12, 0x04, 0x86, 0x01, 0x12, 0x13, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x0f, 0x02, 0x01, 0x12, 0x04, 0x87, 0x01, 0x04, 0x12, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x0f,
    0x02, 0x01, 0x04, 0x12, 0x06, 0x87, 0x01, 0x04, 0x86, 0x01, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x01, 0x05, 0x12, 0x04, 0x87, 0x01, 0x04, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f,
    0x02, 0x01, 0x01, 0x12, 0x04, 0x87, 0x01, 0x0b, 0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x01, 0x03, 0x12, 0x04, 0x87, 0x01, 0x10, 0x11, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x10, 0x12, 0x06,
    0x8a, 0x01, 0x00, 0x8d, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x10, 0x01, 0x12, 0x04, 0x8a,
    0x01, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x00, 0x12, 0x04, 0x8b, 0x01, 0x04,
    0x14, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x04, 0x12, 0x06, 0x8b, 0x01, 0x04, 0x8a,
    0x01, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x05, 0x12, 0x04, 0x8b, 0x01, 0x04,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x01, 0x12, 0x04, 0x8b, 0x01, 0x0b, 0x0f,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x03, 0x12, 0x04, 0x8b, 0x01, 0x12, 0x13, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x01, 0x12, 0x04, 0x8c, 0x01, 0x04, 0x12, 0x0a, 0x0f, 0x0a,
    0x05, 0x04, 0x10, 0x02, 0x01, 0x04, 0x12, 0x06, 0x8c, 0x01, 0x04, 0x8b, 0x01, 0x14, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x05, 0x12, 0x04, 0x8c, 0x01, 0x04, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x10, 0x02, 0x01, 0x01, 0x12, 0x04, 0x8c, 0x01, 0x0b, 0x0d, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x10, 0x02, 0x01, 0x03, 0x12, 0x04, 0x8c, 0x01, 0x10, 0x11, 0x0a, 0x0c, 0x0a, 0x02, 0x04,
    0x11, 0x12, 0x06, 0x8f, 0x01, 0x00, 0x95, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x11, 0x01,
    0x12, 0x04, 0x8f, 0x01, 0x08, 0x16, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x11, 0x08, 0x00, 0x12, 0x06,
    0x90, 0x01, 0x04, 0x92, 0x01, 0x05, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x08, 0x00, 0x01, 0x12,
    0x04, 0x90, 0x01, 0x0a, 0x0d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x00, 0x12, 0x04, 0x91,
    0x01, 0x08, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x06, 0x12, 0x04, 0x91, 0x01,
    0x08, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x01, 0x12, 0x04, 0x91, 0x01, 0x15,
    0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x03, 0x12, 0x04, 0x91, 0x01, 0x1e, 0x1f,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x01, 0x12, 0x04, 0x93, 0x01, 0x04, 0x19, 0x0a, 0x0f,
    0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x04, 0x12, 0x06, 0x93, 0x01, 0x04, 0x92, 0x01, 0x05, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x05, 0x12, 0x04, 0x93, 0x01, 0x04, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x01, 0x12, 0x04, 0x93, 0x01, 0x0b, 0x14, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x11, 0x02, 0x01, 0x03, 0x12, 0x04, 0x93, 0x01, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x11, 0x02, 0x02, 0x12, 0x04, 0x94, 0x01, 0x04, 0x19, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x02, 0x04, 0x12, 0x06, 0x94, 0x01, 0x04, 0x93, 0x01, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x11, 0x02, 0x02, 0x05, 0x12, 0x04, 0x94, 0x01, 0x04, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x02, 0x01, 0x12, 0x04, 0x94, 0x01, 0x0b, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02,
    0x02, 0x03, 0x12, 0x04, 0x94, 0x01, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x12, 0x12, 0x06,
    0x97, 0x01, 0x00, 0x9d, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x12, 0x01, 0x12, 0x04, 0x97,
    0x01, 0x08, 0x14, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x12, 0x08, 0x00, 0x12, 0x06, 0x98, 0x01, 0x04,
    0x9b, 0x01, 0x05, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x08, 0x00, 0x01, 0x12, 0x04, 0x98, 0x01,
    0x0a, 0x0d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x00, 0x12, 0x04, 0x99, 0x01, 0x08, 0x1b,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x05, 0x12, 0x04, 0x99, 0x01, 0x08, 0x0e, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x01, 0x12, 0x04, 0x99, 0x01, 0x0f, 0x16, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x03, 0x12, 0x04, 0x99, 0x01, 0x19, 0x1a, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x12, 0x02, 0x01, 0x12, 0x04, 0x9a, 0x01, 0x08, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x12, 0x02, 0x01, 0x05, 0x12, 0x04, 0x9a, 0x01, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12,
    0x02, 0x01, 0x01, 0x12, 0x04, 0x9a, 0x01, 0x0f, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02,
    0x01, 0x03, 0x12, 0x04, 0x9a, 0x01, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x02,
    0x12, 0x04, 0x9c, 0x01, 0x04, 0x1a, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x02, 0x04, 0x12,
    0x06, 0x9c, 0x01, 0x04, 0x9b, 0x01, 0x05, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x02, 0x05,
    0x12, 0x04, 0x9c, 0x01, 0x04, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x02, 0x01, 0x12,
    0x04, 0x9c, 0x01, 0x0b, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x02, 0x03, 0x12, 0x04,
    0x9c, 0x01, 0x18, 0x19, 0x0a, 0x3e, 0x0a, 0x02, 0x04, 0x13, 0x12, 0x06, 0xa0, 0x01, 0x00, 0xb4,
    0x01, 0x01, 0x1a, 0x30, 0x21, 0x20, 0x4c, 0x69, 0x62, 0x44, 0x69, 0x73, 0x69, 0x72, 0x20, 0x52,
    0x65, 0x75, 0x65, 0x73, 0x74, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x73, 0x65,
    0x6e, 0x74, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x13, 0x01, 0x12, 0x04, 0xa0, 0x01, 0x08,
    0x17, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x13, 0x08, 0x00, 0x12, 0x06, 0xa1, 0x01, 0x04, 0xb3, 0x01,
    0x05, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x08, 0x00, 0x01, 0x12, 0x04, 0xa1, 0x01, 0x0a, 0x0d,
    0x0a, 0x42, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x00, 0x12, 0x04, 0xa4, 0x01, 0x08, 0x1e, 0x1a, 0x34,
    0x21, 0x20, 0x51, 0x75, 0x65, 0x72, 0x79, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x61, 0x76, 0x61, 0x69,
    0x6c, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x73, 0x2e, 0x0a, 0x21, 0x20,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x3a, 0x20, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x5f,
    0x69, 0x64, 0x73, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x05, 0x12, 0x04, 0xa4,
    0x01, 0x08, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa4, 0x01,
    0x0d, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa4, 0x01, 0x1c,
    0x1d, 0x0a, 0x7d, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x01, 0x12, 0x04, 0xa8, 0x01, 0x08, 0x26, 0x1a,
    0x6f, 0x21, 0x20, 0x51, 0x75, 0x65, 0x72, 0x79, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x61, 0x76, 0x61,
    0x69, 0x6c, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x6d, 0x6f, 0x6c, 0x64, 0x73, 0x20, 0x66, 0x6f, 0x72,
    0x20, 0x61, 0x20, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x5f, 0x69, 0x64, 0x0a, 0x21, 0x20, 0x5c, 0x70,
    0x61, 0x72, 0x61, 0x6d, 0x20, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x20, 0x66, 0x72, 0x6f, 0x6d,
    0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x5f,
    0x69, 0x64, 0x73, 0x2e, 0x0a, 0x21, 0x20, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x3a,
    0x20, 0x6d, 0x6f, 0x6c, 0x64, 0x5f, 0x65, 0x6e, 0x74, 0x72, 0x79, 0x5f, 0x69, 0x64, 0x73, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x05, 0x12, 0x04, 0xa8, 0x01, 0x08, 0x0e, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x01, 0x12, 0x04, 0xa8, 0x01, 0x0f, 0x21, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x03, 0x12, 0x04, 0xa8, 0x01, 0x24, 0x25, 0x0a, 0x1f, 0x0a,
    0x04, 0x04, 0x13, 0x02, 0x02, 0x12, 0x04, 0xaa, 0x01, 0x08, 0x2c, 0x1a, 0x11, 0x21, 0x20, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x3a, 0x20, 0x6d, 0x6f, 0x6c, 0x64, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x06, 0x12, 0x04, 0xaa, 0x01, 0x08, 0x16, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x13, 0x02, 0x02, 0x01, 0x12, 0x04, 0xaa, 0x01, 0x17, 0x27, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x13, 0x02, 0x02, 0x03, 0x12, 0x04, 0xaa, 0x01, 0x2a, 0x2b, 0x0a, 0x1f, 0x0a, 0x04, 0x04,
    0x13, 0x02, 0x03, 0x12, 0x04, 0xac, 0x01, 0x08, 0x24, 0x1a, 0x11, 0x21, 0x20, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x3a, 0x20, 0x6e, 0x6f, 0x6e, 0x65, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x13, 0x02, 0x03, 0x05, 0x12, 0x04, 0xac, 0x01, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x13, 0x02, 0x03, 0x01, 0x12, 0x04, 0xac, 0x01, 0x0f, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13,
    0x02, 0x03, 0x03, 0x12, 0x04, 0xac, 0x01, 0x22, 0x23, 0x0a, 0x22, 0x0a, 0x04, 0x04, 0x13, 0x02,
    0x04, 0x12, 0x04, 0xae, 0x01, 0x08, 0x27, 0x1a, 0x14, 0x21, 0x20, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x3a, 0x20, 0x65, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x13, 0x02, 0x04, 0x06, 0x12, 0x04, 0xae, 0x01, 0x08, 0x14, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x13, 0x02, 0x04, 0x01, 0x12, 0x04, 0xae, 0x01, 0x15, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x13, 0x02, 0x04, 0x03, 0x12, 0x04, 0xae, 0x01, 0x25, 0x26, 0x0a, 0x21, 0x0a, 0x04, 0x04, 0x13,
    0x02, 0x05, 0x12, 0x04, 0xb0, 0x01, 0x08, 0x30, 0x1a, 0x13, 0x21, 0x20, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x3a, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x13, 0x02, 0x05, 0x06, 0x12, 0x04, 0xb0, 0x01, 0x08, 0x18, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x13, 0x02, 0x05, 0x01, 0x12, 0x04, 0xb0, 0x01, 0x19, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x13, 0x02, 0x05, 0x03, 0x12, 0x04, 0xb0, 0x01, 0x2e, 0x2f, 0x0a, 0x22, 0x0a, 0x04, 0x04, 0x13,
    0x02, 0x06, 0x12, 0x04, 0xb2, 0x01, 0x08, 0x2c, 0x1a, 0x14, 0x21, 0x20, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x3a, 0x20, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x13, 0x02, 0x06, 0x06, 0x12, 0x04, 0xb2, 0x01, 0x08, 0x16, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x13, 0x02, 0x06, 0x01, 0x12, 0x04, 0xb2, 0x01, 0x17, 0x27, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x13, 0x02, 0x06, 0x03, 0x12, 0x04, 0xb2, 0x01, 0x2a, 0x2b, 0x0a, 0x49, 0x0a, 0x02, 0x04,
    0x14, 0x12, 0x06, 0xb7, 0x01, 0x00, 0xc6, 0x01, 0x01, 0x1a, 0x3b, 0x20, 0x4c, 0x69, 0x62, 0x44,
    0x69, 0x73, 0x69, 0x72, 0x20, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x6d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x73, 0x65, 0x6e, 0x74, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x20, 0x28, 0x73, 0x65, 0x72, 0x76,
    0x69, 0x63, 0x65, 0x29, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x14, 0x01, 0x12, 0x04, 0xb7,
    0x01, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x00, 0x12, 0x04, 0xb8, 0x01, 0x04,
    0x16, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x04, 0x12, 0x06, 0xb8, 0x01, 0x04, 0xb7,
    0x01, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x06, 0x12, 0x04, 0xb8, 0x01, 0x04,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb8, 0x01, 0x0b, 0x11,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb8, 0x01, 0x14, 0x15, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x01, 0x12, 0x04, 0xb9, 0x01, 0x04, 0x1c, 0x0a, 0x0f, 0x0a,
    0x05, 0x04, 0x14, 0x02, 0x01, 0x04, 0x12, 0x06, 0xb9, 0x01, 0x04, 0xb8, 0x01, 0x16, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x05, 0x12, 0x04, 0xb9, 0x01, 0x04, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x14, 0x02, 0x01, 0x01, 0x12, 0x04, 0xb9, 0x01, 0x0b, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x14, 0x02, 0x01, 0x03, 0x12, 0x04, 0xb9, 0x01, 0x1a, 0x1b, 0x0a, 0x65, 0x0a, 0x04, 0x04,
    0x14, 0x02, 0x02, 0x12, 0x04, 0xbd, 0x01, 0x04, 0x23, 0x1a, 0x57, 0x21, 0x20, 0x52, 0x65, 0x74,
    0x75, 0x72, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x20, 0x6f, 0x66, 0x20,
    0x61, 0x6c, 0x6c, 0x20, 0x61, 0x76, 0x61, 0x69, 0x6c, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x67, 0x72,
    0x6f, 0x75, 0x70, 0x20, 0x69, 0x64, 0x73, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20,
    0x69, 0x6e, 0x73, 0x74, 0x61, 0x6e, 0x63, 0x65, 0x2e, 0x0a, 0x21, 0x20, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x3a, 0x20, 0x71, 0x75, 0x65, 0x72, 0x79, 0x5f, 0x67, 0x72, 0x6f, 0x75, 0x70,
    0x73, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02, 0x04, 0x12, 0x04, 0xbd, 0x01, 0x04,
    0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02, 0x05, 0x12, 0x04, 0xbd, 0x01, 0x0d, 0x13,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02, 0x01, 0x12, 0x04, 0xbd, 0x01, 0x14, 0x1d, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02, 0x03, 0x12, 0x04, 0xbd, 0x01, 0x20, 0x22, 0x0a, 0x2c,
    0x0a, 0x04, 0x04, 0x14, 0x02, 0x03, 0x12, 0x04, 0xbf, 0x01, 0x04, 0x28, 0x1a, 0x1e, 0x21, 0x20,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x3a, 0x20, 0x71, 0x75, 0x65, 0x72, 0x79, 0x5f, 0x6d,
    0x6f, 0x6c, 0x64, 0x5f, 0x65, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x14, 0x02, 0x03, 0x04, 0x12, 0x04, 0xbf, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x14, 0x02, 0x03, 0x05, 0x12, 0x04, 0xbf, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14,
    0x02, 0x03, 0x01, 0x12, 0x04, 0xbf, 0x01, 0x14, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02,
    0x03, 0x03, 0x12, 0x04, 0xbf, 0x01, 0x25, 0x27, 0x0a, 0x2a, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x04,
    0x12, 0x04, 0xc1, 0x01, 0x04, 0x18, 0x1a, 0x1c, 0x21, 0x20, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x3a, 0x20, 0x71, 0x75, 0x65, 0x72, 0x79, 0x5f, 0x6d, 0x6f, 0x6c, 0x64, 0x5f, 0x65, 0x6e,
    0x74, 0x72, 0x79, 0x0a, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x04, 0x04, 0x12, 0x06, 0xc1,
    0x01, 0x04, 0xbf, 0x01, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x04, 0x06, 0x12, 0x04,
    0xc1, 0x01, 0x04, 0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x04, 0x01, 0x12, 0x04, 0xc1,
    0x01, 0x0e, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x04, 0x03, 0x12, 0x04, 0xc1, 0x01,
    0x15, 0x17, 0x0a, 0x27, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x05, 0x12, 0x04, 0xc3, 0x01, 0x04, 0x19,
    0x1a, 0x19, 0x21, 0x20, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x3a, 0x20, 0x71, 0x75, 0x65,
    0x72, 0x79, 0x5f, 0x65, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x0a, 0x0a, 0x0f, 0x0a, 0x05, 0x04,
    0x14, 0x02, 0x05, 0x04, 0x12, 0x06, 0xc3, 0x01, 0x04, 0xc1, 0x01, 0x18, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x14, 0x02, 0x05, 0x06, 0x12, 0x04, 0xc3, 0x01, 0x04, 0x0b, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x14, 0x02, 0x05, 0x01, 0x12, 0x04, 0xc3, 0x01, 0x0c, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14,
    0x02, 0x05, 0x03, 0x12, 0x04, 0xc3, 0x01, 0x16, 0x18, 0x0a, 0x2b, 0x0a, 0x04, 0x04, 0x14, 0x02,
    0x06, 0x12, 0x04, 0xc5, 0x01, 0x04, 0x1c, 0x1a, 0x1d, 0x21, 0x20, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x20, 0x71, 0x75, 0x65, 0x72, 0x79, 0x5f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x5f,
    0x65, 0x6e, 0x74, 0x72, 0x79, 0x0a, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x06, 0x04, 0x12,
    0x06, 0xc5, 0x01, 0x04, 0xc3, 0x01, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x06, 0x06,
    0x12, 0x04, 0xc5, 0x01, 0x04, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x06, 0x01, 0x12,
    0x04, 0xc5, 0x01, 0x10, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x06, 0x03, 0x12, 0x04,
    0xc5, 0x01, 0x19, 0x1b, 0x0a, 0x55, 0x0a, 0x02, 0x04, 0x15, 0x12, 0x06, 0xc9, 0x01, 0x00, 0xd1,
    0x01, 0x01, 0x1a, 0x47, 0x20, 0x41, 0x6c, 0x6c, 0x20, 0x70, 0x6f, 0x73, 0x73, 0x69, 0x62, 0x6c,
    0x65, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x20, 0x65, 0x78, 0x63, 0x6c, 0x75,
    0x73, 0x69, 0x76, 0x65, 0x6c, 0x79, 0x20, 0x73, 0x65, 0x6e, 0x74, 0x20, 0x66, 0x72, 0x6f, 0x6d,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x20, 0x74, 0x6f, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x15, 0x01, 0x12, 0x04, 0xc9, 0x01, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x00,
    0x12, 0x04, 0xca, 0x01, 0x04, 0x22, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x04, 0x12,
    0x06, 0xca, 0x01, 0x04, 0xc9, 0x01, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x05,
    0x12, 0x04, 0xca, 0x01, 0x04, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xca, 0x01, 0x0b, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xca, 0x01, 0x20, 0x21, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x15, 0x08, 0x00, 0x12, 0x06, 0xcb, 0x01,
    0x04, 0xd0, 0x01, 0x05, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x08, 0x00, 0x01, 0x12, 0x04, 0xcb,
    0x01, 0x0a, 0x0d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x01, 0x12, 0x04, 0xcc, 0x01, 0x08,
    0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x05, 0x12, 0x04, 0xcc, 0x01, 0x08, 0x0c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x01, 0x12, 0x04, 0xcc, 0x01, 0x0d, 0x11, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x03, 0x12, 0x04, 0xcc, 0x01, 0x14, 0x15, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x15, 0x02, 0x02, 0x12, 0x04, 0xcd, 0x01, 0x08, 0x16, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x15, 0x02, 0x02, 0x05, 0x12, 0x04, 0xcd, 0x01, 0x08, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x15, 0x02, 0x02, 0x01, 0x12, 0x04, 0xcd, 0x01, 0x0d, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15,
    0x02, 0x02, 0x03, 0x12, 0x04, 0xcd, 0x01, 0x14, 0x15, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02,
    0x03, 0x12, 0x04, 0xce, 0x01, 0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x03, 0x06,
    0x12, 0x04, 0xce, 0x01, 0x08, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x03, 0x01, 0x12,
    0x04, 0xce, 0x01, 0x13, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x03, 0x03, 0x12, 0x04,
    0xce, 0x01, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x04, 0x12, 0x04, 0xcf, 0x01,
    0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x04, 0x06, 0x12, 0x04, 0xcf, 0x01, 0x08,
    0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x04, 0x01, 0x12, 0x04, 0xcf, 0x01, 0x19, 0x21,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x04, 0x03, 0x12, 0x04, 0xcf, 0x01, 0x24, 0x25, 0x0a,
    0x55, 0x0a, 0x02, 0x04, 0x16, 0x12, 0x06, 0xd4, 0x01, 0x00, 0xdc, 0x01, 0x01, 0x1a, 0x47, 0x20,
    0x41, 0x6c, 0x6c, 0x20, 0x70, 0x6f, 0x73, 0x73, 0x69, 0x62, 0x6c, 0x65, 0x20, 0x6d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x73, 0x20, 0x65, 0x78, 0x63, 0x6c, 0x75, 0x73, 0x69, 0x76, 0x65, 0x6c,
    0x79, 0x20, 0x73, 0x65, 0x6e, 0x74, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x65,
    0x72, 0x76, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x16, 0x01, 0x12, 0x04, 0xd4,
    0x01, 0x08, 0x12, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x00, 0x12, 0x04, 0xd5, 0x01, 0x04,
    0x22, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x04, 0x12, 0x06, 0xd5, 0x01, 0x04, 0xd4,
    0x01, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x05, 0x12, 0x04, 0xd5, 0x01, 0x04,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x01, 0x12, 0x04, 0xd5, 0x01, 0x0b, 0x1d,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x03, 0x12, 0x04, 0xd5, 0x01, 0x20, 0x21, 0x0a,
    0x0e, 0x0a, 0x04, 0x04, 0x16, 0x08, 0x00, 0x12, 0x06, 0xd6, 0x01, 0x04, 0xdb, 0x01, 0x05, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x16, 0x08, 0x00, 0x01, 0x12, 0x04, 0xd6, 0x01, 0x0a, 0x0d, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x16, 0x02, 0x01, 0x12, 0x04, 0xd7, 0x01, 0x08, 0x16, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x16, 0x02, 0x01, 0x05, 0x12, 0x04, 0xd7, 0x01, 0x08, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x16, 0x02, 0x01, 0x01, 0x12, 0x04, 0xd7, 0x01, 0x0d, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16,
    0x02, 0x01, 0x03, 0x12, 0x04, 0xd7, 0x01, 0x14, 0x15, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02,
    0x02, 0x12, 0x04, 0xd8, 0x01, 0x08, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x02, 0x05,
    0x12, 0x04, 0xd8, 0x01, 0x08, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x02, 0x01, 0x12,
    0x04, 0xd8, 0x01, 0x0d, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x02, 0x03, 0x12, 0x04,
    0xd8, 0x01, 0x14, 0x15, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x03, 0x12, 0x04, 0xd9, 0x01,
    0x08, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x03, 0x06, 0x12, 0x04, 0xd9, 0x01, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x03, 0x01, 0x12, 0x04, 0xd9, 0x01, 0x11, 0x1a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x03, 0x03, 0x12, 0x04, 0xd9, 0x01, 0x1d, 0x1e, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x04, 0x12, 0x04, 0xda, 0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x16, 0x02, 0x04, 0x06, 0x12, 0x04, 0xda, 0x01, 0x08, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x16, 0x02, 0x04, 0x01, 0x12, 0x04, 0xda, 0x01, 0x18, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x16, 0x02, 0x04, 0x03, 0x12, 0x04, 0xda, 0x01, 0x22, 0x23, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x33,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
