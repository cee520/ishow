
pub mod user;
pub mod binance;
// use diesel::expression::{Expression, AsExpression};
// use diesel::pg::Pg;
// use diesel::types::Text;

// type SqlType = <AllColumns as Expression>::SqlType;
// type BoxedQuery<'a> = crates::BoxedQuery<'a, Pg, SqlType>;

// impl Crate {
//     fn all() -> BoxedQuery<'static> {
//         crates::table().select(ALL_COLUMNS).into_boxed()
//     }

//     fn by_name<'a, T>(name: T) -> BoxedQuery<'a>
//     where
//         T: AsExpression<Text>,
//         T::Expression: BoxableExpression<crates::table, Pg>,
//     {
//         Self::all().filter(by_name(name))
//     }
// }