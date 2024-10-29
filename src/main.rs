#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }
}


//en esta linea estamos creando una &mut que va a ser llamada desde algun lugar.
fn set_account_id(account: &mut Account, id: u32) { // Line 1
    account.id = id;
}
 
fn main() {
    let mut account = Account::new(1, String::from("me"));
    //en esta linea creamos &mut 
    set_account_id(&mut account, 3); // Line 8
 
    println!("Account id: {}", account.id);
}