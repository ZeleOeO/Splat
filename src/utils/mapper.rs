use crate::{
    dto::dto::{BillsDTO, UserDTO},
    entities::{bills::Model as Bill, user::Model as User},
};

pub fn user_to_userdto(user: &User) -> UserDTO {
    UserDTO {
        user_name: user.clone().user_name,
        hashed_password: user.clone().hashed_password,
        email: user.clone().email,
        first_name: user.clone().first_name,
        last_name: user.clone().last_name,
        created_at: user.clone().created_at.to_string(),
    }
}

pub fn bill_to_billdto(bill: &Bill) -> BillsDTO {
    BillsDTO {
        title: bill.clone().title,
        description: bill.clone().description,
        total_amount: bill.clone().total_amount,
        status: bill.clone().status,
        category: bill.clone().category,
        created_at: bill.clone().created_at.to_string(),
        due_date: bill.clone().due_date.to_string()
    }
}
