#[derive(Debug)]
enum PaymentMethodType {
    CreditCard(String),
    DebitCard(String),
    PayPal(String),
}

pub fn index(show: bool) {
    if show {
        let visa = PaymentMethodType::CreditCard(String::from("1234-5678-9012-3456"));
        let mastercard = PaymentMethodType::DebitCard(String::from("0987-6543-3210-9876"));
        let paypal = PaymentMethodType::PayPal(String::from("6789-0123-4567-8901"));

        println!("visa: {:?}", visa);
        println!("mastercard: {:?}", mastercard);
        println!("paypal: {:?}", paypal);

        if let PaymentMethodType::CreditCard(visa_number) = visa {
            println!("visa number: {:?}", visa_number);
        }

        if let PaymentMethodType::DebitCard(mastercard_number) = mastercard {
            println!("mastercard number: {:?}", mastercard_number);
        }

        if let PaymentMethodType::PayPal(paypal_number) = paypal {
            println!("paypal number: {:?}", paypal_number);
        }
    }
}
