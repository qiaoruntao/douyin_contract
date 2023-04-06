// This file is generated by rust-protobuf 3.2.0. Do not edit
// .proto file is parsed by protoc --rust-out=...
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
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `pushproto/PushFrame.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

/// 更新时间 : 2023-01-09
#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:PushFrame)
pub struct PushFrame {
    // message fields
    // @@protoc_insertion_point(field:PushFrame.seqid)
    pub seqid: u64,
    // @@protoc_insertion_point(field:PushFrame.logid)
    pub logid: u64,
    // @@protoc_insertion_point(field:PushFrame.service)
    pub service: u64,
    // @@protoc_insertion_point(field:PushFrame.method)
    pub method: u64,
    // @@protoc_insertion_point(field:PushFrame.headersList)
    pub headersList: ::std::vec::Vec<super::PushHeader::PushHeader>,
    // @@protoc_insertion_point(field:PushFrame.payloadEncoding)
    pub payloadEncoding: ::std::string::String,
    // @@protoc_insertion_point(field:PushFrame.payloadType)
    pub payloadType: ::std::string::String,
    // @@protoc_insertion_point(field:PushFrame.payload)
    pub payload: ::std::vec::Vec<u8>,
    // special fields
    // @@protoc_insertion_point(special_field:PushFrame.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PushFrame {
    fn default() -> &'a PushFrame {
        <PushFrame as ::protobuf::Message>::default_instance()
    }
}

impl PushFrame {
    pub fn new() -> PushFrame {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "seqid",
            |m: &PushFrame| { &m.seqid },
            |m: &mut PushFrame| { &mut m.seqid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "logid",
            |m: &PushFrame| { &m.logid },
            |m: &mut PushFrame| { &mut m.logid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "service",
            |m: &PushFrame| { &m.service },
            |m: &mut PushFrame| { &mut m.service },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "method",
            |m: &PushFrame| { &m.method },
            |m: &mut PushFrame| { &mut m.method },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "headersList",
            |m: &PushFrame| { &m.headersList },
            |m: &mut PushFrame| { &mut m.headersList },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "payloadEncoding",
            |m: &PushFrame| { &m.payloadEncoding },
            |m: &mut PushFrame| { &mut m.payloadEncoding },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "payloadType",
            |m: &PushFrame| { &m.payloadType },
            |m: &mut PushFrame| { &mut m.payloadType },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "payload",
            |m: &PushFrame| { &m.payload },
            |m: &mut PushFrame| { &mut m.payload },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PushFrame>(
            "PushFrame",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PushFrame {
    const NAME: &'static str = "PushFrame";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.seqid = is.read_uint64()?;
                },
                16 => {
                    self.logid = is.read_uint64()?;
                },
                24 => {
                    self.service = is.read_uint64()?;
                },
                32 => {
                    self.method = is.read_uint64()?;
                },
                42 => {
                    self.headersList.push(is.read_message()?);
                },
                50 => {
                    self.payloadEncoding = is.read_string()?;
                },
                58 => {
                    self.payloadType = is.read_string()?;
                },
                66 => {
                    self.payload = is.read_bytes()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.seqid != 0 {
            my_size += ::protobuf::rt::uint64_size(1, self.seqid);
        }
        if self.logid != 0 {
            my_size += ::protobuf::rt::uint64_size(2, self.logid);
        }
        if self.service != 0 {
            my_size += ::protobuf::rt::uint64_size(3, self.service);
        }
        if self.method != 0 {
            my_size += ::protobuf::rt::uint64_size(4, self.method);
        }
        for value in &self.headersList {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if !self.payloadEncoding.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.payloadEncoding);
        }
        if !self.payloadType.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.payloadType);
        }
        if !self.payload.is_empty() {
            my_size += ::protobuf::rt::bytes_size(8, &self.payload);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.seqid != 0 {
            os.write_uint64(1, self.seqid)?;
        }
        if self.logid != 0 {
            os.write_uint64(2, self.logid)?;
        }
        if self.service != 0 {
            os.write_uint64(3, self.service)?;
        }
        if self.method != 0 {
            os.write_uint64(4, self.method)?;
        }
        for v in &self.headersList {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        };
        if !self.payloadEncoding.is_empty() {
            os.write_string(6, &self.payloadEncoding)?;
        }
        if !self.payloadType.is_empty() {
            os.write_string(7, &self.payloadType)?;
        }
        if !self.payload.is_empty() {
            os.write_bytes(8, &self.payload)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> PushFrame {
        PushFrame::new()
    }

    fn clear(&mut self) {
        self.seqid = 0;
        self.logid = 0;
        self.service = 0;
        self.method = 0;
        self.headersList.clear();
        self.payloadEncoding.clear();
        self.payloadType.clear();
        self.payload.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PushFrame {
        static instance: PushFrame = PushFrame {
            seqid: 0,
            logid: 0,
            service: 0,
            method: 0,
            headersList: ::std::vec::Vec::new(),
            payloadEncoding: ::std::string::String::new(),
            payloadType: ::std::string::String::new(),
            payload: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PushFrame {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PushFrame").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PushFrame {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PushFrame {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19pushproto/PushFrame.proto\x1a\x1apushproto/PushHeader.proto\"\xfe\
    \x01\n\tPushFrame\x12\x14\n\x05seqid\x18\x01\x20\x01(\x04R\x05seqid\x12\
    \x14\n\x05logid\x18\x02\x20\x01(\x04R\x05logid\x12\x18\n\x07service\x18\
    \x03\x20\x01(\x04R\x07service\x12\x16\n\x06method\x18\x04\x20\x01(\x04R\
    \x06method\x12-\n\x0bheadersList\x18\x05\x20\x03(\x0b2\x0b.PushHeaderR\
    \x0bheadersList\x12(\n\x0fpayloadEncoding\x18\x06\x20\x01(\tR\x0fpayload\
    Encoding\x12\x20\n\x0bpayloadType\x18\x07\x20\x01(\tR\x0bpayloadType\x12\
    \x18\n\x07payload\x18\x08\x20\x01(\x0cR\x07payloadB<\n8cool.scx.live_roo\
    m_watcher.douyin.proto_entity.pushprotoP\x01J\xc2\x04\n\x06\x12\x04\0\0\
    \x11\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x08\x12\x03\x02\0\"\
    \n\t\n\x02\x08\n\x12\x03\x02\0\"\n\x08\n\x01\x08\x12\x03\x03\0Q\n\t\n\
    \x02\x08\x01\x12\x03\x03\0Q\n\t\n\x02\x03\0\x12\x03\x05\0$\n'\n\x02\x04\
    \0\x12\x04\x08\0\x11\x01\x1a\x1b\xe6\x9b\xb4\xe6\x96\xb0\xe6\x97\xb6\xe9\
    \x97\xb4\x20:\x202023-01-09\r\n\n\n\n\x03\x04\0\x01\x12\x03\x08\x08\x11\
    \n\x0b\n\x04\x04\0\x02\0\x12\x03\t\x02\x13\n\x0c\n\x05\x04\0\x02\0\x05\
    \x12\x03\t\x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\t\t\x0e\n\x0c\n\
    \x05\x04\0\x02\0\x03\x12\x03\t\x11\x12\n\x0b\n\x04\x04\0\x02\x01\x12\x03\
    \n\x02\x13\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\n\x02\x08\n\x0c\n\x05\
    \x04\0\x02\x01\x01\x12\x03\n\t\x0e\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\
    \n\x11\x12\n\x0b\n\x04\x04\0\x02\x02\x12\x03\x0b\x02\x15\n\x0c\n\x05\x04\
    \0\x02\x02\x05\x12\x03\x0b\x02\x08\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\
    \x0b\t\x10\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x0b\x13\x14\n\x0b\n\x04\
    \x04\0\x02\x03\x12\x03\x0c\x02\x14\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03\
    \x0c\x02\x08\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03\x0c\t\x0f\n\x0c\n\x05\
    \x04\0\x02\x03\x03\x12\x03\x0c\x12\x13\n\x0b\n\x04\x04\0\x02\x04\x12\x03\
    \r\x02&\n\x0c\n\x05\x04\0\x02\x04\x04\x12\x03\r\x02\n\n\x0c\n\x05\x04\0\
    \x02\x04\x06\x12\x03\r\x0b\x15\n\x0c\n\x05\x04\0\x02\x04\x01\x12\x03\r\
    \x16!\n\x0c\n\x05\x04\0\x02\x04\x03\x12\x03\r$%\n\x0b\n\x04\x04\0\x02\
    \x05\x12\x03\x0e\x02\x1d\n\x0c\n\x05\x04\0\x02\x05\x05\x12\x03\x0e\x02\
    \x08\n\x0c\n\x05\x04\0\x02\x05\x01\x12\x03\x0e\t\x18\n\x0c\n\x05\x04\0\
    \x02\x05\x03\x12\x03\x0e\x1b\x1c\n\x0b\n\x04\x04\0\x02\x06\x12\x03\x0f\
    \x02\x19\n\x0c\n\x05\x04\0\x02\x06\x05\x12\x03\x0f\x02\x08\n\x0c\n\x05\
    \x04\0\x02\x06\x01\x12\x03\x0f\t\x14\n\x0c\n\x05\x04\0\x02\x06\x03\x12\
    \x03\x0f\x17\x18\n\x0b\n\x04\x04\0\x02\x07\x12\x03\x10\x02\x14\n\x0c\n\
    \x05\x04\0\x02\x07\x05\x12\x03\x10\x02\x07\n\x0c\n\x05\x04\0\x02\x07\x01\
    \x12\x03\x10\x08\x0f\n\x0c\n\x05\x04\0\x02\x07\x03\x12\x03\x10\x12\x13b\
    \x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::PushHeader::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(PushFrame::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
