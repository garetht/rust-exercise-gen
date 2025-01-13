// This file is generated by rust-protobuf 3.7.1. Do not edit
// .proto file is parsed by protoc --rs_out=...
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `src/protos/exercises.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:rust.exercises.Exercises)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct Exercises {
    // message fields
    // @@protoc_insertion_point(field:rust.exercises.Exercises.exercise_groups)
    pub exercise_groups: ::std::vec::Vec<ErrorExerciseGroup>,
    // special fields
    // @@protoc_insertion_point(special_field:rust.exercises.Exercises.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Exercises {
    fn default() -> &'a Exercises {
        <Exercises as ::protobuf::Message>::default_instance()
    }
}

impl Exercises {
    pub fn new() -> Exercises {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "exercise_groups",
            |m: &Exercises| { &m.exercise_groups },
            |m: &mut Exercises| { &mut m.exercise_groups },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Exercises>(
            "Exercises",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Exercises {
    const NAME: &'static str = "Exercises";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.exercise_groups.push(is.read_message()?);
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
        for value in &self.exercise_groups {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.exercise_groups {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> Exercises {
        Exercises::new()
    }

    fn clear(&mut self) {
        self.exercise_groups.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Exercises {
        static instance: Exercises = Exercises {
            exercise_groups: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Exercises {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Exercises").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Exercises {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Exercises {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:rust.exercises.ErrorExerciseGroup)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ErrorExerciseGroup {
    // message fields
    // @@protoc_insertion_point(field:rust.exercises.ErrorExerciseGroup.error_codes)
    pub error_codes: ::std::vec::Vec<::std::string::String>,
    // @@protoc_insertion_point(field:rust.exercises.ErrorExerciseGroup.exercises)
    pub exercises: ::std::vec::Vec<Exercise>,
    // special fields
    // @@protoc_insertion_point(special_field:rust.exercises.ErrorExerciseGroup.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ErrorExerciseGroup {
    fn default() -> &'a ErrorExerciseGroup {
        <ErrorExerciseGroup as ::protobuf::Message>::default_instance()
    }
}

impl ErrorExerciseGroup {
    pub fn new() -> ErrorExerciseGroup {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "error_codes",
            |m: &ErrorExerciseGroup| { &m.error_codes },
            |m: &mut ErrorExerciseGroup| { &mut m.error_codes },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "exercises",
            |m: &ErrorExerciseGroup| { &m.exercises },
            |m: &mut ErrorExerciseGroup| { &mut m.exercises },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ErrorExerciseGroup>(
            "ErrorExerciseGroup",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ErrorExerciseGroup {
    const NAME: &'static str = "ErrorExerciseGroup";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.error_codes.push(is.read_string()?);
                },
                18 => {
                    self.exercises.push(is.read_message()?);
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
        for value in &self.error_codes {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.exercises {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.error_codes {
            os.write_string(1, &v)?;
        };
        for v in &self.exercises {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> ErrorExerciseGroup {
        ErrorExerciseGroup::new()
    }

    fn clear(&mut self) {
        self.error_codes.clear();
        self.exercises.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ErrorExerciseGroup {
        static instance: ErrorExerciseGroup = ErrorExerciseGroup {
            error_codes: ::std::vec::Vec::new(),
            exercises: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ErrorExerciseGroup {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ErrorExerciseGroup").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ErrorExerciseGroup {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ErrorExerciseGroup {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:rust.exercises.Exercise)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct Exercise {
    // message fields
    // @@protoc_insertion_point(field:rust.exercises.Exercise.formatted_program)
    pub formatted_program: ::std::string::String,
    /// *
    /// Available variable names in the program. Used for creating
    /// distractors
    // @@protoc_insertion_point(field:rust.exercises.Exercise.variable_names)
    pub variable_names: ::std::vec::Vec<::std::string::String>,
    // @@protoc_insertion_point(field:rust.exercises.Exercise.errors)
    pub errors: ::std::vec::Vec<ErrorMessage>,
    // @@protoc_insertion_point(field:rust.exercises.Exercise.human_errors)
    pub human_errors: ::std::vec::Vec<::std::string::String>,
    /// *
    /// How many primitive units are in this program. Used for
    /// starting with simpler programs
    // @@protoc_insertion_point(field:rust.exercises.Exercise.program_length)
    pub program_length: i32,
    // special fields
    // @@protoc_insertion_point(special_field:rust.exercises.Exercise.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Exercise {
    fn default() -> &'a Exercise {
        <Exercise as ::protobuf::Message>::default_instance()
    }
}

impl Exercise {
    pub fn new() -> Exercise {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "formatted_program",
            |m: &Exercise| { &m.formatted_program },
            |m: &mut Exercise| { &mut m.formatted_program },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "variable_names",
            |m: &Exercise| { &m.variable_names },
            |m: &mut Exercise| { &mut m.variable_names },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "errors",
            |m: &Exercise| { &m.errors },
            |m: &mut Exercise| { &mut m.errors },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "human_errors",
            |m: &Exercise| { &m.human_errors },
            |m: &mut Exercise| { &mut m.human_errors },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "program_length",
            |m: &Exercise| { &m.program_length },
            |m: &mut Exercise| { &mut m.program_length },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Exercise>(
            "Exercise",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Exercise {
    const NAME: &'static str = "Exercise";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.formatted_program = is.read_string()?;
                },
                18 => {
                    self.variable_names.push(is.read_string()?);
                },
                26 => {
                    self.errors.push(is.read_message()?);
                },
                34 => {
                    self.human_errors.push(is.read_string()?);
                },
                40 => {
                    self.program_length = is.read_int32()?;
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
        if !self.formatted_program.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.formatted_program);
        }
        for value in &self.variable_names {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.errors {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.human_errors {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        if self.program_length != 0 {
            my_size += ::protobuf::rt::int32_size(5, self.program_length);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.formatted_program.is_empty() {
            os.write_string(1, &self.formatted_program)?;
        }
        for v in &self.variable_names {
            os.write_string(2, &v)?;
        };
        for v in &self.errors {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        };
        for v in &self.human_errors {
            os.write_string(4, &v)?;
        };
        if self.program_length != 0 {
            os.write_int32(5, self.program_length)?;
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

    fn new() -> Exercise {
        Exercise::new()
    }

    fn clear(&mut self) {
        self.formatted_program.clear();
        self.variable_names.clear();
        self.errors.clear();
        self.human_errors.clear();
        self.program_length = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Exercise {
        static instance: Exercise = Exercise {
            formatted_program: ::std::string::String::new(),
            variable_names: ::std::vec::Vec::new(),
            errors: ::std::vec::Vec::new(),
            human_errors: ::std::vec::Vec::new(),
            program_length: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Exercise {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Exercise").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Exercise {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Exercise {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:rust.exercises.ErrorMessage)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ErrorMessage {
    // message fields
    // @@protoc_insertion_point(field:rust.exercises.ErrorMessage.message)
    pub message: ::std::string::String,
    // @@protoc_insertion_point(field:rust.exercises.ErrorMessage.implicated_variable_names)
    pub implicated_variable_names: ::std::vec::Vec<::std::string::String>,
    // @@protoc_insertion_point(field:rust.exercises.ErrorMessage.code)
    pub code: ::protobuf::MessageField<ErrorMessageCode>,
    // special fields
    // @@protoc_insertion_point(special_field:rust.exercises.ErrorMessage.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ErrorMessage {
    fn default() -> &'a ErrorMessage {
        <ErrorMessage as ::protobuf::Message>::default_instance()
    }
}

impl ErrorMessage {
    pub fn new() -> ErrorMessage {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "message",
            |m: &ErrorMessage| { &m.message },
            |m: &mut ErrorMessage| { &mut m.message },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "implicated_variable_names",
            |m: &ErrorMessage| { &m.implicated_variable_names },
            |m: &mut ErrorMessage| { &mut m.implicated_variable_names },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, ErrorMessageCode>(
            "code",
            |m: &ErrorMessage| { &m.code },
            |m: &mut ErrorMessage| { &mut m.code },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ErrorMessage>(
            "ErrorMessage",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ErrorMessage {
    const NAME: &'static str = "ErrorMessage";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.message = is.read_string()?;
                },
                18 => {
                    self.implicated_variable_names.push(is.read_string()?);
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.code)?;
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
        if !self.message.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.message);
        }
        for value in &self.implicated_variable_names {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        if let Some(v) = self.code.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.message.is_empty() {
            os.write_string(1, &self.message)?;
        }
        for v in &self.implicated_variable_names {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.code.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
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

    fn new() -> ErrorMessage {
        ErrorMessage::new()
    }

    fn clear(&mut self) {
        self.message.clear();
        self.implicated_variable_names.clear();
        self.code.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ErrorMessage {
        static instance: ErrorMessage = ErrorMessage {
            message: ::std::string::String::new(),
            implicated_variable_names: ::std::vec::Vec::new(),
            code: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ErrorMessage {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ErrorMessage").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ErrorMessage {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:rust.exercises.ErrorMessageCode)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ErrorMessageCode {
    // message fields
    // @@protoc_insertion_point(field:rust.exercises.ErrorMessageCode.code)
    pub code: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:rust.exercises.ErrorMessageCode.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ErrorMessageCode {
    fn default() -> &'a ErrorMessageCode {
        <ErrorMessageCode as ::protobuf::Message>::default_instance()
    }
}

impl ErrorMessageCode {
    pub fn new() -> ErrorMessageCode {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "code",
            |m: &ErrorMessageCode| { &m.code },
            |m: &mut ErrorMessageCode| { &mut m.code },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ErrorMessageCode>(
            "ErrorMessageCode",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ErrorMessageCode {
    const NAME: &'static str = "ErrorMessageCode";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.code = is.read_string()?;
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
        if !self.code.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.code);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.code.is_empty() {
            os.write_string(1, &self.code)?;
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

    fn new() -> ErrorMessageCode {
        ErrorMessageCode::new()
    }

    fn clear(&mut self) {
        self.code.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ErrorMessageCode {
        static instance: ErrorMessageCode = ErrorMessageCode {
            code: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ErrorMessageCode {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ErrorMessageCode").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ErrorMessageCode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ErrorMessageCode {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1asrc/protos/exercises.proto\x12\x0erust.exercises\"X\n\tExercises\
    \x12K\n\x0fexercise_groups\x18\x01\x20\x03(\x0b2\".rust.exercises.ErrorE\
    xerciseGroupR\x0eexerciseGroups\"m\n\x12ErrorExerciseGroup\x12\x1f\n\x0b\
    error_codes\x18\x01\x20\x03(\tR\nerrorCodes\x126\n\texercises\x18\x02\
    \x20\x03(\x0b2\x18.rust.exercises.ExerciseR\texercises\"\xde\x01\n\x08Ex\
    ercise\x12+\n\x11formatted_program\x18\x01\x20\x01(\tR\x10formattedProgr\
    am\x12%\n\x0evariable_names\x18\x02\x20\x03(\tR\rvariableNames\x124\n\
    \x06errors\x18\x03\x20\x03(\x0b2\x1c.rust.exercises.ErrorMessageR\x06err\
    ors\x12!\n\x0chuman_errors\x18\x04\x20\x03(\tR\x0bhumanErrors\x12%\n\x0e\
    program_length\x18\x05\x20\x01(\x05R\rprogramLength\"\x9a\x01\n\x0cError\
    Message\x12\x18\n\x07message\x18\x01\x20\x01(\tR\x07message\x12:\n\x19im\
    plicated_variable_names\x18\x02\x20\x03(\tR\x17implicatedVariableNames\
    \x124\n\x04code\x18\x03\x20\x01(\x0b2\x20.rust.exercises.ErrorMessageCod\
    eR\x04code\"&\n\x10ErrorMessageCode\x12\x12\n\x04code\x18\x01\x20\x01(\t\
    R\x04codeJ\xe2\x08\n\x06\x12\x04\0\0&\x01\n\x08\n\x01\x0c\x12\x03\0\0\
    \x12\n\x08\n\x01\x02\x12\x03\x02\0\x17\n\n\n\x02\x04\0\x12\x04\x04\0\x07\
    \x01\n\n\n\x03\x04\0\x01\x12\x03\x04\x08\x11\n>\n\x04\x04\0\x02\0\x12\
    \x03\x05\x022\"1\x20Rust\x20explanation\x20map??\x20but\x20we'd\x20need\
    \x20formatting\n\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\x05\x02\n\n\x0c\n\
    \x05\x04\0\x02\0\x06\x12\x03\x05\x0b\x1d\n\x0c\n\x05\x04\0\x02\0\x01\x12\
    \x03\x05\x1e-\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x0501\n\n\n\x02\x04\
    \x01\x12\x04\t\0\x0c\x01\n\n\n\x03\x04\x01\x01\x12\x03\t\x08\x1a\n\x0b\n\
    \x04\x04\x01\x02\0\x12\x03\n\x02\"\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x03\
    \n\x02\n\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\n\x0b\x11\n\x0c\n\x05\x04\
    \x01\x02\0\x01\x12\x03\n\x12\x1d\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\n\
    \x20!\n\x0b\n\x04\x04\x01\x02\x01\x12\x03\x0b\x02\"\n\x0c\n\x05\x04\x01\
    \x02\x01\x04\x12\x03\x0b\x02\n\n\x0c\n\x05\x04\x01\x02\x01\x06\x12\x03\
    \x0b\x0b\x13\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\x0b\x14\x1d\n\x0c\n\
    \x05\x04\x01\x02\x01\x03\x12\x03\x0b\x20!\n\n\n\x02\x04\x02\x12\x04\x0e\
    \0\x1c\x01\n\n\n\x03\x04\x02\x01\x12\x03\x0e\x08\x10\n\x0b\n\x04\x04\x02\
    \x02\0\x12\x03\x0f\x02\x1f\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03\x0f\x02\
    \x08\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\x0f\t\x1a\n\x0c\n\x05\x04\x02\
    \x02\0\x03\x12\x03\x0f\x1d\x1e\nV\n\x04\x04\x02\x02\x01\x12\x03\x14\x02%\
    \x1aI*\nAvailable\x20variable\x20names\x20in\x20the\x20program.\x20Used\
    \x20for\x20creating\ndistractors\n\n\x0c\n\x05\x04\x02\x02\x01\x04\x12\
    \x03\x14\x02\n\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\x03\x14\x0b\x11\n\x0c\
    \n\x05\x04\x02\x02\x01\x01\x12\x03\x14\x12\x20\n\x0c\n\x05\x04\x02\x02\
    \x01\x03\x12\x03\x14#$\n\x0b\n\x04\x04\x02\x02\x02\x12\x03\x15\x02#\n\
    \x0c\n\x05\x04\x02\x02\x02\x04\x12\x03\x15\x02\n\n\x0c\n\x05\x04\x02\x02\
    \x02\x06\x12\x03\x15\x0b\x17\n\x0c\n\x05\x04\x02\x02\x02\x01\x12\x03\x15\
    \x18\x1e\n\x0c\n\x05\x04\x02\x02\x02\x03\x12\x03\x15!\"\n\x0b\n\x04\x04\
    \x02\x02\x03\x12\x03\x16\x02#\n\x0c\n\x05\x04\x02\x02\x03\x04\x12\x03\
    \x16\x02\n\n\x0c\n\x05\x04\x02\x02\x03\x05\x12\x03\x16\x0b\x11\n\x0c\n\
    \x05\x04\x02\x02\x03\x01\x12\x03\x16\x12\x1e\n\x0c\n\x05\x04\x02\x02\x03\
    \x03\x12\x03\x16!\"\ne\n\x04\x04\x02\x02\x04\x12\x03\x1b\x02\x1b\x1aX*\n\
    How\x20many\x20primitive\x20units\x20are\x20in\x20this\x20program.\x20Us\
    ed\x20for\nstarting\x20with\x20simpler\x20programs\n\n\x0c\n\x05\x04\x02\
    \x02\x04\x05\x12\x03\x1b\x02\x07\n\x0c\n\x05\x04\x02\x02\x04\x01\x12\x03\
    \x1b\x08\x16\n\x0c\n\x05\x04\x02\x02\x04\x03\x12\x03\x1b\x19\x1a\n\n\n\
    \x02\x04\x03\x12\x04\x1e\0\"\x01\n\n\n\x03\x04\x03\x01\x12\x03\x1e\x08\
    \x14\n\x0b\n\x04\x04\x03\x02\0\x12\x03\x1f\x02\x15\n\x0c\n\x05\x04\x03\
    \x02\0\x05\x12\x03\x1f\x02\x08\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03\x1f\
    \t\x10\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03\x1f\x13\x14\n\x0b\n\x04\x04\
    \x03\x02\x01\x12\x03\x20\x020\n\x0c\n\x05\x04\x03\x02\x01\x04\x12\x03\
    \x20\x02\n\n\x0c\n\x05\x04\x03\x02\x01\x05\x12\x03\x20\x0b\x11\n\x0c\n\
    \x05\x04\x03\x02\x01\x01\x12\x03\x20\x12+\n\x0c\n\x05\x04\x03\x02\x01\
    \x03\x12\x03\x20./\n\x0b\n\x04\x04\x03\x02\x02\x12\x03!\x02\x1c\n\x0c\n\
    \x05\x04\x03\x02\x02\x06\x12\x03!\x02\x12\n\x0c\n\x05\x04\x03\x02\x02\
    \x01\x12\x03!\x13\x17\n\x0c\n\x05\x04\x03\x02\x02\x03\x12\x03!\x1a\x1b\n\
    \n\n\x02\x04\x04\x12\x04$\0&\x01\n\n\n\x03\x04\x04\x01\x12\x03$\x08\x18\
    \n\x0b\n\x04\x04\x04\x02\0\x12\x03%\x02\x12\n\x0c\n\x05\x04\x04\x02\0\
    \x05\x12\x03%\x02\x08\n\x0c\n\x05\x04\x04\x02\0\x01\x12\x03%\t\r\n\x0c\n\
    \x05\x04\x04\x02\0\x03\x12\x03%\x10\x11b\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(5);
            messages.push(Exercises::generated_message_descriptor_data());
            messages.push(ErrorExerciseGroup::generated_message_descriptor_data());
            messages.push(Exercise::generated_message_descriptor_data());
            messages.push(ErrorMessage::generated_message_descriptor_data());
            messages.push(ErrorMessageCode::generated_message_descriptor_data());
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
