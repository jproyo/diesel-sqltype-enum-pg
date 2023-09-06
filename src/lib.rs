#![doc = include_str!("../README.md")]
use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Eq;
use syn::{parse_macro_input, DataEnum, DeriveInput};

/// This procedural Macro generates diesel FromSql and ToSql Instances for Enum
///
/// # Example howto use `FromToSql`
/// ```
///
/// pub mod schema {
///     pub mod sql_types {
///         #[derive(diesel::sql_types::SqlType)]
///         #[diesel(postgres_type(name = "myentittyenumsqltype", schema = "myschema"))]
///         pub struct MyEntityEnumSqlType;
///     }
/// }
/// use diesel::deserialize::{self, FromSql};
/// use diesel::pg::{Pg, PgValue};
/// use diesel::serialize::{IsNull, Output, ToSql};
/// use diesel::sql_types::Text;
/// use proc_macro::TokenStream;
/// use quote::quote;
/// use std::io::Write;
/// use std::str::FromStr;
/// use enum_diesel_macros::FromToSql;
/// use schema::sql_types::MyEntityEnumSqlType;
/// use strum_macros::{Display, EnumString};
///
/// #[derive(Debug, PartialEq, EnumString, Display, FromToSql)]
/// #[fromtosql(sql_type = MyEntityEnumSqlType)]
/// enum MyEntityEnum {
///     #[strum(serialize = "ONE")]
///     EnumOne,
///     #[strum(serialize = "TWO")]
///     EnumTwo,
/// }
/// ```
/// As you can see in the above Example, [`MyEntityEnumSqlType`] is the enum type generated in
/// diesel schemas, therefore is the companion enum of [`MyEntityEnum`]
///
/// [`MyEntityEnum`] should either implement [`FromStr`] or [`EnumString`] which is better if you
/// can.
///
#[proc_macro_derive(FromToSql, attributes(fromtosql))]
pub fn describe(input: TokenStream) -> TokenStream {
    let DeriveInput {
        data, ident, attrs, ..
    } = parse_macro_input!(input);

    match data {
        syn::Data::Enum(DataEnum { .. }) => {}
        _ => {
            panic!("Only supported for enum type")
        }
    };

    let binding = attrs
        .iter()
        .filter(|a| a.path().is_ident("fromtosql"))
        .flat_map(|a| {
            let mut p = Vec::new();
            let parser = a
                .parse_args_with(Punctuated::<syn::Ident, Eq>::parse_separated_nonempty)
                .unwrap();
            let ident = parser.first().cloned().map(|f| f.to_string());
            let value = parser.last().cloned();
            if let Some("sql_type") = ident.as_deref() {
                if let Some(value) = value {
                    p.push(value)
                }
            }
            p
        })
        .collect::<Vec<_>>();

    let att = match binding.first() {
        Some(idnt) => idnt,
        None => panic!("`companion` attribute not found"),
    };

    let output = quote! {
         impl ::diesel::serialize::ToSql<#att, ::diesel::pg::Pg> for #ident {
             fn to_sql<'b>(&'b self, out: &mut ::diesel::serialize::Output<'b, '_, ::diesel::pg::Pg>) -> ::diesel::serialize::Result {
                 use ::std::io::Write;
                 out.write_all(self.to_string().as_bytes())?;
                 Ok(::diesel::serialize::IsNull::No)
             }
         }

         impl FromSql<#att, ::diesel::pg::Pg> for #ident {
            fn from_sql(bytes: ::diesel::pg::PgValue) -> ::diesel::deserialize::Result<Self> {
                use ::std::str::FromStr;
                let value: String = <String as FromSql<::diesel::sql_types::Text, ::diesel::pg::Pg>>::from_sql(bytes)?;
                #ident::from_str(value.as_str())
                    .map_err(|e| format!("Error converting from PgValue {:?}", e).into())
            }
        }
    };

    output.into()
}
