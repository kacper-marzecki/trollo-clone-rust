

// pub fn create(register_user: RegisterUser, connection: &PgConnection) -> Result<User, MyStoreError> {
//     Ok(diesel::insert_into(users::table)
//         .values(NewUser {
//             email: register_user.email,
//             company: register_user.company,
//             password: Self::hash_password(register_user.password)?,
//             created_at: Local::now().naive_local()
//         })
//         .get_result(connection)?)
// }
//
// // This might look kind of weird,
// // but if something fails it would chain
// // to our MyStoreError Error,
// // otherwise it will gives us the hash,
// // we still need to return a result
// // so we wrap it in an Ok variant from the Result type.
// pub fn hash_password(plain: String) -> Result<String, > {
//     Ok(hash(plain, DEFAULT_COST)?)
// }