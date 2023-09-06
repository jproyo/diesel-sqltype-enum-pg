![Build](https://github.com/jproyo/diesel-sqltype-enum-pg/actions/workflows/build.yml/badge.svg)
![Release](https://github.com/jproyo/diesel-sqltype-enum-pg/actions/workflows/release.yml/badge.svg)
![crates.io](https://img.shields.io/crates/v/diesel_sqltype_enum_pg.svg)

[API Doc](https://docs.rs/diesel_sqltype_enum_pg/0.1.0/diesel-sqltype-enum-pg/)

# Diesel Enum Derivation for `FromSql` and `ToSql`

This procedural macro simplifies the process of generating Diesel `FromSql` and `ToSql` instances for enums with a specific `SqlType`. It is designed for use with Diesel and Postgres, utilizing `ToString` and `FromStr` traits as the base for conversion. This macro generates the necessary boilerplate code for handling enum conversions between Rust and SQL.

## Table of Contents
- [Usage](#usage)
- [Example](#example)
- [Enums and SqlTypes](#enums-and-sqltypes)
- [Dependencies](#dependencies)

---

## Usage

To use this procedural macro, you'll need to add the `enum_diesel_macros` crate to your project's dependencies in `Cargo.toml`. You can then derive the necessary implementations for your enums. Below is an example of how to use the `FromToSql` macro:

```rust
#[derive(Debug, PartialEq, EnumString, Display, FromToSql)]
#[fromtosql(sql_type = MyEntityEnumSqlType)]
enum MyEntityEnum {
    #[strum(serialize = "ONE")]
    EnumOne,
    #[strum(serialize = "TWO")]
    EnumTwo,
}
```

Make sure to import the required dependencies and modules as shown in the example.

## Example
Here's a step-by-step example of how to use the `FromToSql` macro to generate Diesel `FromSql` and `ToSql` instances for your **enum**:

1. Define your custom SQL type using Diesel's `SqlType` and annotate it with diesel attributes:

```rust
pub mod schema {
    pub mod sql_types {
        #[derive(diesel::sql_types::SqlType)]
        #[diesel(postgres_type(name = "myentittyenumsqltype", schema = "myschema"))]
        pub struct MyEntityEnumSqlType;
    }
}
```

2. Import the necessary Diesel and procedural macro dependencies:

```rust
#[derive(Debug, PartialEq, EnumString, Display, FromToSql)]
#[fromtosql(sql_type = MyEntityEnumSqlType)]
enum MyEntityEnum {
    #[strum(serialize = "ONE")]
    EnumOne,
    #[strum(serialize = "TWO")]
    EnumTwo,
}
```

3. Now, you can use your `MyEntityEnum` enum seamlessly with Diesel queries.

## Enums and SqlTypes
The key to using this macro is to ensure that you have a corresponding `SqlType` defined for your **enum**. In this example, `MyEntityEnumSqlType` represents the SQL type for the `MyEntityEnum` **enum**. This `SqlType` is generated in Diesel schemas and serves as the companion **enum** to `MyEntityEnum`.

Ensure that your **enum** also implements either the `FromStr` or `EnumString` trait, which is crucial for parsing the **enum** from SQL values.

## Dependencies
To use this procedural macro, make sure to include the following dependencies in your `Cargo.toml`:

```toml
[dependencies]
diesel = { version = "2.1", features = ["postgres"] }
strum = "0.25"
strum_macros = "0.25"
diesel-sqltype-enum-pg = "0.1.0"
```


