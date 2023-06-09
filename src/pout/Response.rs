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

//! Generated file from `webcast/im/Response.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

/// 更新时间 : 2023-01-09
#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:Response)
pub struct Response {
    // message fields
    // @@protoc_insertion_point(field:Response.messagesList)
    pub messagesList: ::std::vec::Vec<super::Message::Message>,
    // @@protoc_insertion_point(field:Response.cursor)
    pub cursor: ::std::string::String,
    // @@protoc_insertion_point(field:Response.fetchInterval)
    pub fetchInterval: i64,
    // @@protoc_insertion_point(field:Response.now)
    pub now: i64,
    // @@protoc_insertion_point(field:Response.internalExt)
    pub internalExt: ::std::string::String,
    // @@protoc_insertion_point(field:Response.fetchType)
    pub fetchType: i32,
    // @@protoc_insertion_point(field:Response.routeParamsMap)
    pub routeParamsMap: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    // @@protoc_insertion_point(field:Response.heartbeatDuration)
    pub heartbeatDuration: i64,
    // @@protoc_insertion_point(field:Response.needAck)
    pub needAck: bool,
    // @@protoc_insertion_point(field:Response.pushServer)
    pub pushServer: ::std::string::String,
    // @@protoc_insertion_point(field:Response.liveCursor)
    pub liveCursor: ::std::string::String,
    // @@protoc_insertion_point(field:Response.historyNoMore)
    pub historyNoMore: bool,
    // special fields
    // @@protoc_insertion_point(special_field:Response.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Response {
    fn default() -> &'a Response {
        <Response as ::protobuf::Message>::default_instance()
    }
}

impl Response {
    pub fn new() -> Response {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(12);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "messagesList",
            |m: &Response| { &m.messagesList },
            |m: &mut Response| { &mut m.messagesList },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "cursor",
            |m: &Response| { &m.cursor },
            |m: &mut Response| { &mut m.cursor },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "fetchInterval",
            |m: &Response| { &m.fetchInterval },
            |m: &mut Response| { &mut m.fetchInterval },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "now",
            |m: &Response| { &m.now },
            |m: &mut Response| { &mut m.now },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "internalExt",
            |m: &Response| { &m.internalExt },
            |m: &mut Response| { &mut m.internalExt },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "fetchType",
            |m: &Response| { &m.fetchType },
            |m: &mut Response| { &mut m.fetchType },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "routeParamsMap",
            |m: &Response| { &m.routeParamsMap },
            |m: &mut Response| { &mut m.routeParamsMap },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "heartbeatDuration",
            |m: &Response| { &m.heartbeatDuration },
            |m: &mut Response| { &mut m.heartbeatDuration },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "needAck",
            |m: &Response| { &m.needAck },
            |m: &mut Response| { &mut m.needAck },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "pushServer",
            |m: &Response| { &m.pushServer },
            |m: &mut Response| { &mut m.pushServer },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "liveCursor",
            |m: &Response| { &m.liveCursor },
            |m: &mut Response| { &mut m.liveCursor },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "historyNoMore",
            |m: &Response| { &m.historyNoMore },
            |m: &mut Response| { &mut m.historyNoMore },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Response>(
            "Response",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Response {
    const NAME: &'static str = "Response";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.messagesList.push(is.read_message()?);
                },
                18 => {
                    self.cursor = is.read_string()?;
                },
                24 => {
                    self.fetchInterval = is.read_int64()?;
                },
                32 => {
                    self.now = is.read_int64()?;
                },
                42 => {
                    self.internalExt = is.read_string()?;
                },
                48 => {
                    self.fetchType = is.read_int32()?;
                },
                58 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            10 => key = is.read_string()?,
                            18 => value = is.read_string()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.routeParamsMap.insert(key, value);
                },
                64 => {
                    self.heartbeatDuration = is.read_int64()?;
                },
                72 => {
                    self.needAck = is.read_bool()?;
                },
                82 => {
                    self.pushServer = is.read_string()?;
                },
                90 => {
                    self.liveCursor = is.read_string()?;
                },
                96 => {
                    self.historyNoMore = is.read_bool()?;
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
        for value in &self.messagesList {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if !self.cursor.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.cursor);
        }
        if self.fetchInterval != 0 {
            my_size += ::protobuf::rt::int64_size(3, self.fetchInterval);
        }
        if self.now != 0 {
            my_size += ::protobuf::rt::int64_size(4, self.now);
        }
        if !self.internalExt.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.internalExt);
        }
        if self.fetchType != 0 {
            my_size += ::protobuf::rt::int32_size(6, self.fetchType);
        }
        for (k, v) in &self.routeParamsMap {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::string_size(1, &k);
            entry_size += ::protobuf::rt::string_size(2, &v);
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        if self.heartbeatDuration != 0 {
            my_size += ::protobuf::rt::int64_size(8, self.heartbeatDuration);
        }
        if self.needAck != false {
            my_size += 1 + 1;
        }
        if !self.pushServer.is_empty() {
            my_size += ::protobuf::rt::string_size(10, &self.pushServer);
        }
        if !self.liveCursor.is_empty() {
            my_size += ::protobuf::rt::string_size(11, &self.liveCursor);
        }
        if self.historyNoMore != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.messagesList {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        if !self.cursor.is_empty() {
            os.write_string(2, &self.cursor)?;
        }
        if self.fetchInterval != 0 {
            os.write_int64(3, self.fetchInterval)?;
        }
        if self.now != 0 {
            os.write_int64(4, self.now)?;
        }
        if !self.internalExt.is_empty() {
            os.write_string(5, &self.internalExt)?;
        }
        if self.fetchType != 0 {
            os.write_int32(6, self.fetchType)?;
        }
        for (k, v) in &self.routeParamsMap {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::string_size(1, &k);
            entry_size += ::protobuf::rt::string_size(2, &v);
            os.write_raw_varint32(58)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_string(1, &k)?;
            os.write_string(2, &v)?;
        };
        if self.heartbeatDuration != 0 {
            os.write_int64(8, self.heartbeatDuration)?;
        }
        if self.needAck != false {
            os.write_bool(9, self.needAck)?;
        }
        if !self.pushServer.is_empty() {
            os.write_string(10, &self.pushServer)?;
        }
        if !self.liveCursor.is_empty() {
            os.write_string(11, &self.liveCursor)?;
        }
        if self.historyNoMore != false {
            os.write_bool(12, self.historyNoMore)?;
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

    fn new() -> Response {
        Response::new()
    }

    fn clear(&mut self) {
        self.messagesList.clear();
        self.cursor.clear();
        self.fetchInterval = 0;
        self.now = 0;
        self.internalExt.clear();
        self.fetchType = 0;
        self.routeParamsMap.clear();
        self.heartbeatDuration = 0;
        self.needAck = false;
        self.pushServer.clear();
        self.liveCursor.clear();
        self.historyNoMore = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Response {
        static instance: ::protobuf::rt::Lazy<Response> = ::protobuf::rt::Lazy::new();
        instance.get(Response::new)
    }
}

impl ::protobuf::MessageFull for Response {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Response").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Response {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19webcast/im/Response.proto\x1a\x18webcast/im/Message.proto\"\x80\
    \x04\n\x08Response\x12,\n\x0cmessagesList\x18\x01\x20\x03(\x0b2\x08.Mess\
    ageR\x0cmessagesList\x12\x16\n\x06cursor\x18\x02\x20\x01(\tR\x06cursor\
    \x12$\n\rfetchInterval\x18\x03\x20\x01(\x03R\rfetchInterval\x12\x10\n\
    \x03now\x18\x04\x20\x01(\x03R\x03now\x12\x20\n\x0binternalExt\x18\x05\
    \x20\x01(\tR\x0binternalExt\x12\x1c\n\tfetchType\x18\x06\x20\x01(\x05R\t\
    fetchType\x12E\n\x0erouteParamsMap\x18\x07\x20\x03(\x0b2\x1d.Response.Ro\
    uteParamsMapEntryR\x0erouteParamsMap\x12,\n\x11heartbeatDuration\x18\x08\
    \x20\x01(\x03R\x11heartbeatDuration\x12\x18\n\x07needAck\x18\t\x20\x01(\
    \x08R\x07needAck\x12\x1e\n\npushServer\x18\n\x20\x01(\tR\npushServer\x12\
    \x1e\n\nliveCursor\x18\x0b\x20\x01(\tR\nliveCursor\x12$\n\rhistoryNoMore\
    \x18\x0c\x20\x01(\x08R\rhistoryNoMore\x1aA\n\x13RouteParamsMapEntry\x12\
    \x10\n\x03key\x18\x01\x20\x01(\tR\x03key\x12\x14\n\x05value\x18\x02\x20\
    \x01(\tR\x05value:\x028\x01B=\n9cool.scx.live_room_watcher.douyin.proto_\
    entity.webcast.imP\x01J\x9e\x06\n\x06\x12\x04\0\0\x15\x01\n\x08\n\x01\
    \x0c\x12\x03\0\0\x12\n\x08\n\x01\x08\x12\x03\x02\0\"\n\t\n\x02\x08\n\x12\
    \x03\x02\0\"\n\x08\n\x01\x08\x12\x03\x03\0R\n\t\n\x02\x08\x01\x12\x03\
    \x03\0R\n\t\n\x02\x03\0\x12\x03\x05\0\"\n'\n\x02\x04\0\x12\x04\x08\0\x15\
    \x01\x1a\x1b\xe6\x9b\xb4\xe6\x96\xb0\xe6\x97\xb6\xe9\x97\xb4\x20:\x20202\
    3-01-09\r\n\n\n\n\x03\x04\0\x01\x12\x03\x08\x08\x10\n\x0b\n\x04\x04\0\
    \x02\0\x12\x03\t\x02$\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\t\x02\n\n\x0c\
    \n\x05\x04\0\x02\0\x06\x12\x03\t\x0b\x12\n\x0c\n\x05\x04\0\x02\0\x01\x12\
    \x03\t\x13\x1f\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\t\"#\n\x0b\n\x04\x04\
    \0\x02\x01\x12\x03\n\x02\x14\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\n\x02\
    \x08\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\n\t\x0f\n\x0c\n\x05\x04\0\x02\
    \x01\x03\x12\x03\n\x12\x13\n\x0b\n\x04\x04\0\x02\x02\x12\x03\x0b\x02\x1a\
    \n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x0b\x02\x07\n\x0c\n\x05\x04\0\x02\
    \x02\x01\x12\x03\x0b\x08\x15\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x0b\
    \x18\x19\n\x0b\n\x04\x04\0\x02\x03\x12\x03\x0c\x02\x10\n\x0c\n\x05\x04\0\
    \x02\x03\x05\x12\x03\x0c\x02\x07\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03\
    \x0c\x08\x0b\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\x0c\x0e\x0f\n\x0b\n\
    \x04\x04\0\x02\x04\x12\x03\r\x02\x19\n\x0c\n\x05\x04\0\x02\x04\x05\x12\
    \x03\r\x02\x08\n\x0c\n\x05\x04\0\x02\x04\x01\x12\x03\r\t\x14\n\x0c\n\x05\
    \x04\0\x02\x04\x03\x12\x03\r\x17\x18\n\x0b\n\x04\x04\0\x02\x05\x12\x03\
    \x0e\x02\x16\n\x0c\n\x05\x04\0\x02\x05\x05\x12\x03\x0e\x02\x07\n\x0c\n\
    \x05\x04\0\x02\x05\x01\x12\x03\x0e\x08\x11\n\x0c\n\x05\x04\0\x02\x05\x03\
    \x12\x03\x0e\x14\x15\n\x0b\n\x04\x04\0\x02\x06\x12\x03\x0f\x02)\n\x0c\n\
    \x05\x04\0\x02\x06\x06\x12\x03\x0f\x02\x15\n\x0c\n\x05\x04\0\x02\x06\x01\
    \x12\x03\x0f\x16$\n\x0c\n\x05\x04\0\x02\x06\x03\x12\x03\x0f'(\n\x0b\n\
    \x04\x04\0\x02\x07\x12\x03\x10\x02\x1e\n\x0c\n\x05\x04\0\x02\x07\x05\x12\
    \x03\x10\x02\x07\n\x0c\n\x05\x04\0\x02\x07\x01\x12\x03\x10\x08\x19\n\x0c\
    \n\x05\x04\0\x02\x07\x03\x12\x03\x10\x1c\x1d\n\x0b\n\x04\x04\0\x02\x08\
    \x12\x03\x11\x02\x13\n\x0c\n\x05\x04\0\x02\x08\x05\x12\x03\x11\x02\x06\n\
    \x0c\n\x05\x04\0\x02\x08\x01\x12\x03\x11\x07\x0e\n\x0c\n\x05\x04\0\x02\
    \x08\x03\x12\x03\x11\x11\x12\n\x0b\n\x04\x04\0\x02\t\x12\x03\x12\x02\x19\
    \n\x0c\n\x05\x04\0\x02\t\x05\x12\x03\x12\x02\x08\n\x0c\n\x05\x04\0\x02\t\
    \x01\x12\x03\x12\t\x13\n\x0c\n\x05\x04\0\x02\t\x03\x12\x03\x12\x16\x18\n\
    \x0b\n\x04\x04\0\x02\n\x12\x03\x13\x02\x19\n\x0c\n\x05\x04\0\x02\n\x05\
    \x12\x03\x13\x02\x08\n\x0c\n\x05\x04\0\x02\n\x01\x12\x03\x13\t\x13\n\x0c\
    \n\x05\x04\0\x02\n\x03\x12\x03\x13\x16\x18\n\x0b\n\x04\x04\0\x02\x0b\x12\
    \x03\x14\x02\x1a\n\x0c\n\x05\x04\0\x02\x0b\x05\x12\x03\x14\x02\x06\n\x0c\
    \n\x05\x04\0\x02\x0b\x01\x12\x03\x14\x07\x14\n\x0c\n\x05\x04\0\x02\x0b\
    \x03\x12\x03\x14\x17\x19b\x06proto3\
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
            deps.push(super::Message::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(Response::generated_message_descriptor_data());
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
