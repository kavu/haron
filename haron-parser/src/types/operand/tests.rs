use super::{operands, Operand};

#[test]
fn parse_two_operands() {
    let operand_a = Some(Operand {
        mode: None,
        value: 0,
    });

    let operand_b = Some(Operand {
        mode: None,
        value: -1,
    });

    let expected = (operand_a, operand_b);

    match operands("0, -1".into()) {
        Ok((_, result)) => assert_eq!(expected, result),
        Err(err) => panic!(format!("{}", err)),
    }
}

#[test]
fn parse_one_operand() {
    let operand_a = Some(Operand {
        mode: None,
        value: 1,
    });
    let operand_b = None;
    let expected = (operand_a, operand_b);

    match operands("1".into()) {
        Ok((_, result)) => assert_eq!(expected, result),
        Err(err) => panic!(format!("{}", err)),
    }
}

#[test]
fn parse_invalid_number_of_operands() {
    let operand_a = Some(Operand {
        mode: None,
        value: -1,
    });
    let operand_b = Some(Operand {
        mode: None,
        value: 1222,
    });
    let expected = (operand_a, operand_b);

    let (_, result) = operands("-1, 1222, 2".into()).unwrap();

    assert_eq!(result, expected);
}

#[test]
fn parse_only_b_operand() {
    let operand_b = Some(Operand {
        mode: None,
        value: 1222,
    });
    let expected = (None, operand_b);

    let (_, result) = operands(", 1222".into()).unwrap();

    assert_eq!(result, expected);
}
