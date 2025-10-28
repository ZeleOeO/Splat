#[derive(Debug)]
pub enum BillsStatus {
    Open,
    Paid,
    Cancelled,
}

pub enum BilleeStatus {
    Pending,
    Paid,
    Partial,
}

impl ToString for BillsStatus {
    fn to_string(&self) -> String {
        match self {
            BillsStatus::Open => "open".to_string(),
            BillsStatus::Paid => "paid".to_string(),
            BillsStatus::Cancelled => "cancelled".to_string(),
        }
    }
}

impl ToString for BilleeStatus {
    fn to_string(&self) -> String {
        match self {
            BilleeStatus::Pending => "pending".to_string(),
            BilleeStatus::Paid => "paid".to_string(),
            BilleeStatus::Partial => "partial".to_string(),
        }
    }
}
