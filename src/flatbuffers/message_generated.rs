// automatically generated by the FlatBuffers compiler, do not modify

// @generated


extern crate flatbuffers;

#[allow(unused_imports, dead_code)]
pub mod debra {

    use core::cmp::Ordering;
    use core::mem;

    extern crate flatbuffers;
    use self::flatbuffers::{EndianScalar, Follow};

    #[deprecated(
        since = "2.0.0",
        note = "Use associated constants instead. This will no longer be generated in 2021."
    )]
    pub const ENUM_MIN_MESSAGE_TYPE: i8 = 0;
    #[deprecated(
        since = "2.0.0",
        note = "Use associated constants instead. This will no longer be generated in 2021."
    )]
    pub const ENUM_MAX_MESSAGE_TYPE: i8 = 1;
    #[deprecated(
        since = "2.0.0",
        note = "Use associated constants instead. This will no longer be generated in 2021."
    )]
    #[allow(non_camel_case_types)]
    pub const ENUM_VALUES_MESSAGE_TYPE: [MessageType; 2] =
        [MessageType::ClientRegistration, MessageType::ClientToClient];

    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    #[repr(transparent)]
    pub struct MessageType(pub i8);
    #[allow(non_upper_case_globals)]
    impl MessageType {
        pub const ClientRegistration: Self = Self(0);
        pub const ClientToClient: Self = Self(1);

        pub const ENUM_MIN: i8 = 0;
        pub const ENUM_MAX: i8 = 1;
        pub const ENUM_VALUES: &'static [Self] = &[Self::ClientRegistration, Self::ClientToClient];
        /// Returns the variant's name or "" if unknown.
        pub fn variant_name(self) -> Option<&'static str> {
            match self {
                Self::ClientRegistration => Some("ClientRegistration"),
                Self::ClientToClient => Some("ClientToClient"),
                _ => None,
            }
        }
    }
    impl core::fmt::Debug for MessageType {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            if let Some(name) = self.variant_name() {
                f.write_str(name)
            } else {
                f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
            }
        }
    }
    impl<'a> flatbuffers::Follow<'a> for MessageType {
        type Inner = Self;
        #[inline]
        unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
            let b = flatbuffers::read_scalar_at::<i8>(buf, loc);
            Self(b)
        }
    }

    impl flatbuffers::Push for MessageType {
        type Output = MessageType;
        #[inline]
        unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
            flatbuffers::emplace_scalar::<i8>(dst, self.0);
        }
    }

    impl flatbuffers::EndianScalar for MessageType {
        type Scalar = i8;
        #[inline]
        fn to_little_endian(self) -> i8 {
            self.0.to_le()
        }
        #[inline]
        #[allow(clippy::wrong_self_convention)]
        fn from_little_endian(v: i8) -> Self {
            let b = i8::from_le(v);
            Self(b)
        }
    }

    impl<'a> flatbuffers::Verifiable for MessageType {
        #[inline]
        fn run_verifier(
            v: &mut flatbuffers::Verifier,
            pos: usize,
        ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
            use self::flatbuffers::Verifiable;
            i8::run_verifier(v, pos)
        }
    }

    impl flatbuffers::SimpleToVerifyInSlice for MessageType {}
    pub enum MessageOffset {}
    #[derive(Copy, Clone, PartialEq)]

    pub struct Message<'a> {
        pub _tab: flatbuffers::Table<'a>,
    }

    impl<'a> flatbuffers::Follow<'a> for Message<'a> {
        type Inner = Message<'a>;
        #[inline]
        unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
            Self {
                _tab: flatbuffers::Table::new(buf, loc),
            }
        }
    }

    impl<'a> Message<'a> {
        pub const VT_MESSAGE_TYPE: flatbuffers::VOffsetT = 4;
        pub const VT_CLIENT_ID: flatbuffers::VOffsetT = 6;
        pub const VT_FOR_CLIENT: flatbuffers::VOffsetT = 8;
        pub const VT_MESSAGE: flatbuffers::VOffsetT = 10;

        #[inline]
        pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
            Message { _tab: table }
        }
        #[allow(unused_mut)]
        pub fn create<
            'bldr: 'args,
            'args: 'mut_bldr,
            'mut_bldr,
            A: flatbuffers::Allocator + 'bldr,
        >(
            _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
            args: &'args MessageArgs<'args>,
        ) -> flatbuffers::WIPOffset<Message<'bldr>> {
            let mut builder = MessageBuilder::new(_fbb);
            if let Some(x) = args.message {
                builder.add_message(x);
            }
            builder.add_for_client(args.for_client);
            builder.add_client_id(args.client_id);
            builder.add_message_type(args.message_type);
            builder.finish()
        }

        #[inline]
        pub fn message_type(&self) -> MessageType {
            // Safety:
            // Created from valid Table for this object
            // which contains a valid value in this slot
            unsafe {
                self._tab
                    .get::<MessageType>(
                        Message::VT_MESSAGE_TYPE,
                        Some(MessageType::ClientRegistration),
                    )
                    .unwrap()
            }
        }
        #[inline]
        pub fn client_id(&self) -> i32 {
            // Safety:
            // Created from valid Table for this object
            // which contains a valid value in this slot
            unsafe {
                self._tab
                    .get::<i32>(Message::VT_CLIENT_ID, Some(0))
                    .unwrap()
            }
        }
        #[inline]
        pub fn for_client(&self) -> i32 {
            // Safety:
            // Created from valid Table for this object
            // which contains a valid value in this slot
            unsafe {
                self._tab
                    .get::<i32>(Message::VT_FOR_CLIENT, Some(0))
                    .unwrap()
            }
        }
        #[inline]
        pub fn message(&self) -> Option<flatbuffers::Vector<'a, u8>> {
            // Safety:
            // Created from valid Table for this object
            // which contains a valid value in this slot
            unsafe {
                self._tab
                    .get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(
                        Message::VT_MESSAGE,
                        None,
                    )
            }
        }
    }

    impl flatbuffers::Verifiable for Message<'_> {
        #[inline]
        fn run_verifier(
            v: &mut flatbuffers::Verifier,
            pos: usize,
        ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
            use self::flatbuffers::Verifiable;
            v.visit_table(pos)?
                .visit_field::<MessageType>("message_type", Self::VT_MESSAGE_TYPE, false)?
                .visit_field::<i32>("client_id", Self::VT_CLIENT_ID, false)?
                .visit_field::<i32>("for_client", Self::VT_FOR_CLIENT, false)?
                .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u8>>>(
                    "message",
                    Self::VT_MESSAGE,
                    false,
                )?
                .finish();
            Ok(())
        }
    }
    pub struct MessageArgs<'a> {
        pub message_type: MessageType,
        pub client_id: i32,
        pub for_client: i32,
        pub message: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
    }
    impl Default for MessageArgs<'_> {
        #[inline]
        fn default() -> Self {
            MessageArgs {
                message_type: MessageType::ClientRegistration,
                client_id: 0,
                for_client: 0,
                message: None,
            }
        }
    }

    pub struct MessageBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
        fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
        start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
    }
    impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> MessageBuilder<'a, 'b, A> {
        #[inline]
        pub fn add_message_type(&mut self, message_type: MessageType) {
            self.fbb_.push_slot::<MessageType>(
                Message::VT_MESSAGE_TYPE,
                message_type,
                MessageType::ClientRegistration,
            );
        }
        #[inline]
        pub fn add_client_id(&mut self, client_id: i32) {
            self.fbb_
                .push_slot::<i32>(Message::VT_CLIENT_ID, client_id, 0);
        }
        #[inline]
        pub fn add_for_client(&mut self, for_client: i32) {
            self.fbb_
                .push_slot::<i32>(Message::VT_FOR_CLIENT, for_client, 0);
        }
        #[inline]
        pub fn add_message(
            &mut self,
            message: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u8>>,
        ) {
            self.fbb_
                .push_slot_always::<flatbuffers::WIPOffset<_>>(Message::VT_MESSAGE, message);
        }
        #[inline]
        pub fn new(
            _fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
        ) -> MessageBuilder<'a, 'b, A> {
            let start = _fbb.start_table();
            MessageBuilder {
                fbb_: _fbb,
                start_: start,
            }
        }
        #[inline]
        pub fn finish(self) -> flatbuffers::WIPOffset<Message<'a>> {
            let o = self.fbb_.end_table(self.start_);
            flatbuffers::WIPOffset::new(o.value())
        }
    }

    impl core::fmt::Debug for Message<'_> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            let mut ds = f.debug_struct("Message");
            ds.field("message_type", &self.message_type());
            ds.field("client_id", &self.client_id());
            ds.field("for_client", &self.for_client());
            ds.field("message", &self.message());
            ds.finish()
        }
    }
    #[inline]
    /// Verifies that a buffer of bytes contains a `Message`
    /// and returns it.
    /// Note that verification is still experimental and may not
    /// catch every error, or be maximally performant. For the
    /// previous, unchecked, behavior use
    /// `root_as_message_unchecked`.
    pub fn root_as_message(buf: &[u8]) -> Result<Message, flatbuffers::InvalidFlatbuffer> {
        flatbuffers::root::<Message>(buf)
    }
    #[inline]
    /// Verifies that a buffer of bytes contains a size prefixed
    /// `Message` and returns it.
    /// Note that verification is still experimental and may not
    /// catch every error, or be maximally performant. For the
    /// previous, unchecked, behavior use
    /// `size_prefixed_root_as_message_unchecked`.
    pub fn size_prefixed_root_as_message(
        buf: &[u8],
    ) -> Result<Message, flatbuffers::InvalidFlatbuffer> {
        flatbuffers::size_prefixed_root::<Message>(buf)
    }
    #[inline]
    /// Verifies, with the given options, that a buffer of bytes
    /// contains a `Message` and returns it.
    /// Note that verification is still experimental and may not
    /// catch every error, or be maximally performant. For the
    /// previous, unchecked, behavior use
    /// `root_as_message_unchecked`.
    pub fn root_as_message_with_opts<'b, 'o>(
        opts: &'o flatbuffers::VerifierOptions,
        buf: &'b [u8],
    ) -> Result<Message<'b>, flatbuffers::InvalidFlatbuffer> {
        flatbuffers::root_with_opts::<Message<'b>>(opts, buf)
    }
    #[inline]
    /// Verifies, with the given verifier options, that a buffer of
    /// bytes contains a size prefixed `Message` and returns
    /// it. Note that verification is still experimental and may not
    /// catch every error, or be maximally performant. For the
    /// previous, unchecked, behavior use
    /// `root_as_message_unchecked`.
    pub fn size_prefixed_root_as_message_with_opts<'b, 'o>(
        opts: &'o flatbuffers::VerifierOptions,
        buf: &'b [u8],
    ) -> Result<Message<'b>, flatbuffers::InvalidFlatbuffer> {
        flatbuffers::size_prefixed_root_with_opts::<Message<'b>>(opts, buf)
    }
    #[inline]
    /// Assumes, without verification, that a buffer of bytes contains a Message and returns it.
    /// # Safety
    /// Callers must trust the given bytes do indeed contain a valid `Message`.
    pub unsafe fn root_as_message_unchecked(buf: &[u8]) -> Message {
        flatbuffers::root_unchecked::<Message>(buf)
    }
    #[inline]
    /// Assumes, without verification, that a buffer of bytes contains a size prefixed Message and returns it.
    /// # Safety
    /// Callers must trust the given bytes do indeed contain a valid size prefixed `Message`.
    pub unsafe fn size_prefixed_root_as_message_unchecked(buf: &[u8]) -> Message {
        flatbuffers::size_prefixed_root_unchecked::<Message>(buf)
    }
    #[inline]
    pub fn finish_message_buffer<'a, 'b, A: flatbuffers::Allocator + 'a>(
        fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
        root: flatbuffers::WIPOffset<Message<'a>>,
    ) {
        fbb.finish(root, None);
    }

    #[inline]
    pub fn finish_size_prefixed_message_buffer<'a, 'b, A: flatbuffers::Allocator + 'a>(
        fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
        root: flatbuffers::WIPOffset<Message<'a>>,
    ) {
        fbb.finish_size_prefixed(root, None);
    }
} // pub mod Debra
