#[cfg(test)]
mod tests {

    use std::str::FromStr;
    use sui_literals::sui_literal;
    use sui_types::base_types::{ObjectID, SuiAddress};

    #[test]
    fn test_object_id() {
        let object_id =
            sui_literal!(0x01b0d52321ce82d032430f859c6df0c52eb9ce1a337a81d56d89445db2d624f0_object);
        println!("`object_id`: {:?}", object_id);
        let expected_object_id = ObjectID::from_str(
            "0x01b0d52321ce82d032430f859c6df0c52eb9ce1a337a81d56d89445db2d624f0",
        )
        .unwrap();
        assert_eq!(expected_object_id, object_id);
    }

    #[test]
    fn test_sui_address() {
        let sui_address = sui_literal!(
            0x01b0d52321ce82d032430f859c6df0c52eb9ce1a337a81d56d89445db2d624f0_address
        );
        println!("`sui_address`: {:?}", sui_address);
        let expected_object_id = ObjectID::from_str(
            "0x01b0d52321ce82d032430f859c6df0c52eb9ce1a337a81d56d89445db2d624f0",
        )
        .unwrap();
        let expected_sui_address = SuiAddress::from(expected_object_id);
        assert_eq!(expected_sui_address, sui_address);
    }
}
