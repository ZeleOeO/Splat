use crate::{
    dto::dto::{BilleeDTO, BillsDTO, UserDTO},
    entities::{billee::Model as Billee, bills::Model as Bill, user::Model as User},
};

pub fn user_to_userdto(user: &User) -> UserDTO {
    UserDTO {
        unique_id: user.clone().unique_id,
        user_name: user.clone().user_name,
        email: user.clone().email,
        first_name: user.clone().first_name,
        last_name: user.clone().last_name,
        created_at: user.clone().created_at.to_string(),
    }
}

pub fn bill_to_billdto(bill: &Bill) -> BillsDTO {
    BillsDTO {
        unique_id: bill.clone().unique_id,
        title: bill.clone().title,
        description: bill.clone().description,
        total_amount: bill.clone().total_amount,
        status: bill.clone().status,
        category: bill.clone().category,
        created_at: bill.clone().created_at.to_string(),
        due_date: bill.clone().due_date.to_string(),
    }
}

pub fn billee_to_billeedto(billee: &Billee) -> BilleeDTO {
    BilleeDTO {
        name: billee.clone().name,
        percentage: billee.clone().percentage,
        amount_due: billee.clone().amount_due,
        amount_paid: billee.clone().amount_paid,
        status: billee.clone().status,
        user_id: billee.clone().user_id,
        bill_id: billee.clone().bill_id,
        payment_link: billee.clone().payment_link,
    }
}
