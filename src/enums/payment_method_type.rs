#[derive(Debug)]
enum PaymentMethodType {
    CreditCard(String),
    DebitCard(String),
    PayPal(String, String),
    PayPalV2(Credentials),                           // concrete struct type
    PayPalV3 { username: String, password: String }, // enum struct variant
}

#[derive(Debug)]
struct Credentials {
    username: String,
    password: String,
}

pub fn index(show: bool) {
    if show {
        let visa = PaymentMethodType::CreditCard(String::from("1234-5678-9012-3456"));
        let mastercard = PaymentMethodType::DebitCard(String::from("0987-6543-3210-9876"));
        let paypal = PaymentMethodType::PayPal(
            String::from("user@example.com"),
            String::from("fake-password"),
        );
        let paypal_v2 = PaymentMethodType::PayPalV2(Credentials {
            username: String::from("user@example.com"),
            password: String::from("fake-password"),
        });
        let paypal_v3 = PaymentMethodType::PayPalV3 {
            username: String::from("user@example.com"),
            password: String::from("fake-password"),
        };

        println!("visa: {:?}", visa);
        println!("mastercard: {:?}", mastercard);
        println!("paypal: {:?}", paypal);
        println!("paypal v2: {:?}", paypal_v2);
        println!("paypal v3: {:?}", paypal_v3);

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

        if let PaymentMethodType::PayPalV2(credentials) = paypal_v2 {
            println!("paypal_v2 email: {:?}", credentials.username);
            println!("paypal_v2 password: {:?}", credentials.password);
        }

        if let PaymentMethodType::PayPalV3 { username, password } = paypal_v3 {
            println!("paypal_v3 email: {:?}", username);
            println!("paypal_v3 password: {:?}", password);
        }
    }
}
