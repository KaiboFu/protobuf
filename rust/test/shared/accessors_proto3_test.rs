// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

/// Tests covering accessors for singular bool, int32, int64, and bytes fields
/// on proto3.
use googletest::prelude::*;
use protobuf::Optional;
use unittest_proto3::{TestAllTypes, TestAllTypes_};
use unittest_proto3_optional::{TestProto3Optional, TestProto3Optional_};

#[test]
fn test_fixed32_accessors() {
    let mut msg = TestAllTypes::new();
    assert_that!(msg.optional_fixed32(), eq(0));

    msg.set_optional_fixed32(42);
    assert_that!(msg.optional_fixed32(), eq(42));

    msg.set_optional_fixed32(u32::default());
    assert_that!(msg.optional_fixed32(), eq(0));

    msg.set_optional_fixed32(43);
    assert_that!(msg.optional_fixed32(), eq(43));
}

#[test]
fn test_bool_accessors() {
    let mut msg = TestAllTypes::new();
    assert_that!(msg.optional_bool(), eq(false));

    msg.set_optional_bool(true);
    assert_that!(msg.optional_bool(), eq(true));

    msg.set_optional_bool(bool::default());
    assert_that!(msg.optional_bool(), eq(false));
}

#[test]
fn test_bytes_accessors() {
    let mut msg = TestAllTypes::new();
    // Note: even though it's named 'optional_bytes', the field is actually not
    // proto3 optional, so it does not support presence.
    assert_that!(*msg.optional_bytes(), empty());

    msg.set_optional_bytes(b"accessors_test");
    assert_that!(msg.optional_bytes(), eq(b"accessors_test"));

    {
        let s = Vec::from(&b"hello world"[..]);
        msg.set_optional_bytes(&s[..]);
    }
    assert_that!(msg.optional_bytes(), eq(b"hello world"));

    msg.set_optional_bytes(b"");
    assert_that!(*msg.optional_bytes(), empty());
}

#[test]
fn test_optional_bytes_accessors() {
    let mut msg = TestProto3Optional::new();
    assert_that!(*msg.optional_bytes(), empty());
    assert_that!(msg.optional_bytes_opt(), eq(Optional::Unset(&b""[..])));

    {
        let s = Vec::from(&b"hello world"[..]);
        msg.set_optional_bytes(&s[..]);
    }
    assert_that!(msg.optional_bytes(), eq(b"hello world"));
    assert_that!(msg.optional_bytes_opt(), eq(Optional::Set(&b"hello world"[..])));

    msg.set_optional_bytes(b"");
    assert_that!(*msg.optional_bytes(), empty());
    assert_that!(msg.optional_bytes_opt(), eq(Optional::Set(&b""[..])));

    msg.set_optional_bytes(b"\xffbinary\x85non-utf8");
    assert_that!(msg.optional_bytes(), eq(b"\xffbinary\x85non-utf8"));
    assert_that!(msg.optional_bytes_opt(), eq(Optional::Set(&b"\xffbinary\x85non-utf8"[..])));
}

#[test]
fn test_string_accessors() {
    let mut msg = TestAllTypes::new();
    // Note: even though it's named 'optional_string', the field is actually not
    // proto3 optional, so it does not support presence.
    assert_that!(*msg.optional_string().as_bytes(), empty());

    msg.set_optional_string("accessors_test");
    assert_that!(msg.optional_string(), eq("accessors_test"));

    {
        let s = String::from("hello world");
        msg.set_optional_string(&s[..]);
    }
    assert_that!(msg.optional_string(), eq("hello world"));

    msg.set_optional_string("");
    assert_that!(*msg.optional_string().as_bytes(), empty());
}

#[test]
fn test_optional_string_accessors() {
    let mut msg = TestProto3Optional::new();
    assert_that!(*msg.optional_string().as_bytes(), empty());
    assert_that!(msg.optional_string_opt(), eq(Optional::Unset("".into())));

    {
        let s = String::from("hello world");
        msg.set_optional_string(&s[..]);
    }
    assert_that!(msg.optional_string(), eq("hello world"));
    assert_that!(msg.optional_string_opt(), eq(Optional::Set("hello world".into())));

    msg.set_optional_string("accessors_test");
    assert_that!(msg.optional_string(), eq("accessors_test"));
    assert_that!(msg.optional_string_opt(), eq(Optional::Set("accessors_test".into())));

    msg.set_optional_string("");
    assert_that!(*msg.optional_string().as_bytes(), empty());
    assert_that!(msg.optional_string_opt(), eq(Optional::Set("".into())));
}

#[test]
fn test_nested_enum_accessors() {
    use TestAllTypes_::NestedEnum;

    let mut msg = TestAllTypes::new();
    assert_that!(msg.optional_nested_enum(), eq(NestedEnum::Zero));

    msg.set_optional_nested_enum(NestedEnum::Baz);
    assert_that!(msg.optional_nested_enum(), eq(NestedEnum::Baz));

    msg.set_optional_nested_enum(NestedEnum::default());
    assert_that!(msg.optional_nested_enum(), eq(NestedEnum::Zero));
}

#[test]
fn test_optional_nested_enum_accessors() {
    use TestProto3Optional_::NestedEnum;

    let mut msg = TestProto3Optional::new();
    assert_that!(msg.optional_nested_enum(), eq(NestedEnum::Unspecified));
    assert_that!(msg.optional_nested_enum_opt(), eq(Optional::Unset(NestedEnum::Unspecified)));

    msg.set_optional_nested_enum(NestedEnum::Baz);
    assert_that!(msg.optional_nested_enum_opt(), eq(Optional::Set(NestedEnum::Baz)));
    assert_that!(msg.optional_nested_enum(), eq(NestedEnum::Baz));

    msg.set_optional_nested_enum(NestedEnum::default());
    assert_that!(msg.optional_nested_enum(), eq(NestedEnum::Unspecified));
    assert_that!(msg.optional_nested_enum_opt(), eq(Optional::Set(NestedEnum::Unspecified)));
}

#[test]
fn test_foreign_enum_accessors() {
    use unittest_proto3::ForeignEnum;

    let mut msg = TestAllTypes::new();
    assert_that!(msg.optional_foreign_enum(), eq(ForeignEnum::ForeignZero));

    msg.set_optional_foreign_enum(ForeignEnum::ForeignBaz);
    assert_that!(msg.optional_foreign_enum(), eq(ForeignEnum::ForeignBaz));

    msg.set_optional_foreign_enum(ForeignEnum::default());
    assert_that!(msg.optional_foreign_enum(), eq(ForeignEnum::ForeignZero));
}

#[test]
fn test_oneof_accessors() {
    use TestAllTypes_::OneofField::*;

    let mut msg = TestAllTypes::new();
    assert_that!(msg.oneof_field(), matches_pattern!(not_set(_)));

    msg.set_oneof_uint32(7);
    assert_that!(msg.oneof_uint32_opt(), eq(Optional::Set(7)));
    assert_that!(msg.oneof_field(), matches_pattern!(OneofUint32(eq(7))));

    msg.clear_oneof_uint32();
    assert_that!(msg.oneof_uint32_opt(), eq(Optional::Unset(0)));
    assert_that!(msg.oneof_field(), matches_pattern!(not_set(_)));

    msg.oneof_nested_message_mut().or_default(); // Cause the nested_message field to become set.

    assert_that!(msg.oneof_bytes_opt(), matches_pattern!(Optional::Unset(_)));
    assert_that!(msg.oneof_field(), matches_pattern!(OneofNestedMessage(_)));

    msg.set_oneof_uint32(7);
    msg.set_oneof_bytes(b"123");
    assert_that!(msg.oneof_uint32_opt(), eq(Optional::Unset(0)));
    assert_that!(msg.oneof_field(), matches_pattern!(OneofBytes(eq(b"123"))));

    msg.clear_oneof_bytes();
    assert_that!(msg.oneof_field(), matches_pattern!(not_set(_)));
}

#[test]
fn test_oneof_accessors_view_long_lifetime() {
    use TestAllTypes_::OneofField::*;

    let mut msg = TestAllTypes::new();
    msg.set_oneof_uint32(7);

    // The oneof-view accessor on MsgViews should maintain the longest lifetime (can
    // outlive the message view).
    let oneof = {
        let view = msg.as_view();
        view.oneof_field()
    };
    assert_that!(oneof, matches_pattern!(OneofUint32(eq(7))));
}

#[test]
fn test_oneof_enum_accessors() {
    use unittest_proto3::{
        TestOneof2,
        TestOneof2_::{Foo, FooCase, NestedEnum},
    };

    let mut msg = TestOneof2::new();
    assert_that!(msg.foo_enum_opt(), eq(Optional::Unset(NestedEnum::Unknown)));
    assert_that!(msg.foo(), matches_pattern!(Foo::not_set(_)));
    assert_that!(msg.foo_case(), matches_pattern!(FooCase::not_set));

    msg.set_foo_enum(NestedEnum::Bar);
    assert_that!(msg.foo_enum_opt(), eq(Optional::Set(NestedEnum::Bar)));
    assert_that!(msg.foo(), matches_pattern!(Foo::FooEnum(eq(NestedEnum::Bar))));
    assert_that!(msg.foo_case(), matches_pattern!(FooCase::FooEnum));
}

#[test]
fn test_submsg_setter() {
    use TestAllTypes_::*;

    let mut nested = NestedMessage::new();
    nested.set_bb(7);

    let mut parent = TestAllTypes::new();
    parent.set_optional_nested_message(nested);

    assert_that!(parent.optional_nested_message().bb(), eq(7));
}
