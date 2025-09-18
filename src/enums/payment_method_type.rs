#[derive(Debug)]
enum PaymentMethodType {
    CreditCard(String),
    DebitCard(String),
    PayPal(String, String),
}

pub fn index(show: bool) {
    if show {
        let visa = PaymentMethodType::CreditCard(String::from("1234-5678-9012-3456"));
        let mastercard = PaymentMethodType::DebitCard(String::from("0987-6543-3210-9876"));
        let paypal = PaymentMethodType::PayPal(
            String::from("user@example.com"),
            String::from("fake-password"),
        );

        println!("visa: {:?}", visa);
        println!("mastercard: {:?}", mastercard);
        println!("paypal: {:?}", paypal);

        if let PaymentMethodType::CreditCard(visa_number) = visa {
            println!("visa number: {:?}", visa_number);
        }

        if let PaymentMethodType::DebitCard(mastercard_number) = mastercard {
            println!("mastercard number: {:?}", mastercard_number);
        }

        if let PaymentMethodType::PayPal(email, password) = paypal {
            println!("paypal email: {:?}", email);
            println!("paypal password: {:?}", password);
        }
    }
}
