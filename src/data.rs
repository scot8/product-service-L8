use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Contoso Ultra HD 4K TV".to_string(),
            price: 499.99,
            description: "Experience stunning picture quality with the Contoso Ultra HD 4K TV. Featuring vibrant colors, crisp details, and a sleek design to enhance your living room.".to_string(),
            image: "/4ktv.jpg".to_string()
        },
        Product {
            id: 2,
            name: "Contoso Wireless Noise-Canceling Headphones".to_string(),
            price: 199.99,
            description: "Immerse yourself in pure audio with Contoso Wireless Noise-Canceling Headphones. Enjoy up to 30 hours of battery life and superior sound quality.".to_string(),
            image: "/headphones.jpg".to_string()
        },
        Product {
            id: 3,
            name: "Contoso Smart Home Hub".to_string(),
            price: 89.99,
            description: "Control your smart home devices seamlessly with the Contoso Smart Home Hub. Compatible with major brands, offering voice control and easy setup.".to_string(),
            image: "/smarthub.jpg".to_string()
        },
        Product {
            id: 4,
            name: "Contoso Gaming Laptop".to_string(),
            price: 999.99,
            description: "Level up your gaming experience with the Contoso Gaming Laptop. Powered by the latest graphics technology and a high-refresh-rate display.".to_string(),
            image: "/gaminglaptop.jpg".to_string()
        },
        Product {
            id: 6,
            name: "Contoso Smartwatch Pro".to_string(),
            price: 249.99,
            description: "Stay connected with the Contoso Smartwatch Pro. Track your fitness, receive notifications, and customize your watch face.".to_string(),
            image: "/smartwatch.jpg".to_string()
        },
        Product {
            id: 7,
            name: "Contoso 1TB External SSD".to_string(),
            price: 119.99,
            description: "Store your files securely with the Contoso 1TB External SSD. Fast transfer speeds and a durable design for on-the-go use.".to_string(),
            image: "/externalssd.jpg".to_string()
        },
        Product {
            id: 8,
            name: "Contoso Wireless Keyboard and Mouse Combo".to_string(),
            price: 39.99,
            description: "Enhance your productivity with the Contoso Wireless Keyboard and Mouse Combo. Ergonomic design and long battery life for all-day comfort.".to_string(),
            image: "/keyboardmouse.jpg".to_string()
        },
        Product {
            id: 9,
            name: "Contoso Home Security Camera".to_string(),
            price: 129.99,
            description: "Keep your home secure with the Contoso Home Security Camera. Features HD video, night vision, and cloud storage support.".to_string(),
            image: "/securitycamera.jpg".to_string()
        },
        Product {
            id: 10,
            name: "Contoso Fast Charging Power Bank".to_string(),
            price: 29.99,
            description: "Stay powered on the go with the Contoso Fast Charging Power Bank. Compact design with multiple ports and fast charging technology.".to_string(),
            image: "/powerbank.jpg".to_string()
        }
    ]
}