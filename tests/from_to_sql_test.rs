use diesel_sqltype_enum_pg::FromToSql;
use strum_macros::Display;

use diesel::deserialize::FromSql;
use strum_macros::EnumString;

struct EnumStructSqlType;

#[derive(Debug, PartialEq, EnumString, Display, FromToSql)]
#[fromtosql(sql_type = EnumStructSqlType)]
enum EnumStruct {
    EnumOne,
    EnumTwo,
}

#[test]
fn test_truth() {
    assert!(true);
}
