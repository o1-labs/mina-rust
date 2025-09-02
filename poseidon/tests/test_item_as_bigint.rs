use poseidon::hash::Item;

#[test]
fn test_item_bool_as_bigint() {
    let item_true = Item::Bool(true);
    let item_false = Item::Bool(false);

    assert_eq!(item_true.as_bigint(), 1);
    assert_eq!(item_false.as_bigint(), 0);
}

#[test]
fn test_item_u2_as_bigint() {
    let item_0 = Item::U2(0);
    let item_1 = Item::U2(1);
    let item_2 = Item::U2(2);
    let item_3 = Item::U2(3);

    assert_eq!(item_0.as_bigint(), 0);
    assert_eq!(item_1.as_bigint(), 1);
    assert_eq!(item_2.as_bigint(), 2);
    assert_eq!(item_3.as_bigint(), 3);
}

#[test]
fn test_item_u8_as_bigint() {
    let item_0 = Item::U8(0);
    let item_max = Item::U8(u8::MAX);
    let item_mid = Item::U8(128);

    assert_eq!(item_0.as_bigint(), 0);
    assert_eq!(item_max.as_bigint(), 255);
    assert_eq!(item_mid.as_bigint(), 128);
}

#[test]
fn test_item_u32_as_bigint() {
    let item_0 = Item::U32(0);
    let item_max = Item::U32(u32::MAX);
    let item_mid = Item::U32(0x80000000);

    assert_eq!(item_0.as_bigint(), 0);
    assert_eq!(item_max.as_bigint(), u32::MAX as u64);
    assert_eq!(item_mid.as_bigint(), 0x80000000);
}

#[test]
fn test_item_u64_as_bigint() {
    let item_0 = Item::U64(0);
    let item_max = Item::U64(u64::MAX);
    let item_mid = Item::U64(0x8000000000000000);

    assert_eq!(item_0.as_bigint(), 0);
    assert_eq!(item_max.as_bigint(), u64::MAX);
    assert_eq!(item_mid.as_bigint(), 0x8000000000000000);
}

#[test]
fn test_item_u48_as_bigint() {
    // Test with all zeros
    let item_zeros = Item::U48([0, 0, 0, 0, 0, 0]);
    assert_eq!(item_zeros.as_bigint(), 0);

    // Test with all ones (max 48-bit value)
    let item_max = Item::U48([0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    // 48 bits all set = 0xFFFFFFFFFFFF
    assert_eq!(item_max.as_bigint(), 0xFFFFFFFFFFFF);

    // Test with specific pattern - little-endian bytes
    let item_pattern = Item::U48([0x01, 0x02, 0x03, 0x04, 0x05, 0x06]);
    // In big-endian format: 0x060504030201
    assert_eq!(item_pattern.as_bigint(), 0x060504030201);

    // Test with high byte set
    let item_high = Item::U48([0x00, 0x00, 0x00, 0x00, 0x00, 0x80]);
    assert_eq!(item_high.as_bigint(), 0x800000000000);
}

#[test]
fn test_item_u48_edge_cases() {
    // Test powers of 2
    let item_1 = Item::U48([0x01, 0x00, 0x00, 0x00, 0x00, 0x00]);
    assert_eq!(item_1.as_bigint(), 1);

    let item_256 = Item::U48([0x00, 0x01, 0x00, 0x00, 0x00, 0x00]);
    assert_eq!(item_256.as_bigint(), 0x0100);

    let item_65536 = Item::U48([0x00, 0x00, 0x01, 0x00, 0x00, 0x00]);
    assert_eq!(item_65536.as_bigint(), 0x010000);

    // Test byte boundary values
    let item_boundary = Item::U48([0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00]);
    assert_eq!(item_boundary.as_bigint(), 0x00FF00FF00FF);
}

#[test]
fn test_item_nbits() {
    assert_eq!(Item::Bool(true).nbits(), 1);
    assert_eq!(Item::Bool(false).nbits(), 1);

    assert_eq!(Item::U2(0).nbits(), 2);
    assert_eq!(Item::U2(3).nbits(), 2);

    assert_eq!(Item::U8(0).nbits(), 8);
    assert_eq!(Item::U8(255).nbits(), 8);

    assert_eq!(Item::U32(0).nbits(), 32);
    assert_eq!(Item::U32(u32::MAX).nbits(), 32);

    assert_eq!(Item::U48([0; 6]).nbits(), 48);
    assert_eq!(Item::U48([0xFF; 6]).nbits(), 48);

    assert_eq!(Item::U64(0).nbits(), 64);
    assert_eq!(Item::U64(u64::MAX).nbits(), 64);
}

#[test]
fn test_item_u48_byte_order() {
    // Test that U48 uses correct byte order interpretation
    // The bytes should be interpreted as: [byte0, byte1, byte2, byte3, byte4, byte5]
    // And converted to big-endian u64

    let test_cases = vec![
        ([0x01, 0x00, 0x00, 0x00, 0x00, 0x00], 0x000000000001u64),
        ([0x00, 0x01, 0x00, 0x00, 0x00, 0x00], 0x000000000100u64),
        ([0x00, 0x00, 0x01, 0x00, 0x00, 0x00], 0x000000010000u64),
        ([0x00, 0x00, 0x00, 0x01, 0x00, 0x00], 0x000001000000u64),
        ([0x00, 0x00, 0x00, 0x00, 0x01, 0x00], 0x000100000000u64),
        ([0x00, 0x00, 0x00, 0x00, 0x00, 0x01], 0x010000000000u64),
    ];

    for (bytes, expected) in test_cases {
        let item = Item::U48(bytes);
        assert_eq!(
            item.as_bigint(),
            expected,
            "U48({:02X?}) should equal 0x{:012X}",
            bytes,
            expected
        );
    }
}
