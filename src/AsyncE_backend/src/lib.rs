#![allow(non_snake_case)]
#[ic_cdk::query]
pub fn login() -> String {
    let id = ic_cdk::api::caller();
    ic_cdk::println!("id: {}", id);
    format!("{}", id)
}
