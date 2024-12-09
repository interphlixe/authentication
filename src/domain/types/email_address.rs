use lettre::Address;


pub enum EmailAddress {
    New(Address),
    Verified(Address)
}