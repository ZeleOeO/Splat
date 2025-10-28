#[derive(Debug)]
pub enum BillsStatus {
    Open,
    Paid,
    Cancelled
}

impl ToString for BillsStatus  {
    fn to_string(&self) -> String {
        match self {
            BillsStatus::Open => "open".to_string(),
            BillsStatus::Paid => "paid".to_string(),
            BillsStatus::Cancelled => "cancelled".to_string(),
        }
    }
}
