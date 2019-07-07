// This file is generated by rust-protobuf 2.7.0. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

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
//! Generated file from `ab.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_7_0;

#[derive(PartialEq,Clone,Default)]
pub struct BroadcastResponse {
    // message fields
    pub status: Status,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a BroadcastResponse {
    fn default() -> &'a BroadcastResponse {
        <BroadcastResponse as ::protobuf::Message>::default_instance()
    }
}

impl BroadcastResponse {
    pub fn new() -> BroadcastResponse {
        ::std::default::Default::default()
    }

    // .proto.Status status = 1;


    pub fn get_status(&self) -> Status {
        self.status
    }
    pub fn clear_status(&mut self) {
        self.status = Status::UNKNOWN;
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: Status) {
        self.status = v;
    }
}

impl ::protobuf::Message for BroadcastResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.status, 1, &mut self.unknown_fields)?
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
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.status != Status::UNKNOWN {
            os.write_enum(1, self.status.value())?;
        }
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
        Self::descriptor_static()
    }

    fn new() -> BroadcastResponse {
        BroadcastResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Status>>(
                    "status",
                    |m: &BroadcastResponse| { &m.status },
                    |m: &mut BroadcastResponse| { &mut m.status },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BroadcastResponse>(
                    "BroadcastResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static BroadcastResponse {
        static mut instance: ::protobuf::lazy::Lazy<BroadcastResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BroadcastResponse,
        };
        unsafe {
            instance.get(BroadcastResponse::new)
        }
    }
}

impl ::protobuf::Clear for BroadcastResponse {
    fn clear(&mut self) {
        self.status = Status::UNKNOWN;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BroadcastResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BroadcastResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Status {
    UNKNOWN = 0,
    SUCCESS = 200,
    BAD_REQUEST = 400,
    NOT_FOUND = 404,
    SERVICE_UNAVAILABLE = 503,
}

impl ::protobuf::ProtobufEnum for Status {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Status> {
        match value {
            0 => ::std::option::Option::Some(Status::UNKNOWN),
            200 => ::std::option::Option::Some(Status::SUCCESS),
            400 => ::std::option::Option::Some(Status::BAD_REQUEST),
            404 => ::std::option::Option::Some(Status::NOT_FOUND),
            503 => ::std::option::Option::Some(Status::SERVICE_UNAVAILABLE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Status] = &[
            Status::UNKNOWN,
            Status::SUCCESS,
            Status::BAD_REQUEST,
            Status::NOT_FOUND,
            Status::SERVICE_UNAVAILABLE,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
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

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x08ab.proto\x12\x05proto\x1a\x0cmirbft.proto\":\n\x11BroadcastRespons\
    e\x12%\n\x06status\x18\x01\x20\x01(\x0e2\r.proto.StatusR\x06status*_\n\
    \x06Status\x12\x0b\n\x07UNKNOWN\x10\0\x12\x0c\n\x07SUCCESS\x10\xc8\x01\
    \x12\x10\n\x0bBAD_REQUEST\x10\x90\x03\x12\x0e\n\tNOT_FOUND\x10\x94\x03\
    \x12\x18\n\x13SERVICE_UNAVAILABLE\x10\xf7\x032J\n\x0fAtomicBroadcast\x12\
    7\n\tBroadcast\x12\x0e.proto.Message\x1a\x18.proto.BroadcastResponse\"\0\
    b\x06proto3\
";

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
