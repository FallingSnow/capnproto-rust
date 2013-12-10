/*
 * Copyright (c) 2013, David Renshaw (dwrenshaw@gmail.com)
 *
 * See the LICENSE file in the capnproto-rust root directory.
 */

#[feature(globs)];

#[link(name = "test", vers = "alpha", author = "dwrensha")];

#[crate_type = "bin"];

extern mod capnp;

pub mod test_capnp;

#[test]
fn testPrimList () {
    use capnp::message::*;
    use test_capnp::*;

    // Make the first segment small to force allocation of a second segment.
    MessageBuilder::new(50,
                        SUGGESTED_ALLOCATION_STRATEGY,
                        |message| {

        let testPrimList = message.init_root::<TestPrimList::Builder>();

        let uint8_list = testPrimList.init_uint8_list(100);

        for i in range(0, uint8_list.size()) {
            uint8_list.set(i, i as u8);
        }

        let uint64List = testPrimList.init_uint64_list(20);

        for i in range(0, uint64List.size()) {
            uint64List.set(i, i as u64);
        }

        let boolList = testPrimList.init_bool_list(65);

        boolList.set(0, true);
        boolList.set(1, true);
        boolList.set(2, true);
        boolList.set(3, true);
        boolList.set(5, true);
        boolList.set(8, true);
        boolList.set(13, true);
        boolList.set(64, true);

        assert!(boolList[0]);
        assert!(!boolList[4]);
        assert!(!boolList[63]);
        assert!(boolList[64]);

        let voidList = testPrimList.init_void_list(1025);
        voidList.set(257, ());

        testPrimList.as_reader(|testPrimListReader| {
            let uint8List = testPrimListReader.get_uint8_list();
            for i in range(0, uint8List.size()) {
                assert!(uint8List[i] == i as u8);
            }
            let uint64List = testPrimListReader.get_uint64_list();
            for i in range(0, uint64List.size()) {
                 assert!(uint64List[i] == i as u64);
            }

            let boolList = testPrimListReader.get_bool_list();
            assert!(boolList[0]);
            assert!(boolList[1]);
            assert!(boolList[2]);
            assert!(boolList[3]);
            assert!(!boolList[4]);
            assert!(boolList[5]);
            assert!(!boolList[6]);
            assert!(!boolList[7]);
            assert!(boolList[8]);
            assert!(!boolList[9]);
            assert!(!boolList[10]);
            assert!(!boolList[11]);
            assert!(!boolList[12]);
            assert!(boolList[13]);
            assert!(!boolList[63]);
            assert!(boolList[64]);

            assert!(testPrimListReader.get_void_list().size() == 1025);
        });
    });
}

#[test]
fn testBigStruct() {

    use capnp::message::*;
    use test_capnp::*;

    // Make the first segment small to force allocation of a second segment.
    MessageBuilder::new(5,
                        SUGGESTED_ALLOCATION_STRATEGY,
                        |message| {

        let bigStruct = message.init_root::<BigStruct::Builder>();

        bigStruct.set_bool_field(false);
        bigStruct.set_int8_field(-128);
        bigStruct.set_int16_field(0);
        bigStruct.set_int32_field(1009);

        let inner = bigStruct.init_struct_field();
        inner.set_float64_field(0.1234567);

        inner.set_bool_field_b(true);

        bigStruct.set_bool_field(true);

        bigStruct.as_reader(|bigStructReader| {
            assert!(bigStructReader.get_int8_field() == -128);
            assert!(bigStructReader.get_int32_field() == 1009);

            let innerReader = bigStructReader.get_struct_field();
            assert!(!innerReader.get_bool_field_a());
            assert!(innerReader.get_bool_field_b());
            assert!(innerReader.get_float64_field() == 0.1234567);
        });
    });
}

#[test]
fn testComplexList () {
    use capnp::message::*;
    use test_capnp::*;

    MessageBuilder::new_default(|message| {

        let testComplexList = message.init_root::<TestComplexList::Builder>();

        let enumList = testComplexList.init_enum_list(100);

        for i in range::<uint>(0, 10) {
            enumList.set(i, AnEnum::Qux);
        }
        for i in range::<uint>(10, 20) {
            enumList.set(i, AnEnum::Bar);
        }

        testComplexList.as_reader(|complexListReader| {
            let enumListReader = complexListReader.get_enum_list();
            for i in range::<uint>(0,10) {
                match enumListReader[i] {
                    Some(AnEnum::Qux) => {}
                    _ => fail!()
                }
            }
            for i in range::<uint>(10,20) {
                match enumListReader[i] {
                    Some(AnEnum::Bar) => {}
                    _ => fail!()
                }
            }
        });
    });
}


fn main () {
}
